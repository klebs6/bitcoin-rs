// ---------------- [ File: bitcoinleveldb-repair/src/extract_meta_data.rs ]
crate::ix!();

impl Repairer {

    pub fn extract_meta_data(&mut self) {
        trace!(
            table_count = self.table_numbers.len(),
            "Repairer::extract_meta_data: start"
        );

        for i in 0..self.table_numbers.len() {
            self.scan_table(self.table_numbers[i]);
        }

        trace!(
            tables_scanned = self.table_numbers.len(),
            tables_kept = self.tables.len(),
            "Repairer::extract_meta_data: done"
        );
    }
}
