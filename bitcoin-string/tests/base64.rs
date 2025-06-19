// ---------------- [ File: bitcoin-string/tests/base64.rs ]
use bitcoin_imports::*;
use bitcoin_string::*;

//-------------------------------------------[.cpp/bitcoin/src/test/base64_tests.cpp]

#[test] fn base64_testvectors() {
    todo!();
    /*
    
        static const std::string vstrIn[]  = {"","f","fo","foo","foob","fooba","foobar"};
        static const std::string vstrOut[] = {"","Zg==","Zm8=","Zm9v","Zm9vYg==","Zm9vYmE=","Zm9vYmFy"};
        for (unsigned int i=0; i<std::size(vstrIn); i++)
        {
            std::string strEnc = EncodeBase64(vstrIn[i]);
            BOOST_CHECK_EQUAL(strEnc, vstrOut[i]);
            std::string strDec = DecodeBase64(strEnc);
            BOOST_CHECK_EQUAL(strDec, vstrIn[i]);
        }

        // Decoding strings with embedded NUL characters should fail
        bool failure;
        (c_void)DecodeBase64("invalid\0"s, &failure);
        BOOST_CHECK(failure);
        (c_void)DecodeBase64("nQB/pZw="s, &failure);
        BOOST_CHECK(!failure);
        (c_void)DecodeBase64("nQB/pZw=\0invalid"s, &failure);
        BOOST_CHECK(failure);
        (c_void)DecodeBase64("nQB/pZw=invalid\0"s, &failure);
        BOOST_CHECK(failure);

    */
}
