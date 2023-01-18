crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/dbwrapper.h]

pub const DBWRAPPER_PREALLOC_KEY_SIZE:   usize = 64;
pub const DBWRAPPER_PREALLOC_VALUE_SIZE: usize = 1024;

/**
  | These should be considered an implementation
  | detail of the specific database.
  |
  */
pub mod dbwrapper {

    use super::*;

    /**
      | Handle database error by throwing dbwrapper_error
      | exception.
      |
      */
    pub fn handle_error(status: &leveldb::Status) -> Result<(),String> {
        
        if status.is_ok() {
            return Ok(());
        }

        let errmsg: String = "Fatal LevelDB error: ".to_owned() + status.to_string().as_str();

        log_printf!("%s\n", errmsg);

        log_printf!("You can use -debug=leveldb to get more complete diagnostic messages\n");

        return Err(dbwrapper_error(&errmsg).to_owned());
    }

    /**
      | Work around circular dependency, as
      | well as for testing in dbwrapper_tests.
      | 
      | Database obfuscation should be considered
      | an implementation detail of the specific
      | database.
      |
      */
    pub fn get_obfuscate_key<'a>(w: &'a DBWrapper) -> &'a Vec<u8> {
        
        &w.obfuscate_key
    }
}

///----------------------
pub struct DBWrapper {

    /**
      | custom environment this database is using
      | (may be nullptr in case of default
      | environment)
      */
    penv:          Rc<RefCell<dyn leveldb::Env>>,

    /**
      | database options used
      |
      */
    options:       leveldb::Options,

    /**
      | options used when reading from the database
      |
      */
    readoptions:   leveldb::ReadOptions,

    /**
      | options used when iterating over values
      | of the database
      |
      */
    iteroptions:   leveldb::ReadOptions,

    /**
      | options used when writing to the database
      |
      */
    writeoptions:  leveldb::WriteOptions,

    /**
      | options used when sync writing to the
      | database
      |
      */
    syncoptions:   leveldb::WriteOptions,

    /**
      | the database itself
      |
      */
    pdb:           Rc<RefCell<dyn leveldb::DB>>,

    /**
      | the name of this database
      |
      */
    name:          String,

    /**
      | a key used for optional XOR-obfuscation
      | of the database
      |
      */
    obfuscate_key: Vec<u8>,
}

lazy_static!{

    /**
      | the key under which the obfuscation
      | key is stored
      |
      | Prefixed with null character to avoid
      | collisions with other keys
      |
      | We must use a string constructor which
      | specifies length so that we copy past the
      | null-terminator.
      */
    pub static ref OBFUSCATE_KEY_KEY: String = String::from("\000obfuscate_key");

    /**
      | the length of the obfuscate key in number
      | of bytes
      |
      */
    pub static ref OBFUSCATE_KEY_NUM_BYTES: usize = 8;
}

impl Drop for DBWrapper {
    fn drop(&mut self) {
        todo!();
        /*
        delete pdb;
        pdb = nullptr;
        delete options.filter_policy;
        options.filter_policy = nullptr;
        delete options.info_log;
        options.info_log = nullptr;
        delete options.block_cache;
        options.block_cache = nullptr;
        delete penv;
        options.env = nullptr;
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/dbwrapper.cpp]
impl DBWrapper {

    pub fn read<K, V>(&self, 
        key:   &K,
        value: &mut V) -> bool {

        let mut ss_key: DataStream = DataStream::new(SER_DISK.try_into().unwrap(), CLIENT_VERSION);

        ss_key.reserve(DBWRAPPER_PREALLOC_KEY_SIZE);

        ss_key.stream(&key);

        let mut sl_key: leveldb::Slice = leveldb::Slice::from_ptr_len(ss_key.data() as *mut u8, ss_key.size());

        let mut str_value = String::default();

        let status: leveldb::Status = (*self.pdb).borrow_mut().get(&self.readoptions, &sl_key, &mut str_value as *mut String);

        if !status.is_ok() {

            if status.is_not_found() {
                return false;
            }

            log_printf!("LevelDB read failure: %s\n", status.to_string());

            dbwrapper::handle_error(&status);
        }
    
        todo!();
        /*
            try {
                DataStream ssValue(MakeUCharSpan(strValue), SER_DISK, CLIENT_VERSION);
                ssValue.Xor(obfuscate_key);
                ssValue >> value;
            } catch (const std::exception&) {
                return false;
            }
            return true;
        */
    }
    
