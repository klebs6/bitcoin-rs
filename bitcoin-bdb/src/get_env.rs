// ---------------- [ File: bitcoin-bdb/src/get_env.rs ]
crate::ix!();

lazy_static!{
    /*
    RecursiveMutex cs_db;
    std::map<std::string, std::weak_ptr<BerkeleyEnvironment>> g_dbenvs GUARDED_BY(cs_db); /// Map from directory name to db environment.
    */
}

/**
  | Get BerkeleyEnvironment given a directory
  | path.
  |
  | @param[in] env_directory
  | 
  | Path to environment directory
  | 
  | -----------
  | @return
  | 
  | A shared pointer to the BerkeleyEnvironment
  | object for the wallet directory, never
  | empty because ~BerkeleyEnvironment
  | erases the weak pointer from the g_dbenvs
  | map. @post A new BerkeleyEnvironment
  | weak pointer is inserted into g_dbenvs
  | if the directory path key was not already
  | in the map.
  |
  */
pub fn get_berkeley_env(env_directory: &Path) -> Arc<BerkeleyEnvironment> {
    
    todo!();
        /*
            LOCK(cs_db);
        auto inserted = g_dbenvs.emplace(fs::PathToString(env_directory), std::weak_ptr<BerkeleyEnvironment>());
        if (inserted.second) {
            auto env = std::make_shared<BerkeleyEnvironment>(env_directory);
            inserted.first->second = env;
            return env;
        }
        return inserted.first->second.lock();
        */
}
