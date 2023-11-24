crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/util_threadnames_tests.cpp]

pub mod util_threadnames_tests {
    use super::*;

    pub const TEST_THREAD_NAME_BASE: &'static str = "test_thread.";

    /**
      | Run a bunch of threads to all call util::ThreadRename.
      | 
      | -----------
      | @return
      | 
      | the set of name each thread has after
      | attempted renaming.
      |
      */
    pub fn rename_en_masse(num_threads: i32) -> HashSet<String> {
        
        todo!();
            /*
                std::vector<std::thread> threads;
            std::set<std::string> names;
            std::mutex lock;

            auto RenameThisThread = [&](int i) {
                util::ThreadRename(TEST_THREAD_NAME_BASE + ToString(i));
                std::lock_guard<std::mutex> guard(lock);
                names.insert(util::ThreadGetInternalName());
            };

            for (int i = 0; i < num_threads; ++i) {
                threads.push_back(std::thread(RenameThisThread, i));
            }

            for (std::thread& thread : threads) thread.join();

            return names;
            */
    }

    /**
      | Rename a bunch of threads with the same
      | basename (expect_multiple=true),
      | ensuring suffixes are applied properly.
      |
      */
    #[test] fn util_threadnames_test_rename_threaded() {
        todo!();
        /*
        
        #if !defined(HAVE_THREAD_LOCAL)
            // This test doesn't apply to platforms where we don't have thread_local.
            return;
        #endif

            std::set<std::string> names = RenameEnMasse(100);

            BOOST_CHECK_EQUAL(names.size(), 100U);

            // Names "test_thread.[n]" should exist for n = [0, 99]
            for (int i = 0; i < 100; ++i) {
                BOOST_CHECK(names.find(TEST_THREAD_NAME_BASE + ToString(i)) != names.end());
            }


        */
    }
}

