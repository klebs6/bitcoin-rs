// ---------------- [ File: bitcoinleveldb-versionset/src/manifest_file_number.rs ]
crate::ix!();

impl ManifestFileNumber for VersionSet {

    /// Return the current manifest file number
    fn manifest_file_number(&self) -> u64 {
        let n: u64 = VersionSet::manifest_file_number(self);

        trace!(
            manifest_file_number = n,
            "VersionSet::manifest_file_number (ManifestFileNumber)"
        );

        n
    }
}

impl VersionSet {

    pub fn get_manifest_file_number(&self) -> u64 {
        let n: u64 = <VersionSet as ManifestFileNumber>::manifest_file_number(self);

        trace!(
            manifest_file_number = n,
            "VersionSet::get_manifest_file_number"
        );

        n
    }
}

#[cfg(test)]
mod manifest_file_number_exhaustive_test_suite {
    use super::*;
    use tracing::{debug, trace};

    #[traced_test]
    fn manifest_file_number_accessors_match_and_reflect_updates() {
        let env = PosixEnv::shared();
        let options = Box::new(Options::with_env(env));

        let icmp = Box::new(options.internal_key_comparator());
        let mut table_cache = Box::new(TableCache::new(&"tmp".to_string(), options.as_ref(), 1));

        let mut vs = VersionSet::new(
            &"tmp".to_string(),
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        vs.set_manifest_file_number(9);

        let n = vs.get_manifest_file_number();
        debug!(n, "manifest file number");
        assert_eq!(
            n,
            <VersionSet as ManifestFileNumber>::manifest_file_number(&vs),
            "wrapper and trait must agree"
        );
        assert_eq!(n, 9, "manifest file number must reflect updates");
    }
}
