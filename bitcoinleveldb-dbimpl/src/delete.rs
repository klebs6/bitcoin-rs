// ---------------- [ File: bitcoinleveldb-dbimpl/src/delete.rs ]
crate::ix!();

impl Delete for DBImpl {

    fn delete(&mut self, 
        options: &WriteOptions,
        key_:     &Slice) -> crate::Status {
        
        todo!();
        /*
            return DB::Delete(options, key);
        */
    }
}
