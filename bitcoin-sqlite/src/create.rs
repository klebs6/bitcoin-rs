// ---------------- [ File: bitcoin-sqlite/src/create.rs ]
crate::ix!();

pub fn make_sq_lite_database(
    path:    &Path,
    options: &DatabaseOptions,
    status:  &mut DatabaseStatus,
    error:   &mut BilingualStr) -> Box<sqlite3::Connection> {
    
    todo!();
        /*
            try {
            fs::path data_file = SQLiteDataFile(path);
            auto db = std::make_unique<SQLiteDatabase>(data_file.parent_path(), data_file);
            if (options.verify && !db->Verify(error)) {
                status = DatabaseStatus::FAILED_VERIFY;
                return nullptr;
            }
            status = DatabaseStatus::SUCCESS;
            return db;
        } catch (const std::runtime_error& e) {
            status = DatabaseStatus::FAILED_LOAD;
            error = Untranslated(e.what());
            return nullptr;
        }
        */
}

