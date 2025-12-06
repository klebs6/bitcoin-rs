crate::ix!();

pub trait Delete {

    /**
      | Remove the database entry (if any) for "key".
      | Returns OK on success, and a non-OK status on
      | error.  It is not an error if "key" did not
      | exist in the database.
      |
      | Note: consider setting options.sync = true.
      */
    fn delete(&mut self, 
        opt: &WriteOptions,
        key_: &Slice) -> crate::Status {
        
        todo!();
        /*
            WriteBatch batch;
      batch.Delete(key);
      return Write(opt, &batch);
        */
    }
}
