// ---------------- [ File: bitcoinleveldb-repair/src/run.rs ]
crate::ix!();

impl Repairer {
    pub fn run(&mut self) -> crate::Status {
        trace!(dbname = %self.dbname, "Repairer::run: start");

        let mut status: crate::Status = self.find_files();

        if status.is_ok() {
            self.convert_log_files_to_tables();
            self.extract_meta_data();
            status = self.write_descriptor();
        }

        if status.is_ok() {
            let mut bytes: u64 = 0;
            for i in 0..self.tables.len() {
                bytes = bytes.wrapping_add(*self.tables[i].meta.file_size());
            }

            info!(
                dbname = %self.dbname,
                recovered_files = self.tables.len(),
                recovered_bytes = bytes,
                "**** Repaired leveldb; Some data may have been lost. ****"
            );
        } else {
            warn!(
                dbname = %self.dbname,
                status = %status.to_string(),
                "Repairer::run: failed"
            );
        }

        trace!(dbname = %self.dbname, "Repairer::run: done");
        status
    }
}
