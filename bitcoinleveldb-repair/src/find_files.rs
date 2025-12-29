// ---------------- [ File: bitcoinleveldb-repair/src/find_files.rs ]
crate::ix!();

impl Repairer {
    
    pub fn find_files(&mut self) -> crate::Status {
        trace!(dbname = %self.dbname, "Repairer::find_files: start");

        let mut filenames: Vec<String> = Vec::new();
        let status = self
            .env
            .get_children(&self.dbname, &mut filenames as *mut Vec<String>);

        if !status.is_ok() {
            debug!(
                dbname = %self.dbname,
                status = %status.to_string(),
                "Repairer::find_files: GetChildren failed"
            );
            return status;
        }

        if filenames.is_empty() {
            let msg1 = Slice::from(self.dbname.as_bytes());
            let msg2 = Slice::from(&b"repair found no files"[..]);
            error!(
                dbname = %self.dbname,
                "Repairer::find_files: no files found"
            );
            return crate::Status::io_error(&msg1, Some(&msg2));
        }

        let mut number: u64 = 0;
        let mut ty: FileType = FileType::LogFile;

        for i in 0..filenames.len() {
            if parse_file_name(&filenames[i], &mut number as *mut u64, &mut ty as *mut FileType)
            {
                if matches!(ty, FileType::DescriptorFile) {
                    self.manifests.push(filenames[i].clone());
                } else {
                    let next = number.wrapping_add(1);
                    if next > self.next_file_number {
                        self.next_file_number = next;
                    }

                    match ty {
                        FileType::LogFile => {
                            self.logs.push(number);
                        }
                        FileType::TableFile => {
                            self.table_numbers.push(number);
                        }
                        _ => {
                            // Ignore other files
                        }
                    }
                }
            }
        }

        debug!(
            manifests = self.manifests.len(),
            logs = self.logs.len(),
            tables = self.table_numbers.len(),
            next_file_number = self.next_file_number,
            "Repairer::find_files: done"
        );

        status
    }
}
