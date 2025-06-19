crate::ix!();

pub fn decode_base32_bytes(
        p:          *const u8,
        pf_invalid: Option<*mut bool>) -> Vec<u8> {
    
    todo!();
        /*
            static const int decode32_table[256] =
        {
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 26, 27, 28, 29, 30, 31, -1, -1, -1, -1,
            -1, -1, -1, -1, -1,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14,
            15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1, -1,  0,  1,  2,
             3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
            23, 24, 25, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
        };

        const char* e = p;
        std::vector<uint8_t> val;
        val.reserve(strlen(p));
        while (*p != 0) {
            int x = decode32_table[(unsigned char)*p];
            if (x == -1) break;
            val.push_back(x);
            ++p;
        }

        std::vector<unsigned char> ret;
        ret.reserve((val.size() * 5) / 8);
        bool valid = ConvertBits<5, 8, false>([&](unsigned char c) { ret.push_back(c); }, val.begin(), val.end());

        const char* q = p;
        while (valid && *p != 0) {
            if (*p != '=') {
                valid = false;
                break;
            }
            ++p;
        }
        valid = valid && (p - e) % 8 == 0 && p - q < 8;
        if (pf_invalid) *pf_invalid = !valid;

        return ret;
        */
}

pub fn decode_base32(
        str_:       &str,
        pf_invalid: Option<*mut bool>) -> String {
    
    todo!();
        /*
            if (!ValidAsCString(str)) {
            if (pf_invalid) {
                *pf_invalid = true;
            }
            return {};
        }
        std::vector<unsigned char> vchRet = DecodeBase32(str.c_str(), pf_invalid);
        return std::string((const char*)vchRet.data(), vchRet.size());
        */
}

pub fn decode_base64_bytes(
        p:          *const u8,
        pf_invalid: Option<*mut bool>) -> Vec<u8> {
    
    todo!();
        /*
            static const int decode64_table[256] =
        {
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, 62, -1, -1, -1, 63, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1,
            -1, -1, -1, -1, -1,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14,
            15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1, -1, 26, 27, 28,
            29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
            49, 50, 51, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
        };

        const char* e = p;
        std::vector<uint8_t> val;
        val.reserve(strlen(p));
        while (*p != 0) {
            int x = decode64_table[(unsigned char)*p];
            if (x == -1) break;
            val.push_back(x);
            ++p;
        }

        std::vector<unsigned char> ret;
        ret.reserve((val.size() * 3) / 4);
        bool valid = ConvertBits<6, 8, false>([&](unsigned char c) { ret.push_back(c); }, val.begin(), val.end());

        const char* q = p;
        while (valid && *p != 0) {
            if (*p != '=') {
                valid = false;
                break;
            }
            ++p;
        }
        valid = valid && (p - e) % 4 == 0 && p - q < 4;
        if (pf_invalid) *pf_invalid = !valid;

        return ret;
        */
}

pub fn decode_base64(
        str_:       &str,
        pf_invalid: Option<&mut bool>) -> String {
    
    todo!();
        /*
            if (!ValidAsCString(str)) {
            if (pf_invalid) {
                *pf_invalid = true;
            }
            return {};
        }
        std::vector<unsigned char> vchRet = DecodeBase64(str.c_str(), pf_invalid);
        return std::string((const char*)vchRet.data(), vchRet.size());
        */
}
