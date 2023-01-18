crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/main.cpp]

/**
  | See https://www.boost.org/doc/libs/1_71_0/libs/test/doc/html/boost_test/utf_reference/link_references/link_boost_test_module_macro.html
  |
  */
lazy_static!{
    /*
    #define BOOST_TEST_MODULE Bitcoin Core Test Suite
    */
}

/**
  | Redirect debug log to unit_test.log
  | files
  |
  */
lazy_static!{
    /*
    const std::function<c_void(const std::string&)> G_TEST_LOG_FUN = [](const std::string& s) {
        static const bool should_log{std::any_of(
            &boost::unit_test::framework::master_test_suite().argv[1],
            &boost::unit_test::framework::master_test_suite().argv[boost::unit_test::framework::master_test_suite().argc],
            [](const char* arg) {
                return std::string{"DEBUG_LOG_OUT"} == arg;
            })};
        if (!should_log) return;
        std::cout << s;
    };
    */
}

