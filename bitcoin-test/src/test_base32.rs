crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/base32_tests.cpp]

#[cfg(test)]
pub mod base32_tests {
    #[test] fn base32_testvectors() {
        todo!();
        /*
        
            static const std::string vstrIn[]  = {"","f","fo","foo","foob","fooba","foobar"};
            static const std::string vstrOut[] = {"","my======","mzxq====","mzxw6===","mzxw6yq=","mzxw6ytb","mzxw6ytboi======"};
            static const std::string vstrOutNoPadding[] = {"","my","mzxq","mzxw6","mzxw6yq","mzxw6ytb","mzxw6ytboi"};
            for (unsigned int i=0; i<std::size(vstrIn); i++)
            {
                std::string strEnc = EncodeBase32(vstrIn[i]);
                BOOST_CHECK_EQUAL(strEnc, vstrOut[i]);
                strEnc = EncodeBase32(vstrIn[i], false);
                BOOST_CHECK_EQUAL(strEnc, vstrOutNoPadding[i]);
                std::string strDec = DecodeBase32(vstrOut[i]);
                BOOST_CHECK_EQUAL(strDec, vstrIn[i]);
            }

            // Decoding strings with embedded NUL characters should fail
            bool failure;
            (c_void)DecodeBase32("invalid\0"s, &failure); // correct size, invalid due to \0
            BOOST_CHECK(failure);
            (c_void)DecodeBase32("AWSX3VPP"s, &failure); // valid
            BOOST_CHECK(!failure);
            (c_void)DecodeBase32("AWSX3VPP\0invalid"s, &failure); // correct size, invalid due to \0
            BOOST_CHECK(failure);
            (c_void)DecodeBase32("AWSX3VPPinvalid"s, &failure); // invalid size
            BOOST_CHECK(failure);

        */
    }
}
