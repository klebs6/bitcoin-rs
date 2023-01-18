crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/wallet/sqlite.h]

//-------------------------------------------[.cpp/bitcoin/src/wallet/sqlite.cpp]

pub fn error_log_callback(
        arg:  *mut c_void,
        code: i32,
        msg:  *const u8)  {
    
    todo!();
        /*
            // From sqlite3_config() documentation for the SQLITE_CONFIG_LOG option:
        // "The c_void pointer that is the second argument to SQLITE_CONFIG_LOG is passed through as
        // the first parameter to the application-defined logger function whenever that function is
        // invoked."
        // Assert that this is the case:
        assert(arg == nullptr);
        LogPrintf("SQLite Error. Code: %d. Message: %s\n", code, msg);
        */
}
