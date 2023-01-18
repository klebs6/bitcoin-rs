crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/bip32.h]
//-------------------------------------------[.cpp/bitcoin/src/util/bip32.cpp]

/**
  | Parse an HD keypaths like "m/7/0'/2000".
  |
  */
pub fn parse_hd_keypath(
        keypath_str: &String,
        keypath:     &mut Vec<u32>) -> bool {
    
    todo!();
        /*
            std::stringstream ss(keypath_str);
        std::string item;
        bool first = true;
        while (std::getline(ss, item, '/')) {
            if (item.compare("m") == 0) {
                if (first) {
                    first = false;
                    continue;
                }
                return false;
            }
            // Finds whether it is hardened
            uint32_t path = 0;
            size_t pos = item.find("'");
            if (pos != std::string::npos) {
                // The hardened tick can only be in the last index of the string
                if (pos != item.size() - 1) {
                    return false;
                }
                path |= 0x80000000;
                item = item.substr(0, item.size() - 1); // Drop the last character which is the hardened tick
            }

            // Ensure this is only numbers
            if (item.find_first_not_of( "0123456789" ) != std::string::npos) {
                return false;
            }
            uint32_t number;
            if (!ParseUInt32(item, &number)) {
                return false;
            }
            path |= number;

            keypath.push_back(path);
            first = false;
        }
        return true;
        */
}

pub fn format_hd_keypath(path: &Vec<u32>) -> String {
    
    todo!();
        /*
            std::string ret;
        for (auto i : path) {
            ret += strprintf("/%i", (i << 1) >> 1);
            if (i >> 31) ret += '\'';
        }
        return ret;
        */
}

/**
  | Write HD keypaths as strings
  |
  */
pub fn write_hd_keypath(keypath: &Vec<u32>) -> String {
    
    todo!();
        /*
            return "m" + FormatHDKeypath(keypath);
        */
}

