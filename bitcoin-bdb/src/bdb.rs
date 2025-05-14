// ---------------- [ File: bitcoin-bdb/src/bdb.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/wallet/bdb.h]

pub const DEFAULT_WALLET_DBLOGSIZE: u32 = 100;
pub const DEFAULT_WALLET_PRIVDB:    bool = true;

/**
  | An instance of this class represents
  | one database.
  | 
  | For BerkeleyDB this is just a (env, strFile)
  | tuple.
  |
  */
pub struct BerkeleyDatabase {

    base:     WalletDatabase,

    /**
      | Pointer to shared database environment.
      | 
      | Normally there is only one BerkeleyDatabase
      | object per
      | 
      | BerkeleyEnvivonment, but in the special,
      | backwards compatible case where multiple
      | wallet BDB data files are loaded from
      | the same directory, this will point
      | to a shared instance that gets freed
      | when the last data file is closed.
      |
      */
    env:      Arc<BerkeleyEnvironment>,


    /**
      | Database pointer. This is initialized
      | lazily and reset during flushes, so
      | it can be null.
      |
      */
    db:       Box<libdb::Db>,

    str_file: String,
}

impl Drop for BerkeleyDatabase {
    fn drop(&mut self) {
        todo!();
        /*
            if (env) {
            LOCK(cs_db);
            env->CloseDb(strFile);
            assert(!m_db);
            size_t erased = env->m_databases.erase(strFile);
            assert(erased == 1);
            env->m_fileids.erase(strFile);
        }
        */
    }
}

impl BerkeleyDatabase {
    
    /**
      | Verifies the environment and database
      | file
      |
      */
    pub fn verify(&mut self, error_str: &mut BilingualStr) -> bool {
        
        todo!();
        /*
        fs::path walletDir = env->Directory();
        fs::path file_path = walletDir / strFile;

        LogPrintf("Using BerkeleyDB version %s\n", BerkeleyDatabaseVersion());
        LogPrintf("Using wallet %s\n", fs::PathToString(file_path));

        if (!env->Open(errorStr)) {
            return false;
        }

        if (fs::exists(file_path))
        {
            assert(m_refcount == 0);

            Db db(env->dbenv.get(), 0);
            int result = db.verify(strFile.c_str(), nullptr, nullptr, 0);
            if (result != 0) {
                errorStr = strprintf(_("%s corrupt. Try using the wallet tool bitcoin-wallet to salvage or restoring a backup."), fs::quoted(fs::PathToString(file_path)));
                return false;
            }
        }
        // also return true if files does not exists
        return true;
        */
    }

    /**
      | Open the database if it is not already
      | opened.
      |
      */
    pub fn open(&mut self)  {
        
        todo!();
        /*
            unsigned int nFlags = DB_THREAD | DB_CREATE;

        {
            LOCK(cs_db);
            bilingual_str open_err;
            if (!env->Open(open_err))
                throw std::runtime_error("BerkeleyDatabase: Failed to open database environment.");

            if (m_db == nullptr) {
                int ret;
                std::unique_ptr<Db> pdb_temp = std::make_unique<Db>(env->dbenv.get(), 0);

                bool fMockDb = env->IsMock();
                if (fMockDb) {
                    DbMpoolFile* mpf = pdb_temp->get_mpf();
                    ret = mpf->set_flags(DB_MPOOL_NOFILE, 1);
                    if (ret != 0) {
                        throw std::runtime_error(strprintf("BerkeleyDatabase: Failed to configure for no temp file backing for database %s", strFile));
                    }
                }

                ret = pdb_temp->open(nullptr,                             // Txn pointer
                                fMockDb ? nullptr : strFile.c_str(),      // Filename
                                fMockDb ? strFile.c_str() : "main",       // Logical db name
                                DB_BTREE,                                 // Database type
                                nFlags,                                   // Flags
                                0);

                if (ret != 0) {
                    throw std::runtime_error(strprintf("BerkeleyDatabase: Error %d, can't open database %s", ret, strFile));
                }

                // Call CheckUniqueFileid on the containing BDB environment to
                // avoid BDB data consistency bugs that happen when different data
                // files in the same environment have the same fileid.
                CheckUniqueFileid(*env, strFile, *pdb_temp, this->env->m_fileids[strFile]);

                m_db.reset(pdb_temp.release());

            }
        }
        */
    }
    
