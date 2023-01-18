crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/compress_tests.cpp]

/**
  | amounts 0.00000001 .. 0.00100000
  |
  */
pub const NUM_MULTIPLES_UNIT: usize = 100000;

/**
  | amounts 0.01 .. 100.00
  |
  */
pub const NUM_MULTIPLES_CENT: usize = 10000;

/**
  | amounts 1 .. 10000
  |
  */
pub const NUM_MULTIPLES_1BTC: usize = 10000;

/**
  | amounts 50 .. 21000000
  |
  */
pub const NUM_MULTIPLES_50BTC: usize = 420000;

#[cfg(test)]
#[fixture(BasicTestingSetup)]
pub mod compress_tests {

    pub fn test_encode(in_: u64) -> bool {
        
        todo!();
            /*
                return in == DecompressAmount(CompressAmount(in));
            */
    }

    pub fn test_decode(in_: u64) -> bool {
        
        todo!();
            /*
                return in == CompressAmount(DecompressAmount(in));
            */
    }

    pub fn test_pair(
            dec: u64,
            enc: u64) -> bool {
        
        todo!();
            /*
                return CompressAmount(dec) == enc &&
                   DecompressAmount(enc) == dec;
            */
    }

    #[test] fn compress_amounts() {
        todo!();
        /*
        
            BOOST_CHECK(TestPair(            0,       0x0));
            BOOST_CHECK(TestPair(            1,       0x1));
            BOOST_CHECK(TestPair(         CENT,       0x7));
            BOOST_CHECK(TestPair(         COIN,       0x9));
            BOOST_CHECK(TestPair(      50*COIN,      0x32));
            BOOST_CHECK(TestPair(21000000*COIN, 0x1406f40));

            for (uint64_t i = 1; i <= NUM_MULTIPLES_UNIT; i++)
                BOOST_CHECK(TestEncode(i));

            for (uint64_t i = 1; i <= NUM_MULTIPLES_CENT; i++)
                BOOST_CHECK(TestEncode(i * CENT));

            for (uint64_t i = 1; i <= NUM_MULTIPLES_1BTC; i++)
                BOOST_CHECK(TestEncode(i * COIN));

            for (uint64_t i = 1; i <= NUM_MULTIPLES_50BTC; i++)
                BOOST_CHECK(TestEncode(i * 50 * COIN));

            for (uint64_t i = 0; i < 100000; i++)
                BOOST_CHECK(TestDecode(i));

        */
    }

    #[test] fn compress_script_to_ckey_id() {
        todo!();
        /*
        
            // case CKeyID
            CKey key;
            key.MakeNewKey(true);
            CPubKey pubkey = key.GetPubKey();

            CScript script = CScript() << OP_DUP << OP_HASH160 << ToByteVector(pubkey.GetID()) << OP_EQUALVERIFY << OP_CHECKSIG;
            BOOST_CHECK_EQUAL(script.size(), 25U);

            CompressedScript out;
            bool done = CompressScript(script, out);
            BOOST_CHECK_EQUAL(done, true);

            // Check compressed script
            BOOST_CHECK_EQUAL(out.size(), 21U);
            BOOST_CHECK_EQUAL(out[0], 0x00);
            BOOST_CHECK_EQUAL(memcmp(out.data() + 1, script.data() + 3, 20), 0); // compare the 20 relevant chars of the CKeyId in the script

        */
    }

    #[test] fn compress_script_to_cscript_id() {
        todo!();
        /*
        
            // case CScriptID
            CScript script, redeemScript;
            script << OP_HASH160 << ToByteVector(CScriptID(redeemScript)) << OP_EQUAL;
            BOOST_CHECK_EQUAL(script.size(), 23U);

            CompressedScript out;
            bool done = CompressScript(script, out);
            BOOST_CHECK_EQUAL(done, true);

            // Check compressed script
            BOOST_CHECK_EQUAL(out.size(), 21U);
            BOOST_CHECK_EQUAL(out[0], 0x01);
            BOOST_CHECK_EQUAL(memcmp(out.data() + 1, script.data() + 2, 20), 0); // compare the 20 relevant chars of the CScriptId in the script

        */
    }

    #[test] fn compress_script_to_compressed_pubkey_id() {
        todo!();
        /*
        
            CKey key;
            key.MakeNewKey(true); // case compressed PubKeyID

            CScript script = CScript() << ToByteVector(key.GetPubKey()) << OP_CHECKSIG; // COMPRESSED_PUBLIC_KEY_SIZE (33)
            BOOST_CHECK_EQUAL(script.size(), 35U);

            CompressedScript out;
            bool done = CompressScript(script, out);
            BOOST_CHECK_EQUAL(done, true);

            // Check compressed script
            BOOST_CHECK_EQUAL(out.size(), 33U);
            BOOST_CHECK_EQUAL(memcmp(out.data(), script.data() + 1, 1), 0);
            BOOST_CHECK_EQUAL(memcmp(out.data() + 1, script.data() + 2, 32), 0); // compare the 32 chars of the compressed CPubKey

        */
    }

    #[test] fn compress_script_to_uncompressed_pubkey_id() {
        todo!();
        /*
        
            CKey key;
            key.MakeNewKey(false); // case uncompressed PubKeyID
            CScript script =  CScript() << ToByteVector(key.GetPubKey()) << OP_CHECKSIG; // PUBLIC_KEY_SIZE (65)
            BOOST_CHECK_EQUAL(script.size(), 67U);                   // 1 char code + 65 char pubkey + OP_CHECKSIG

            CompressedScript out;
            bool done = CompressScript(script, out);
            BOOST_CHECK_EQUAL(done, true);

            // Check compressed script
            BOOST_CHECK_EQUAL(out.size(), 33U);
            BOOST_CHECK_EQUAL(memcmp(out.data() + 1, script.data() + 2, 32), 0); // first 32 chars of CPubKey are copied into out[1:]
            BOOST_CHECK_EQUAL(out[0], 0x04 | (script[65] & 0x01)); // least significant bit (lsb) of last char of pubkey is mapped into out[0]

        */
    }
}
