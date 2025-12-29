// ---------------- [ File: bitcoinleveldb-versionset/src/get_options_ptr.rs ]
crate::ix!();

impl GetOptionsPtr for VersionSet {
    fn options(&self) -> *const Options {
        let p: *const Options = VersionSet::options(self);

        trace!(
            options_ptr = %format!("{:p}", p),
            "VersionSet::options: returning Options pointer"
        );

        // NOTE: The VersionSet stores a raw pointer to Options that is owned elsewhere.
        p
    }
}

impl VersionSet {
    pub fn get_options_ptr(&self) -> *const Options {
        let options_ptr: *const Options = <VersionSet as GetOptionsPtr>::options(self);

        trace!(
            options_ptr = %format!("{:p}", options_ptr),
            "VersionSet::get_options_ptr"
        );

        options_ptr
    }
}
