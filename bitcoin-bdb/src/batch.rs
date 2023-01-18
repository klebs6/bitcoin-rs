crate::ix!();

/* ----------------- BerkeleyBatch  ----------------- */

/**
  | RAII class that provides access to a
  | Berkeley database
  |
  */
pub struct BerkeleyBatch {
    base:           DatabaseBatch,
    pdb:            Rc<RefCell<libdb::Db>>,
    str_file:       String,
    active_txn:     Rc<RefCell<libdb::Transaction>>,
    cursor:         Rc<RefCell<libdb::Cursor>>,
    read_only:      bool,
    flush_on_close: bool,
    env:            Rc<RefCell<BerkeleyEnvironment>>,
    database:       Rc<RefCell<BerkeleyDatabase>>,
}

impl Drop for BerkeleyBatch {
    fn drop(&mut self) {
        todo!();
        /*
            Close();
        m_database.RemoveRef();
        */
    }
}

pub mod berkeley_batch {

    use super::*;

    /**
      | RAII class that automatically cleanses
      | its data on destruction
      |
      */
    pub struct SafeDbt {
        dbt: libdb::DBT,
    }

    impl Drop for SafeDbt {
        fn drop(&mut self) {
            todo!();
            /*
                if (m_dbt.get_data() != nullptr) {
                // Clear memory, e.g. in case it was a private key
                memory_cleanse(m_dbt.get_data(), m_dbt.get_size());
                // under DB_DBT_MALLOC, data is malloced by the Dbt, but must be
                // freed by the caller.
                // https://docs.oracle.com/cd/E17275_01/html/api_reference/C/dbt.html
                if (m_dbt.get_flags() & DB_DBT_MALLOC) {
                    free(m_dbt.get_data());
                }
            }
            */
        }
    }

    impl Into<libdb::DBT> for SafeDbt {
        
        /**
          | conversion operator to access the underlying
          | Dbt
          |
          */
        #[inline] fn into(self) -> libdb::DBT {
            todo!();
            /*
                return &m_dbt;
            */
        }
    }

    impl Default for SafeDbt {

        /**
          | construct Dbt with internally-managed
          | data
          |
          */
        fn default() -> Self {
        
            todo!();
            /*
                m_dbt.set_flags(DB_DBT_MALLOC);
            */
        }
    }

    impl SafeDbt {

        /**
          | construct Dbt with provided data
          |
          */
        pub fn new(
            data: *mut c_void,
            size: usize) -> Self {
        
            todo!();
            /*
                : m_dbt(data, size)
            */
        }
        
        /**
          | delegate to Dbt
          |
          */
        pub fn get_data(&self)  {
            
            todo!();
            /*
                return m_dbt.get_data();
            */
        }
        
        pub fn get_size(&self) -> u32 {
            
            todo!();
            /*
                return m_dbt.get_size();
            */
        }
    }
}

impl BerkeleyBatch {

    pub fn close(&mut self)  {
        
        todo!();
        /*
            if (!pdb)
            return;
        if (activeTxn)
            activeTxn->abort();
        activeTxn = nullptr;
        pdb = nullptr;
        CloseCursor();

        if (fFlushOnClose)
            Flush();
        */
    }
    
    pub fn start_cursor(&mut self) -> bool {
        
        todo!();
        /*
            assert(!m_cursor);
        if (!pdb)
            return false;
        int ret = pdb->cursor(nullptr, &m_cursor, 0);
        return ret == 0;
        */
    }
    
    pub fn read_at_cursor(&mut self, 
        ss_key:   &mut DataStream,
        ss_value: &mut DataStream,
        complete: &mut bool) -> bool {
        
        todo!();
        /*
            complete = false;
        if (m_cursor == nullptr) return false;
        // Read at cursor
        SafeDbt datKey;
        SafeDbt datValue;
        int ret = m_cursor->get(datKey, datValue, DB_NEXT);
        if (ret == DB_NOTFOUND) {
            complete = true;
        }
        if (ret != 0)
            return false;
        else if (datKey.get_data() == nullptr || datValue.get_data() == nullptr)
            return false;

        // Convert to streams
        ssKey.SetType(SER_DISK);
        ssKey.clear();
        ssKey.write((char*)datKey.get_data(), datKey.get_size());
        ssValue.SetType(SER_DISK);
        ssValue.clear();
        ssValue.write((char*)datValue.get_data(), datValue.get_size());
        return true;
        */
    }
    
