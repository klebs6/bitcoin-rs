// ---------------- [ File: bitcoinleveldb-dbconstructor/src/db_constructor.rs ]
crate::ix!();
 
///--------------------
pub struct DBConstructor {
    base:       Constructor,
    comparator: Box<dyn SliceComparator>,
    db:         *mut dyn DB,
}

impl Drop for DBConstructor {
    fn drop(&mut self) {
        todo!();
        /*
            delete db_;
        */
    }
}

impl DBConstructor {

    pub fn new(cmp: Box<dyn SliceComparator>) -> Self {
    
        todo!();
        /*
        : constructor(cmp),
        : comparator(cmp),

            db_ = nullptr;
        NewDB();
        */
    }
    
    pub fn finish_impl(&mut self, 
        options: &Options,
        data:    &KVMap) -> crate::Status {
        
        todo!();
        /*
            delete db_;
        db_ = nullptr;
        NewDB();
        for (const auto& kvp : data) {
          WriteBatch batch;
          batch.Put(kvp.first, kvp.second);
          ASSERT_TRUE(db_->Write(WriteOptions(), &batch).ok());
        }
        return Status::OK();
        */
    }
    
    pub fn new_iterator(&self) -> *mut LevelDBIterator {
        
        todo!();
        /*
            return db_->NewIterator(ReadOptions());
        */
    }
    
    pub fn db(&self) -> *mut dyn DB {
        
        todo!();
        /*
            return db_;
        */
    }
    
    pub fn newdb(&mut self)  {
        
        todo!();
        /*
            std::string name = test::TmpDir() + "/table_testdb";

        Options options;
        options.comparator = comparator_;
        Status status = DestroyDB(name, options);
        ASSERT_TRUE(status.ok()) << status.ToString();

        options.create_if_missing = true;
        options.error_if_exists = true;
        options.write_buffer_size = 10000;  // Something small to force merging
        status = DB::Open(options, name, &db_);
        ASSERT_TRUE(status.ok()) << status.ToString();
        */
    }
}
