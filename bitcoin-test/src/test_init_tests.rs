// ---------------- [ File: bitcoin-test/src/test_init_tests.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/wallet/test/init_tests.cpp]

#[cfg(test)]
#[fixture(InitWalletDirTestingSetup)]
pub mod init_tests {

    #[test] fn walletinit_verify_walletdir_default() {
        todo!();
        /*
        
            SetWalletDir(m_walletdir_path_cases["default"]);
            bool result = m_wallet_client->verify();
            BOOST_CHECK(result == true);
            fs::path walletdir = fs::PathFromString(gArgs.GetArg("-walletdir", ""));
            fs::path expected_path = fs::canonical(m_walletdir_path_cases["default"]);
            BOOST_CHECK_EQUAL(walletdir, expected_path);

        */
    }

    #[test] fn walletinit_verify_walletdir_custom() {
        todo!();
        /*
        
            SetWalletDir(m_walletdir_path_cases["custom"]);
            bool result = m_wallet_client->verify();
            BOOST_CHECK(result == true);
            fs::path walletdir = fs::PathFromString(gArgs.GetArg("-walletdir", ""));
            fs::path expected_path = fs::canonical(m_walletdir_path_cases["custom"]);
            BOOST_CHECK_EQUAL(walletdir, expected_path);

        */
    }

    #[test] fn walletinit_verify_walletdir_does_not_exist() {
        todo!();
        /*
        
            SetWalletDir(m_walletdir_path_cases["nonexistent"]);
            {
                ASSERT_DEBUG_LOG("does not exist");
                bool result = m_wallet_client->verify();
                BOOST_CHECK(result == false);
            }

        */
    }

    #[test] fn walletinit_verify_walletdir_is_not_directory() {
        todo!();
        /*
        
            SetWalletDir(m_walletdir_path_cases["file"]);
            {
                ASSERT_DEBUG_LOG("is not a directory");
                bool result = m_wallet_client->verify();
                BOOST_CHECK(result == false);
            }

        */
    }

    #[test] fn walletinit_verify_walletdir_is_not_relative() {
        todo!();
        /*
        
            SetWalletDir(m_walletdir_path_cases["relative"]);
            {
                ASSERT_DEBUG_LOG("is a relative path");
                bool result = m_wallet_client->verify();
                BOOST_CHECK(result == false);
            }

        */
    }

    #[test] fn walletinit_verify_walletdir_no_trailing() {
        todo!();
        /*
        
            SetWalletDir(m_walletdir_path_cases["trailing"]);
            bool result = m_wallet_client->verify();
            BOOST_CHECK(result == true);
            fs::path walletdir = fs::PathFromString(gArgs.GetArg("-walletdir", ""));
            fs::path expected_path = fs::canonical(m_walletdir_path_cases["default"]);
            BOOST_CHECK_EQUAL(walletdir, expected_path);

        */
    }

    #[test] fn walletinit_verify_walletdir_no_trailing2() {
        todo!();
        /*
        
            SetWalletDir(m_walletdir_path_cases["trailing2"]);
            bool result = m_wallet_client->verify();
            BOOST_CHECK(result == true);
            fs::path walletdir = fs::PathFromString(gArgs.GetArg("-walletdir", ""));
            fs::path expected_path = fs::canonical(m_walletdir_path_cases["default"]);
            BOOST_CHECK_EQUAL(walletdir, expected_path);

        */
    }
}
