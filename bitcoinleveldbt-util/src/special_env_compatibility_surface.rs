// ---------------- [ File: bitcoinleveldb-testutil/src/special_env_compatibility_surface.rs ]
crate::ix!();

/// Invariant: preserves `SpecialEnv` delegation semantics while adapting the call signatures
/// expected by this crate.
pub trait DBTestSpecialEnvCompatibilitySurface {
    /// Precondition: `dir` names a directory in the wrapped environment.
    /// Postcondition: forwards enumeration to `base_mut()` and preserves returned `Status`.
    fn get_children(&mut self, dir: &String, result: &mut Vec<String>) -> Status;

    /// Precondition: `filename` names a file path in the wrapped environment.
    /// Postcondition: forwards deletion to `base_mut()` and preserves returned `Status`.
    fn delete_file(&mut self, filename: &String) -> Status;

    /// Precondition: `from` and `to` are filesystem paths in the wrapped environment.
    /// Postcondition: forwards rename to `base_mut()` and preserves returned `Status`.
    fn rename_file(&mut self, from: &String, to: &String) -> Status;
}

impl DBTestSpecialEnvCompatibilitySurface for SpecialEnv {
    fn get_children(&mut self, dir: &String, result: &mut Vec<String>) -> Status {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.specialenv_compat.get_children.enter",
            dir_len = dir.len()
        );

        let s = self.base_mut().get_children(dir, result as *mut Vec<String>);

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.specialenv_compat.get_children.exit",
            ok = s.is_ok(),
            result_len = result.len()
        );

        s
    }

    fn delete_file(&mut self, filename: &String) -> Status {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.specialenv_compat.delete_file.enter",
            filename_len = filename.len()
        );

        let s = self.base_mut().delete_file(filename);

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.specialenv_compat.delete_file.exit",
            ok = s.is_ok()
        );

        s
    }

    fn rename_file(&mut self, from: &String, to: &String) -> Status {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.specialenv_compat.rename_file.enter",
            from_len = from.len(),
            to_len = to.len()
        );

        let s = self.base_mut().rename_file(from, to);

        tracing::trace!(
            target: "bitcoinleveldb_dbtest::dbtest",
            label = "dbtest.specialenv_compat.rename_file.exit",
            ok = s.is_ok()
        );

        s
    }
}
