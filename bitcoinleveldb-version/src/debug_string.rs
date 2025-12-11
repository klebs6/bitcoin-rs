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

#[cfg(test)]
mod version_debug_string_behavior_tests {
    use super::*;
    use super::version_test_helpers as helpers;

    #[traced_test]
    fn debug_string_contains_header_for_each_level_even_when_empty() {
        let version = helpers::build_empty_version();
        let s = version.debug_string();

        for level in 0..NUM_LEVELS {
            let header = format!("--- level {} ---", level);
            assert!(
                s.contains(&header),
                "debug_string must contain a header line for level {}",
                level
            );
        }
    }

    #[traced_test]
    fn debug_string_includes_file_metadata_for_present_files() {
        let mut version = helpers::build_empty_version();
        let file_number: u64 = 11;
        let file_size: u64 = 1234;

        {
            let files = version.files_mut();
            files[0].push(helpers::build_file_meta_boxed(
                file_number,
                file_size,
                "k",
                "t",
            ));
        }

        let s = version.debug_string();
        let pattern = format!(" {}:{}", file_number, file_size);
        assert!(
            s.contains(&pattern),
            "debug_string must describe files as '<number>:<size>'"
        );
    }
}
