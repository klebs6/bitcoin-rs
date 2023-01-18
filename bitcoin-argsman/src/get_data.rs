crate::ix!();

/**
  | Windows: C:\Users\Username\AppData\Roaming\Bitcoin
  |
  | macOS: ~/Library/Application Support/Bitcoin
  | 
  | Unix-like: ~/.bitcoin
  |
  */
#[cfg(target_os = "windows")]
pub fn get_default_data_dir() -> PathBuf {

    let mut path: PathBuf = get_special_folder_path(csidl_appdata);
    path.push("Bitcoin");
    path
}

#[cfg(target_os = "macos")]
pub fn get_default_data_dir() -> PathBuf {

    let mut path_ret = get_home_dir();

    path_ret.push("Library/Application Support/Bitcoin");

    path_ret
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn get_default_data_dir() -> PathBuf {

    let mut path_ret = get_home_dir();

    //  Unix-like
    path_ret.push(".bitcoin");

    path_ret
}

#[cfg(not(target_os = "windows"))]
pub fn get_home_dir() -> PathBuf {

    let psz_home = env::var("HOME");

    if psz_home.is_err() || psz_home.as_ref().unwrap().is_empty() {
        PathBuf::from("/")
    } else {
        PathBuf::from(psz_home.unwrap())
    }
}

impl ArgsManagerInner {

    /**
      | Get data directory path
      | 
      | 
      | -----------
      | @return
      | 
      | Absolute path on success, otherwise
      | an empty path when a non-directory path
      | would be returned @post Returned directory
      | path is created unless it is empty
      |
      */
    pub fn get_data_dir_base(&self) -> PathBuf {
        self.get_data_dir(false)
    }

    /**
      | Get data directory path with appended
      | network identifier
      | 
      | -----------
      | @return
      | 
      | Absolute path on success, otherwise
      | an empty path when a non-directory path
      | would be returned @post Returned directory
      | path is created unless it is empty
      |
      */
    pub fn get_data_dir_net(&self) -> PathBuf {
        self.get_data_dir(true)
    }

    /**
      | Get data directory path
      | 
      | -----------
      | @param net_specific
      | 
      | Append network identifier to the returned
      | path
      | 
      | -----------
      | @return
      | 
      | Absolute path on success, otherwise
      | an empty path when a non-directory path
      | would be returned @post Returned directory
      | path is created unless it is empty
      |
      */
    pub fn get_data_dir(&self, net_specific: bool) -> PathBuf {
        
        let maybe_cached = match net_specific {
            true   => self.cached_network_datadir_path.as_ref(),
            false  => self.cached_datadir_path.as_ref()
        };

        // Cache the path to avoid calling
        // create_directories on every call of
        // this function
        if let Some(path) = maybe_cached {
            return path.to_path_buf();
        }

        let mut buf: PathBuf = PathBuf::new();

        let datadir: String = self.get_arg("-datadir","");

        if datadir.len() != 0 {

            let arg_path = std::fs::canonicalize(Path::new(&datadir)).unwrap();

            buf.push(arg_path);

            if !buf.as_path().is_dir() {
                buf.clear();
                return buf.to_path_buf();
            }

        } else {
            buf.push(get_default_data_dir());
        }

        if net_specific {
            buf.push(base_params().data_dir());
        }

        if let Ok(result) = std::fs::create_dir_all(buf.as_path()) {

            let mut subdir = buf.clone();

            subdir.push("wallets");

            // This is the first run, create
            // wallets subdirectory too
            std::fs::create_dir_all(subdir);
        }

        strip_redundant_last_elements_of_path(&mut buf);

        buf
    }
}
