// ---------------- [ File: bitcoinleveldb-db/src/db.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/db.h]

// Update CMakeLists.txt if you change these
pub const MAJOR_VERSION: i32 = 1;
pub const MINOR_VERSION: i32 = 22;

/**
  | Destroy the contents of the specified database.
  | Be very careful using this method.
  |
  | Note: For backwards compatibility, if DestroyDB
  | is unable to list the database files,
  | Status::OK() will still be returned masking
  | this failure.
  */
pub fn destroydb(
        name:    &String,
        options: &Options) -> crate::Status {
    
    todo!();
        /*
        
        */
}

/**
  | If a DB cannot be opened, you may attempt to
  | call this method to resurrect as much of the
  | contents of the database as possible.
  |
  | Some data may be lost, so be careful when
  | calling this function on a database that
  | contains important information.
  */
pub fn repairdb(
        dbname:  &String,
        options: &Options) -> crate::Status {
    
    todo!();
        /*
        
        */
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/c.cc]

pub struct LevelDB {
    rep: Rc<RefCell<dyn DB>>,
}

pub struct LevelDBSnapshot {
    rep: Rc<dyn Snapshot>,
}

pub struct LevelDBReadOptions {
    rep: ReadOptions,
}

pub struct LevelDBWriteOptions {
    rep: WriteOptions,
}

pub struct LevelDBOptions {
    rep: Options,
}

pub struct LevelDBCache {
    rep: Rc<RefCell<crate::Cache>>,
}

pub struct LevelDBSeqFile {
    rep: Rc<RefCell<dyn SequentialFile>>,
}

pub struct LevelDBRandomFile {
    rep: Rc<RefCell<dyn RandomAccessFile>>,
}

pub struct LevelDBWritableFile {
    rep: Rc<RefCell<dyn WritableFile>>,
}

pub struct LevelDBLogger {
    rep: Rc<RefCell<dyn Logger>>,
}

pub struct LevelDBFileLock {
    rep: Rc<RefCell<Box<dyn FileLock>>>,
}

///-----------------
pub struct LevelDBComparator {

    state:      *mut c_void,

    destructor: fn(_0: *mut c_void) -> c_void,

    compare:    fn(
            _0:   *mut c_void,
            a:    *const u8,
            alen: usize,
            b:    *const u8,
            blen: usize
    ) -> i32,

    name:       fn(_0: *mut c_void) -> *const u8,
}

impl Comparator<Slice> for LevelDBComparator {
    fn compare(&self, 
        a: &Slice,
        b: &Slice) -> Ordering {
        
        todo!();
        /*
            return (*compare_)(state_, a.data(), a.size(), b.data(), b.size());
        */
    }
}

impl FindShortestSeparator for LevelDBComparator {

    /**
      | No-ops since the C binding does not support
      | key shortening methods.
      |
      */
    fn find_shortest_separator(&self, 
        _0: &mut Vec<u8>,
        _1: &[u8])  {
        
        todo!();
        /*
        
        */
    }
}

impl FindShortSuccessor for LevelDBComparator {

    fn find_short_successor(&self, key_: &mut Vec<u8>)  {
        
        todo!();
        /*
        
        */
    }
}

impl Drop for LevelDBComparator {
    fn drop(&mut self) {
        todo!();
        /*
            (*destructor_)(state_);
        */
    }
}

impl Name for LevelDBComparator {

    fn name(&self) -> *const u8 {
        
        todo!();
        /*
            return (*name_)(state_);
        */
    }
}

///-----------------
pub struct LevelDBFilterPolicy {
    state:      *mut c_void,

    destructor: fn(_0: *mut c_void) -> c_void,

    name:       fn(_0: *mut c_void) -> *const u8,

    create:     fn(
            _0:               *mut c_void,
            key_array:        *const *const u8,
            key_length_array: *const usize,
            num_keys:         i32,
            filter_length:    *mut usize
    ) -> *mut u8,

    key_match:  fn(
            _0:            *mut c_void,
            key_:           *const u8,
            length:        usize,
            filter:        *const u8,
            filter_length: usize
    ) -> u8,
}

impl FilterPolicy for LevelDBFilterPolicy {

}

impl Drop for LevelDBFilterPolicy {
    fn drop(&mut self) {
        todo!();
        /*
            (*destructor_)(state_);
        */
    }
}

impl Name for LevelDBFilterPolicy {
    fn name(&self) -> *const u8 {
        
        todo!();
        /*
            return (*name_)(state_);
        */
    }
}

