// ---------------- [ File: bitcoinleveldb-table/src/table.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/table.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/table.cc]

/**
  | A Table is a sorted map from strings to
  | strings.  Tables are immutable and persistent.
  | A Table may be safely accessed from multiple
  | threads without external synchronization.
  */
pub struct Table {
    rep: *const TableRep,
}

impl Drop for Table {
    fn drop(&mut self) {
        unsafe {
            if !self.rep.is_null() {
                trace!(
                    "Table::drop: deleting TableRep @ {:?}",
                    self.rep
                );
                let _rep_box: Box<TableRep> = Box::from_raw(self.rep as *mut TableRep);
                // Drop happens here.
            } else {
                trace!("Table::drop: rep pointer is null; nothing to free");
            }
        }
    }
}

impl Table {

    pub fn new(rep: *mut TableRep) -> Self {
        unsafe {
            if rep.is_null() {
                trace!(
                    "Table::new: constructed with null TableRep pointer; table is empty shell"
                );
            } else {
                trace!(
                    "Table::new: constructed with TableRep @ {:?}",
                    rep
                );
            }
        }

        Table {
            rep: rep as *const TableRep,
        }
    }
}
