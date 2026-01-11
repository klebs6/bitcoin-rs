// ---------------- [ File: bitcoinleveldb-modeldb/src/modeldb.rs ]
crate::ix!();

pub struct ModelDB {
    base:    Box<dyn DB>,
    options: Options,
    map:     KVMap,
}

impl ModelDB {

    pub fn new(options: &Options) -> Self {
    
        todo!();
        /*
        : options(options),

        
        */
    }
    
    pub fn put(&mut self, 
        o: &WriteOptions,
        k: &Slice,
        v: &Slice) -> crate::Status {
        
        todo!();
        /*
            return DB::Put(o, k, v);
        */
    }
    
    pub fn delete(&mut self, 
        o:   &WriteOptions,
        key_: &Slice) -> crate::Status {
        
        todo!();
        /*
            return DB::Delete(o, key);
        */
    }
    
    pub fn get(&mut self, 
        options: &ReadOptions,
        key_:     &Slice,
        value:   *mut String) -> crate::Status {
        
        todo!();
        /*
            assert(false);  // Not implemented
        return Status::NotFound(key);
        */
    }
    
    pub fn new_iterator(&mut self, options: &ReadOptions) -> *mut LevelDBIterator {
        
        todo!();
        /*
            if (options.snapshot == nullptr) {
          KVMap* saved = new KVMap;
          *saved = map_;
          return new ModelIter(saved, true);
        } else {
          const KVMap* snapshot_state =
              &(reinterpret_cast<const ModelSnapshot*>(options.snapshot)->map_);
          return new ModelIter(snapshot_state, false);
        }
        */
    }
    
    pub fn get_snapshot(&mut self) -> *const dyn Snapshot {
        
        todo!();
        /*
            ModelSnapshot* snapshot = new ModelSnapshot;
        snapshot->map_ = map_;
        return snapshot;
        */
    }
    
    pub fn release_snapshot(&mut self, snapshot: *const dyn Snapshot)  {
        
        todo!();
        /*
            delete reinterpret_cast<const ModelSnapshot*>(snapshot);
        */
    }
    
    pub fn write(&mut self, 
        options: &WriteOptions,
        batch:   *mut WriteBatch) -> crate::Status {
        
        todo!();
        /*
            class Handler : public WriteBatch::Handler {
         
          KVMap* map_;
          c_void Put(const Slice& key, const Slice& value) override {
            (*map_)[key.ToString()] = value.ToString();
          }
          c_void Delete(const Slice& key) override { map_->erase(key.ToString()); }
        };
        Handler handler;
        handler.map_ = &map_;
        return batch->Iterate(&handler);
        */
    }
    
    pub fn get_property(&mut self, 
        property: &str,
        value:    *mut String) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn get_approximate_sizes(&mut self, 
        r:     *const bitcoinleveldb_slice::Range,
        n:     i32,
        sizes: *mut u64)  {
        
        todo!();
        /*
            for (int i = 0; i < n; i++) {
          sizes[i] = 0;
        }
        */
    }
    
    pub fn compact_range(&mut self, 
        start: *const Slice,
        end:   *const Slice)  {
        
        todo!();
        /*
        
        */
    }
}
