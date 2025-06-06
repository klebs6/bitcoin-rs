// ---------------- [ File: bitcoin-test/src/test_fs.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/fs_tests.cpp]

#[cfg(test)]
#[fixture(BasicTestingSetup)]
pub mod fs_tests {

    #[test] fn fsbridge_pathtostring() {
        todo!();
        /*
        
            std::string u8_str = "fs_tests_₿_🏃";
            BOOST_CHECK_EQUAL(fs::PathToString(fs::PathFromString(u8_str)), u8_str);
            BOOST_CHECK_EQUAL(fs::u8path(u8_str).u8string(), u8_str);
            BOOST_CHECK_EQUAL(fs::PathFromString(u8_str).u8string(), u8_str);
            BOOST_CHECK_EQUAL(fs::PathToString(fs::u8path(u8_str)), u8_str);
        #ifndef WIN32
            // On non-windows systems, verify that arbitrary byte strings containing
            // invalid UTF-8 can be round tripped successfully with PathToString and
            // PathFromString. On non-windows systems, paths are just byte strings so
            // these functions do not do any encoding. On windows, paths are Unicode,
            // and these functions do encoding and decoding, so the behavior of this
            // test would be undefined.
            std::string invalid_u8_str = "\xf0";
            BOOST_CHECK_EQUAL(invalid_u8_str.size(), 1);
            BOOST_CHECK_EQUAL(fs::PathToString(fs::PathFromString(invalid_u8_str)), invalid_u8_str);
        #endif

        */
    }

    #[test] fn fsbridge_stem() {
        todo!();
        /*
        
            std::string test_filename = "fs_tests_₿_🏃.dat";
            std::string expected_stem = "fs_tests_₿_🏃";
            BOOST_CHECK_EQUAL(fs::PathToString(fs::PathFromString(test_filename).stem()), expected_stem);

        */
    }

    #[test] fn fsbridge_fstream() {
        todo!();
        /*
        
            fs::path tmpfolder = m_args.GetDataDirBase();
            // tmpfile1 should be the same as tmpfile2
            fs::path tmpfile1 = tmpfolder / "fs_tests_₿_🏃";
            fs::path tmpfile2 = tmpfolder / "fs_tests_₿_🏃";
            {
                fsbridge::ofstream file(tmpfile1);
                file << "bitcoin";
            }
            {
                fsbridge::ifstream file(tmpfile2);
                std::string input_buffer;
                file >> input_buffer;
                BOOST_CHECK_EQUAL(input_buffer, "bitcoin");
            }
            {
                fsbridge::ifstream file(tmpfile1, std::ios_base::in | std::ios_base::ate);
                std::string input_buffer;
                file >> input_buffer;
                BOOST_CHECK_EQUAL(input_buffer, "");
            }
            {
                fsbridge::ofstream file(tmpfile2, std::ios_base::out | std::ios_base::app);
                file << "tests";
            }
            {
                fsbridge::ifstream file(tmpfile1);
                std::string input_buffer;
                file >> input_buffer;
                BOOST_CHECK_EQUAL(input_buffer, "bitcointests");
            }
            {
                fsbridge::ofstream file(tmpfile2, std::ios_base::out | std::ios_base::trunc);
                file << "bitcoin";
            }
            {
                fsbridge::ifstream file(tmpfile1);
                std::string input_buffer;
                file >> input_buffer;
                BOOST_CHECK_EQUAL(input_buffer, "bitcoin");
            }
            {
                // Join an absolute path and a relative path.
                fs::path p = fsbridge::AbsPathJoin(tmpfolder, "fs_tests_₿_🏃");
                BOOST_CHECK(p.is_absolute());
                BOOST_CHECK_EQUAL(tmpfile1, p);
            }
            {
                // Join two absolute paths.
                fs::path p = fsbridge::AbsPathJoin(tmpfile1, tmpfile2);
                BOOST_CHECK(p.is_absolute());
                BOOST_CHECK_EQUAL(tmpfile2, p);
            }
            {
                // Ensure joining with empty paths does not add trailing path components.
                BOOST_CHECK_EQUAL(tmpfile1, fsbridge::AbsPathJoin(tmpfile1, ""));
                BOOST_CHECK_EQUAL(tmpfile1, fsbridge::AbsPathJoin(tmpfile1, {}));
            }
            {
                fs::path p1 = GetUniquePath(tmpfolder);
                fs::path p2 = GetUniquePath(tmpfolder);
                fs::path p3 = GetUniquePath(tmpfolder);

                // Ensure that the parent path is always the same.
                BOOST_CHECK_EQUAL(tmpfolder, p1.parent_path());
                BOOST_CHECK_EQUAL(tmpfolder, p2.parent_path());
                BOOST_CHECK_EQUAL(tmpfolder, p3.parent_path());

                // Ensure that generated paths are actually different.
                BOOST_CHECK(p1 != p2);
                BOOST_CHECK(p2 != p3);
                BOOST_CHECK(p1 != p3);
            }

        */
    }
}
