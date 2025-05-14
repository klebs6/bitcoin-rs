// ---------------- [ File: bitcoin-bdb/src/wallet_database_fileid.rs ]
crate::ix!();

pub struct WalletDatabaseFileId {
    value: [u8; libdb::DB_FILE_ID_LEN],
}

impl PartialEq<WalletDatabaseFileId> for WalletDatabaseFileId {
    
    #[inline] fn eq(&self, other: &WalletDatabaseFileId) -> bool {
        todo!();
        /*
            return memcmp(value, &rhs.value, sizeof(value)) == 0;
        */
    }
}

impl Eq for WalletDatabaseFileId {}
