// ---------------- [ File: bitcoinleveldb-repair/src/write_descriptor.rs ]
crate::ix!();

impl Repairer {
    
    pub fn write_descriptor(&mut self) -> crate::Status {
        use std::ptr;

        trace!(dbname = %self.dbname, "Repairer::write_descriptor: start");

        let tmp = temp_file_name(&self.dbname, 1);

        let mut file_ptr: *mut Box<dyn WritableFile> = ptr::null_mut();
        let mut status = self.env.new_writable_file(&tmp, &mut file_ptr);

        if !status.is_ok() {
            debug!(
                file = %tmp,
                status = %status.to_string(),
                "Repairer::write_descriptor: NewWritableFile failed"
            );
            return status;
        }

        {
            let mut file_holder: Box<Box<dyn WritableFile>> = unsafe {
                assert!(
                    !file_ptr.is_null(),
                    "Repairer::write_descriptor: env returned null WritableFile"
                );
                Box::from_raw(file_ptr)
            };

            let mut max_sequence: SequenceNumber = 0;
            for i in 0..self.tables.len() {
                if max_sequence < self.tables[i].max_sequence {
                    max_sequence = self.tables[i].max_sequence;
                }
            }

            let cmp_ptr = self.icmp.user_comparator();
            let cmp_name = unsafe {
                if cmp_ptr.is_null() {
                    std::borrow::Cow::Borrowed("")
                } else {
                    (&*cmp_ptr).name()
                }
            };
            let cmp_slice = Slice::from(cmp_name.as_ref().as_bytes());

            self.edit.set_comparator_name(&cmp_slice);
            self.edit.set_log_number(0);
            self.edit.set_next_file(self.next_file_number);
            self.edit.set_last_sequence(max_sequence);

            for i in 0..self.tables.len() {
                // TODO(opt): separate out into multiple levels
                let t = &self.tables[i];
                self.edit.add_file(
                    0,
                    *t.meta.number(),
                    *t.meta.file_size(),
                    t.meta.smallest(),
                    t.meta.largest(),
                );
            }

            // fprintf(stderr, "NewDescriptor:\n%s\n", edit_.DebugString().c_str());
            {
                let mut logw = LogWriter::new(file_holder.as_mut());

                let mut record = String::new();
                self.edit.encode_to(&mut record as *mut String);

                status = logw.add_record(&record);
            }

            if status.is_ok() {
                status = file_holder.as_mut().close();
            }

            // `file_holder` drops here, mirroring `delete file;`.
        }

        if !status.is_ok() {
            let s_del = self.env.delete_file(&tmp);
            debug!(
                file = %tmp,
                ok = s_del.is_ok(),
                status = %s_del.to_string(),
                "Repairer::write_descriptor: delete tmp after failure"
            );
            return status;
        }

        // Discard older manifests
        for i in 0..self.manifests.len() {
            let mut full = self.dbname.clone();
            full.push('/');
            full.push_str(&self.manifests[i]);
            self.archive_file(&full);
        }

        // Install new manifest
        let dest = descriptor_file_name(&self.dbname, 1);
        status = self.env.rename_file(&tmp, &dest);

        if status.is_ok() {
            status = set_current_file(&mut *self.env, &self.dbname, 1);
        } else {
            let _ = self.env.delete_file(&tmp);
        }

        debug!(
            dbname = %self.dbname,
            ok = status.is_ok(),
            status = %status.to_string(),
            "Repairer::write_descriptor: done"
        );

        status
    }
}

#[cfg(test)]
mod write_descriptor_manifest_suite {
    use super::*;
    use crate::repairer_test_harness::*;
    use tracing::{debug, info, trace, warn};

    #[traced_test]
    fn write_descriptor_archives_old_manifests_and_installs_new_manifest_and_current() {
        let db = EphemeralDbDir::new("write-descriptor-installs");
        let dbname: String = db.path_string();

        let old_manifest = descriptor_file_name(&dbname, 2);
        touch_file(&old_manifest);

        let current = format!("{}/CURRENT", dbname);
        write_text_file(&current, "MANIFEST-000002\n");

        let options = Options::default();
        let mut repairer = Repairer::new(&dbname, &options);

        // Populate `manifests` from the directory listing.
        let st_find = repairer.find_files();
        info!(ok = st_find.is_ok(), status = %st_find.to_string(), "find_files returned");
        assert!(st_find.is_ok(), "expected ok find_files: {}", st_find.to_string());

        trace!(dbname = %dbname, "calling write_descriptor directly");
        let st = repairer.write_descriptor();

        info!(ok = st.is_ok(), status = %st.to_string(), "write_descriptor returned");
        assert!(st.is_ok(), "expected ok write_descriptor: {}", st.to_string());

        let _ = assert_archived(&old_manifest);

        let new_manifest = descriptor_file_name(&dbname, 1);
        debug!(new_manifest = %new_manifest, "checking new manifest path");
        assert!(path_exists(&new_manifest), "expected new manifest file to exist");

        let current_guess = read_current_file_guess(&dbname).unwrap_or_default();
        debug!(current = %current_guess, "CURRENT contents (best-effort)");
        assert!(
            current_guess.contains("MANIFEST-000001") || current_guess.contains("MANIFEST-1"),
            "expected CURRENT to mention manifest 000001; got: {:?}",
            current_guess
        );
    }

    #[traced_test]
    fn write_descriptor_succeeds_even_with_no_tables_discovered() {
        let db = EphemeralDbDir::new("write-descriptor-no-tables");
        let dbname: String = db.path_string();

        let sentinel = format!("{}/SENTINEL", dbname);
        touch_file(&sentinel);

        let options = Options::default();
        let mut repairer = Repairer::new(&dbname, &options);

        let st_find = repairer.find_files();
        assert!(st_find.is_ok(), "expected ok find_files: {}", st_find.to_string());

        trace!("calling write_descriptor with empty tables set");
        let st = repairer.write_descriptor();

        info!(ok = st.is_ok(), status = %st.to_string(), "write_descriptor returned");
        assert!(st.is_ok(), "expected ok write_descriptor: {}", st.to_string());

        let new_manifest = descriptor_file_name(&dbname, 1);
        assert!(path_exists(&new_manifest), "expected manifest created");
    }
}
