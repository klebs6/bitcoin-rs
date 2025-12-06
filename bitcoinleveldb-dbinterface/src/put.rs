crate::ix!();

pub trait Put {

    /**
      | Set the database entry for "key" to "value".
      | Returns OK on success, and a non-OK status on
      | error.
      |
      | Note: consider setting options.sync = true.
      |
      | Default implementations of convenience
      | methods that subclasses of DB can call
      | if they wish
      |
      */
    fn put(&mut self, 
        opt:   &WriteOptions,
        key_:   &Slice,
        value: &Slice) -> crate::Status {
        
        todo!();
        /*
            WriteBatch batch;
      batch.Put(key, value);
      return Write(opt, &batch);
        */
    }
}
