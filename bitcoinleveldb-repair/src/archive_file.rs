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
