// ---------------- [ File: bitcoin-base58/tests/base58.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/base58_tests.cpp]

pub fn read_json(jsondata: &String) -> UniValue {
    
    todo!();
        /*
        
        */
}

#[cfg(test)]
#[fixture(BasicTestingSetup)]
pub mod base58_tests {

    /**
      | Goal: test low-level base58 encoding
      | functionality
      |
      */
    #[test] fn base58_encode() {
        todo!();
        /*
        
            UniValue tests = read_json(std::string(json_tests::base58_encode_decode, json_tests::base58_encode_decode + sizeof(json_tests::base58_encode_decode)));
            for (unsigned int idx = 0; idx < tests.size(); idx++) {
                UniValue test = tests[idx];
                std::string strTest = test.write();
                if (test.size() < 2) // Allow for extra stuff (useful for comments)
                {
                    BOOST_ERROR("Bad test: " << strTest);
                    continue;
                }
                std::vector<unsigned char> sourcedata = ParseHex(test[0].get_str());
                std::string base58string = test[1].get_str();
                BOOST_CHECK_MESSAGE(
                            EncodeBase58(sourcedata) == base58string,
                            strTest);
            }

        */
    }

    /**
      | Goal: test low-level base58 decoding
      | functionality
      |
      */
    #[test] fn base58_decode() {
        todo!();
        /*
        
            UniValue tests = read_json(std::string(json_tests::base58_encode_decode, json_tests::base58_encode_decode + sizeof(json_tests::base58_encode_decode)));
            std::vector<unsigned char> result;

            for (unsigned int idx = 0; idx < tests.size(); idx++) {
                UniValue test = tests[idx];
                std::string strTest = test.write();
                if (test.size() < 2) // Allow for extra stuff (useful for comments)
                {
                    BOOST_ERROR("Bad test: " << strTest);
                    continue;
                }
                std::vector<unsigned char> expected = ParseHex(test[0].get_str());
                std::string base58string = test[1].get_str();
                BOOST_CHECK_MESSAGE(DecodeBase58(base58string, result, 256), strTest);
                BOOST_CHECK_MESSAGE(result.size() == expected.size() && std::equal(result.begin(), result.end(), expected.begin()), strTest);
            }

            BOOST_CHECK(!DecodeBase58("invalid"s, result, 100));
            BOOST_CHECK(!DecodeBase58("invalid\0"s, result, 100));
            BOOST_CHECK(!DecodeBase58("\0invalid"s, result, 100));

            BOOST_CHECK(DecodeBase58("good"s, result, 100));
            BOOST_CHECK(!DecodeBase58("bad0IOl"s, result, 100));
            BOOST_CHECK(!DecodeBase58("goodbad0IOl"s, result, 100));
            BOOST_CHECK(!DecodeBase58("good\0bad0IOl"s, result, 100));

            // check that DecodeBase58 skips whitespace, but still fails with unexpected non-whitespace at the end.
            BOOST_CHECK(!DecodeBase58(" \t\n\v\f\r skip \r\f\v\n\t a", result, 3));
            BOOST_CHECK( DecodeBase58(" \t\n\v\f\r skip \r\f\v\n\t ", result, 3));
            std::vector<unsigned char> expected = ParseHex("971a55");
            BOOST_CHECK_EQUAL_COLLECTIONS(result.begin(), result.end(), expected.begin(), expected.end());

            BOOST_CHECK(DecodeBase58Check("3vQB7B6MrGQZaxCuFg4oh"s, result, 100));
            BOOST_CHECK(!DecodeBase58Check("3vQB7B6MrGQZaxCuFg4oi"s, result, 100));
            BOOST_CHECK(!DecodeBase58Check("3vQB7B6MrGQZaxCuFg4oh0IOl"s, result, 100));
            BOOST_CHECK(!DecodeBase58Check("3vQB7B6MrGQZaxCuFg4oh\0" "0IOl"s, result, 100));

        */
    }

    #[test] fn base58_random_encode_decode() {
        todo!();
        /*
        
            for (int n = 0; n < 1000; ++n) {
                unsigned int len = 1 + InsecureRandBits(8);
                unsigned int zeroes = InsecureRandBool() ? InsecureRandRange(len + 1) : 0;
                auto data = Cat(std::vector<unsigned char>(zeroes, '\000'), g_insecure_rand_ctx.randbytes(len - zeroes));
                auto encoded = EncodeBase58Check(data);
                std::vector<unsigned char> decoded;
                auto ok_too_small = DecodeBase58Check(encoded, decoded, InsecureRandRange(len));
                BOOST_CHECK(!ok_too_small);
                auto ok = DecodeBase58Check(encoded, decoded, len + InsecureRandRange(257 - len));
                BOOST_CHECK(ok);
                BOOST_CHECK(data == decoded);
            }

        */
    }
}
