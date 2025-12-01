# bitcoin-mem

Precise, low‑overhead accounting of heap usage for Bitcoin-style data structures.

This crate is a small, focused port of Bitcoin Core's `memusage.h` logic to Rust. It exposes a family of traits for estimating dynamic (heap) memory consumption of values and containers, together with helpers that mirror the C++ allocator bucketing model.

---

## High-level overview

`bitcoin-mem` is designed for systems where memory is a first‑class resource: nodes, indexers, and services whose correctness and robustness depend on staying within strict memory budgets. Instead of using coarse metrics like "length of vector" or "number of map entries", this crate estimates heap usage in **bytes**, closely following what would be charged by a `malloc`/`free` allocator with bucketed size classes, as done in Bitcoin Core.

Key ideas:

* **Dynamic vs. recursive usage**: distinguish between the bytes owned directly by a value/container and bytes owned by its transitive children.
* **Incremental usage**: estimate the *marginal* allocation cost of inserting one additional element into a container.
* **Allocator bucketing**: `malloc_usage` accounts for allocator rounding/overhead using fixed bucket sizes (8‑ or 16‑byte)
  depending on pointer width.

This enables predictable memory profiling and budget enforcement for complex object graphs without requiring a global allocator hook.

---

## Core traits

### `DynamicUsage`

```rust
pub trait DynamicUsage {
    fn dynamic_usage(&self) -> usize;
}
```

`DynamicUsage` measures **owned heap memory directly associated with the value**, ignoring any children unless specified otherwise.

Examples:

* For primitive scalars (`i32`, `u64`, `f32`, etc.), `dynamic_usage` is always `0` (they live on the stack or inside other allocations).
* For `Vec<T>`, `dynamic_usage` returns the heap capacity in bytes for the underlying buffer, accounting for allocator bucket rounding via `malloc_usage`.
* For `HashMap`, `HashSet`, `Arc`, `Box`, `Option`, and various bitcoin-rs abstractions, `dynamic_usage` approximates the heap footprint of the container or handle, ignoring nested contents unless clearly part of the same allocation block.

This separation enforces that you think carefully about ownership and allocation boundaries.

### `RecursiveDynamicUsage`

```rust
pub trait RecursiveDynamicUsage {
    fn recursive_dynamic_usage(&self) -> usize;
}

pub fn recursive_dynamic_usage<T: RecursiveDynamicUsage>(x: &T) -> usize {
    x.recursive_dynamic_usage()
}
```

`RecursiveDynamicUsage` optionally walks into children to compute the **transitive** heap usage of a structure, according to semantics that match Bitcoin Core:

* `Vec<T: DynamicUsage>` currently counts only the backing buffer (`self.dynamic_usage()`), not the elements. This mirrors the C++ implementation, which treats inner accounting as an opt‑in concern for clients.
* `Arc<X: RecursiveDynamicUsage + DynamicUsage>` adds its own allocation cost (header + payload) and the recursive usage of the inner value.
* `Option<T: RecursiveDynamicUsage>` recurses into `Some` and charges zero for `None`.
* `Amo<T>` (a convenience alias in bitcoin-rs, modeled as `Arc<RwLock<Option<T>>>`) charges its own structure and recursively charges for the contained `T` when present.

This duality (`DynamicUsage` vs `RecursiveDynamicUsage`) lets you choose between fast, shallow accounting and more precise whole‑object accounting.

### `IncrementalDynamicUsage`

```rust
pub trait IncrementalDynamicUsage {
    fn incremental_dynamic_usage(&self) -> usize;
}
```

`IncrementalDynamicUsage` estimates **the additional heap allocation caused by inserting one more element** into a container, holding everything else fixed. This is particularly useful for admission control and cache policies:

* `StdHashMap<K, V, S>` and `StdHashSet<T, S>` approximate the cost of one more node (`StlTreeNode<..>`), passed through `malloc_usage`.
* For `Arc<T> where T: IncrementalDynamicUsage`, the incremental cost is delegated to the inner value, assuming the `Arc` itself is already allocated.

