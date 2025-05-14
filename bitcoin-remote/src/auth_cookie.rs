// ---------------- [ File: bitcoin-remote/src/auth_cookie.rs ]
crate::ix!();

/**
  | Username used when cookie authentication
  | is in use (arbitrary, only for recognizability
  | in debugging/logging purposes)
  |
  */
pub const COOKIEAUTH_USER: &'static str = "__cookie__";

/**
  | Default name for auth cookie file
  |
  */
pub const COOKIEAUTH_FILE: &'static str = ".cookie";

/**
  | Get name of RPC authentication cookie
  | file
  |
  */
pub fn get_auth_cookie_file(temp: Option<bool>) -> Box<Path> {

    let temp: bool = temp.unwrap_or(false);

    todo!();
        /*
            std::string arg = gArgs.GetArg("-rpccookiefile", COOKIEAUTH_FILE);
        if (temp) {
            arg += ".tmp";
        }
        return AbsPathForConfigVal(fs::PathFromString(arg));
        */
}

/**
  | Generate a new RPC authentication cookie
  | and write it to disk
  |
  */
pub fn generate_auth_cookie(cookie_out: &mut String) -> bool {
    
    todo!();
        /*
            const size_t COOKIE_SIZE = 32;
        unsigned char rand_pwd[COOKIE_SIZE];
        GetRandBytes(rand_pwd, COOKIE_SIZE);
        std::string cookie = COOKIEAUTH_USER + ":" + HexStr(rand_pwd);

        /** the umask determines what permissions are used to create this file -
         * these are set to 077 in init.cpp unless overridden with -sysperms.
         */
        fsbridge::ofstream file;
        fs::path filepath_tmp = GetAuthCookieFile(true);
        file.open(filepath_tmp);
        if (!file.is_open()) {
            LogPrintf("Unable to open cookie authentication file %s for writing\n", fs::PathToString(filepath_tmp));
            return false;
        }
        file << cookie;
        file.close();

        fs::path filepath = GetAuthCookieFile(false);
        if (!RenameOver(filepath_tmp, filepath)) {
            LogPrintf("Unable to rename cookie authentication file %s to %s\n", fs::PathToString(filepath_tmp), fs::PathToString(filepath));
            return false;
        }
        LogPrintf("Generated RPC authentication cookie %s\n", fs::PathToString(filepath));

        if (cookie_out)
            *cookie_out = cookie;
        return true;
        */
}

/**
  | Read the RPC authentication cookie
  | from disk
  |
  */
pub fn get_auth_cookie(cookie_out: &mut String) -> bool {

    todo!();
        /*
            fsbridge::ifstream file;
        std::string cookie;
        fs::path filepath = GetAuthCookieFile();
        file.open(filepath);
        if (!file.is_open())
            return false;
        std::getline(file, cookie);
        file.close();

        if (cookie_out)
            *cookie_out = cookie;
        return true;
        */
}

/**
  | Delete RPC authentication cookie from
  | disk
  |
  */
pub fn delete_auth_cookie()  {
    
    todo!();
        /*
            try {
            fs::remove(GetAuthCookieFile());
        } catch (const fs::filesystem_error& e) {
            LogPrintf("%s: Unable to remove random auth cookie file: %s\n", __func__, fsbridge::get_filesystem_error_message(e));
        }
        */
}
