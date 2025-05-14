// ---------------- [ File: bitcoin-sqlite/src/batch.rs ]
crate::ix!();

/**
  | RAII class that provides access to a
  | WalletDatabase
  |
  */
pub struct SQLiteBatch<'a> {
    base:           DatabaseBatch,
    database:       Rc<RefCell<sqlite3::Connection>>,
    cursor_init:    bool, // default = false
    read_stmt:      *mut SQLite3Stmt<'a>, // default = { nullptr }
    insert_stmt:    *mut SQLite3Stmt<'a>, // default = { nullptr }
    overwrite_stmt: *mut SQLite3Stmt<'a>, // default = { nullptr }
    delete_stmt:    *mut SQLite3Stmt<'a>, // default = { nullptr }
    cursor_stmt:    *mut SQLite3Stmt<'a>, // default = { nullptr }
}

pub type SQLite3Stmt<'a> = sqlite3::Statement<'a>;

impl<'a> Drop for SQLiteBatch<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            Close();
        */
    }
}

impl<'a> SQLiteBatch<'a> {
    
    pub fn new(database: &mut sqlite::Connection) -> Self {
    
        todo!();
        /*
        : database(database),

            // Make sure we have a db handle
        assert(m_database.m_db);

        SetupSQLStatements();
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            // If m_db is in a transaction (i.e. not in autocommit mode), then abort the transaction in progress
        if (m_database.m_db && sqlite3_get_autocommit(m_database.m_db) == 0) {
            if (TxnAbort()) {
                LogPrintf("SQLiteBatch: Batch closed unexpectedly without the transaction being explicitly committed or aborted\n");
            } else {
                LogPrintf("SQLiteBatch: Batch closed and failed to abort transaction\n");
            }
        }

        // Free all of the prepared statements
        const std::vector<std::pair<sqlite3_stmt**, const char*>> statements{
            {&m_read_stmt, "read"},
            {&m_insert_stmt, "insert"},
            {&m_overwrite_stmt, "overwrite"},
            {&m_delete_stmt, "delete"},
            {&m_cursor_stmt, "cursor"},
        };

        for (const auto& [stmt_prepared, stmt_description] : statements) {
            int res = sqlite3_finalize(*stmt_prepared);
            if (res != SQLITE_OK) {
                LogPrintf("SQLiteBatch: Batch closed but could not finalize %s statement: %s\n",
                          stmt_description, sqlite3_errstr(res));
            }
            *stmt_prepared = nullptr;
        }
        */
    }
    
    pub fn read_key(&mut self, 
        key:   DataStream,
        value: &mut DataStream) -> bool {
        
        todo!();
        /*
            if (!m_database.m_db) return false;
        assert(m_read_stmt);

        // Bind: leftmost parameter in statement is index 1
        int res = sqlite3_bind_blob(m_read_stmt, 1, key.data(), key.size(), SQLITE_STATIC);
        if (res != SQLITE_OK) {
            LogPrintf("%s: Unable to bind statement: %s\n", __func__, sqlite3_errstr(res));
            sqlite3_clear_bindings(m_read_stmt);
            sqlite3_reset(m_read_stmt);
            return false;
        }
        res = sqlite3_step(m_read_stmt);
        if (res != SQLITE_ROW) {
            if (res != SQLITE_DONE) {
                // SQLITE_DONE means "not found", don't log an error in that case.
                LogPrintf("%s: Unable to execute statement: %s\n", __func__, sqlite3_errstr(res));
            }
            sqlite3_clear_bindings(m_read_stmt);
            sqlite3_reset(m_read_stmt);
            return false;
        }
        // Leftmost column in result is index 0
        const char* data = reinterpret_cast<const char*>(sqlite3_column_blob(m_read_stmt, 0));
        int data_size = sqlite3_column_bytes(m_read_stmt, 0);
        value.write(data, data_size);

        sqlite3_clear_bindings(m_read_stmt);
        sqlite3_reset(m_read_stmt);
        return true;
        */
    }
    
    pub fn write_key(&mut self, 
        key:       DataStream,
        value:     DataStream,
        overwrite: Option<bool>) -> bool {

        let overwrite: bool = overwrite.unwrap_or(true);
        
        todo!();
        /*
            if (!m_database.m_db) return false;
        assert(m_insert_stmt && m_overwrite_stmt);

        sqlite3_stmt* stmt;
        if (overwrite) {
            stmt = m_overwrite_stmt;
        } else {
            stmt = m_insert_stmt;
        }

        // Bind: leftmost parameter in statement is index 1
        // Insert index 1 is key, 2 is value
        int res = sqlite3_bind_blob(stmt, 1, key.data(), key.size(), SQLITE_STATIC);
        if (res != SQLITE_OK) {
            LogPrintf("%s: Unable to bind key to statement: %s\n", __func__, sqlite3_errstr(res));
            sqlite3_clear_bindings(stmt);
            sqlite3_reset(stmt);
            return false;
        }
        res = sqlite3_bind_blob(stmt, 2, value.data(), value.size(), SQLITE_STATIC);
        if (res != SQLITE_OK) {
            LogPrintf("%s: Unable to bind value to statement: %s\n", __func__, sqlite3_errstr(res));
            sqlite3_clear_bindings(stmt);
            sqlite3_reset(stmt);
            return false;
        }

        // Execute
        res = sqlite3_step(stmt);
        sqlite3_clear_bindings(stmt);
        sqlite3_reset(stmt);
        if (res != SQLITE_DONE) {
            LogPrintf("%s: Unable to execute statement: %s\n", __func__, sqlite3_errstr(res));
        }
        return res == SQLITE_DONE;
        */
    }
    