This closely mirrors the C++ `memusage::IncrementalDynamicUsage` logic used in Bitcoin Core to evaluate incremental memory impact of map and set insertions.

---

## Allocator model: `malloc_usage`

```rust
#[inline]
pub fn malloc_usage(alloc: usize) -> usize {
    if alloc == 0 {
        trace!("malloc_usage(0) -> 0");
        return 0;
    }
    let ptr_sz = core::mem::size_of::<*const ()>();
    let result = match ptr_sz {
        8 => ((alloc + 31) >> 4) << 4, // 16‑byte buckets on 64‑bit
        4 => ((alloc + 15) >> 3) << 3, // 8‑byte buckets on 32‑bit
        _ => panic!("Unsupported pointer size: {}", ptr_sz),
    };
    trace!("malloc_usage({}) -> {} (ptr_sz={})", alloc, result, ptr_sz);
    result
}
```

`malloc_usage` models allocator behavior instead of just returning `alloc`:

* On 64‑bit platforms, requested sizes are rounded up to multiples of 16 bytes.
* On 32‑bit platforms, they are rounded up to multiples of 8 bytes.

This reflects the bucketed allocation strategy assumed in Bitcoin Core's memory accounting, making your Rust numbers comparable to C++ node metrics.

You generally do not call `malloc_usage` directly; the provided `DynamicUsage` / `IncrementalDynamicUsage` implementations invoke it for you when computing container costs.

---

## Important helper types

These internal structs approximate the layout of C++ STL nodes used by Bitcoin Core for its accounting logic:

```rust
pub struct UnorderedNode<X>  {
    base: X,
    ptr:  *mut c_void,
}

pub struct StlTreeNode<X>  {
    color:  i32,
    parent: *mut c_void,
    left:   *mut c_void,
    right:  *mut c_void,
    x:      X,
}

pub struct StlSharedCounter  {
    class_type: *mut c_void,
    use_count:  usize,
    weak_count: usize,
}
```

They are not intended as generic data structures; they exist to approximate per‑node overhead for `HashMap`, `HashSet`, and reference‑counting objects in a way that is consistent with the upstream C++ implementation.

---

## Provided trait implementations

The crate includes out‑of‑the‑box implementations for a range of standard and bitcoin-rs types:

### Primitives and pointers

```rust
impl DynamicUsage for i8    { fn dynamic_usage(&self) -> usize { 0 } }
impl DynamicUsage for u8    { fn dynamic_usage(&self) -> usize { 0 } }
impl DynamicUsage for i16   { fn dynamic_usage(&self) -> usize { 0 } }
impl DynamicUsage for u16   { fn dynamic_usage(&self) -> usize { 0 } }
impl DynamicUsage for i32   { fn dynamic_usage(&self) -> usize { 0 } }
impl DynamicUsage for u32   { fn dynamic_usage(&self) -> usize { 0 } }
impl DynamicUsage for i64   { fn dynamic_usage(&self) -> usize { 0 } }
impl DynamicUsage for u64   { fn dynamic_usage(&self) -> usize { 0 } }
impl DynamicUsage for f32   { fn dynamic_usage(&self) -> usize { 0 } }
impl DynamicUsage for f64   { fn dynamic_usage(&self) -> usize { 0 } }
impl<X> DynamicUsage for *mut X   { fn dynamic_usage(&self) -> usize { 0 } }
impl<X> DynamicUsage for *const X { fn dynamic_usage(&self) -> usize { 0 } }
```

All primitives and raw pointers are treated as non‑owning with respect to heap memory.

The macro `impl_recursive_for_primitive!` also marks these as `RecursiveDynamicUsage`, delegating to `dynamic_usage`.

### `Vec<T>`

