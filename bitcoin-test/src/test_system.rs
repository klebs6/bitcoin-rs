crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/system_tests.cpp]

#[cfg(ENABLE_EXTERNAL_SIGNER)]
lazy_static!{
    /*
    #if defined(WIN32) && !defined(__kernel_entry)
    // A workaround for boost 1.71 incompatibility with mingw-w64 compiler.
    // For details see https://github.com/bitcoin/bitcoin/pull/22348.
    #define __kernel_entry
    #endif
    #include <boost/process.hpp>
    */
}

#[cfg(test)]
#[fixture(BasicTestingSetup)]
pub mod system_tests {

    /**
      | At least one test is required (in case
      | ENABLE_EXTERNAL_SIGNER is not defined).
      |
      | Workaround for
      | https://github.com/bitcoin/bitcoin/issues/19128
      */
    #[test] fn dummy() {
        todo!();
        /*
        
            BOOST_CHECK(true);

        */
    }

    #[cfg(ENABLE_EXTERNAL_SIGNER)]
    pub fn check_message(ex: &RuntimeError) -> bool {
        
        todo!();
            /*
                // On Linux & Mac: "No such file or directory"
            // On Windows: "The system cannot find the file specified."
            const std::string what(ex.what());
            BOOST_CHECK(what.find("file") != std::string::npos);
            return true;
            */
    }

    #[cfg(ENABLE_EXTERNAL_SIGNER)]
    pub fn check_message_false(ex: &RuntimeError) -> bool {
        
        todo!();
            /*
                BOOST_CHECK_EQUAL(ex.what(), std::string("RunCommandParseJSON error: process(false) returned 1: \n"));
            return true;
            */
    }

    #[cfg(ENABLE_EXTERNAL_SIGNER)]
    pub fn check_message_std_err(ex: &RuntimeError) -> bool {
        
        todo!();
            /*
                const std::string what(ex.what());
            BOOST_CHECK(what.find("RunCommandParseJSON error:") != std::string::npos);
            return checkMessage(ex);
            */
    }

    #[cfg(ENABLE_EXTERNAL_SIGNER)]
    #[test] fn run_command() {
        todo!();
        /*
        
            {
                const UniValue result = RunCommandParseJSON("");
                BOOST_CHECK(result.isNull());
            }
            {
        #ifdef WIN32
                // Windows requires single quotes to prevent escaping double quotes from the JSON...
                const UniValue result = RunCommandParseJSON("echo '{\"success\": true}'");
        #else
                // ... but Linux and macOS echo a single quote if it's used
                const UniValue result = RunCommandParseJSON("echo \"{\"success\": true}\"");
        #endif
                BOOST_CHECK(result.isObject());
                const UniValue& success = find_value(result, "success");
                BOOST_CHECK(!success.isNull());
                BOOST_CHECK_EQUAL(success.getBool(), true);
            }
            {
                // An invalid command is handled by Boost
                BOOST_CHECK_EXCEPTION(RunCommandParseJSON("invalid_command"), boost::process::process_error, checkMessage); // Command failed
            }
            {
                // Return non-zero exit code, no output to stderr
                BOOST_CHECK_EXCEPTION(RunCommandParseJSON("false"), std::runtime_error, checkMessageFalse);
            }
            {
                // Return non-zero exit code, with error message for stderr
                BOOST_CHECK_EXCEPTION(RunCommandParseJSON("ls nosuchfile"), std::runtime_error, checkMessageStdErr);
            }
            {
                BOOST_REQUIRE_THROW(RunCommandParseJSON("echo \"{\""), std::runtime_error); // Unable to parse JSON
            }
            // Test std::in, except for Windows
        #ifndef WIN32
            {
                const UniValue result = RunCommandParseJSON("cat", "{\"success\": true}");
                BOOST_CHECK(result.isObject());
                const UniValue& success = find_value(result, "success");
                BOOST_CHECK(!success.isNull());
                BOOST_CHECK_EQUAL(success.getBool(), true);
            }
        #endif

        */
    }
}
