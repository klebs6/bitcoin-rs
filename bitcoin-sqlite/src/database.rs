// ---------------- [ File: bitcoin-sqlite/src/database.rs ]
crate::ix!();

/**
  | An instance of this class represents
  | one SQLite3 database.
  |
  */
pub struct SQLiteDatabase {
    base:      WalletDatabase,
    mock:      bool, // default = { false }
    dir_path:  String,
    file_path: String,
    db:        *mut sqlite3::Connection, // default = { nullptr }
}

impl Drop for SQLiteDatabase {

    fn drop(&mut self) {
        todo!();
        /*
            Cleanup();
        */
    }
}

impl SQLiteDatabase {
    
    /**
      | These functions are unused
      |
      */
    pub fn add_ref(&mut self)  {
        
        todo!();
        /*
            assert(false);
        */
    }
    
    pub fn remove_ref(&mut self)  {
        
        todo!();
        /*
            assert(false);
        */
    }

    /**
      | No-op. See comment on SQLiteDatabase::Flush
      |
      | No-ops
      | 
      | SQLite always flushes everything to
      | the database file after each transaction
      | (each Read/Write/Erase that we do is
      | its own transaction unless we called
      | 
      | TxnBegin) so there is no need to have
      | Flush or Periodic Flush.
      | 
      | There is no DB env to reload, so ReloadDbEnv
      | has nothing to do
      |
      */
    pub fn flush(&mut self)  { }
    
