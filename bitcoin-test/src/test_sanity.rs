crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/sanity_tests.cpp]

#[cfg(test)]
#[BOOST_FIXTURE_TEST_SUITE(sanity_tests, BasicTestingSetup)]
pub mod sanity_tests {

    #[test] fn basic_sanity() {
        todo!();
        /*
        
          BOOST_CHECK_MESSAGE(glibcxx_sanity_test() == true, "stdlib sanity test");
          BOOST_CHECK_MESSAGE(ECC_InitSanityCheck() == true, "secp256k1 sanity test");
          BOOST_CHECK_MESSAGE(ChronoSanityCheck() == true, "chrono epoch test");

        */
    }
}
