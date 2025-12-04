// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder_file_size.rs ]
crate::ix!();

impl TableBuilder {

    /**
      | Size of the file generated so far. If
      | invoked after a successful Finish()
      | call, returns the size of the final generated
      | file.
      |
      */
    pub fn file_size(&self) -> u64 {
        unsafe {
            assert!(
                !self.rep.is_null(),
                "TableBuilder::file_size: rep pointer is null"
            );
            let r = &*self.rep;
            r.offset
        }
    }

    pub fn ok(&self) -> bool {
        self.status().is_ok()
    }
}
