crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/wallet/bdb.cpp]

/**
  | Return object giving access to Berkeley
  | database at specified path.
  |
  */
pub fn make_berkeley_database(
        path:    &Path,
        options: &DatabaseOptions,
        status:  &mut DatabaseStatus,
        error:   &mut BilingualStr) -> Box<BerkeleyDatabase> {
    
    todo!();
        /*
            fs::path data_file = BDBDataFile(path);
        std::unique_ptr<BerkeleyDatabase> db;
        {
            LOCK(cs_db); // Lock env.m_databases until insert in BerkeleyDatabase constructor
            std::string data_filename = fs::PathToString(data_file.filename());
            std::shared_ptr<BerkeleyEnvironment> env = GetBerkeleyEnv(data_file.parent_path());
            if (env->m_databases.count(data_filename)) {
                error = Untranslated(strprintf("Refusing to load database. Data file '%s' is already loaded.", fs::PathToString(env->Directory() / data_filename)));
                status = DatabaseStatus::FAILED_ALREADY_LOADED;
                return nullptr;
            }
            db = std::make_unique<BerkeleyDatabase>(std::move(env), std::move(data_filename));
        }

        if (options.verify && !db->Verify(error)) {
            status = DatabaseStatus::FAILED_VERIFY;
            return nullptr;
        }

        status = DatabaseStatus::SUCCESS;
        return db;
        */
}

/**
  | Perform sanity check of runtime BDB
  | version versus linked BDB version.
  |
  */
pub fn berkeley_database_sanity_check() -> bool {
    
    todo!();
        /*
            int major, minor;
        DbEnv::version(&major, &minor, nullptr);

        /* If the major version differs, or the minor version of library is *older*
         * than the header that was compiled against, flag an error.
         */
        if (major != DB_VERSION_MAJOR || minor < DB_VERSION_MINOR) {
            LogPrintf("BerkeleyDB database version conflict: header version is %d.%d, library version is %d.%d\n",
                DB_VERSION_MAJOR, DB_VERSION_MINOR, major, minor);
            return false;
        }

        return true;
        */
}

pub fn berkeley_database_version() -> String {
    
    todo!();
        /*
            return DbEnv::version(nullptr, nullptr, nullptr);
        */
}
