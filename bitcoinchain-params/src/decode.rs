crate::ix!();

pub fn decode_destination(
    str_:      &str,
    params:    Option<&ChainParams>,
    error_str: Option<&mut str>) -> TxDestination {
    
    todo!();
        /*
            std::vector<unsigned char> data;
        u160 hash;
        error_str = "";
        if (DecodeBase58Check(str, data, 21)) {
            // base58-encoded Bitcoin addresses.
            // Public-key-hash-addresses have version 0 (or 111 testnet).
            // The data vector contains RIPEMD160(SHA256(pubkey)), where pubkey is the serialized public key.
            const std::vector<unsigned char>& pubkey_prefix = params.Base58Prefix(CChainParams::PUBKEY_ADDRESS);
            if (data.size() == hash.size() + pubkey_prefix.size() && std::equal(pubkey_prefix.begin(), pubkey_prefix.end(), data.begin())) {
                std::copy(data.begin() + pubkey_prefix.size(), data.end(), hash.begin());
                return PKHash(hash);
            }
            // Script-hash-addresses have version 5 (or 196 testnet).
            // The data vector contains RIPEMD160(SHA256(cscript)), where cscript is the serialized redemption script.
            const std::vector<unsigned char>& script_prefix = params.Base58Prefix(CChainParams::SCRIPT_ADDRESS);
            if (data.size() == hash.size() + script_prefix.size() && std::equal(script_prefix.begin(), script_prefix.end(), data.begin())) {
                std::copy(data.begin() + script_prefix.size(), data.end(), hash.begin());
                return ScriptHash(hash);
            }

            // Set potential error message.
            // This message may be changed if the address can also be interpreted as a Bech32 address.
            error_str = "Invalid prefix for Base58-encoded address";
        }
        data.clear();
        const auto dec = bech32::Decode(str);
        if ((dec.encoding == bech32::Encoding::BECH32 || dec.encoding == bech32::Encoding::BECH32M) && dec.data.size() > 0) {
            // Bech32 decoding
            error_str = "";
            if (dec.hrp != params.Bech32HRP()) {
                error_str = "Invalid prefix for Bech32 address";
                return CNoDestination();
            }
            int version = dec.data[0]; // The first 5 bit symbol is the witness version (0-16)
            if (version == 0 && dec.encoding != bech32::Encoding::BECH32) {
                error_str = "Version 0 witness address must use Bech32 checksum";
                return CNoDestination();
            }
            if (version != 0 && dec.encoding != bech32::Encoding::BECH32M) {
                error_str = "Version 1+ witness address must use Bech32m checksum";
                return CNoDestination();
            }
            // The rest of the symbols are converted witness program bytes.
            data.reserve(((dec.data.size() - 1) * 5) / 8);
            if (ConvertBits<5, 8, false>([&](unsigned char c) { data.push_back(c); }, dec.data.begin() + 1, dec.data.end())) {
                if (version == 0) {
                    {
                        WitnessV0KeyHash keyid;
                        if (data.size() == keyid.size()) {
                            std::copy(data.begin(), data.end(), keyid.begin());
                            return keyid;
                        }
                    }
                    {
                        WitnessV0ScriptHash scriptid;
                        if (data.size() == scriptid.size()) {
                            std::copy(data.begin(), data.end(), scriptid.begin());
                            return scriptid;
                        }
                    }

                    error_str = "Invalid Bech32 v0 address data size";
                    return CNoDestination();
                }

                if (version == 1 && data.size() == WITNESS_V1_TAPROOT_SIZE) {
                    const_assert(WITNESS_V1_TAPROOT_SIZE == WitnessV1Taproot::size());
                    WitnessV1Taproot tap;
                    std::copy(data.begin(), data.end(), tap.begin());
                    return tap;
                }

                if (version > 16) {
                    error_str = "Invalid Bech32 address witness version";
                    return CNoDestination();
                }

                if (data.size() < 2 || data.size() > BECH32_WITNESS_PROG_MAX_LEN) {
                    error_str = "Invalid Bech32 address data size";
                    return CNoDestination();
                }

                WitnessUnknown unk;
                unk.version = version;
                std::copy(data.begin(), data.end(), unk.program);
                unk.length = data.size();
                return unk;
            }
        }

        // Set error message if address can't be interpreted as Base58 or Bech32.
        if (error_str.empty()) error_str = "Invalid address format";

        return CNoDestination();
        */
}

