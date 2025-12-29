// ---------------- [ File: bitcoinleveldb-repair/src/convert_log_files_to_tables.rs ]
crate::ix!();

impl Repairer {
    pub fn convert_log_files_to_tables(&mut self) {
        trace!(
            log_count = self.logs.len(),
            "Repairer::convert_log_files_to_tables: start"
        );

        for i in 0..self.logs.len() {
            let lognum = self.logs[i];
            let logname = log_file_name(&self.dbname, lognum);

            trace!(lognum, file = %logname, "Repairer::convert_log_files_to_tables: converting log");

            let status = self.convert_log_to_table(lognum);

            if !status.is_ok() {
                warn!(
                    lognum,
                    status = %status.to_string(),
                    "Repairer::convert_log_files_to_tables: ignoring conversion error"
                );
            }

            self.archive_file(&logname);
        }

        trace!("Repairer::convert_log_files_to_tables: done");
    }
}
