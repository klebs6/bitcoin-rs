// ---------------- [ File: bitcoin-bdb/src/berkeley_env.rs ]
crate::ix!();

pub struct BerkeleyEnvironment {

    db_env_init: bool,
    mock_db:     bool,

    /**
      | Don't change into fs::path, as that
      | can result in shutdown problems/crashes
      | caused by a static initialized internal
      | pointer.
      |
      */
    str_path:    String,

    dbenv:       Box<libdb::Env>,
    databases:   HashMap<String,Amo<BerkeleyDatabase>>,
    fileids:     HashMap<String,WalletDatabaseFileId>,

    /**
      | In C++, we had std::condition_variable_any
      | The condition_variable_any class is
      | a generalization of
      | std::condition_variable. Whereas
      | std::condition_variable works only on
      | std::unique_lock<std::mutex>,
      | condition_variable_any can operate on any lock
      | that meets the BasicLockable requirements.
      */
    db_in_use:   std::sync::Condvar,
}

impl Drop for BerkeleyEnvironment {
    fn drop(&mut self) {
        todo!();
        /*
            LOCK(cs_db);
        g_dbenvs.erase(strPath);
        Close();
        */
    }
}

impl Default for BerkeleyEnvironment {

    /**
      | Construct an in-memory mock Berkeley
      | environment for testing
      |
      */
    fn default() -> Self {
    
        todo!();
        /*


            Reset();

        LogPrint(BCLog::WALLETDB, "BerkeleyEnvironment::MakeMock\n");

        dbenv->set_cachesize(1, 0, 1);
        dbenv->set_lg_bsize(10485760 * 4);
        dbenv->set_lg_max(10485760);
        dbenv->set_lk_max_locks(10000);
        dbenv->set_lk_max_objects(10000);
        dbenv->set_flags(DB_AUTO_COMMIT, 1);
        dbenv->log_set_config(DB_LOG_IN_MEMORY, 1);
        int ret = dbenv->open(nullptr,
                             DB_CREATE |
                                 DB_INIT_LOCK |
                                 DB_INIT_LOG |
                                 DB_INIT_MPOOL |
                                 DB_INIT_TXN |
                                 DB_THREAD |
                                 DB_PRIVATE,
                             S_IRUSR | S_IWUSR);
        if (ret > 0) {
            throw std::runtime_error(strprintf("BerkeleyEnvironment::MakeMock: Error %d opening database environment.", ret));
        }

        fDbEnvInit = true;
        fMockDb = true;
        */
    }
}

impl BerkeleyEnvironment {

    pub fn is_mock(&self) -> bool {
        
        todo!();
        /*
            return fMockDb;
        */
    }
    
    pub fn is_initialized(&self) -> bool {
        
        todo!();
        /*
            return fDbEnvInit;
        */
    }
    
    pub fn directory(&self) -> Box<Path> {
        
        todo!();
        /*
            return fs::PathFromString(strPath);
        */
    }
    
    pub fn txn_begin(&mut self, flags: Option<i32>) -> *mut libdb::DbTxn {
        let flags: i32 = flags.unwrap_or(
            libdb::DB_TXN_WRITE_NOSYNC);

        todo!();
        /*
            DbTxn* ptxn = nullptr;
            int ret = dbenv->txn_begin(nullptr, &ptxn, flags);
            if (!ptxn || ret != 0)
                return nullptr;
            return ptxn;
        */
    }
    
    pub fn close_db(&mut self, str_file: &String)  {
        
        todo!();
        /*
            {
            LOCK(cs_db);
            auto it = m_databases.find(strFile);
            assert(it != m_databases.end());
            BerkeleyDatabase& database = it->second.get();
            if (database.m_db) {
                // Close the database handle
                database.m_db->close(0);
                database.m_db.reset();
            }
        }
        */
    }
    
    pub fn reload_db_env(&mut self)  {
        
        todo!();
        /*
        // Make sure that no Db's are in use
        AssertLockNotHeld(cs_db);
        std::unique_lock<RecursiveMutex> lock(cs_db);
        m_db_in_use.wait(lock, [this](){
            for (auto& db : m_databases) {
                if (db.second.get().m_refcount > 0) return false;
            }
            return true;
        });

        std::vector<std::string> filenames;
        for (auto it : m_databases) {
            filenames.push_back(it.first);
        }
        // Close the individual Db's
        for (const std::string& filename : filenames) {
            CloseDb(filename);
        }
        // Reset the environment
        Flush(true); // This will flush and close the environment
        Reset();
        bilingual_str open_err;
        Open(open_err);
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
        if (!fDbEnvInit)
            return;

        fDbEnvInit = false;

        for (auto& db : m_databases) {
            BerkeleyDatabase& database = db.second.get();
            assert(database.m_refcount <= 0);
            if (database.m_db) {
                database.m_db->close(0);
                database.m_db.reset();
            }
        }

        FILE* error_file = nullptr;
        dbenv->get_errfile(&error_file);

        int ret = dbenv->close(0);
        if (ret != 0)
            LogPrintf("BerkeleyEnvironment::Close: Error %d closing database environment: %s\n", ret, DbEnv::strerror(ret));
        if (!fMockDb)
            DbEnv((u_int32_t)0).remove(strPath.c_str(), 0);

        if (error_file) fclose(error_file);

        UnlockDirectory(fs::PathFromString(strPath), ".walletlock");
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
        dbenv.reset(new DbEnv(DB_CXX_NO_EXCEPTIONS));
        fDbEnvInit = false;
        fMockDb = false;
        */
    }
    
    pub fn new(dir_path: Box<&Path>) -> Self {
    
        todo!();
        /*


            : strPath(fs::PathToString(dir_path))
        Reset();
        */
    }
    
