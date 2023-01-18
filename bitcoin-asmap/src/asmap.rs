crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/asmap.h]
//-------------------------------------------[.cpp/bitcoin/src/util/asmap.cpp]

/**
  | Read asmap from provided binary file
  |
  */
pub fn decode_asmap(path: Box<Path>) -> Vec<bool> {
    
    todo!();
        /*
        std::vector<bool> bits;
        FILE *filestr = fsbridge::fopen(path, "rb");
        CAutoFile file(filestr, SER_DISK, CLIENT_VERSION);
        if (file.IsNull()) {
            LogPrintf("Failed to open asmap file from disk\n");
            return bits;
        }
        fseek(filestr, 0, SEEK_END);
        int length = ftell(filestr);
        LogPrintf("Opened asmap file %s (%d bytes) from disk\n", fs::quoted(fs::PathToString(path)), length);
        fseek(filestr, 0, SEEK_SET);
        uint8_t cur_byte;
        for (int i = 0; i < length; ++i) {
            file >> cur_byte;
            for (int bit = 0; bit < 8; ++bit) {
                bits.push_back((cur_byte >> bit) & 1);
            }
        }
        if (!SanityCheckASMap(bits, 128)) {
            LogPrintf("Sanity check of asmap file %s failed\n", fs::quoted(fs::PathToString(path)));
            return {};
        }
        return bits;
        */
}

