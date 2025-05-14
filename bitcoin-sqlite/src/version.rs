// ---------------- [ File: bitcoin-sqlite/src/version.rs ]
crate::ix!();

pub const WALLET_SCHEMA_VERSION: i32 = 0;

lazy_static!{
    /*
    static Mutex g_sqlite_mutex;
    static int g_sqlite_count GUARDED_BY(g_sqlite_mutex) = 0;
    */
}

pub fn sqlite_database_version() -> String {
    
    todo!();
        /*
            return std::string(sqlite3_libversion());
        */
}