impl CreateFilter for LevelDBFilterPolicy {

    fn create_filter(
        &self, 
        keys: *const Slice,
        n:    i32,
        dst:  &mut Vec<u8>)  {
        
        todo!();
        /*
            std::vector<const char*> key_pointers(n);
            std::vector<size_t> key_sizes(n);
            for (int i = 0; i < n; i++) {
              key_pointers[i] = keys[i].data();
              key_sizes[i] = keys[i].size();
            }
            size_t len;
            char* filter = (*create_)(state_, &key_pointers[0], &key_sizes[0], n, &len);
            dst->append(filter, len);
            free(filter);
        */
    }
}

impl KeyMayMatch for LevelDBFilterPolicy {
    fn key_may_match(&self, 
        key_:    &Slice,
        filter: &Slice) -> bool {
        
        todo!();
        /*
            return (*key_match_)(state_, key.data(), key.size(), filter.data(),
                                 filter.size());
        */
    }
}

///-----------------
pub struct LevelDBEnv {
    rep:        Rc<RefCell<dyn Env>>,
    is_default: bool,
}

pub fn save_error(
        errptr: *mut *mut u8,
        s:      &Status) -> bool {
    
    todo!();
        /*
            assert(errptr != nullptr);
          if (s.ok()) {
            return false;
          } else if (*errptr == nullptr) {
            *errptr = strdup(s.ToString().c_str());
          } else {
            // TODO(sanjay): Merge with existing error?
            free(*errptr);
            *errptr = strdup(s.ToString().c_str());
          }
          return true;
        */
}

pub fn copy_string(str_: &String) -> *mut u8 {
    
    todo!();
        /*
            char* result = reinterpret_cast<char*>(malloc(sizeof(char) * str.size()));
          memcpy(result, str.data(), sizeof(char) * str.size());
          return result;
        */
}

pub fn leveldb_open(
        options: *const LevelDBOptions,
        name:    *const u8,
        errptr:  *mut *mut u8) -> *mut LevelDB {
    
    todo!();
        /*
            DB* db;
          if (SaveError(errptr, DB::Open(options->rep, std::string(name), &db))) {
            return nullptr;
          }
          leveldb_t* result = new leveldb_t;
          result->rep = db;
          return result;
        */
}

pub fn leveldb_close(db: *mut LevelDB)  {
    
    todo!();
        /*
            delete db->rep;
          delete db;
        */
}

pub fn leveldb_put(
        db:      *mut LevelDB,
        options: *const LevelDBWriteOptions,
        key_:     *const u8,
        keylen:  usize,
        val:     *const u8,
        vallen:  usize,
        errptr:  *mut *mut u8)  {
    
    todo!();
        /*
            SaveError(errptr,
                    db->rep->Put(options->rep, Slice(key, keylen), Slice(val, vallen)));
        */
}

pub fn leveldb_delete(
        db:      *mut LevelDB,
        options: *const LevelDBWriteOptions,
        key_:     *const u8,
        keylen:  usize,
        errptr:  *mut *mut u8)  {
    
    todo!();
        /*
            SaveError(errptr, db->rep->Delete(options->rep, Slice(key, keylen)));
        */
}

pub fn leveldb_write(
        db:      *mut LevelDB,
        options: *const LevelDBWriteOptions,
        batch:   *mut LevelDBWriteBatch,
        errptr:  *mut *mut u8)  {
    
    todo!();
        /*
            SaveError(errptr, db->rep->Write(options->rep, &batch->rep));
        */
}

pub fn leveldb_get(
        db:      *mut LevelDB,
        options: *const LevelDBReadOptions,
        key_:     *const u8,
        keylen:  usize,
        vallen:  *mut usize,
        errptr:  *mut *mut u8) -> *mut u8 {
    
    todo!();
        /*
            char* result = nullptr;
          std::string tmp;
          Status s = db->rep->Get(options->rep, Slice(key, keylen), &tmp);
          if (s.ok()) {
            *vallen = tmp.size();
            result = CopyString(tmp);
          } else {
            *vallen = 0;
            if (!s.IsNotFound()) {
              SaveError(errptr, s);
            }
          }
          return result;
        */
}

pub fn leveldb_create_iterator(
        db:      *mut LevelDB,
        options: *const LevelDBReadOptions) -> *mut LevelDBIterator {
    
    todo!();
        /*
            leveldb_iterator_t* result = new leveldb_iterator_t;
          result->rep = db->rep->NewIterator(options->rep);
          return result;
        */
}

