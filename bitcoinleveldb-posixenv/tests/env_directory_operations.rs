// ---------------- [ File: bitcoinleveldb-posixenv/tests/env_directory_operations.rs ]
use bitcoinleveldb_file::*;
use bitcoinleveldb_posixtools::*;
use bitcoinleveldb_posixenv::*;
use bitcoinleveldb_env::*;
use bitcoinleveldb_slice::*;
use bitcoin_imports::*;

#[traced_test]
fn env_directory_create_delete_and_get_children() {
    trace!("env_directory_create_delete_and_get_children: start");

    let test_env = EnvTest::default();
    let env_rc   = test_env.env().clone();

    let mut base_dir = String::new();
    {
        let mut env = env_rc.borrow_mut();
        let status  = env.get_test_directory(&mut base_dir);
        assert!(
            status.is_ok(),
            "GetTestDirectory failed: {}",
            status.to_string()
        );
    }

    let subdir_name = "fs_ops_subdir".to_owned();
    let subdir_path = format!("{}/{}", base_dir, subdir_name);

    // Cleanup any pre-existing directory.
    {
        let mut env = env_rc.borrow_mut();
        let status  = env.delete_dir(&subdir_path);
        if !(status.is_ok() || status.is_not_found()) {
            warn!(
                dir    = %subdir_path,
                status = %status.to_string(),
                "env_directory_create_delete_and_get_children: initial DeleteDir \
                 returned unexpected status (ignored)"
            );
        }
    }

    // Create the directory.
    {
        let mut env = env_rc.borrow_mut();
        let status  = env.create_dir(&subdir_path);
        assert!(
            status.is_ok(),
            "CreateDir failed: {}",
            status.to_string()
        );
    }

    // The directory should exist.
    {
        let mut env = env_rc.borrow_mut();
        assert!(
            env.file_exists(&subdir_path),
            "Subdirectory should exist after CreateDir"
        );
    }

    // It should appear in the parent's children listing.
    {
        let mut env = env_rc.borrow_mut();
        let mut children: Vec<String> = Vec::new();

        let status = env.get_children(&base_dir, &mut children as *mut Vec<String>);
        assert!(
            status.is_ok(),
            "GetChildren failed: {}",
            status.to_string()
        );

        assert!(
            children.iter().any(|name| name == &subdir_name),
            "Expected to find newly created subdirectory {} in children of {}",
            subdir_name,
            base_dir
        );
    }

    // Delete the directory.
    {
        let mut env = env_rc.borrow_mut();
        let status  = env.delete_dir(&subdir_path);
        assert!(
            status.is_ok(),
            "DeleteDir failed: {}",
            status.to_string()
        );
    }

    // It should no longer exist.
    {
        let mut env = env_rc.borrow_mut();
        assert!(
            !env.file_exists(&subdir_path),
            "Subdirectory should not exist after DeleteDir"
        );
    }

    // And it should be gone from the children listing.
    {
        let mut env = env_rc.borrow_mut();
        let mut children: Vec<String> = Vec::new();

        let status = env.get_children(&base_dir, &mut children as *mut Vec<String>);
        assert!(
            status.is_ok(),
            "GetChildren (post-delete) failed: {}",
            status.to_string()
        );

        assert!(
            !children.iter().any(|name| name == &subdir_name),
            "Subdirectory {} should not appear in children after DeleteDir",
            subdir_name
        );
    }

    info!("env_directory_create_delete_and_get_children: completed");
}
