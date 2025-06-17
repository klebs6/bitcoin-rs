// ---------------- [ File: bitcoin-mem/src/impl_recursive_dynamic_usage_for_primitive.rs ]
crate::ix!();

// ---------------- dynamic_usage.rs (additions) ----------------

// Blanket‑free leaf implementations so that primitive values and common
// containers can participate in recursive accounting without requiring
// specialisation.
//
// NOTE: we intentionally restrict ourselves to concrete types (no blanket
// impl for all `T: DynamicUsage`) to avoid coherence overlap with `Arc<T>`,
// `Amo<T>`, `Option<T>`, etc.

// ---- 1.  Primitive numeric leaves  -----------------------------------------
macro_rules! impl_recursive_for_primitive {
    ($($t:ty),* $(,)?) => {$(
        impl RecursiveDynamicUsage for $t {
            #[inline]
            fn recursive_dynamic_usage(&self) -> usize {
                self.dynamic_usage()        // primitives have zero anyway
            }
        }
    )*};
}
impl_recursive_for_primitive!(i8, u8, i16, u16, i32, u32, i64, u64, f32, f64);

// ---- 2.  Vec<T> (shallow) ---------------------------------------------------
impl<T: DynamicUsage> RecursiveDynamicUsage for Vec<T> {
    #[inline]
    fn recursive_dynamic_usage(&self) -> usize {
        // Follow C++ behaviour: account for the backing buffer only,
        // *not* the elements inside.
        self.dynamic_usage()
    }
}
