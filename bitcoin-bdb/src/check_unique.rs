crate::ix!();

/**
  | Make sure database has a unique fileid within
  | the environment. If it doesn't, throw an
  | error. BDB caches do not work properly when
  | more than one open database has the same
  | fileid (values written to one database may
  | show up in reads to other databases).
  |
  | BerkeleyDB generates unique fileids by default
  | (https://docs.oracle.com/cd/E17275_01/html/programmer_reference/program_copy.html),
  | so bitcoin should never create different
  | databases with the same fileid, but this error
  | can be triggered if users manually copy
  | database files.
  */
pub fn check_unique_fileid(
        env:      &BerkeleyEnvironment,
        filename: &String,
        db:       &mut libdb::Db,
        fileid:   &mut WalletDatabaseFileId)  {
    
    todo!();
        /*
            if (env.IsMock()) return;

        int ret = db.get_mpf()->get_fileid(fileid.value);
        if (ret != 0) {
            throw std::runtime_error(strprintf("BerkeleyDatabase: Can't open database %s (get_fileid failed with %d)", filename, ret));
        }

        for (const auto& item : env.m_fileids) {
            if (fileid == item.second && &fileid != &item.second) {
                throw std::runtime_error(strprintf("BerkeleyDatabase: Can't open database %s (duplicates fileid %s from %s)", filename,
                    HexStr(item.second.value), item.first));
            }
        }
        */
}

