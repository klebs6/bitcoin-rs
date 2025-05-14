// ---------------- [ File: bitcoin-sqlite/src/pragma.rs ]
crate::ix!();

pub fn read_pragma_integer(
        db:          *mut sqlite3::Connection,
        key:         &String,
        description: &String,
        error:       &mut BilingualStr) -> Option<i32> {
    
    todo!();
        /*
            std::string stmt_text = strprintf("PRAGMA %s", key);
        sqlite3_stmt* pragma_read_stmt{nullptr};
        int ret = sqlite3_prepare_v2(db, stmt_text.c_str(), -1, &pragma_read_stmt, nullptr);
        if (ret != SQLITE_OK) {
            sqlite3_finalize(pragma_read_stmt);
            error = Untranslated(strprintf("SQLiteDatabase: Failed to prepare the statement to fetch %s: %s", description, sqlite3_errstr(ret)));
            return std::nullopt;
        }
        ret = sqlite3_step(pragma_read_stmt);
        if (ret != SQLITE_ROW) {
            sqlite3_finalize(pragma_read_stmt);
            error = Untranslated(strprintf("SQLiteDatabase: Failed to fetch %s: %s", description, sqlite3_errstr(ret)));
            return std::nullopt;
        }
        int result = sqlite3_column_int(pragma_read_stmt, 0);
        sqlite3_finalize(pragma_read_stmt);
        return result;
        */
}

pub fn set_pragma(
        db:      *mut sqlite3::Connection,
        key:     &String,
        value:   &String,
        err_msg: &String)  {
    
    todo!();
        /*
            std::string stmt_text = strprintf("PRAGMA %s = %s", key, value);
        int ret = sqlite3_exec(db, stmt_text.c_str(), nullptr, nullptr, nullptr);
        if (ret != SQLITE_OK) {
            throw std::runtime_error(strprintf("SQLiteDatabase: %s: %s\n", err_msg, sqlite3_errstr(ret)));
        }
        */
}
