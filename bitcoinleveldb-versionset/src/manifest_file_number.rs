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
