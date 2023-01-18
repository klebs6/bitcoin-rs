crate::ix!();

/**
  | Batch of changes queued to be written
  | to a CDBWrapper
  |
  */
pub struct DBBatch {
    parent:        Rc<DBWrapper>,
    batch:         leveldb::WriteBatch,
    ss_key:        DataStream,
    ss_value:      DataStream,
    size_estimate: usize,
}

impl DBBatch {

    /**
      | @param[in] _parent
      | 
      | CDBWrapper that this batch is to be submitted
      | to
      |
      */
    pub fn new(parent: &DBWrapper) -> Self {

        todo!();
        /*
        : parent(_parent),
        : ss_key(SER_DISK, CLIENT_VERSION),
        : ss_value(SER_DISK, CLIENT_VERSION),
        : size_estimate(0),

        */
    }
    
    pub fn clear(&mut self)  {
        self.batch.clear();
        self.size_estimate = 0;
    }
    
    pub fn write<K, V>(&mut self, 
        key:   &K,
        value: &V)  {

        self.ss_key.reserve(DBWRAPPER_PREALLOC_KEY_SIZE);

        self.ss_key.stream(&key);

        let sl_key: leveldb::Slice 
        = leveldb::Slice::from_ptr_len(self.ss_key.data() as *mut u8, self.ss_key.size());
    
        self.ss_value.reserve(DBWRAPPER_PREALLOC_VALUE_SIZE);

        self.ss_value.stream(&value);

        self.ss_value.xor(dbwrapper::get_obfuscate_key(&self.parent));

        let sl_value: leveldb::Slice 
        = leveldb::Slice::from_ptr_len(self.ss_value.data() as *mut u8, self.ss_value.size());

        self.batch.put(&sl_key, &sl_value);

        //  LevelDB serializes writes as:
        //  - byte: header
        //  - varint: key length (1 byte up to 127B, 2 bytes up to 16383B, ...)
        //  - byte[]: key
        //  - varint: value length
        //  - byte[]: value
        //  The formula below assumes the key and value are both less than 16k.
        self.size_estimate += 
        3 
        + match sl_key.size() > 127 { true => 1, false => 0 }
        + sl_key.size() 
        + match sl_value.size() > 127 { true => 1, false => 0 }
        + sl_value.size();

        self.ss_key.clear();
        self.ss_value.clear();
    }
    
    pub fn erase<K>(&mut self, key: &K)  {

        self.ss_key.reserve(DBWRAPPER_PREALLOC_KEY_SIZE);

        self.ss_key.stream(&key);

        let sl_key: leveldb::Slice 
        = leveldb::Slice::from_ptr_len(self.ss_key.data() as *mut u8, self.ss_key.size());

        self.batch.delete(&sl_key);

        //  LevelDB serializes erases as:
        //  - byte: header
        //  - varint: key length
        //  - byte[]: key
        //  The formula below assumes the key is less than 16kB.
        self.size_estimate += 
        2 
        + match sl_key.size() > 127 { true => 1, false => 1 }
        + sl_key.size();

        self.ss_key.clear();
    }
    
    pub fn size_estimate(&self) -> usize {
        self.size_estimate
    }
}