pub fn leveldb_create_snapshot(db: *mut LevelDB) -> *const LevelDBSnapshot {
    
    todo!();
        /*
            leveldb_snapshot_t* result = new leveldb_snapshot_t;
          result->rep = db->rep->GetSnapshot();
          return result;
        */
}

pub fn leveldb_release_snapshot(
        db:       *mut LevelDB,
        snapshot: *const LevelDBSnapshot)  {
    
    todo!();
        /*
            db->rep->ReleaseSnapshot(snapshot->rep);
          delete snapshot;
        */
}

pub fn leveldb_property_value(
        db:       *mut LevelDB,
        propname: *const u8) -> *mut u8 {
    
    todo!();
        /*
            std::string tmp;
          if (db->rep->GetProperty(Slice(propname), &tmp)) {
            // We use strdup() since we expect human readable output.
            return strdup(tmp.c_str());
          } else {
            return nullptr;
          }
        */
}

pub fn leveldb_approximate_sizes(
        db:                  *mut LevelDB,
        num_ranges:          i32,
        range_start_key_:     *const *const u8,
        range_start_key_len: *const usize,
        range_limit_key_:     *const *const u8,
        range_limit_key_len: *const usize,
        sizes:               *mut u64)  {
    
    todo!();
        /*
            Range* ranges = new Range[num_ranges];
          for (int i = 0; i < num_ranges; i++) {
            ranges[i].start = Slice(range_start_key[i], range_start_key_len[i]);
            ranges[i].limit = Slice(range_limit_key[i], range_limit_key_len[i]);
          }
          db->rep->GetApproximateSizes(ranges, num_ranges, sizes);
          delete[] ranges;
        */
}

pub fn leveldb_compact_range(
        db:            *mut LevelDB,
        start_key_:     *const u8,
        start_key_len: usize,
        limit_key_:     *const u8,
        limit_key_len: usize)  {
    
    todo!();
        /*
            Slice a, b;
          db->rep->CompactRange(
              // Pass null Slice if corresponding "const char*" is null
              (start_key ? (a = Slice(start_key, start_key_len), &a) : nullptr),
              (limit_key ? (b = Slice(limit_key, limit_key_len), &b) : nullptr));
        */
}

pub fn leveldb_destroy_db(
        options: *const LevelDBOptions,
        name:    *const u8,
        errptr:  *mut *mut u8)  {
    
    todo!();
        /*
            SaveError(errptr, DestroyDB(name, options->rep));
        */
}

pub fn leveldb_repair_db(
        options: *const LevelDBOptions,
        name:    *const u8,
        errptr:  *mut *mut u8)  {
    
    todo!();
        /*
            SaveError(errptr, RepairDB(name, options->rep));
        */
}

pub fn leveldb_iter_destroy(iter: *mut LevelDBIterator)  {
    
    todo!();
        /*
            delete iter->rep;
          delete iter;
        */
}

pub fn leveldb_iter_valid(iter: *const LevelDBIterator) -> u8 {
    
    todo!();
        /*
            return iter->rep->Valid();
        */
}

pub fn leveldb_iter_seek_to_first(iter: *mut LevelDBIterator)  {
    
    todo!();
        /*
            iter->rep->SeekToFirst();
        */
}

pub fn leveldb_iter_seek_to_last(iter: *mut LevelDBIterator)  {
    
    todo!();
        /*
            iter->rep->SeekToLast();
        */
}

pub fn leveldb_iter_seek(
        iter: *mut LevelDBIterator,
        k:    *const u8,
        klen: usize)  {
    
    todo!();
        /*
            iter->rep->Seek(Slice(k, klen));
        */
}

pub fn leveldb_iter_next(iter: *mut LevelDBIterator)  {
    
    todo!();
        /*
            iter->rep->Next();
        */
}

pub fn leveldb_iter_prev(iter: *mut LevelDBIterator)  {
    
    todo!();
        /*
            iter->rep->Prev();
        */
}

pub fn leveldb_iter_key(
        iter: *const LevelDBIterator,
        klen: *mut usize) -> *const u8 {
    
    todo!();
        /*
            Slice s = iter->rep->key();
          *klen = s.size();
          return s.data();
        */
}

