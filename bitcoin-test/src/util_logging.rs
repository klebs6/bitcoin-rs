// ---------------- [ File: bitcoin-test/src/util_logging.rs ]
crate::ix!();



macro_rules! assert_debug_log {
    ($message:ident) => {
        /*
                DebugLogHelper PASTE2(debugloghelper, __COUNTER__)(message)
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/test/util/logging.h]
//-------------------------------------------[.cpp/bitcoin/src/test/util/logging.cpp]

pub struct DebugLogHelper {
    message:          String,
    found:            bool, // default = { false }
    print_connection: Box<dyn Iterator<Item = fn(_0: &String) -> ()>>,
    match_:           debug_log_helper::MatchFn,
}

pub mod debug_log_helper {

    /**
      | Custom match checking function.
      | 
      | Invoked with pointers to lines containing
      | matching strings, and with null if check_found()
      | is called without any successful match.
      | 
      | - Can return true to enable default DebugLogHelper
      | behavior of:
      | 
      | - (1) ending search after first successful
      | match, and
      | 
      | - (2) raising an error in check_found
      | if no match was found
      | 
      | - Can return false to do the opposite
      | in either case.
      |
      */
    pub type MatchFn = fn(line: *const String) -> bool;
}

impl Drop for DebugLogHelper {
    fn drop(&mut self) {
        todo!();
        /*
            check_found();
        */
    }
}

impl DebugLogHelper {
    
    pub fn new(
        message: String,
        match_:  Option<debug_log_helper::MatchFn>) -> Self {

        let match_ = match_.unwrap_or(|_: *const String| { true } );
    
        todo!();
        /*


            : m_message{std::move(message)}, m_match(std::move(match))

        m_print_connection = LogInstance().PushBackCallback(
            [this](const std::string& s) {
                if (m_found) return;
                m_found = s.find(m_message) != std::string::npos && m_match(&s);
            });
        noui_test_redirect();
        */
    }
    
    pub fn check_found(&mut self)  {
        
        todo!();
        /*
            noui_reconnect();
        LogInstance().DeleteCallback(m_print_connection);
        if (!m_found && m_match(nullptr)) {
            throw std::runtime_error(strprintf("'%s' not found in debug log\n", m_message));
        }
        */
    }
}
