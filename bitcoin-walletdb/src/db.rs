// ---------------- [ File: bitcoin-walletdb/src/db.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/wallet/db.h]

pub fn split_wallet_path(
        wallet_path:       &Path,
        env_directory:     &mut Path,
        database_filename: &mut String)  {
    
    todo!();
        /*
        
        */
}


/**
  | RAII class that provides access to a
  | DummyDatabase. Never fails.
  |
  */
pub struct DummyBatch {
    base: DatabaseBatch,
}

impl DummyBatch {
    
    pub fn read_key(&mut self, 
        key:   DataStream,
        value: &mut DataStream) -> bool {
        
        todo!();
        /*
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
            return true;
        */
    }
    
    pub fn erase_key(&mut self, key: DataStream) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn has_key(&mut self, key: DataStream) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn flush(&mut self)  { }
    
    pub fn close(&mut self)  { }
    
    pub fn start_cursor(&mut self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn read_at_cursor(&mut self, 
        ss_key:   &mut DataStream,
        ss_value: &mut DataStream,
        complete: &mut bool) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn close_cursor(&mut self)  { }
    
    pub fn txn_begin(&mut self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn txn_commit(&mut self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn txn_abort(&mut self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

/**
  | A dummy WalletDatabase that does nothing
  | and never fails. Only used by unit tests.
  |
  */
pub struct DummyDatabase {
    base: WalletDatabase,
}

impl DummyDatabase {

    pub fn open(&mut self)  {
        
        
    }
    
    pub fn add_ref(&mut self)  {
        
        
    }
    
    pub fn remove_ref(&mut self)  {
        
        
    }
    
    pub fn rewrite(&mut self, psz_skip: Option<*const u8>) -> bool {

        todo!();
        /*
            return true;
        */
    }
    
    pub fn backup(&self, str_dest: &String) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn close(&mut self)  {
        
        
    }
    
    pub fn flush(&mut self)  {
        
        
    }
    
    pub fn periodic_flush(&mut self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn increment_update_counter(&mut self)  {
        
        todo!();
        /*
            ++nUpdateCounter;
        */
    }
    
    pub fn reload_db_env(&mut self)  {
        
        
    }
    
    pub fn filename(&mut self) -> String {
        
        todo!();
        /*
            return "dummy";
        */
    }
    
    pub fn format(&mut self) -> String {
        
        todo!();
        /*
            return "dummy";
        */
    }
    
    pub fn make_batch(&mut self, flush_on_close: Option<bool>) -> Box<DatabaseBatch> {
        let flush_on_close: bool = flush_on_close.unwrap_or(true);

        todo!();
        /*
            return std::make_unique<DummyBatch>();
        */
    }
}

pub enum DatabaseFormat {
    BERKELEY,
    SQLITE,
}

pub struct DatabaseOptions {
    require_existing:  bool, // default = false
    require_create:    bool, // default = false
    require_format:    Option<DatabaseFormat>,
    create_flags:      u64, // default = 0
    create_passphrase: SecureString,
    verify:            bool, // default = true
}

pub enum DatabaseStatus {
    SUCCESS,
    FAILED_BAD_PATH,
    FAILED_BAD_FORMAT,
    FAILED_ALREADY_LOADED,
    FAILED_ALREADY_EXISTS,
    FAILED_NOT_FOUND,
    FAILED_CREATE,
    FAILED_LOAD,
    FAILED_VERIFY,
    FAILED_ENCRYPT,
}

pub fn make_database(
        path:    &Path,
        options: &DatabaseOptions,
        status:  &mut DatabaseStatus,
        error:   &mut BilingualStr) -> Box<WalletDatabase> {
    todo!();
}

//-------------------------------------------[.cpp/bitcoin/src/wallet/db.cpp]

/**
  | Recursively list database paths in
  | directory.
  |
  */
pub fn list_databases(wallet_dir: &Path) -> Vec<Box<Path>> {
    
    todo!();
        /*
            const size_t offset = wallet_dir.native().size() + (wallet_dir == wallet_dir.root_name() ? 0 : 1);
        std::vector<fs::path> paths;
        boost::system::error_code ec;

        for (auto it = fs::recursive_directory_iterator(wallet_dir, ec); it != fs::recursive_directory_iterator(); it.increment(ec)) {
            if (ec) {
                if (fs::is_directory(*it)) {
                    it.no_push();
                    LogPrintf("%s: %s %s -- skipping.\n", __func__, ec.message(), fs::PathToString(it->path()));
                } else {
                    LogPrintf("%s: %s %s\n", __func__, ec.message(), fs::PathToString(it->path()));
                }
                continue;
            }

            try {
                // Get wallet path relative to walletdir by removing walletdir from the wallet path.
                // This can be replaced by boost::filesystem::lexically_relative once boost is bumped to 1.60.
                const auto path_str = it->path().native().substr(offset);
                const fs::path path{path_str.begin(), path_str.end()};

                if (it->status().type() == fs::directory_file &&
                    (IsBDBFile(BDBDataFile(it->path())) || IsSQLiteFile(SQLiteDataFile(it->path())))) {
                    // Found a directory which contains wallet.dat btree file, add it as a wallet.
                    paths.emplace_back(path);
                } else if (it.level() == 0 && it->symlink_status().type() == fs::regular_file && IsBDBFile(it->path())) {
                    if (it->path().filename() == "wallet.dat") {
                        // Found top-level wallet.dat btree file, add top level directory ""
                        // as a wallet.
                        paths.emplace_back();
                    } else {
                        // Found top-level btree file not called wallet.dat. Current bitcoin
                        // software will never create these files but will allow them to be
                        // opened in a shared database environment for backwards compatibility.
                        // Add it to the list of available wallets.
                        paths.emplace_back(path);
                    }
                }
            } catch (const std::exception& e) {
                LogPrintf("%s: Error scanning %s: %s\n", __func__, fs::PathToString(it->path()), e.what());
                it.no_push();
            }
        }

        return paths;
        */
}

pub fn bdb_data_file(wallet_path: &Path) -> Box<Path> {
    
    todo!();
        /*
            if (fs::is_regular_file(wallet_path)) {
            // Special case for backwards compatibility: if wallet path points to an
            // existing file, treat it as the path to a BDB data file in a parent
            // directory that also contains BDB log files.
            return wallet_path;
        } else {
            // Normal case: Interpret wallet path as a directory path containing
            // data and log files.
            return wallet_path / "wallet.dat";
        }
        */
}

pub fn sq_lite_data_file(path: &Path) -> Box<Path> {
    
    todo!();
        /*
            return path / "wallet.dat";
        */
}

pub fn is_bdb_file(path: &Path) -> bool {
    
    todo!();
        /*
            if (!fs::exists(path)) return false;

        // A Berkeley DB Btree file has at least 4K.
        // This check also prevents opening lock files.
        boost::system::error_code ec;
        auto size = fs::file_size(path, ec);
        if (ec) LogPrintf("%s: %s %s\n", __func__, ec.message(), fs::PathToString(path));
        if (size < 4096) return false;

        fsbridge::ifstream file(path, std::ios::binary);
        if (!file.is_open()) return false;

        file.seekg(12, std::ios::beg); // Magic bytes start at offset 12
        uint32_t data = 0;
        file.read((char*) &data, sizeof(data)); // Read 4 bytes of file to compare against magic

        // Berkeley DB Btree magic bytes, from:
        //  https://github.com/file/file/blob/5824af38469ec1ca9ac3ffd251e7afe9dc11e227/magic/Magdir/database#L74-L75
        //  - big endian systems - 00 05 31 62
        //  - little endian systems - 62 31 05 00
        return data == 0x00053162 || data == 0x62310500;
        */
}

pub fn is_sq_lite_file(path: &Path) -> bool {
    
    todo!();
        /*
            if (!fs::exists(path)) return false;

        // A SQLite Database file is at least 512 bytes.
        boost::system::error_code ec;
        auto size = fs::file_size(path, ec);
        if (ec) LogPrintf("%s: %s %s\n", __func__, ec.message(), fs::PathToString(path));
        if (size < 512) return false;

        fsbridge::ifstream file(path, std::ios::binary);
        if (!file.is_open()) return false;

        // Magic is at beginning and is 16 bytes long
        char magic[16];
        file.read(magic, 16);

        // Application id is at offset 68 and 4 bytes long
        file.seekg(68, std::ios::beg);
        char app_id[4];
        file.read(app_id, 4);

        file.close();

        // Check the magic, see https://sqlite.org/fileformat2.html
        std::string magic_str(magic, 16);
        if (magic_str != std::string("SQLite format 3", 16)) {
            return false;
        }

        // Check the application id matches our network magic
        return memcmp(Params().MessageStart(), app_id, 4) == 0;
        */
}
