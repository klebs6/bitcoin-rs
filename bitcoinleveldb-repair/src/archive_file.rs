// ---------------- [ File: bitcoinleveldb-repair/src/archive_file.rs ]
crate::ix!();

impl Repairer {
    pub fn archive_file(&mut self, fname: &String) {
        trace!(file = %fname, "Repairer::archive_file: start");

        // Move into another directory.  E.g., for
        //    dir/foo
        // rename to
        //    dir/lost/foo
        let slash_pos = fname.rfind('/');

        let mut new_dir = String::new();
        if let Some(pos) = slash_pos {
            new_dir.push_str(&fname[..pos]);
        }
        new_dir.push_str("/lost");

        // Ignore error
        let s_create = self.env.create_dir(&new_dir);
        if !s_create.is_ok() {
            debug!(
                dir = %new_dir,
                status = %s_create.to_string(),
                "Repairer::archive_file: CreateDir failed (ignored)"
            );
        } else {
            trace!(dir = %new_dir, "Repairer::archive_file: CreateDir ok");
        }

        let base = match slash_pos {
            Some(pos) => &fname[(pos + 1)..],
            None => fname.as_str(),
        };

        let mut new_file = new_dir.clone();
        new_file.push('/');
        new_file.push_str(base);

        let s_rename = self.env.rename_file(fname, &new_file);

        info!(
            src = %fname,
            dst = %new_file,
            ok = s_rename.is_ok(),
            status = %s_rename.to_string(),
            "Repairer::archive_file: archiving"
        );

        trace!(file = %fname, "Repairer::archive_file: done");
    }
}

#[cfg(test)]
mod archive_file_behavior_suite {
    use super::*;
    use crate::repairer_test_harness::*;
    use tracing::{debug, info, trace, warn};

    #[traced_test]
    fn archive_file_moves_file_into_lost_directory_preserving_contents() {
        let db = EphemeralDbDir::new("archive-file-basic");
        let dbname = db.path_string();

        let options = Options::default();
        let mut repairer = Repairer::new(&dbname, &options);

        let src = format!("{}/foo.txt", dbname);
        write_text_file(&src, "hello-archive");
        assert!(path_exists(&src));

        trace!(src = %src, "calling archive_file");
        repairer.archive_file(&src);

        let dst = expected_archive_destination(&src);
        info!(src = %src, dst = %dst, "verifying archive_file moved source");

        assert!(!path_exists(&src), "expected src to be moved: {}", src);
        assert!(path_exists(&dst), "expected dst to exist: {}", dst);
        assert_eq!(read_text_file(&dst), "hello-archive");
    }

    #[traced_test]
    fn archive_file_works_when_lost_directory_already_exists() {
        let db = EphemeralDbDir::new("archive-file-lost-exists");
        let dbname = db.path_string();

        let options = Options::default();
        let mut repairer = Repairer::new(&dbname, &options);

        let lost_dir = format!("{}/lost", dbname);
        std::fs::create_dir_all(&lost_dir).unwrap();
        assert!(is_directory(&lost_dir));

        let src = format!("{}/bar.bin", dbname);
        write_text_file(&src, "payload");
        trace!(src = %src, lost_dir = %lost_dir, "calling archive_file");
        repairer.archive_file(&src);

        let dst = expected_archive_destination(&src);
        assert!(!path_exists(&src));
        assert!(path_exists(&dst));
        assert_eq!(read_text_file(&dst), "payload");
    }

    #[traced_test]
    fn archive_file_attempts_rename_even_if_create_dir_fails_and_leaves_src_on_failure() {
        let db = EphemeralDbDir::new("archive-file-createdir-fails");
        let dbname = db.path_string();

        let options = Options::default();
        let mut repairer = Repairer::new(&dbname, &options);

        // Create a file at {dbname}/lost, forcing CreateDir("{dbname}/lost") to fail.
        let lost_as_file = format!("{}/lost", dbname);
        write_text_file(&lost_as_file, "not-a-dir");
        assert!(path_exists(&lost_as_file));
        assert!(!is_directory(&lost_as_file));

        let src = format!("{}/baz.txt", dbname);
        write_text_file(&src, "keep-me");
        assert!(path_exists(&src));

        trace!(src = %src, lost_as_file = %lost_as_file, "calling archive_file with CreateDir expected to fail");
        repairer.archive_file(&src);

        // With a non-directory parent, the rename should fail; `archive_file` ignores the error.
        // Verify that src still exists and dst does not.
        let dst = expected_archive_destination(&src);
        debug!(src = %src, dst = %dst, "verifying archive_file failure behavior");
        assert!(path_exists(&src), "expected src to remain after failed rename");
        assert!(!path_exists(&dst), "expected dst to not exist after failed rename");
    }

    #[traced_test]
    fn archive_file_moves_file_from_nested_directory_into_nested_lost_directory() {
        let db = EphemeralDbDir::new("archive-file-nested");
        let dbname = db.path_string();

        let options = Options::default();
        let mut repairer = Repairer::new(&dbname, &options);

        let subdir = format!("{}/subdir", dbname);
        std::fs::create_dir_all(&subdir).unwrap();

        let src = format!("{}/subdir/nested.txt", dbname);
        write_text_file(&src, "nested");
        assert!(path_exists(&src));

        trace!(src = %src, "calling archive_file for nested file");
        repairer.archive_file(&src);

        let dst = expected_archive_destination(&src);
        info!(src = %src, dst = %dst, "verifying nested archive destination");
        assert!(!path_exists(&src));
        assert!(path_exists(&dst));
        assert_eq!(read_text_file(&dst), "nested");
    }
}