    pub fn open(&mut self, err: &mut BilingualStr) -> bool {
        
        todo!();
        /*

        if (fDbEnvInit) {
            return true;
        }

        fs::path pathIn = fs::PathFromString(strPath);
        TryCreateDirectories(pathIn);
        if (!LockDirectory(pathIn, ".walletlock")) {
            LogPrintf("Cannot obtain a lock on wallet directory %s. Another instance may be using it.\n", strPath);
            err = strprintf(_("Error initializing wallet database environment %s!"), fs::quoted(fs::PathToString(Directory())));
            return false;
        }

        fs::path pathLogDir = pathIn / "database";
        TryCreateDirectories(pathLogDir);
        fs::path pathErrorFile = pathIn / "db.log";
        LogPrintf("BerkeleyEnvironment::Open: LogDir=%s ErrorFile=%s\n", fs::PathToString(pathLogDir), fs::PathToString(pathErrorFile));

        unsigned int nEnvFlags = 0;
        if (gArgs.GetBoolArg("-privdb", DEFAULT_WALLET_PRIVDB))
            nEnvFlags |= DB_PRIVATE;

        dbenv->set_lg_dir(fs::PathToString(pathLogDir).c_str());
        dbenv->set_cachesize(0, 0x100000, 1); // 1 MiB should be enough for just the wallet
        dbenv->set_lg_bsize(0x10000);
        dbenv->set_lg_max(1048576);
        dbenv->set_lk_max_locks(40000);
        dbenv->set_lk_max_objects(40000);
        dbenv->set_errfile(fsbridge::fopen(pathErrorFile, "a")); /// debug
        dbenv->set_flags(DB_AUTO_COMMIT, 1);
        dbenv->set_flags(DB_TXN_WRITE_NOSYNC, 1);
        dbenv->log_set_config(DB_LOG_AUTO_REMOVE, 1);
        int ret = dbenv->open(strPath.c_str(),
                             DB_CREATE |
                                 DB_INIT_LOCK |
                                 DB_INIT_LOG |
                                 DB_INIT_MPOOL |
                                 DB_INIT_TXN |
                                 DB_THREAD |
                                 DB_RECOVER |
                                 nEnvFlags,
                             S_IRUSR | S_IWUSR);
        if (ret != 0) {
            LogPrintf("BerkeleyEnvironment::Open: Error %d opening database environment: %s\n", ret, DbEnv::strerror(ret));
            int ret2 = dbenv->close(0);
            if (ret2 != 0) {
                LogPrintf("BerkeleyEnvironment::Open: Error %d closing failed database environment: %s\n", ret2, DbEnv::strerror(ret2));
            }
            Reset();
            err = strprintf(_("Error initializing wallet database environment %s!"), fs::quoted(fs::PathToString(Directory())));
            if (ret == DB_RUNRECOVERY) {
                err += Untranslated(" ") + _("This error could occur if this wallet was not shutdown cleanly and was last loaded using a build with a newer version of Berkeley DB. If so, please use the software that last loaded this wallet");
            }
            return false;
        }

        fDbEnvInit = true;
        fMockDb = false;
        return true;
        */
    }
    
    pub fn checkpointlsn(&mut self, str_file: &String)  {
        
        todo!();
        /*
        dbenv->txn_checkpoint(0, 0, 0);
        if (fMockDb)
            return;
        dbenv->lsn_reset(strFile.c_str(), 0);
        */
    }
    
    pub fn flush(&mut self, shutdown: bool)  {
        
        todo!();
        /*
        int64_t nStart = GetTimeMillis();
        // Flush log data to the actual data file on all files that are not in use
        LogPrint(BCLog::WALLETDB, "BerkeleyEnvironment::Flush: [%s] Flush(%s)%s\n", strPath, fShutdown ? "true" : "false", fDbEnvInit ? "" : " database not started");
        if (!fDbEnvInit)
            return;
        {
            LOCK(cs_db);
            bool no_dbs_accessed = true;
            for (auto& db_it : m_databases) {
                std::string strFile = db_it.first;
                int nRefCount = db_it.second.get().m_refcount;
                if (nRefCount < 0) continue;
                LogPrint(BCLog::WALLETDB, "BerkeleyEnvironment::Flush: Flushing %s (refcount = %d)...\n", strFile, nRefCount);
                if (nRefCount == 0) {
                    // Move log data to the dat file
                    CloseDb(strFile);
                    LogPrint(BCLog::WALLETDB, "BerkeleyEnvironment::Flush: %s checkpoint\n", strFile);
                    dbenv->txn_checkpoint(0, 0, 0);
                    LogPrint(BCLog::WALLETDB, "BerkeleyEnvironment::Flush: %s detach\n", strFile);
                    if (!fMockDb)
                        dbenv->lsn_reset(strFile.c_str(), 0);
                    LogPrint(BCLog::WALLETDB, "BerkeleyEnvironment::Flush: %s closed\n", strFile);
                    nRefCount = -1;
                } else {
                    no_dbs_accessed = false;
                }
            }
            LogPrint(BCLog::WALLETDB, "BerkeleyEnvironment::Flush: Flush(%s)%s took %15dms\n", fShutdown ? "true" : "false", fDbEnvInit ? "" : " database not started", GetTimeMillis() - nStart);
            if (fShutdown) {
                char** listp;
                if (no_dbs_accessed) {
                    dbenv->log_archive(&listp, DB_ARCH_REMOVE);
                    Close();
                    if (!fMockDb) {
                        fs::remove_all(fs::PathFromString(strPath) / "database");
                    }
                }
            }
        }
        */
    }
}

