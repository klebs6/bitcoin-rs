// ---------------- [ File: bitcoinleveldb-util/src/no_destructor_test.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/no_destructor_test.cc]

/**
  | A struct used to verify that it is never destructed.
  | 
  | We test constructor argument forwarding,
  | plus we deliberately call `std::process::abort()`
  | in `drop()` so if the destructor ever runs, the
  | process ends immediately.
  */
#[derive(Debug, Getters, Setters, Builder)]
#[getset(get = "pub", set = "pub")]
pub struct DoNotDestruct {
    /// Used to check constructor argument forwarding.
    a: u32,
    b: u64,
}

impl Drop for DoNotDestruct {
    fn drop(&mut self) {
        error!("DoNotDestruct destructor called! Aborting...");
        process::abort(); // Mimic std::abort() from C++ 
    }
}

impl DoNotDestruct {
    pub fn new(a: u32, b: u64) -> Self {
        info!("Constructing DoNotDestruct with a=0x{:x}, b=0x{:x}", a, b);
        Self { a, b }
    }
}

// We replicate the constants from C++.
const GOLDEN_A: u32 = 0xdeadbeef;
const GOLDEN_B: u64 = 0xaabbccddeeffaabb;

/// A trivial test struct for grouping these tests (if needed).
#[derive(Debug, Getters, Setters, Builder)]
pub struct NoDestructorTest {}

#[traced_test]
fn no_destructor_test_stack_instance() {
    info!("Running no_destructor_test_stack_instance");
    let instance = super::NoDestructor::new(DoNotDestruct::new(GOLDEN_A, GOLDEN_B));
    unsafe {
        assert_eq!(*(*instance.get()).a(), GOLDEN_A, "Stack instance 'a' should match");
        assert_eq!(*(*instance.get()).b(), GOLDEN_B, "Stack instance 'b' should match");
    }
    info!("Completed no_destructor_test_stack_instance");
}

#[traced_test]
fn no_destructor_test_static_instance() {
    info!("Running no_destructor_test_static_instance");
    // A static variable in Rust is typically declared outside of a function,
    // but for demonstration, we can do a lazy_static or once_cell.
    // We'll show it with a global once_cell here:
    static ONCE: once_cell::sync::Lazy<super::NoDestructor<DoNotDestruct>> = once_cell::sync::Lazy::new(|| {
        info!("Initializing static NoDestructor for DoNotDestruct");
        super::NoDestructor::new(DoNotDestruct::new(GOLDEN_A, GOLDEN_B))
    });
    unsafe {
        assert_eq!(*(*ONCE.get()).a(), GOLDEN_A, "Static instance 'a' should match");
        assert_eq!(*(*ONCE.get()).b(), GOLDEN_B, "Static instance 'b' should match");
    }
    info!("Completed no_destructor_test_static_instance");
}