pub fn leveldb_iter_value(
        iter: *const LevelDBIterator,
        vlen: *mut usize) -> *const u8 {
    
    todo!();
        /*
            Slice s = iter->rep->value();
          *vlen = s.size();
          return s.data();
        */
}

pub fn leveldb_iter_get_error(
        iter:   *const LevelDBIterator,
        errptr: *mut *mut u8)  {
    
    todo!();
        /*
            SaveError(errptr, iter->rep->status());
        */
}

pub fn leveldb_options_create() -> *mut LevelDBOptions {
    
    todo!();
        /*
            return new leveldb_options_t;
        */
}

pub fn leveldb_options_destroy(options: *mut LevelDBOptions)  {
    
    todo!();
        /*
            delete options;
        */
}

pub fn leveldb_options_set_comparator(
        opt: *mut LevelDBOptions,
        cmp: *mut LevelDBComparator)  {
    
    todo!();
        /*
            opt->rep.comparator = cmp;
        */
}

pub fn leveldb_options_set_filter_policy(
        opt:    *mut LevelDBOptions,
        policy: *mut LevelDBFilterPolicy)  {
    
    todo!();
        /*
            opt->rep.filter_policy = policy;
        */
}

pub fn leveldb_options_set_create_if_missing(
        opt: *mut LevelDBOptions,
        v:   u8)  {
    
    todo!();
        /*
            opt->rep.create_if_missing = v;
        */
}

pub fn leveldb_options_set_error_if_exists(
        opt: *mut LevelDBOptions,
        v:   u8)  {
    
    todo!();
        /*
            opt->rep.error_if_exists = v;
        */
}

pub fn leveldb_options_set_paranoid_checks(
        opt: *mut LevelDBOptions,
        v:   u8)  {
    
    todo!();
        /*
            opt->rep.paranoid_checks = v;
        */
}

pub fn leveldb_options_set_env(
        opt: *mut LevelDBOptions,
        env: *mut LevelDBEnv)  {
    
    todo!();
        /*
            opt->rep.env = (env ? env->rep : nullptr);
        */
}

pub fn leveldb_options_set_info_log(
        opt: *mut LevelDBOptions,
        l:   *mut LevelDBLogger)  {
    
    todo!();
        /*
            opt->rep.info_log = (l ? l->rep : nullptr);
        */
}

pub fn leveldb_options_set_write_buffer_size(
        opt: *mut LevelDBOptions,
        s:   usize)  {
    
    todo!();
        /*
            opt->rep.write_buffer_size = s;
        */
}

pub fn leveldb_options_set_max_open_files(
        opt: *mut LevelDBOptions,
        n:   i32)  {
    
    todo!();
        /*
            opt->rep.max_open_files = n;
        */
}

pub fn leveldb_options_set_cache(
        opt: *mut LevelDBOptions,
        c:   *mut LevelDBCache)  {
    
    todo!();
        /*
            opt->rep.block_cache = c->rep;
        */
}

pub fn leveldb_options_set_block_size(
        opt: *mut LevelDBOptions,
        s:   usize)  {
    
    todo!();
        /*
            opt->rep.block_size = s;
        */
}

pub fn leveldb_options_set_block_restart_interval(
        opt: *mut LevelDBOptions,
        n:   i32)  {
    
    todo!();
        /*
            opt->rep.block_restart_interval = n;
        */
}

pub fn leveldb_options_set_max_file_size(
        opt: *mut LevelDBOptions,
        s:   usize)  {
    
    todo!();
        /*
            opt->rep.max_file_size = s;
        */
}

pub fn leveldb_options_set_compression(
        opt: *mut LevelDBOptions,
        t:   i32)  {
    
    todo!();
        /*
            opt->rep.compression = static_cast<CompressionType>(t);
        */
}

pub fn leveldb_comparator_create(
        state:      *mut c_void,
        destructor: fn(_0: *mut c_void) -> c_void,
        compare:    fn(
                _0:   *mut c_void,
                a:    *const u8,
                alen: usize,
                b:    *const u8,
                blen: usize
        ) -> i32,
        name:       fn(_0: *mut c_void) -> *const u8) -> *mut LevelDBComparator {
    
    todo!();
        /*
            leveldb_comparator_t* result = new leveldb_comparator_t;
          result->state_ = state;
          result->destructor_ = destructor;
          result->compare_ = compare;
          result->name_ = name;
          return result;
        */
}

pub fn leveldb_comparator_destroy(cmp: *mut LevelDBComparator)  {
    
    todo!();
        /*
            delete cmp;
        */
}

