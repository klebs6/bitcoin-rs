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

impl Table {

    #[inline]
    pub fn rep_ptr(&self) -> *const TableRep {
        trace!(
            "Table::rep_ptr: returning rep pointer {:?}",
            self.rep
        );
        self.rep
    }

    #[inline]
    pub fn rep_mut_ptr(&mut self) -> *mut TableRep {
        trace!(
            "Table::rep_mut_ptr: returning mutable rep pointer {:?}",
            self.rep as *mut TableRep
        );
        self.rep as *mut TableRep
    }

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

impl Drop for Table {

    fn drop(&mut self) {
        unsafe {
            let rep_ptr = self.rep_mut_ptr();
            if !rep_ptr.is_null() {
                trace!(
                    "Table::drop: deleting TableRep @ {:?}",
                    rep_ptr
                );
                let _rep_box: Box<TableRep> = Box::from_raw(rep_ptr);
            } else {
                trace!("Table::drop: rep pointer is null; nothing to free");
            }
        }
    }
}