    pub fn close_cursor(&mut self)  {
        
        todo!();
        /*
            if (!m_cursor) return;
        m_cursor->close();
        m_cursor = nullptr;
        */
    }
    
    pub fn txn_begin(&mut self) -> bool {
        
        todo!();
        /*
            if (!pdb || activeTxn)
            return false;
        DbTxn* ptxn = env->TxnBegin();
        if (!ptxn)
            return false;
        activeTxn = ptxn;
        return true;
        */
    }
    
    pub fn txn_commit(&mut self) -> bool {
        
        todo!();
        /*
            if (!pdb || !activeTxn)
            return false;
        int ret = activeTxn->commit(0);
        activeTxn = nullptr;
        return (ret == 0);
        */
    }
    
    pub fn txn_abort(&mut self) -> bool {
        
        todo!();
        /*
            if (!pdb || !activeTxn)
            return false;
        int ret = activeTxn->abort();
        activeTxn = nullptr;
        return (ret == 0);
        */
    }
    
    pub fn read_key(&mut self, 
        key:   DataStream,
        value: &mut DataStream) -> bool {
        
        todo!();
        /*
            if (!pdb)
            return false;

        SafeDbt datKey(key.data(), key.size());

        SafeDbt datValue;
        int ret = pdb->get(activeTxn, datKey, datValue, 0);
        if (ret == 0 && datValue.get_data() != nullptr) {
            value.write((char*)datValue.get_data(), datValue.get_size());
            return true;
        }
        return false;
        */
    }
    
    pub fn write_key(&mut self, 
        key:       DataStream,
        value:     DataStream,
        overwrite: Option<bool>) -> bool {

        let overwrite: bool = overwrite.unwrap_or(true);
        
        todo!();
        /*
            if (!pdb)
            return false;
        if (fReadOnly)
            assert(!"Write called on database in read-only mode");

        SafeDbt datKey(key.data(), key.size());

        SafeDbt datValue(value.data(), value.size());

        int ret = pdb->put(activeTxn, datKey, datValue, (overwrite ? 0 : DB_NOOVERWRITE));
        return (ret == 0);
        */
    }
    
    pub fn erase_key(&mut self, key: DataStream) -> bool {
        
        todo!();
        /*
            if (!pdb)
            return false;
        if (fReadOnly)
            assert(!"Erase called on database in read-only mode");

        SafeDbt datKey(key.data(), key.size());

        int ret = pdb->del(activeTxn, datKey, 0);
        return (ret == 0 || ret == DB_NOTFOUND);
        */
    }
    
    pub fn has_key(&mut self, key: DataStream) -> bool {
        
        todo!();
        /*
            if (!pdb)
            return false;

        SafeDbt datKey(key.data(), key.size());

        int ret = pdb->exists(activeTxn, datKey, 0);
        return ret == 0;
        */
    }

    pub fn new(
        database:          &mut BerkeleyDatabase,
        read_only:         bool,
        flush_on_close_in: Option<bool>) -> Self {

        let flush_on_close_in: bool =
                 flush_on_close_in.unwrap_or(true);
    
        todo!();
        /*
        : pdb(nullptr),
        : active_txn(nullptr),
        : cursor(nullptr),
        : database(database),

            database.AddRef();
        database.Open();
        fReadOnly = read_only;
        fFlushOnClose = fFlushOnCloseIn;
        env = database.env.get();
        pdb = database.m_db.get();
        strFile = database.strFile;
        if (!Exists(std::string("version"))) {
            bool fTmp = fReadOnly;
            fReadOnly = false;
            Write(std::string("version"), CLIENT_VERSION);
            fReadOnly = fTmp;
        }
        */
    }
    
    pub fn flush(&mut self)  {
        
        todo!();
        /*
            if (activeTxn)
            return;

        // Flush database activity from memory pool to disk log
        unsigned int nMinutes = 0;
        if (fReadOnly)
            nMinutes = 1;

        if (env) { // env is nullptr for dummy databases (i.e. in tests). Don't actually flush if env is nullptr so we don't segfault
            env->dbenv->txn_checkpoint(nMinutes ? gArgs.GetIntArg("-dblogsize", DEFAULT_WALLET_DBLOGSIZE) * 1024 : 0, nMinutes, 0);
        }
        */
    }
}