```rust
impl<X> DynamicUsage for Vec<X> {
    #[inline]
    fn dynamic_usage(&self) -> usize {
        let bytes = self.capacity() * core::mem::size_of::<X>();
        malloc_usage(bytes)
    }
}

impl<T: DynamicUsage> RecursiveDynamicUsage for Vec<T> {
    #[inline]
    fn recursive_dynamic_usage(&self) -> usize {
        // Matches C++: account for the buffer only, not the elements.
        self.dynamic_usage()
    }
}
```

This implementation uses `capacity()` rather than `len()`, which is crucial for tracking over‑allocation and future growth potential.

### `PreVector<T, N>`

`PreVector<T, N>` comes from bitcoin-rs and represents a small‑vector optimization where up to `N` elements are stored inline, and excess elements spill into a heap buffer.

```rust
impl<T: Default, const N: usize> DynamicUsage for PreVector<T, N> {
    #[inline]
    fn dynamic_usage(&self) -> usize {
        let inline_cap = N;
        let heap_cap   = self.capacity();

        if heap_cap <= inline_cap {
            0
        } else {
            let bytes = heap_cap * core::mem::size_of::<T>();
            malloc_usage(bytes)
        }
    }
}
```

This allows precise accounting for structures that try to stay on the stack most of the time.

### Hash maps and sets

Two families are supported:

* `HashMap` / `HashSet` from `bitcoin-rs` (likely re‑exports with custom parameters)
* `StdHashMap` / `StdHashSet` from `std::collections`

Each has `DynamicUsage` and `IncrementalDynamicUsage` implemented in terms of an approximate `StlTreeNode` layout:

```rust
impl<T, S> DynamicUsage for HashSet<T, S>
where S: BuildHasher
{
    #[inline]
    fn dynamic_usage(&self) -> usize {
        let node_sz = core::mem::size_of::<StlTreeNode<T>>();
        let bytes   = self.len() * node_sz;
        malloc_usage(bytes)
    }
}

impl<K, V, S> DynamicUsage for HashMap<K, V, S>
where S: BuildHasher
{
    #[inline]
    fn dynamic_usage(&self) -> usize {
        let node_sz = core::mem::size_of::<StlTreeNode<(K, V)>>();
        let bytes   = self.len() * node_sz;
        malloc_usage(bytes)
    }
}

impl<T, S> DynamicUsage for StdHashSet<T, S>
where S: BuildHasher
{
    #[inline]
    fn dynamic_usage(&self) -> usize {
        let node_sz = core::mem::size_of::<StlTreeNode<T>>();
        let bytes   = self.len() * node_sz;
        malloc_usage(bytes)
    }
}

impl<K, V, S> DynamicUsage for StdHashMap<K, V, S>
where S: BuildHasher
{
    #[inline]
    fn dynamic_usage(&self) -> usize {
        let node_sz = core::mem::size_of::<StlTreeNode<(K, V)>>();
        let bytes   = self.len() * node_sz;
        malloc_usage(bytes)
    }
}
```

Incremental costs:

```rust
impl<K, S> IncrementalDynamicUsage for HashSet<K, S>
where S: BuildHasher
{
    #[inline]
    fn incremental_dynamic_usage(&self) -> usize {
        malloc_usage(core::mem::size_of::<StlTreeNode<K>>())
    }
}

impl<K, V, S> IncrementalDynamicUsage for HashMap<K, V, S>
where S: BuildHasher
{
    #[inline]
    fn incremental_dynamic_usage(&self) -> usize {
        malloc_usage(core::mem::size_of::<StlTreeNode<(K, V)>>())
    }
}

impl<T, S> IncrementalDynamicUsage for StdHashSet<T, S>
where S: BuildHasher
{
    #[inline]
    fn incremental_dynamic_usage(&self) -> usize {
        malloc_usage(core::mem::size_of::<StlTreeNode<T>>())
    }
}

impl<K, V, S> IncrementalDynamicUsage for StdHashMap<K, V, S>
where S: BuildHasher
{
    #[inline]
    fn incremental_dynamic_usage(&self) -> usize {
        malloc_usage(core::mem::size_of::<StlTreeNode<(K, V)>>())
    }
}
```

