// ---------------- [ File: bitcoin-fuzz/src/fuzz_secp256k1_ec_seckey_import_export_der.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/secp256k1_ec_seckey_import_export_der.cpp]

pub fn ec_seckey_import_der(
        ctx:       *const Secp256k1Context,
        out32:     *mut u8,
        seckey:    *const u8,
        seckeylen: usize) -> i32 {
    
    todo!();
        /*
        
        */
}

pub fn ec_seckey_export_der(
        ctx:        *const Secp256k1Context,
        seckey:     *mut u8,
        seckeylen:  *mut usize,
        key32:      *const u8,
        compressed: bool) -> i32 {
    
    todo!();
        /*
        
        */
}

#[fuzz_test] fn secp256k1_ec_seckey_import_export_der() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider{buffer.data(), buffer.size()};
        secp256k1_context* secp256k1_context_sign = secp256k1_context_create(SECP256K1_CONTEXT_SIGN);
        {
            std::vector<uint8_t> out32(32);
            (c_void)ec_seckey_import_der(secp256k1_context_sign, out32.data(), ConsumeFixedLengthByteVector(fuzzed_data_provider, CKey::SIZE).data(), CKey::SIZE);
        }
        {
            std::vector<uint8_t> seckey(CKey::SIZE);
            const std::vector<uint8_t> key32 = ConsumeFixedLengthByteVector(fuzzed_data_provider, 32);
            size_t seckeylen = CKey::SIZE;
            const bool compressed = fuzzed_data_provider.ConsumeBool();
            const bool exported = ec_seckey_export_der(secp256k1_context_sign, seckey.data(), &seckeylen, key32.data(), compressed);
            if (exported) {
                std::vector<uint8_t> out32(32);
                const bool imported = ec_seckey_import_der(secp256k1_context_sign, out32.data(), seckey.data(), seckey.size()) == 1;
                assert(imported && key32 == out32);
            }
        }
        secp256k1_context_destroy(secp256k1_context_sign);

    */
}
