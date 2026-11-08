// Pure admission contracts: no database, environment mutation or child process.
#[path = "support/execution_controls.rs"]
mod execution_controls;
use execution_controls::{Controls, Profile, controls_valid, environment_valid};

#[test]
fn admission_accepts_complete_controls() {
    let case = Controls { memory: "536870912", high: "402653184", pids: "64", swap_limit_text: "0",
        oom: "1", cpu: "100000 100000", core_limit: "0 0 bytes", file_limit: "16777216 16777216 bytes" };
    let stress = Controls { memory: "2147483648", high: "1610612736", file_limit: "268435456 268435456 bytes", ..case };
    assert!(controls_valid(Profile::Case, case));
    assert!(controls_valid(Profile::Stress, stress));
    assert!(controls_valid(Profile::Case, Controls { cpu: "50000 100000", ..case }));
    for (name, profile) in [("harness_empty", Profile::Case), ("harness_randomized", Profile::Stress)] {
        let data = format!("/private/run/case-data-{}", name);
        assert!(environment_valid(name, profile, name, "1", "301", "off", &data, &data, &data, &data, &data));
    }
    eprintln!("\nHARNESS_CONTROL_CONTRACT_OK case=admission_accepts_complete_controls");
}

#[test]
fn admission_rejects_missing_or_weakened_controls() {
    let good = Controls { memory: "536870912", high: "402653184", pids: "64", swap_limit_text: "0",
        oom: "1", cpu: "100000 100000", core_limit: "0 0 bytes", file_limit: "16777216 16777216 bytes" };
    for bad in [Controls { memory: "max", ..good }, Controls { memory: "536870913", ..good },
        Controls { memory: "0", ..good }, Controls { high: "max", ..good }, Controls { high: "402653185", ..good },
        Controls { pids: "max", ..good }, Controls { pids: "65", ..good }, Controls { swap_limit_text: "max", ..good },
        Controls { oom: "0", ..good }, Controls { cpu: "max 100000", ..good }, Controls { cpu: "100001 100000", ..good },
        Controls { cpu: "0 100000", ..good }, Controls { cpu: "100000 0", ..good }, Controls { cpu: "100000", ..good },
        Controls { core_limit: "unlimited unlimited bytes", ..good }, Controls { core_limit: "1 1 bytes", ..good },
        Controls { file_limit: "unlimited unlimited bytes", ..good }, Controls { file_limit: "16777216 16777217 bytes", ..good }] {
        assert!(!controls_valid(Profile::Case, bad));
    }
    assert!(!controls_valid(Profile::Stress, good));
    let name = "harness_empty"; let data = "/private/run/case-data-harness_empty";
    for (case_token, protected, seed, logging, tmpdir) in [("other", "1", "301", "off", data),
        (name, "", "301", "off", data), (name, "1", "302", "off", data),
        (name, "1", "301", "trace", data), (name, "1", "301", "off", "/tmp")] {
        assert!(!environment_valid(name, Profile::Case, case_token, protected, seed, logging, data, tmpdir, data, data, data));
    }
    assert!(!environment_valid(name, Profile::Stress, name, "1", "301", "off", data, data, data, data, data));
    assert!(!environment_valid("unknown", Profile::Case, "unknown", "1", "301", "off", data, data, data, data, data));
    assert!(!environment_valid(name, Profile::Case, name, "1", "301", "off", "relative", "relative", "relative", "relative", "relative"));
    eprintln!("\nHARNESS_CONTROL_CONTRACT_OK case=admission_rejects_missing_or_weakened_controls");
}