The model is intentionally simple: one node per element, sized as `StlTreeNode<..>`.

### `Box<T>`

```rust
impl<X> DynamicUsage for Box<X> {
    #[inline]
    fn dynamic_usage(&self) -> usize {
        let bytes = core::mem::size_of::<X>();
        malloc_usage(bytes)
    }
}
```

`Box<X>` is modeled as a single allocation big enough to hold `X`.

### `Arc<T>`

```rust
impl<X> DynamicUsage for Arc<X> {
    #[inline]
    fn dynamic_usage(&self) -> usize {
        // Arc allocates a single block containing a header (2×usize) and the payload.
        let header_bytes = 2 * core::mem::size_of::<usize>();
        let total = header_bytes + core::mem::size_of::<X>();
        malloc_usage(total)
    }
}

impl<X> RecursiveDynamicUsage for Arc<X>
where X: RecursiveDynamicUsage + DynamicUsage
{
    fn recursive_dynamic_usage(&self) -> usize {
        let own   = DynamicUsage::dynamic_usage(self);
        let inner = recursive_dynamic_usage(&**self);
        own + inner
    }
}

impl<T> IncrementalDynamicUsage for Arc<T>
where T: IncrementalDynamicUsage + ?Sized
{
    #[inline]
    fn incremental_dynamic_usage(&self) -> usize {
        (**self).incremental_dynamic_usage()
    }
}
```

This reflects a consolidated allocation that stores both reference counts and the payload.

### `Option<T>`

```rust
impl<T> DynamicUsage for Option<T> {
    #[inline]
    fn dynamic_usage(&self) -> usize { 0 }
}

impl<T: RecursiveDynamicUsage> RecursiveDynamicUsage for Option<T> {
    fn recursive_dynamic_usage(&self) -> usize {
        match self {
            Some(inner) => recursive_dynamic_usage(inner),
            None        => 0,
        }
    }
}
```

`Option<T>` does not own additional heap memory beyond what `T` would already own; recursive accounting still walks into `Some` values if requested.

### `Amo<T>` (Arc + RwLock + Option)

`Amo<T>` is an alias defined elsewhere in `bitcoin-rs` as:

```rust
// conceptual form
pub type Amo<T> = Arc<parking_lot::RwLock<Option<T>>>;
```

`bitcoin-mem` provides memory accounting tailored to this pattern:

```rust
impl<T> DynamicUsage for Amo<T> {
    #[inline]
    fn dynamic_usage(&self) -> usize {
        let header_bytes = 2 * core::mem::size_of::<usize>(); // Arc header
        let payload      = core::mem::size_of::<parking_lot::RwLock<Option<T>>>();
        let total        = header_bytes + payload;
        malloc_usage(total)
    }
}

impl<X> RecursiveDynamicUsage for Amo<X>
where X: RecursiveDynamicUsage + DynamicUsage
{
    fn recursive_dynamic_usage(&self) -> usize {
        let own = DynamicUsage::dynamic_usage(self);

        let inner = {
            // work with the outer `Option<X>`
            let guard = self.getopt();       // &Option<X>
            guard.as_ref()
                 .map(recursive_dynamic_usage)
                 .unwrap_or(0)
        };

        own + inner
    }
}
```

The result is a robust estimate for shared, locked state with optional payload.

---

## Usage examples

### Basic usage with `Vec` and maps

