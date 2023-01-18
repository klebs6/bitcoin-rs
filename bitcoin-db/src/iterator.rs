crate::ix!();

///------------------------
pub struct DBIterator<'a> {
    parent: Rc<DBWrapper>,
    piter:  &'a mut dyn leveldb::LevelDBIteratorInterface,
}

impl<'a> Drop for DBIterator<'a> {
    fn drop(&mut self) {
        todo!();
        /*
            delete piter;
        */
    }
}

impl<'a> DBIterator<'a> {

    /**
      | @param[in] _parent
      | 
      | Parent CDBWrapper instance.
      | ----------
      | @param[in] _piter
      | 
      | The original leveldb iterator.
      |
      */
    pub fn new(
        parent: &DBWrapper,
        piter:  *mut leveldb::LevelDBIterator) -> Self {
    
        todo!();
        /*
        : parent(_parent),
        : piter(_piter),
        */
    }
    
    pub fn seek<K>(&mut self, key: &K)  {

        let mut ss_key: DataStream = DataStream::new(SER_DISK.try_into().unwrap(), CLIENT_VERSION);

        ss_key.reserve(DBWRAPPER_PREALLOC_KEY_SIZE);
        ss_key.stream(&key);

        let sl_key: leveldb::Slice 
        = leveldb::Slice::from_ptr_len(ss_key.data() as *mut u8, ss_key.size());

        (*self.piter).seek(&sl_key);
    }
    
    pub fn get_key<K>(&mut self, key: &mut K) -> bool {
    
        let sl_key: leveldb::Slice = (*self.piter).key();

        let mut try_block = || -> TryBlockResult::<_,&'static str> {

            let slice = unsafe { 
                std::ptr::slice_from_raw_parts(
                    sl_key.data(), 
                    sl_key.size()
                ).as_ref().unwrap()
            };

            let mut ss_key: DataStream 
                = DataStream::new_with_slice(
                    slice, 
                    SER_DISK.try_into().unwrap(), 
                    CLIENT_VERSION
                );

            ss_key.stream_into(&mut *key);

            TryBlockResult::Success
        };

        match try_block() {
            TryBlockResult::Return(v)  => return v,
            TryBlockResult::Err(e)  => {
                return false;
            },

            TryBlockResult::Success => { }
            TryBlockResult::Break   => { }
        }

        true
    }
    
    pub fn get_value<V>(&mut self, value: &mut V) -> bool {
    
        let sl_value: leveldb::Slice = (*self.piter).value();

        todo!();
        /*
            try {
                DataStream ssValue(MakeUCharSpan(slValue), SER_DISK, CLIENT_VERSION);
                ssValue.Xor(dbwrapper_:GetObfuscateKey(parent));
                ssValue >> value;
            } catch (const std::exception&) {
                return false;
            }
        */

        true
    }
    
    pub fn get_value_size(&mut self) -> usize {
        
        (*self.piter).value().size()
    }

    pub fn valid(&self) -> bool {
        
        (*self.piter).valid()
    }
    
    pub fn seek_to_first(&mut self)  {
        
        (*self.piter).seek_to_first();
    }
    
    pub fn next(&mut self)  {
        
        (*self.piter).next();
    }
}

