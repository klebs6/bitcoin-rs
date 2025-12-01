// ---------------- [ File: bitcoinleveldb-posixenv/tests/result_slot.rs ]
use bitcoinleveldb_posixenv::*;
use bitcoin_imports::*;

#[traced_test]
fn env_result_slot_initialization_sets_null_pointer() {
    trace!("env_result_slot_initialization_sets_null_pointer: start");

    let mut raw: *mut Box<i32> = 0xdead_beefusize as *mut Box<i32>;

    initialize_posix_env_result_slot::<i32>(
        "env_result_slot_initialization_sets_null_pointer",
        &mut raw,
    );

    assert!(
        raw.is_null(),
        "initialize_posix_env_result_slot should set the output pointer to null"
    );

    info!("env_result_slot_initialization_sets_null_pointer: completed");
}

#[traced_test]
fn env_result_slot_store_boxed_result_round_trips_value() {
    trace!("env_result_slot_store_boxed_result_round_trips_value: start");

    let mut raw: *mut Box<i32> = std::ptr::null_mut();

    initialize_posix_env_result_slot::<i32>(
        "env_result_slot_store_boxed_result_round_trips_value",
        &mut raw,
    );

    let inner = Box::new(42_i32);

    let status = store_posix_env_boxed_result::<i32>(
        "env_result_slot_store_boxed_result_round_trips_value",
        &mut raw,
        inner,
    );

    assert!(
        status.is_ok(),
        "store_posix_env_boxed_result should return OK, got {}",
        status.to_string()
    );

    assert!(
        !raw.is_null(),
        "store_posix_env_boxed_result should set a non-null pointer"
    );

    let outer: Box<Box<i32>> = unsafe { Box::from_raw(raw) };
    let value = **outer;

    assert_eq!(
        value, 42_i32,
        "round-tripped value through store_posix_env_boxed_result must match"
    );

    info!("env_result_slot_store_boxed_result_round_trips_value: completed");
}
