// ---------------- [ File: bitcoinleveldb-dbinterface/src/get_property.rs ]
crate::ix!();

pub trait GetProperty {

    /// DB implementations can export properties about their state via this method.  
    ///
    /// If "property" is a valid property understood by this DB implementation, fills "*value" with its current value and returns true.
    ///
    /// Otherwise returns false.
    /// 
    /// Valid property names include:
    /// 
    ///  "leveldb.num-files-at-level<N>" - return the number of files at level <N>, where <N> is an ASCII representation of a level number (e.g. "0").
    /// 
    ///  "leveldb.stats" - returns a multi-line string that describes statistics about the internal operation of the DB.
    /// 
    ///  "leveldb.sstables" - returns a multi-line string that describes all of the sstables that make up the db contents.
    /// 
    ///  "leveldb.approximate-memory-usage" - returns the approximate number of bytes of memory in use by the DB.
    ///
    fn get_property(&mut self, 
        property: &str,
        value:    *mut String) -> bool;
}
