// ---------------- [ File: bitcoin-db/src/serdeser.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/addrdb.h]
//-------------------------------------------[.cpp/bitcoin/src/addrdb.cpp]

pub fn serializedb<Stream, Data>(
        stream: &mut Stream,
        data:   &Data) -> bool {

    todo!();
        /*
        // Write and commit header, data
        try {
            CHashWriter hasher(stream.GetType(), stream.GetVersion());
            stream << Params().MessageStart() << data;
            hasher << Params().MessageStart() << data;
            stream << hasher.GetHash();
        } catch (const std::exception& e) {
            return error("%s: Serialize or I/O error - %s", __func__, e.what());
        }

        return true;
        */
}


pub fn deserializedb<Stream, Data>(
        stream:    &mut Stream,
        data:      &mut Data,
        check_sum: Option<bool>)  {
    let check_sum: bool = check_sum.unwrap_or(true);
    todo!();
        /*
        CHashVerifier<Stream> verifier(&stream);
        // de-serialize file header (network specific magic number) and ..
        unsigned char pchMsgTmp[4];
        verifier >> pchMsgTmp;
        // ... verify the network matches ours
        if (memcmp(pchMsgTmp, Params().MessageStart(), sizeof(pchMsgTmp))) {
            throw std::runtime_error{"Invalid network magic number"};
        }

        // de-serialize data
        verifier >> data;

        // verify checksum
        if (fCheckSum) {
            uint256 hashTmp;
            stream >> hashTmp;
            if (hashTmp != verifier.GetHash()) {
                throw std::runtime_error{"Checksum mismatch, data corrupted"};
            }
        }
        */
}
