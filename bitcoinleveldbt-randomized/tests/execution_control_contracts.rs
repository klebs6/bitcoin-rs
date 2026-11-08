// Pure admission contracts: no database, environment worker or native child.
#[path = "support/execution_controls.rs"]
mod execution_controls;

#[test]
fn randomized_stress_limits_reject_unbounded_controls() {
    use execution_controls::stress_limits_valid as valid;
    assert!(valid("2147483648", "64", "0", "1", "100000 100000"));
    assert!(valid("536870912", "32", "0", "1", "50000 100000"));
    for controls in [
        ("max", "64", "0", "1", "100000 100000"),
        ("0", "64", "0", "1", "100000 100000"),
        ("2147483649", "64", "0", "1", "100000 100000"),
        ("536870912", "max", "0", "1", "100000 100000"),
        ("536870912", "65", "0", "1", "100000 100000"),
        ("536870912", "0", "0", "1", "100000 100000"),
        ("536870912", "64", "max", "1", "100000 100000"),
        ("536870912", "64", "1", "1", "100000 100000"),
        ("536870912", "64", "0", "0", "100000 100000"),
        ("536870912", "64", "0", "1", "max 100000"),
        ("536870912", "64", "0", "1", "100001 100000"),
        ("536870912", "64", "0", "1", "0 100000"),
        ("536870912", "64", "0", "1", "100000 0"),
        ("536870912", "64", "0", "1", "100000"),
    ] { assert!(!valid(controls.0, controls.1, controls.2, controls.3, controls.4)); }
    eprintln!("\nRANDOMIZED_CONTROL_OK case=randomized_stress_limits_reject_unbounded_controls");
}

#[test]
fn randomized_stress_policy_rejects_wrong_seed_logging_and_data_paths() {
    use execution_controls::stress_policy_valid as valid;
    let case = "db_test_randomized";
    let data = "/private/owned-run/case-data-db_test_randomized";
    assert!(valid("off", "301", data, data, data, data, data, case));
    for controls in [
        ("trace", "301", data, data, data, data, data, case),
        ("off", "302", data, data, data, data, data, case),
        ("off", "301", data, "/tmp", data, data, data, case),
        ("off", "301", data, data, "/tmp", data, data, case),
        ("off", "301", data, data, data, "/tmp", data, case),
        ("off", "301", data, data, data, data, "/tmp", case),
        ("off", "301", "relative", "relative", "relative", "relative", "relative", case),
        ("off", "301", "/", "/", "/", "/", "/", case),
        ("off", "301", data, data, data, data, data, "other_test"),
        ("off", "301", data, data, data, data, data, "db_test_bisect_randomized"),
    ] { assert!(!valid(controls.0, controls.1, controls.2, controls.3, controls.4, controls.5, controls.6, controls.7)); }
    eprintln!("\nRANDOMIZED_CONTROL_OK case=randomized_stress_policy_rejects_wrong_seed_logging_and_data_paths");
}
