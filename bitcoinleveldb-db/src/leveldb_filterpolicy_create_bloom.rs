// ---------------- [ File: bitcoinleveldb-db/src/leveldb_filterpolicy_create_bloom.rs ]
crate::ix!();

pub fn leveldb_filterpolicy_create_bloom(bits_per_key_: i32) -> *mut LevelDBFilterPolicy {
    
    todo!();
        /*
            // Make a leveldb_filterpolicy_t, but override all of its methods so
          // they delegate to a NewBloomFilterPolicy() instead of user
          // supplied C functions.
          struct Wrapper : public leveldb_filterpolicy_t {
            static c_void DoNothing(c_void*) {}

            ~Wrapper() { delete rep_; }
            const char* Name() const { return rep_->Name(); }
            c_void CreateFilter(const Slice* keys, int n, std::string* dst) const {
              return rep_->CreateFilter(keys, n, dst);
            }
            bool KeyMayMatch(const Slice& key, const Slice& filter) const {
              return rep_->KeyMayMatch(key, filter);
            }

            const FilterPolicy* rep_;
          };
          Wrapper* wrapper = new Wrapper;
          wrapper->rep_ = NewBloomFilterPolicy(bits_per_key);
          wrapper->state_ = nullptr;
          wrapper->destructor_ = &Wrapper::DoNothing;
          return wrapper;
        */
}
