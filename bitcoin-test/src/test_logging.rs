// ---------------- [ File: bitcoin-test/src/test_logging.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/logging_tests.cpp]

#[cfg(test)]
#[BasicTestingSetup]
pub mod logging_tests {

    #[test] fn logging_timer() {
        todo!();
        /*
        
            SetMockTime(1);
            auto micro_timer = LogFlags::Timer<std::chrono::microseconds>("tests", "end_msg");
            SetMockTime(2);
            BOOST_CHECK_EQUAL(micro_timer.LogMsg("test micros"), "tests: test micros (1000000μs)");

            SetMockTime(1);
            auto ms_timer = LogFlags::Timer<std::chrono::milliseconds>("tests", "end_msg");
            SetMockTime(2);
            BOOST_CHECK_EQUAL(ms_timer.LogMsg("test ms"), "tests: test ms (1000.00ms)");

            SetMockTime(1);
            auto sec_timer = LogFlags::Timer<std::chrono::seconds>("tests", "end_msg");
            SetMockTime(2);
            BOOST_CHECK_EQUAL(sec_timer.LogMsg("test secs"), "tests: test secs (1.00s)");

        */
    }
}