    pub fn write<K, V>(&mut self, 
        key:   &K,
        value: &V,
        sync:  Option<bool>) -> bool {

        let sync: bool = sync.unwrap_or(false);

        todo!();
        /*
            CDBBatch batch(*this);
            batch.Write(key, value);
            return WriteBatch(batch, fSync);
        */
    }
    
    pub fn exists<K>(&self, key: &K) -> bool {
    
        todo!();
        /*
            DataStream ssKey(SER_DISK, CLIENT_VERSION);
            ssKey.reserve(DBWRAPPER_PREALLOC_KEY_SIZE);
            ssKey << key;
            leveldb::Slice slKey((const char*)ssKey.data(), ssKey.len());

            std::string strValue;
            leveldb::Status status = pdb->Get(readoptions, slKey, &strValue);
            if (!status.ok()) {
                if (status.IsNotFound())
                    return false;
                LogPrintf("LevelDB read failure: %s\n", status.ToString());
                dbwrapper::HandleError(status);
            }
            return true;
        */
    }
    
    pub fn erase<K>(&mut self, 
        key:  &K,
        sync: Option<bool>) -> bool {

        let sync: bool = sync.unwrap_or(false);

        todo!();
        /*
            CDBBatch batch(*this);
            batch.Erase(key);
            return WriteBatch(batch, fSync);
        */
    }
    
    pub fn new_iterator(&mut self) -> *mut DBIterator {
        
        todo!();
        /*
            return new CDBIterator(*this, pdb->NewIterator(iteroptions));
        */
    }

    pub fn estimate_size<K>(&self, 
        key_begin: &K,
        key_end:   &K) -> usize {
    
        todo!();
        /*
            DataStream ssKey1(SER_DISK, CLIENT_VERSION), ssKey2(SER_DISK, CLIENT_VERSION);
            ssKey1.reserve(DBWRAPPER_PREALLOC_KEY_SIZE);
            ssKey2.reserve(DBWRAPPER_PREALLOC_KEY_SIZE);
            ssKey1 << key_begin;
            ssKey2 << key_end;
            leveldb::Slice slKey1((const char*)ssKey1.data(), ssKey1.len());
            leveldb::Slice slKey2((const char*)ssKey2.data(), ssKey2.len());
            uint64_t size = 0;
            leveldb::Range range(slKey1, slKey2);
            pdb->GetApproximateSizes(&range, 1, &size);
            return size;
        */
    }

    /**
      | Compact a certain range of keys in the
      | database.
      |
      */
    pub fn compact_range<K>(&self, 
        key_begin: &K,
        key_end:   &K)  {
    
        todo!();
        /*
            DataStream ssKey1(SER_DISK, CLIENT_VERSION), ssKey2(SER_DISK, CLIENT_VERSION);
            ssKey1.reserve(DBWRAPPER_PREALLOC_KEY_SIZE);
            ssKey2.reserve(DBWRAPPER_PREALLOC_KEY_SIZE);
            ssKey1 << key_begin;
            ssKey2 << key_end;
            leveldb::Slice slKey1((const char*)ssKey1.data(), ssKey1.len());
            leveldb::Slice slKey2((const char*)ssKey2.data(), ssKey2.len());
            pdb->CompactRange(&slKey1, &slKey2);
        */
    }
    