    pub fn periodic_flush(&mut self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn reload_db_env(&mut self)  {
        
    }
    
    pub fn increment_update_counter(&mut self)  {
        
        todo!();
        /*
            ++nUpdateCounter;
        */
    }
    
    pub fn filename(&mut self) -> String {
        
        todo!();
        /*
            return m_file_path;
        */
    }
    
    pub fn format(&mut self) -> String {
        
        todo!();
        /*
            return "sqlite";
        */
    }

    /**
      | Create DB handle to real database
      |
      */
    pub fn new(
        dir_path:  &Path,
        file_path: &Path,
        mock:      Option<bool>) -> Self {

        let mock: bool = mock.unwrap_or(false);
    
        todo!();
        /*

            : WalletDatabase(), m_mock(mock), m_dir_path(fs::PathToString(dir_path)), m_file_path(fs::PathToString(file_path))

        {
            LOCK(g_sqlite_mutex);
            LogPrintf("Using SQLite Version %s\n", SQLiteDatabaseVersion());
            LogPrintf("Using wallet %s\n", m_dir_path);

            if (++g_sqlite_count == 1) {
                // Setup logging
                int ret = sqlite3_config(SQLITE_CONFIG_LOG, ErrorLogCallback, nullptr);
                if (ret != SQLITE_OK) {
                    throw std::runtime_error(strprintf("SQLiteDatabase: Failed to setup error log: %s\n", sqlite3_errstr(ret)));
                }
                // Force serialized threading mode
                ret = sqlite3_config(SQLITE_CONFIG_SERIALIZED);
                if (ret != SQLITE_OK) {
                    throw std::runtime_error(strprintf("SQLiteDatabase: Failed to configure serialized threading mode: %s\n", sqlite3_errstr(ret)));
                }
            }
            int ret = sqlite3_initialize(); // This is a no-op if sqlite3 is already initialized
            if (ret != SQLITE_OK) {
                throw std::runtime_error(strprintf("SQLiteDatabase: Failed to initialize SQLite: %s\n", sqlite3_errstr(ret)));
            }
        }

        try {
            Open();
        } catch (const std::runtime_error&) {
            // If open fails, cleanup this object and rethrow the exception
            Cleanup();
            throw;
        }
        */
    }
    
    pub fn cleanup(&mut self)  {
        
        todo!();
        /*
            Close();

        LOCK(g_sqlite_mutex);
        if (--g_sqlite_count == 0) {
            int ret = sqlite3_shutdown();
            if (ret != SQLITE_OK) {
                LogPrintf("SQLiteDatabase: Failed to shutdown SQLite: %s\n", sqlite3_errstr(ret));
            }
        }
        */
    }
    
    pub fn verify(&mut self, error: &mut BilingualStr) -> bool {
        
        todo!();
        /*
            assert(m_db);

        // Check the application ID matches our network magic
        auto read_result = ReadPragmaInteger(m_db, "application_id", "the application id", error);
        if (!read_result.has_value()) return false;
        uint32_t app_id = static_cast<uint32_t>(read_result.value());
        uint32_t net_magic = ReadBE32(Params().MessageStart());
        if (app_id != net_magic) {
            error = strprintf(_("SQLiteDatabase: Unexpected application id. Expected %u, got %u"), net_magic, app_id);
            return false;
        }

        // Check our schema version
        read_result = ReadPragmaInteger(m_db, "user_version", "sqlite wallet schema version", error);
        if (!read_result.has_value()) return false;
        int32_t user_ver = read_result.value();
        if (user_ver != WALLET_SCHEMA_VERSION) {
            error = strprintf(_("SQLiteDatabase: Unknown sqlite wallet schema version %d. Only version %d is supported"), user_ver, WALLET_SCHEMA_VERSION);
            return false;
        }

        sqlite3_stmt* stmt{nullptr};
        int ret = sqlite3_prepare_v2(m_db, "PRAGMA integrity_check", -1, &stmt, nullptr);
        if (ret != SQLITE_OK) {
            sqlite3_finalize(stmt);
            error = strprintf(_("SQLiteDatabase: Failed to prepare statement to verify database: %s"), sqlite3_errstr(ret));
            return false;
        }
        while (true) {
            ret = sqlite3_step(stmt);
            if (ret == SQLITE_DONE) {
                break;
            }
            if (ret != SQLITE_ROW) {
                error = strprintf(_("SQLiteDatabase: Failed to execute statement to verify database: %s"), sqlite3_errstr(ret));
                break;
            }
            const char* msg = (const char*)sqlite3_column_text(stmt, 0);
            if (!msg) {
                error = strprintf(_("SQLiteDatabase: Failed to read database verification error: %s"), sqlite3_errstr(ret));
                break;
            }
            std::string str_msg(msg);
            if (str_msg == "ok") {
                continue;
            }
            if (error.empty()) {
                error = _("Failed to verify database") + Untranslated("\n");
            }
            error += Untranslated(strprintf("%s\n", str_msg));
        }
        sqlite3_finalize(stmt);
        return error.empty();
        */
    }
    
    /**
      | Open the database if it is not already
      | opened
      |
      */
    pub fn open(&mut self)  {
        
        todo!();
        /*
            int flags = SQLITE_OPEN_FULLMUTEX | SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE;
        if (m_mock) {
            flags |= SQLITE_OPEN_MEMORY; // In memory database for mock db
        }

        if (m_db == nullptr) {
            if (!m_mock) {
                TryCreateDirectories(fs::PathFromString(m_dir_path));
            }
            int ret = sqlite3_open_v2(m_file_path.c_str(), &m_db, flags, nullptr);
            if (ret != SQLITE_OK) {
                throw std::runtime_error(strprintf("SQLiteDatabase: Failed to open database: %s\n", sqlite3_errstr(ret)));
            }
            ret = sqlite3_extended_result_codes(m_db, 1);
            if (ret != SQLITE_OK) {
                throw std::runtime_error(strprintf("SQLiteDatabase: Failed to enable extended result codes: %s\n", sqlite3_errstr(ret)));
            }
        }

        if (sqlite3_db_readonly(m_db, "main") != 0) {
            throw std::runtime_error("SQLiteDatabase: Database opened in readonly mode but read-write permissions are needed");
        }

        // Acquire an exclusive lock on the database
        // First change the locking mode to exclusive
        SetPragma(m_db, "locking_mode", "exclusive", "Unable to change database locking mode to exclusive");
        // Now begin a transaction to acquire the exclusive lock. This lock won't be released until we close because of the exclusive locking mode.
        int ret = sqlite3_exec(m_db, "BEGIN EXCLUSIVE TRANSACTION", nullptr, nullptr, nullptr);
        if (ret != SQLITE_OK) {
            throw std::runtime_error("SQLiteDatabase: Unable to obtain an exclusive lock on the database, is it being used by another instance of " PACKAGE_NAME "?\n");
        }
        ret = sqlite3_exec(m_db, "COMMIT", nullptr, nullptr, nullptr);
        if (ret != SQLITE_OK) {
            throw std::runtime_error(strprintf("SQLiteDatabase: Unable to end exclusive lock transaction: %s\n", sqlite3_errstr(ret)));
        }

        // Enable fullfsync for the platforms that use it
        SetPragma(m_db, "fullfsync", "true", "Failed to enable fullfsync");

        if (gArgs.GetBoolArg("-unsafesqlitesync", false)) {
            // Use normal synchronous mode for the journal
            LogPrintf("WARNING SQLite is configured to not wait for data to be flushed to disk. Data loss and corruption may occur.\n");
            SetPragma(m_db, "synchronous", "OFF", "Failed to set synchronous mode to OFF");
        }

        // Make the table for our key-value pairs
        // First check that the main table exists
        sqlite3_stmt* check_main_stmt{nullptr};
        ret = sqlite3_prepare_v2(m_db, "SELECT name FROM sqlite_master WHERE type='table' AND name='main'", -1, &check_main_stmt, nullptr);
        if (ret != SQLITE_OK) {
            throw std::runtime_error(strprintf("SQLiteDatabase: Failed to prepare statement to check table existence: %s\n", sqlite3_errstr(ret)));
        }
        ret = sqlite3_step(check_main_stmt);
        if (sqlite3_finalize(check_main_stmt) != SQLITE_OK) {
            throw std::runtime_error(strprintf("SQLiteDatabase: Failed to finalize statement checking table existence: %s\n", sqlite3_errstr(ret)));
        }
        bool table_exists;
        if (ret == SQLITE_DONE) {
            table_exists = false;
        } else if (ret == SQLITE_ROW) {
            table_exists = true;
        } else {
            throw std::runtime_error(strprintf("SQLiteDatabase: Failed to execute statement to check table existence: %s\n", sqlite3_errstr(ret)));
        }

        // Do the db setup things because the table doesn't exist only when we are creating a new wallet
        if (!table_exists) {
            ret = sqlite3_exec(m_db, "CREATE TABLE main(key BLOB PRIMARY KEY NOT NULL, value BLOB NOT NULL)", nullptr, nullptr, nullptr);
            if (ret != SQLITE_OK) {
                throw std::runtime_error(strprintf("SQLiteDatabase: Failed to create new database: %s\n", sqlite3_errstr(ret)));
            }

            // Set the application id
            uint32_t app_id = ReadBE32(Params().MessageStart());
            SetPragma(m_db, "application_id", strprintf("%d", static_cast<int32_t>(app_id)),
                      "Failed to set the application id");

            // Set the user version
            SetPragma(m_db, "user_version", strprintf("%d", WALLET_SCHEMA_VERSION),
                      "Failed to set the wallet schema version");
        }
        */
    }
    
    /**
      | Rewrite the entire database on disk
      |
      */
    pub fn rewrite(&mut self, skip: *const u8) -> bool {
        
        todo!();
        /*
            // Rewrite the database using the VACUUM command: https://sqlite.org/lang_vacuum.html
        int ret = sqlite3_exec(m_db, "VACUUM", nullptr, nullptr, nullptr);
        return ret == SQLITE_OK;
        */
    }
    
    /**
      | Back up the entire database to a file.
      |
      */
    pub fn backup(&self, dest: &String) -> bool {
        
        todo!();
        /*
            sqlite3* db_copy;
        int res = sqlite3_open(dest.c_str(), &db_copy);
        if (res != SQLITE_OK) {
            sqlite3_close(db_copy);
            return false;
        }
        sqlite3_backup* backup = sqlite3_backup_init(db_copy, "main", m_db, "main");
        if (!backup) {
            LogPrintf("%s: Unable to begin backup: %s\n", __func__, sqlite3_errmsg(m_db));
            sqlite3_close(db_copy);
            return false;
        }
        // Specifying -1 will copy all of the pages
        res = sqlite3_backup_step(backup, -1);
        if (res != SQLITE_DONE) {
            LogPrintf("%s: Unable to backup: %s\n", __func__, sqlite3_errstr(res));
            sqlite3_backup_finish(backup);
            sqlite3_close(db_copy);
            return false;
        }
        res = sqlite3_backup_finish(backup);
        sqlite3_close(db_copy);
        return res == SQLITE_OK;
        */
    }
    
    /**
      | Close the database
      |
      */
    pub fn close(&mut self)  {
        
        todo!();
        /*
            int res = sqlite3_close(m_db);
        if (res != SQLITE_OK) {
            throw std::runtime_error(strprintf("SQLiteDatabase: Failed to close database: %s\n", sqlite3_errstr(res)));
        }
        m_db = nullptr;
        */
    }
    
    /**
      | Make a SQLiteBatch connected to this
      | database
      |
      */
    pub fn make_batch(&mut self, flush_on_close: Option<bool>) -> Box<DatabaseBatch> {

        let flush_on_close: bool = flush_on_close.unwrap_or(true);
        
        todo!();
        /*
            // We ignore flush_on_close because we don't do manual flushing for SQLite
        return std::make_unique<SQLiteBatch>(*this);
        */
    }
}
