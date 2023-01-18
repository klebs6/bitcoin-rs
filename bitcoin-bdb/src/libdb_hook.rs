//TODO: there is a rust crate called libdb, which
//looks like it might work... but there are some
//problems statically linking berkeleydb on osx
pub mod libdb {
    pub type NotLinked   = i32;//TODO
    pub type Db          = NotLinked;
    pub type DbTxn       = NotLinked;
    pub type Cursor      = NotLinked;
    pub type Transaction = NotLinked;
    pub type DBT         = NotLinked;
    pub type Env         = NotLinked;
    pub const DB_TXN_WRITE_NOSYNC: i32 = 0;
    pub const DB_FILE_ID_LEN:    usize = 0;
}

