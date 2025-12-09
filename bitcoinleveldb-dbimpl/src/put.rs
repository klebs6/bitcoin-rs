// ---------------- [ File: bitcoinleveldb-dbimpl/src/put.rs ]
crate::ix!();

impl Put for DBImpl {

    /**
      | Convenience methods
      |
      */
    fn put(&mut self, 
        o:   &WriteOptions,
        key_: &Slice,
        val: &Slice) -> crate::Status {
        
        todo!();
        /*
            return DB::Put(o, key, val);
        */
    }
}
