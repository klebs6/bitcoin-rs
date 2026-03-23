// ---------------- [ File: bitcoinleveldbt-util/src/register_test.rs ]
crate::ix!();

/**
  | Register the specified test. Typically
  | not used directly, but invoked via the
  | macro expansion of TEST.
  |
  */
pub fn register_test(
    base: *const u8,
    name: *const u8,
    func: fn(),
) -> bool {
    trace!(
        target: "bitcoinleveldbt_util::harness",
        event = "register_test_entry",
        base_ptr = (base as usize),
        name_ptr = (name as usize)
    );

    let base_label = if base.is_null() {
        String::new()
    } else {
        unsafe {
            CStr::from_ptr(base as *const c_char)
                .to_string_lossy()
                .into_owned()
        }
    };

    let name_label = if name.is_null() {
        String::new()
    } else {
        unsafe {
            CStr::from_ptr(name as *const c_char)
                .to_string_lossy()
                .into_owned()
        }
    };

    let mut guard = BITCOINLEVELDB_TEST_HARNESS_REGISTERED_TESTS.lock();

    let test = TestBuilder::default()
        .base(base_label)
        .name(name_label)
        .func(func)
        .build()
        .unwrap();

    guard.push(test);

    trace!(
        target: "bitcoinleveldbt_util::harness",
        event = "register_test_exit",
        registered_count = guard.len()
    );

    true
}
