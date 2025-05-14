// ---------------- [ File: bitcoin-bech32m/tests/bech32m.rs ]
crate::ix!();

#[test] fn bech_32m_testvectors_valid() {
    todo!();
    /*
    
        static const std::string CASES[] = {
            "A1LQFN3A",
            "a1lqfn3a",
            "an83characterlonghumanreadablepartthatcontainsthetheexcludedcharactersbioandnumber11sg7hg6",
            "abcdef1l7aum6echk45nj3s0wdvt2fg8x9yrzpqzd3ryx",
            "11llllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllludsr8",
            "split1checkupstagehandshakeupstreamerranterredcaperredlc445v",
            "?1v759aa"
        };
        for (const std::string& str : CASES) {
            const auto dec = bech32::Decode(str);
            BOOST_CHECK(dec.encoding == bech32::Encoding::BECH32M);
            std::string recode = bech32::Encode(bech32::Encoding::BECH32M, dec.hrp, dec.data);
            BOOST_CHECK(!recode.empty());
            BOOST_CHECK(CaseInsensitiveEqual(str, recode));
        }

    */
}

#[test] fn bech_32m_testvectors_invalid() {
    todo!();
    /*
    
        static const std::string CASES[] = {
            " 1xj0phk",
            "\x7f""1g6xzxy",
            "\x80""1vctc34",
            "an84characterslonghumanreadablepartthatcontainsthetheexcludedcharactersbioandnumber11d6pts4",
            "qyrz8wqd2c9m",
            "1qyrz8wqd2c9m",
            "y1b0jsk6g",
            "lt1igcx5c0",
            "in1muywd",
            "mm1crxm3i",
            "au1s5cgom",
            "M1VUXWEZ",
            "16plkw9",
            "1p2gdwpf"
        };
        for (const std::string& str : CASES) {
            const auto dec = bech32::Decode(str);
            BOOST_CHECK(dec.encoding == bech32::Encoding::INVALID);
        }

    */
}
