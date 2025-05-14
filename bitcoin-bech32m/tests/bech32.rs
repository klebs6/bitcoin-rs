// ---------------- [ File: bitcoin-bech32m/tests/bech32.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/bech32_tests.cpp]

#[test] fn bech32_testvectors_valid() {
    todo!();
    /*
    
        static const std::string CASES[] = {
            "A12UEL5L",
            "a12uel5l",
            "an83characterlonghumanreadablepartthatcontainsthenumber1andtheexcludedcharactersbio1tt5tgs",
            "abcdef1qpzry9x8gf2tvdw0s3jn54khce6mua7lmqqqxw",
            "11qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqc8247j",
            "split1checkupstagehandshakeupstreamerranterredcaperred2y9e3w",
            "?1ezyfcl",
        };
        for (const std::string& str : CASES) {
            const auto dec = bech32::Decode(str);
            BOOST_CHECK(dec.encoding == bech32::Encoding::BECH32);
            std::string recode = bech32::Encode(bech32::Encoding::BECH32, dec.hrp, dec.data);
            BOOST_CHECK(!recode.empty());
            BOOST_CHECK(CaseInsensitiveEqual(str, recode));
        }

    */
}

#[test] fn bech32_testvectors_invalid() {
    todo!();
    /*
    
        static const std::string CASES[] = {
            " 1nwldj5",
            "\x7f""1axkwrx",
            "\x80""1eym55h",
            "an84characterslonghumanreadablepartthatcontainsthenumber1andtheexcludedcharactersbio1569pvx",
            "pzry9x0s0muk",
            "1pzry9x0s0muk",
            "x1b4n0q5v",
            "li1dgmt3",
            "de1lg7wt\xff",
            "A1G7SGD8",
            "10a06t8",
            "1qzzfhee",
            "a12UEL5L",
            "A12uEL5L",
        };
        for (const std::string& str : CASES) {
            const auto dec = bech32::Decode(str);
            BOOST_CHECK(dec.encoding == bech32::Encoding::INVALID);
        }

    */
}