    pub fn increment_update_counter(&mut self)  {
        
        todo!();
        /*
            ++nUpdateCounter;
        */
    }
    
    /**
      | Rewrite the entire database on disk,
      | with the exception of key pszSkip if
      | non-zero
      |
      */
    pub fn rewrite(&mut self, psz_skip: *const u8) -> bool {
        
        todo!();
        /*
            while (true) {
            {
                LOCK(cs_db);
                if (m_refcount <= 0) {
                    // Flush log data to the dat file
                    env->CloseDb(strFile);
                    env->CheckpointLSN(strFile);
                    m_refcount = -1;

                    bool fSuccess = true;
                    LogPrintf("BerkeleyBatch::Rewrite: Rewriting %s...\n", strFile);
                    std::string strFileRes = strFile + ".rewrite";
                    { // surround usage of db with extra {}
                        BerkeleyBatch db(*this, true);
                        std::unique_ptr<Db> pdbCopy = std::make_unique<Db>(env->dbenv.get(), 0);

                        int ret = pdbCopy->open(nullptr,               // Txn pointer
                                                strFileRes.c_str(), // Filename
                                                "main",             // Logical db name
                                                DB_BTREE,           // Database type
                                                DB_CREATE,          // Flags
                                                0);
                        if (ret > 0) {
                            LogPrintf("BerkeleyBatch::Rewrite: Can't create database file %s\n", strFileRes);
                            fSuccess = false;
                        }

                        if (db.StartCursor()) {
                            while (fSuccess) {
                                DataStream ssKey(SER_DISK, CLIENT_VERSION);
                                DataStream ssValue(SER_DISK, CLIENT_VERSION);
                                bool complete;
                                bool ret1 = db.ReadAtCursor(ssKey, ssValue, complete);
                                if (complete) {
                                    break;
                                } else if (!ret1) {
                                    fSuccess = false;
                                    break;
                                }
                                if (pszSkip &&
                                    strncmp((const char*)ssKey.data(), pszSkip, std::min(ssKey.size(), strlen(pszSkip))) == 0)
                                    continue;
                                if (strncmp((const char*)ssKey.data(), "\x07version", 8) == 0) {
                                    // Update version:
                                    ssValue.clear();
                                    ssValue << CLIENT_VERSION;
                                }
                                Dbt datKey(ssKey.data(), ssKey.size());
                                Dbt datValue(ssValue.data(), ssValue.size());
                                int ret2 = pdbCopy->put(nullptr, &datKey, &datValue, DB_NOOVERWRITE);
                                if (ret2 > 0)
                                    fSuccess = false;
                            }
                            db.CloseCursor();
                        }
                        if (fSuccess) {
                            db.Close();
                            env->CloseDb(strFile);
                            if (pdbCopy->close(0))
                                fSuccess = false;
                        } else {
                            pdbCopy->close(0);
                        }
                    }
                    if (fSuccess) {
                        Db dbA(env->dbenv.get(), 0);
                        if (dbA.remove(strFile.c_str(), nullptr, 0))
                            fSuccess = false;
                        Db dbB(env->dbenv.get(), 0);
                        if (dbB.rename(strFileRes.c_str(), nullptr, strFile.c_str(), 0))
                            fSuccess = false;
                    }
                    if (!fSuccess)
                        LogPrintf("BerkeleyBatch::Rewrite: Failed to rewrite database file %s\n", strFileRes);
                    return fSuccess;
                }
            }
            UninterruptibleSleep(std::chrono::milliseconds{100});
        }
        */
    }

    /**
      | Create DB handle to real database
      |
      */
    pub fn new(
        env:      Arc<BerkeleyEnvironment>,
        filename: String) -> Self {
    
        todo!();
        /*
        : wallet_database(),
        : env(std::move(env)),
        : str_file(std::move(filename)),

            auto inserted = this->env->m_databases.emplace(strFile, std::ref(*this));
            assert(inserted.second);
        */
    }

    /**
      | Return path to main database filename
      |
      */
    pub fn filename(&mut self) -> String {
        
        todo!();
        /*
            return fs::PathToString(env->Directory() / strFile);
        */
    }
    