pub fn leveldb_filterpolicy_create(
        state:         *mut c_void,
        destructor:    fn(_0: *mut c_void) -> c_void,
        create_filter: fn(
                _0:               *mut c_void,
                key_array:        *const *const u8,
                key_length_array: *const usize,
                num_keys:         i32,
                filter_length:    *mut usize
        ) -> *mut u8,
        key_may_match: fn(
                _0:            *mut c_void,
                key_:           *const u8,
                length:        usize,
                filter:        *const u8,
                filter_length: usize
        ) -> u8,
        name:          fn(_0: *mut c_void) -> *mut u8) -> *mut LevelDBFilterPolicy {
    
    todo!();
        /*
            leveldb_filterpolicy_t* result = new leveldb_filterpolicy_t;
          result->state_ = state;
          result->destructor_ = destructor;
          result->create_ = create_filter;
          result->key_match_ = key_may_match;
          result->name_ = name;
          return result;
        */
}

pub fn leveldb_filterpolicy_destroy(filter: *mut LevelDBFilterPolicy)  {
    
    todo!();
        /*
            delete filter;
        */
}

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

pub fn leveldb_readoptions_create() -> *mut LevelDBReadOptions {
    
    todo!();
        /*
            return new leveldb_readoptions_t;
        */
}

pub fn leveldb_readoptions_destroy(opt: *mut LevelDBReadOptions)  {
    
    todo!();
        /*
            delete opt;
        */
}

pub fn leveldb_readoptions_set_verify_checksums(
        opt: *mut LevelDBReadOptions,
        v:   u8)  {
    
    todo!();
        /*
            opt->rep.verify_checksums = v;
        */
}

pub fn leveldb_readoptions_set_fill_cache(
        opt: *mut LevelDBReadOptions,
        v:   u8)  {
    
    todo!();
        /*
            opt->rep.fill_cache = v;
        */
}

pub fn leveldb_readoptions_set_snapshot(
        opt:  *mut LevelDBReadOptions,
        snap: *const LevelDBSnapshot)  {
    
    todo!();
        /*
            opt->rep.snapshot = (snap ? snap->rep : nullptr);
        */
}

pub fn leveldb_writeoptions_create() -> *mut LevelDBWriteOptions {
    
    todo!();
        /*
            return new leveldb_writeoptions_t;
        */
}

pub fn leveldb_writeoptions_destroy(opt: *mut LevelDBWriteOptions)  {
    
    todo!();
        /*
            delete opt;
        */
}

pub fn leveldb_writeoptions_set_sync(
        opt: *mut LevelDBWriteOptions,
        v:   u8)  {
    
    todo!();
        /*
            opt->rep.sync = v;
        */
}

pub fn leveldb_cache_create_lru(capacity: usize) -> *mut LevelDBCache {
    
    todo!();
        /*
            leveldb_cache_t* c = new leveldb_cache_t;
          c->rep = NewLRUCache(capacity);
          return c;
        */
}

pub fn leveldb_cache_destroy(cache: *mut LevelDBCache)  {
    
    todo!();
        /*
            delete cache->rep;
          delete cache;
        */
}

pub fn leveldb_create_default_env() -> *mut LevelDBEnv {
    
    todo!();
        /*
            leveldb_env_t* result = new leveldb_env_t;
          result->rep = Env::Default();
          result->is_default = true;
          return result;
        */
}

pub fn leveldb_env_destroy(env: *mut LevelDBEnv)  {
    
    todo!();
        /*
            if (!env->is_default) delete env->rep;
          delete env;
        */
}

pub fn leveldb_env_get_test_directory(env: *mut LevelDBEnv) -> *mut u8 {
    
    todo!();
        /*
            std::string result;
          if (!env->rep->GetTestDirectory(&result).ok()) {
            return nullptr;
          }

          char* buffer = static_cast<char*>(malloc(result.size() + 1));
          memcpy(buffer, result.data(), result.size());
          buffer[result.size()] = '\0';
          return buffer;
        */
}

pub fn leveldb_free(ptr: *mut c_void)  {
    
    todo!();
        /*
            free(ptr);
        */
}

pub fn leveldb_major_version() -> i32 {
    
    todo!();
        /*
            return kMajorVersion;
        */
}

pub fn leveldb_minor_version() -> i32 {
    
    todo!();
        /*
            return kMinorVersion;
        */
}
