// ---------------- [ File: bitcoinleveldb-versionsetutil/src/file_set.rs ]
crate::ix!();

pub type VersionSetBuilderFileSet = HashSet<*mut FileMetaData,BySmallestKeyComparator>;

#[cfg(test)]
mod version_set_file_set_spec {
    use super::*;

    #[traced_test]
    fn verify_version_set_builder_file_set_is_sized() {
        // This test ensures the alias is well-formed and the underlying type
        // can be instantiated in a generic context.
        let _size = core::mem::size_of::<VersionSetBuilderFileSet>();
        assert!(
            _size > 0,
            "VersionSetBuilderFileSet should have a non-zero size"
        );
    }
}