    pub fn format(&mut self) -> String {
        
        todo!();
        /*
            return "bdb";
        */
    }

    /**
      | flush the wallet passively (TRY_LOCK)
      | ideal to be called periodically
      |
      */
    pub fn periodic_flush(&mut self) -> bool {
        
        todo!();
        /*
            // Don't flush if we can't acquire the lock.
        TRY_LOCK(cs_db, lockDb);
        if (!lockDb) return false;

        // Don't flush if any databases are in use
        for (auto& it : env->m_databases) {
            if (it.second.get().m_refcount > 0) return false;
        }

        // Don't flush if there haven't been any batch writes for this database.
        if (m_refcount < 0) return false;

        LogPrint(BCLog::WALLETDB, "Flushing %s\n", strFile);
        int64_t nStart = GetTimeMillis();

        // Flush wallet file so it's self contained
        env->CloseDb(strFile);
        env->CheckpointLSN(strFile);
        m_refcount = -1;

        LogPrint(BCLog::WALLETDB, "Flushed %s %dms\n", strFile, GetTimeMillis() - nStart);

        return true;
        */
    }
    
    /**
      | Back up the entire database to a file.
      |
      */
    pub fn backup(&self, str_dest: &String) -> bool {
        
        todo!();
        /*
            while (true)
        {
            {
                LOCK(cs_db);
                if (m_refcount <= 0)
                {
                    // Flush log data to the dat file
                    env->CloseDb(strFile);
                    env->CheckpointLSN(strFile);

                    // Copy wallet file
                    fs::path pathSrc = env->Directory() / strFile;
                    fs::path pathDest(fs::PathFromString(strDest));
                    if (fs::is_directory(pathDest))
                        pathDest /= fs::PathFromString(strFile);

                    try {
                        if (fs::equivalent(pathSrc, pathDest)) {
                            LogPrintf("cannot backup to wallet source file %s\n", fs::PathToString(pathDest));
                            return false;
                        }

                        fs::copy_file(pathSrc, pathDest, fs::copy_option::overwrite_if_exists);
                        LogPrintf("copied %s to %s\n", strFile, fs::PathToString(pathDest));
                        return true;
                    } catch (const fs::filesystem_error& e) {
                        LogPrintf("error copying %s to %s - %s\n", strFile, fs::PathToString(pathDest), fsbridge::get_filesystem_error_message(e));
                        return false;
                    }
                }
            }
            UninterruptibleSleep(std::chrono::milliseconds{100});
        }
        */
    }
    
    /**
      | Make sure all changes are flushed to
      | database file.
      |
      */
    pub fn flush(&mut self)  {
        
        todo!();
        /*
            env->Flush(false);
        */
    }
    
    /**
      | Flush to the database file and close
      | the database.
      | 
      | Also close the environment if no other
      | databases are open in it.
      |
      */
    pub fn close(&mut self)  {
        
        todo!();
        /*
            env->Flush(true);
        */
    }
    
    pub fn reload_db_env(&mut self)  {
        
        todo!();
        /*
            env->ReloadDbEnv();
        */
    }
    
    /**
      | Indicate the a new database user has
      | began using the database.
      |
      */
    pub fn add_ref(&mut self)  {
        
        todo!();
        /*
            LOCK(cs_db);
        if (m_refcount < 0) {
            m_refcount = 1;
        } else {
            m_refcount++;
        }
        */
    }
    
    /**
      | Indicate that database user has stopped
      | using the database and that it could
      | be flushed or closed.
      |
      */
    pub fn remove_ref(&mut self)  {
        
        todo!();
        /*
            LOCK(cs_db);
        m_refcount--;
        if (env) env->m_db_in_use.notify_all();
        */
    }
    
    /**
      | Make a BerkeleyBatch connected to this
      | database
      |
      */
    pub fn make_batch(&mut self, flush_on_close: Option<bool>) -> Box<DatabaseBatch> {

        let flush_on_close: bool = flush_on_close.unwrap_or(true);
        
        todo!();
        /*
            return std::make_unique<BerkeleyBatch>(*this, false, flush_on_close);
        */
    }
}
