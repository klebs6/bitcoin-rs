crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/wallet/test/db_tests.cpp]

#[cfg(test)]
#[fixture(BasicTestingSetup)]
pub mod db_tests {

    pub fn get_wallet_env(
            path:              &Path,
            database_filename: &mut String) -> Arc<BerkeleyEnvironment> {
        
        todo!();
            /*
                fs::path data_file = BDBDataFile(path);
            database_filename = fs::PathToString(data_file.filename());
            return GetBerkeleyEnv(data_file.parent_path());
            */
    }

    #[test] fn getwalletenv_file() {
        todo!();
        /*
        
            std::string test_name = "test_name.dat";
            const fs::path datadir = gArgs.GetDataDirNet();
            fs::path file_path = datadir / test_name;
            fs::ofstream f(file_path);
            f.close();

            std::string filename;
            std::shared_ptr<BerkeleyEnvironment> env = GetWalletEnv(file_path, filename);
            BOOST_CHECK_EQUAL(filename, test_name);
            BOOST_CHECK_EQUAL(env->Directory(), datadir);

        */
    }

    #[test] fn getwalletenv_directory() {
        todo!();
        /*
        
            std::string expected_name = "wallet.dat";
            const fs::path datadir = gArgs.GetDataDirNet();

            std::string filename;
            std::shared_ptr<BerkeleyEnvironment> env = GetWalletEnv(datadir, filename);
            BOOST_CHECK_EQUAL(filename, expected_name);
            BOOST_CHECK_EQUAL(env->Directory(), datadir);

        */
    }

    #[test] fn getwalletenv_g_dbenvs_multiple() {
        todo!();
        /*
        
            fs::path datadir = gArgs.GetDataDirNet() / "1";
            fs::path datadir_2 = gArgs.GetDataDirNet() / "2";
            std::string filename;

            std::shared_ptr<BerkeleyEnvironment> env_1 = GetWalletEnv(datadir, filename);
            std::shared_ptr<BerkeleyEnvironment> env_2 = GetWalletEnv(datadir, filename);
            std::shared_ptr<BerkeleyEnvironment> env_3 = GetWalletEnv(datadir_2, filename);

            BOOST_CHECK(env_1 == env_2);
            BOOST_CHECK(env_2 != env_3);

        */
    }

    #[test] fn getwalletenv_g_dbenvs_free_instance() {
        todo!();
        /*
        
            fs::path datadir = gArgs.GetDataDirNet() / "1";
            fs::path datadir_2 = gArgs.GetDataDirNet() / "2";
            std::string filename;

            std::shared_ptr <BerkeleyEnvironment> env_1_a = GetWalletEnv(datadir, filename);
            std::shared_ptr <BerkeleyEnvironment> env_2_a = GetWalletEnv(datadir_2, filename);
            env_1_a.reset();

            std::shared_ptr<BerkeleyEnvironment> env_1_b = GetWalletEnv(datadir, filename);
            std::shared_ptr<BerkeleyEnvironment> env_2_b = GetWalletEnv(datadir_2, filename);

            BOOST_CHECK(env_1_a != env_1_b);
            BOOST_CHECK(env_2_a == env_2_b);

        */
    }
}
