// ---------------- [ File: bitcoinleveldb-version/src/debug_string.rs ]
crate::ix!();

impl Version {

    /// Return a human readable string that describes this version's contents.
    /// 
    pub fn debug_string(&self) -> String {
        trace!(
            "Version::debug_string: generating string for {} levels",
            NUM_LEVELS
        );

        let mut r = String::new();

        for level in 0..NUM_LEVELS {
            use core::fmt::Write;

            write!(&mut r, "--- level {} ---\n", level)
                .expect("write! to String cannot fail");

            let files_level = &self.files()[level];
            trace!(
                "Version::debug_string: level {} has {} files",
                level,
                files_level.len()
            );

            for &fptr in files_level.iter() {
                if fptr.is_null() {
                    warn!(
                        "Version::debug_string: null FileMetaData pointer at level {}",
                        level
                    );
                    continue;
                }

                unsafe {
                    let f: &FileMetaData = &*fptr;
                    let number    = *f.number();
                    let file_size = *f.file_size();
                    let smallest  = f.smallest().debug_string();
                    let largest   = f.largest().debug_string();

                    write!(
                        &mut r,
                        " {}:{}[{} .. {}]\n",
                        number,
                        file_size,
                        smallest,
                        largest
                    )
                    .expect("write! to String cannot fail");
                }
            }
        }

        debug!(
            "Version::debug_string: completed; len={}",
            r.len()
        );
        r
    }
}