```rust
use bitcoin_mem::{DynamicUsage, RecursiveDynamicUsage, IncrementalDynamicUsage};
use std::collections::HashMap as StdHashMap;

fn main() {
    let v: Vec<u8> = vec![0; 1024];
    let v_bytes = v.dynamic_usage();
    println!("Vec<u8> dynamic bytes (approx alloc): {}", v_bytes);

    let mut m: StdHashMap<u64, u64> = StdHashMap::new();
    m.insert(1, 2);
    m.insert(3, 4);

    let map_bytes = m.dynamic_usage();
    let map_inc   = m.incremental_dynamic_usage();

    println!("map dynamic bytes: {}", map_bytes);
    println!("map incremental bytes for next insert: {}", map_inc);
}
```

### Recursive accounting with `Arc` and `Amo`

```rust
use bitcoin_mem::{DynamicUsage, RecursiveDynamicUsage};
use std::sync::Arc;

#[derive(Default)]
struct Node {
    data: Vec<u8>,
}

impl DynamicUsage for Node {
    fn dynamic_usage(&self) -> usize {
        self.data.dynamic_usage()
    }
}

impl RecursiveDynamicUsage for Node {
    fn recursive_dynamic_usage(&self) -> usize {
        // Here we treat the Vec as the only child; we could extend this easily.
        self.data.recursive_dynamic_usage()
    }
}

fn main() {
    let node = Arc::new(Node { data: vec![0; 2048] });

    let shallow = node.dynamic_usage();          // Arc header + Node payload
    let deep    = node.recursive_dynamic_usage(); // header + payload + transitive children

    println!("shallow bytes: {}", shallow);
    println!("deep bytes: {}", deep);
}
```

### Enforcing a memory budget

```rust
use bitcoin_mem::{DynamicUsage, IncrementalDynamicUsage};
use std::collections::HashMap;

fn insert_with_budget<K, V>(
    map: &mut HashMap<K, V>,
    key: K,
    value: V,
    budget_bytes: usize,
) -> Result<(), &'static str>
where
    K: std::hash::Hash + Eq,
{
    let current = map.dynamic_usage();
    let delta   = map.incremental_dynamic_usage();

    if current + delta > budget_bytes {
        return Err("memory budget exceeded");
    }

    map.insert(key, value);
    Ok(())
}
```

This pattern can be applied to mempool admission, cache growth, and other budget‑sensitive subsystems.

---

## Design notes and caveats

* **Approximation**: The accounting is intentionally approximate. It is calibrated to match Bitcoin Core's expectations, not any particular Rust allocator implementation.
* **Non‑recursive defaults**: For containers like `Vec<T>`, recursive behavior intentionally does *not* traverse elements by default, for performance and consistency with C++. If you need fully recursive accounting, implement `RecursiveDynamicUsage` for your own types and compose as needed.
* **Ownership semantics**: Traits measure memory that is *owned* (or effectively reserved) by the value, not arbitrary pointers it may reference. For example, `*const T` is charged as zero.
* **No global allocator hooks**: All accounting is based on type‑level layout and container state (`len`, `capacity`). No runtime allocator instrumentation is used.

---

## Integration in `bitcoin-rs`

`bitcoin-mem` is part of the [`bitcoin-rs`](https://github.com/klebs6/bitcoin-rs) project and is intended to be consumed primarily by other crates in that workspace. You can still depend on it standalone:

```toml
[dependencies]
bitcoin-mem = "0.1.19"
```

Then import and extend the traits for your own domain types:

```rust
use bitcoin_mem::{DynamicUsage, RecursiveDynamicUsage};

struct BlockIndex {
    txids: Vec<[u8; 32]>,
}

impl DynamicUsage for BlockIndex {
    fn dynamic_usage(&self) -> usize {
        self.txids.dynamic_usage()
    }
}

impl RecursiveDynamicUsage for BlockIndex {
    fn recursive_dynamic_usage(&self) -> usize {
        self.txids.recursive_dynamic_usage()
    }
}
```

This makes your custom index objects participate in the same accounting framework as core data structures.

---

## License

This crate is licensed under the MIT license.

See the repository for details:

<https://github.com/klebs6/bitcoin-rs>
