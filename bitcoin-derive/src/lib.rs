// ---------------- [ File: bitcoin-derive/src/lib.rs ]
extern crate proc_macro;

use proc_macro::TokenStream;
//use quote::quote;
//use syn;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/port/thread_annotations.h]
mod annotate_thread_safety_attr; use annotate_thread_safety_attr::*;

#[proc_macro_attribute]
pub fn no_copy(attr: TokenStream, item: TokenStream) -> TokenStream {
    /*
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
    */
    item
}

#[proc_macro_attribute]
pub fn no_move(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn UNLOCK_FUNCTION(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    annotate_thread_safety_attr(
        "UNLOCK_FUNCTION",
        "Function releases the specified mutex(es); typically applied to destructors or unlock APIs.",
        attr,
        item,
    )
}

#[proc_macro_attribute]
pub fn ASSERT_EXCLUSIVE_LOCK(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    annotate_thread_safety_attr(
        "ASSERT_EXCLUSIVE_LOCK",
        "Asserts (for analysis) that the current thread holds the specified lock(s) exclusively.",
        attr,
        item,
    )
}

#[proc_macro_attribute]
pub fn fuzz_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn fuzz(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn Q_SLOT(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn Q_OBJECT(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn Q_SIGNAL(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn Q_SIGNALS(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn Q_SLOTS(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn NO_THREAD_SAFETY_ANALYSIS(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    annotate_thread_safety_attr(
        "NO_THREAD_SAFETY_ANALYSIS",
        "Suppresses Clang's thread safety analysis for this item.",
        attr,
        item,
    )
}

#[proc_macro_attribute]
pub fn Q_PROPERTY(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn Q_METATYPE(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn signal(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn EXCLUSIVE_TRYLOCK_FUNCTION(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    annotate_thread_safety_attr(
        "EXCLUSIVE_TRYLOCK_FUNCTION",
        "Function attempts to acquire an exclusive lock; the result indicates success.",
        attr,
        item,
    )
}

#[proc_macro_attribute]
pub fn LIFETIMEBOUND(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn LOCK_RETURNED(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    annotate_thread_safety_attr(
        "LOCK_RETURNED",
        "Function returns a reference to the specified lock (used for alias analysis).",
        attr,
        item,
    )
}

#[proc_macro_attribute]
pub fn THREAD_ANNOTATION_ATTRIBUTE__(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    annotate_thread_safety_attr(
        "THREAD_ANNOTATION_ATTRIBUTE__",
        "Low-level wrapper macro in C++ that expands to __attribute__((...)) under Clang; \
         used to build higher-level thread-safety annotations.",
        attr,
        item,
    )
}

#[proc_macro_attribute]
pub fn GUARDED_BY(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    annotate_thread_safety_attr(
        "GUARDED_BY",
        "Marks that a data member is protected by the specified mutex (exclusive lock). \
         Maps to 'guarded_by(x)' in Clang Thread Safety Analysis.",
        attr,
        item,
    )
}

#[proc_macro_attribute]
pub fn PT_GUARDED_BY(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    annotate_thread_safety_attr(
        "PT_GUARDED_BY",
        "Marks that the memory location pointed to is guarded by the specified mutex. \
         Maps to 'pt_guarded_by(x)'.",
        attr,
        item,
    )
}

#[proc_macro_attribute]
pub fn ACQUIRED_AFTER(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    annotate_thread_safety_attr(
        "ACQUIRED_AFTER",
        "Declares a lock ordering: this lock must be acquired after the given locks.",
        attr,
        item,
    )
}

#[proc_macro_attribute]
pub fn ACQUIRED_BEFORE(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    annotate_thread_safety_attr(
        "ACQUIRED_BEFORE",
        "Declares a lock ordering: this lock must be acquired before the given locks.",
        attr,
        item,
    )
}

#[proc_macro_attribute]
pub fn EXCLUSIVE_LOCKS_REQUIRED(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    annotate_thread_safety_attr(
        "EXCLUSIVE_LOCKS_REQUIRED",
        "Function requires the specified mutexes to be held (exclusive) on entry.",
        attr,
        item,
    )
}

#[proc_macro_attribute]
pub fn SHARED_LOCKS_REQUIRED(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    annotate_thread_safety_attr(
        "SHARED_LOCKS_REQUIRED",
        "Function requires the specified mutexes to be held in shared (reader) mode on entry.",
        attr,
        item,
    )
}

#[proc_macro_attribute]
pub fn LOCKS_EXCLUDED(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    annotate_thread_safety_attr(
        "LOCKS_EXCLUDED",
        "Function must not be called while holding any of the specified locks.",
        attr,
        item,
    )
}

#[proc_macro_attribute]
pub fn LOCKABLE(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    annotate_thread_safety_attr(
        "LOCKABLE",
        "Marks a type as a lockable mutex type (supports Lock/Unlock semantics).",
        attr,
        item,
    )
}

#[proc_macro_attribute]
pub fn SCOPED_LOCKABLE(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    annotate_thread_safety_attr(
        "SCOPED_LOCKABLE",
        "Marks an RAII guard type that acquires a lock on construction and releases it on drop.",
        attr,
        item,
    )
}

#[proc_macro_attribute]
pub fn EXCLUSIVE_LOCK_FUNCTION(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    annotate_thread_safety_attr(
        "EXCLUSIVE_LOCK_FUNCTION",
        "Function acquires an exclusive (writer) lock on the specified mutex(es).",
        attr,
        item,
    )
}

#[proc_macro_attribute]
pub fn SHARED_LOCK_FUNCTION(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    annotate_thread_safety_attr(
        "SHARED_LOCK_FUNCTION",
        "Function acquires a shared (reader) lock on the specified mutex(es).",
        attr,
        item,
    )
}

#[proc_macro_attribute]
pub fn SHARED_TRYLOCK_FUNCTION(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    annotate_thread_safety_attr(
        "SHARED_TRYLOCK_FUNCTION",
        "Function attempts to acquire a shared lock; the result indicates success.",
        attr,
        item,
    )
}

#[proc_macro_attribute]
pub fn ASSERT_SHARED_LOCK(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    annotate_thread_safety_attr(
        "ASSERT_SHARED_LOCK",
        "Asserts (for analysis) that the current thread holds the specified lock(s) in shared mode.",
        attr,
        item,
    )
}
