crate::ix!();

/**
  | Return object for accessing temporary
  | in-memory database.
  |
  */
pub fn create_mock_wallet_database() -> Box<WalletDatabase> {
    
    todo!();
        /*
            #ifdef USE_SQLITE
        return std::make_unique<SQLiteDatabase>("", "", true);
    #elif USE_BDB
        return std::make_unique<BerkeleyDatabase>(std::make_shared<BerkeleyEnvironment>(), "");
    #endif
        */
}

/**
  | Return object for accessing dummy database
  | with no read/write capabilities.
  |
  */
pub fn create_dummy_wallet_database() -> Box<WalletDatabase> {
    
    todo!();
        /*
            return std::make_unique<DummyDatabase>();
        */
}

pub fn make_database(
        path:    &Path,
        options: &DatabaseOptions,
        status:  &mut DatabaseStatus,
        error:   &mut BilingualStr) -> Box<WalletDatabase> {
    
    todo!();
        /*
            bool exists;
        try {
            exists = fs::symlink_status(path).type() != fs::file_not_found;
        } catch (const fs::filesystem_error& e) {
            error = Untranslated(strprintf("Failed to access database path '%s': %s", fs::PathToString(path), fsbridge::get_filesystem_error_message(e)));
            status = DatabaseStatus::FAILED_BAD_PATH;
            return nullptr;
        }

        std::optional<DatabaseFormat> format;
        if (exists) {
            if (IsBDBFile(BDBDataFile(path))) {
                format = DatabaseFormat::BERKELEY;
            }
            if (IsSQLiteFile(SQLiteDataFile(path))) {
                if (format) {
                    error = Untranslated(strprintf("Failed to load database path '%s'. Data is in ambiguous format.", fs::PathToString(path)));
                    status = DatabaseStatus::FAILED_BAD_FORMAT;
                    return nullptr;
                }
                format = DatabaseFormat::SQLITE;
            }
        } else if (options.require_existing) {
            error = Untranslated(strprintf("Failed to load database path '%s'. Path does not exist.", fs::PathToString(path)));
            status = DatabaseStatus::FAILED_NOT_FOUND;
            return nullptr;
        }

        if (!format && options.require_existing) {
            error = Untranslated(strprintf("Failed to load database path '%s'. Data is not in recognized format.", fs::PathToString(path)));
            status = DatabaseStatus::FAILED_BAD_FORMAT;
            return nullptr;
        }

        if (format && options.require_create) {
            error = Untranslated(strprintf("Failed to create database path '%s'. Database already exists.", fs::PathToString(path)));
            status = DatabaseStatus::FAILED_ALREADY_EXISTS;
            return nullptr;
        }

        // A db already exists so format is set, but options also specifies the format, so make sure they agree
        if (format && options.require_format && format != options.require_format) {
            error = Untranslated(strprintf("Failed to load database path '%s'. Data is not in required format.", fs::PathToString(path)));
            status = DatabaseStatus::FAILED_BAD_FORMAT;
            return nullptr;
        }

        // Format is not set when a db doesn't already exist, so use the format specified by the options if it is set.
        if (!format && options.require_format) format = options.require_format;

        // If the format is not specified or detected, choose the default format based on what is available. We prefer BDB over SQLite for now.
        if (!format) {
    #ifdef USE_SQLITE
            format = DatabaseFormat::SQLITE;
    #endif
    #ifdef USE_BDB
            format = DatabaseFormat::BERKELEY;
    #endif
        }

        if (format == DatabaseFormat::SQLITE) {
    #ifdef USE_SQLITE
            return MakeSQLiteDatabase(path, options, status, error);
    #endif
            error = Untranslated(strprintf("Failed to open database path '%s'. Build does not support SQLite database format.", fs::PathToString(path)));
            status = DatabaseStatus::FAILED_BAD_FORMAT;
            return nullptr;
        }

    #ifdef USE_BDB
        return MakeBerkeleyDatabase(path, options, status, error);
    #endif
        error = Untranslated(strprintf("Failed to open database path '%s'. Build does not support Berkeley DB database format.", fs::PathToString(path)));
        status = DatabaseStatus::FAILED_BAD_FORMAT;
        return nullptr;
        */
}