    pub fn erase_key(&mut self, key: DataStream) -> bool {
        
        todo!();
        /*
            if (!m_database.m_db) return false;
        assert(m_delete_stmt);

        // Bind: leftmost parameter in statement is index 1
        int res = sqlite3_bind_blob(m_delete_stmt, 1, key.data(), key.size(), SQLITE_STATIC);
        if (res != SQLITE_OK) {
            LogPrintf("%s: Unable to bind statement: %s\n", __func__, sqlite3_errstr(res));
            sqlite3_clear_bindings(m_delete_stmt);
            sqlite3_reset(m_delete_stmt);
            return false;
        }

        // Execute
        res = sqlite3_step(m_delete_stmt);
        sqlite3_clear_bindings(m_delete_stmt);
        sqlite3_reset(m_delete_stmt);
        if (res != SQLITE_DONE) {
            LogPrintf("%s: Unable to execute statement: %s\n", __func__, sqlite3_errstr(res));
        }
        return res == SQLITE_DONE;
        */
    }
    
    pub fn has_key(&mut self, key: DataStream) -> bool {
        
        todo!();
        /*
            if (!m_database.m_db) return false;
        assert(m_read_stmt);

        // Bind: leftmost parameter in statement is index 1
        bool ret = false;
        int res = sqlite3_bind_blob(m_read_stmt, 1, key.data(), key.size(), SQLITE_STATIC);
        if (res == SQLITE_OK) {
            res = sqlite3_step(m_read_stmt);
            if (res == SQLITE_ROW) {
                ret = true;
            }
        }

        sqlite3_clear_bindings(m_read_stmt);
        sqlite3_reset(m_read_stmt);
        return ret;
        */
    }
    
    pub fn start_cursor(&mut self) -> bool {
        
        todo!();
        /*
            assert(!m_cursor_init);
        if (!m_database.m_db) return false;
        m_cursor_init = true;
        return true;
        */
    }
    
    pub fn read_at_cursor(&mut self, 
        key:      &mut DataStream,
        value:    &mut DataStream,
        complete: &mut bool) -> bool {
        
        todo!();
        /*
            complete = false;

        if (!m_cursor_init) return false;

        int res = sqlite3_step(m_cursor_stmt);
        if (res == SQLITE_DONE) {
            complete = true;
            return true;
        }
        if (res != SQLITE_ROW) {
            LogPrintf("SQLiteBatch::ReadAtCursor: Unable to execute cursor step: %s\n", sqlite3_errstr(res));
            return false;
        }

        // Leftmost column in result is index 0
        const char* key_data = reinterpret_cast<const char*>(sqlite3_column_blob(m_cursor_stmt, 0));
        int key_data_size = sqlite3_column_bytes(m_cursor_stmt, 0);
        key.write(key_data, key_data_size);
        const char* value_data = reinterpret_cast<const char*>(sqlite3_column_blob(m_cursor_stmt, 1));
        int value_data_size = sqlite3_column_bytes(m_cursor_stmt, 1);
        value.write(value_data, value_data_size);
        return true;
        */
    }
    
    pub fn close_cursor(&mut self)  {
        
        todo!();
        /*
            sqlite3_reset(m_cursor_stmt);
        m_cursor_init = false;
        */
    }
    
    pub fn txn_begin(&mut self) -> bool {
        
        todo!();
        /*
            if (!m_database.m_db || sqlite3_get_autocommit(m_database.m_db) == 0) return false;
        int res = sqlite3_exec(m_database.m_db, "BEGIN TRANSACTION", nullptr, nullptr, nullptr);
        if (res != SQLITE_OK) {
            LogPrintf("SQLiteBatch: Failed to begin the transaction\n");
        }
        return res == SQLITE_OK;
        */
    }
    
    pub fn txn_commit(&mut self) -> bool {
        
        todo!();
        /*
            if (!m_database.m_db || sqlite3_get_autocommit(m_database.m_db) != 0) return false;
        int res = sqlite3_exec(m_database.m_db, "COMMIT TRANSACTION", nullptr, nullptr, nullptr);
        if (res != SQLITE_OK) {
            LogPrintf("SQLiteBatch: Failed to commit the transaction\n");
        }
        return res == SQLITE_OK;
        */
    }
    
    pub fn txn_abort(&mut self) -> bool {
        
        todo!();
        /*
            if (!m_database.m_db || sqlite3_get_autocommit(m_database.m_db) != 0) return false;
        int res = sqlite3_exec(m_database.m_db, "ROLLBACK TRANSACTION", nullptr, nullptr, nullptr);
        if (res != SQLITE_OK) {
            LogPrintf("SQLiteBatch: Failed to abort the transaction\n");
        }
        return res == SQLITE_OK;
        */
    }
    
    pub fn setup_sql_statements(&mut self)  {
        
        todo!();
        /*
            const std::vector<std::pair<sqlite3_stmt**, const char*>> statements{
            {&m_read_stmt, "SELECT value FROM main WHERE key = ?"},
            {&m_insert_stmt, "INSERT INTO main VALUES(?, ?)"},
            {&m_overwrite_stmt, "INSERT or REPLACE into main values(?, ?)"},
            {&m_delete_stmt, "DELETE FROM main WHERE key = ?"},
            {&m_cursor_stmt, "SELECT key, value FROM main"},
        };

        for (const auto& [stmt_prepared, stmt_text] : statements) {
            if (*stmt_prepared == nullptr) {
                int res = sqlite3_prepare_v2(m_database.m_db, stmt_text, -1, stmt_prepared, nullptr);
                if (res != SQLITE_OK) {
                    throw std::runtime_error(strprintf(
                        "SQLiteDatabase: Failed to setup SQL statements: %s\n", sqlite3_errstr(res)));
                }
            }
        }
        */
    }
}