    /**
      | @param[in] path
      | 
      | Location in the filesystem where leveldb
      | data will be stored.
      | ----------
      | @param[in] nCacheSize
      | 
      | Configures various leveldb cache settings.
      | ----------
      | @param[in] fMemory
      | 
      | If true, use leveldb's memory environment.
      | ----------
      | @param[in] fWipe
      | 
      | If true, remove all existing data.
      | ----------
      | @param[in] obfuscate
      | 
      | If true, store data obfuscated via simple
      | XOR. If false, XOR with a zero'd byte
      | array.
      |
      */
    pub fn new(
        path:         &Path,
        n_cache_size: usize,
        memory:       Option<bool>,
        wipe:         Option<bool>,
        obfuscate:    Option<bool>) -> Self {

        let memory:    bool = memory.unwrap_or(false);
        let wipe:      bool = wipe.unwrap_or(false);
        let obfuscate: bool = obfuscate.unwrap_or(false);
    
        todo!();
        /*


            : m_name{fs::PathToString(path.stem())}

        penv = nullptr;
        readoptions.verify_checksums = true;
        iteroptions.verify_checksums = true;
        iteroptions.fill_cache = false;
        syncoptions.sync = true;
        options = GetOptions(nCacheSize);
        options.create_if_missing = true;
        if (fMemory) {
            penv = leveldb::NewMemEnv(leveldb::Env::Default());
            options.env = penv;
        } else {
            if (fWipe) {
                LogPrintf("Wiping LevelDB in %s\n", fs::PathToString(path));
                leveldb::Status result = leveldb::DestroyDB(fs::PathToString(path), options);
                dbwrapper_:HandleError(result);
            }
            TryCreateDirectories(path);
            LogPrintf("Opening LevelDB in %s\n", fs::PathToString(path));
        }
        leveldb::Status status = leveldb::DB::Open(options, fs::PathToString(path), &pdb);
        dbwrapper_:HandleError(status);
        LogPrintf("Opened LevelDB successfully\n");

        if (gArgs.GetBoolArg("-forcecompactdb", false)) {
            LogPrintf("Starting database compaction of %s\n", fs::PathToString(path));
            pdb->CompactRange(nullptr, nullptr);
            LogPrintf("Finished database compaction of %s\n", fs::PathToString(path));
        }

        // The base-case obfuscation key, which is a noop.
        obfuscate_key = std::vector<unsigned char>(OBFUSCATE_KEY_NUM_BYTES, '\000');

        bool key_exists = Read(OBFUSCATE_KEY_KEY, obfuscate_key);

        if (!key_exists && obfuscate && IsEmpty()) {
            // Initialize non-degenerate obfuscation if it won't upset
            // existing, non-obfuscated data.
            std::vector<unsigned char> new_key = CreateObfuscateKey();

            // Write `new_key` so we don't obfuscate the key with itself
            Write(OBFUSCATE_KEY_KEY, new_key);
            obfuscate_key = new_key;

            LogPrintf("Wrote new obfuscate key for %s: %s\n", fs::PathToString(path), HexStr(obfuscate_key));
        }

        LogPrintf("Using obfuscation key for %s: %s\n", fs::PathToString(path), HexStr(obfuscate_key));
        */
    }
    
    pub fn write_batch(&mut self, 
        batch: &mut DBBatch,
        sync:  Option<bool>) -> bool {

        let sync: bool = sync.unwrap_or(false);
        
        let log_memory: bool = log_accept_category(LogFlags::LEVELDB);

        let mut mem_before: f64 = 0.0;

        todo!();
        /*
        if (log_memory) {
            mem_before = DynamicMemoryUsage() / 1024.0 / 1024;
        }
        leveldb::Status status = pdb->Write(fSync ? syncoptions : writeoptions, &batch.batch);
        dbwrapper::HandleError(status);
        if (log_memory) {
            double mem_after = DynamicMemoryUsage() / 1024.0 / 1024;
            LogPrint(BCLog::LEVELDB, "WriteBatch memory usage: db=%s, before=%.1fMiB, after=%.1fMiB\n",
                     m_name, mem_before, mem_after);
        }
        return true;
        */
    }
    
    /**
      | Get an estimate of LevelDB memory usage
      | (in bytes).
      |
      */
    pub fn dynamic_memory_usage(&self) -> Option<usize> {

        let mut memory = String::default();
        let mut parsed: Option<usize> = None;

        let log_fail = || {
            log_print!(BCLog::LEVELDB, "Failed to get approximate-memory-usage property\n");
        };
        
        if !(*self.pdb).borrow_mut().get_property("leveldb.approximate-memory-usage", &mut memory as *mut String) 
        {
            log_fail();
            return None;

        } else {

            let maybe_val = memory.parse::<usize>().ok();

            if maybe_val.is_none() {

                log_fail();
                return None;
            }

            parsed = maybe_val;
        }

        parsed
    }

    /**
      | Returns a string (consisting of 8 random
      | bytes) suitable for use as an obfuscating
      | XOR key.
      |
      */
    pub fn create_obfuscate_key(&self) -> Vec<u8> {
        
        let mut ret: Vec::<u8> = Vec::<u8>::with_capacity(*OBFUSCATE_KEY_NUM_BYTES);

        get_rand_bytes(ret.as_mut_slice(), *OBFUSCATE_KEY_NUM_BYTES as i32);

        return ret;
    }
    
    /**
      | Return true if the database managed
      | by this class contains no entries.
      |
      */
    pub fn is_empty(&mut self) -> bool {
        
        let mut it: Box::<DBIterator> = unsafe { Box::<DBIterator>::from_raw(self.new_iterator()) };

        (*it).seek_to_first();

        !((*it).valid())
    }
}
