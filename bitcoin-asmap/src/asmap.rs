/*!
This function, `decode_asmap`, reads an ASMAP from
a provided binary file and returns a vector of boolean
values. The ASMAP file contains a compact representation of
the IP-to-ASN mapping used for bucketing peers in the
Bitcoin network based on their Autonomous System Number
(ASN). The purpose of this function is to load the ASMAP
data from the file and convert it into a more easily
accessible data structure.

Here's a brief explanation of the C++ code:

1. It initializes an empty vector of boolean values called
   `bits`.

2. It attempts to open the provided binary file using the
   `fsbridge::fopen` function.

3. If the file cannot be opened, a log message is printed,
   and the empty `bits` vector is returned.

4. The code reads the file's length by seeking to the end
   and using `ftell` to get the current position.

5. The file is then read byte by byte, and each byte's bits
   are added to the `bits` vector.

6. After the entire file is read, a sanity check is
   performed on the `bits` vector using the
   `SanityCheckASMap` function. If the sanity check fails,
   a log message is printed, and an empty vector is
   returned.

7. If the sanity check passes, the `bits` vector is
   returned.

In Rust, the `decode_asmap` function will have a similar
structure. It will read the binary file from the given path,
convert it into a vector of boolean values, perform a sanity
check on the data, and return the vector. The `todo!();`
macro is a placeholder that should be replaced with the Rust
implementation that follows the same logic as the C++ code
provided.
*/

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

