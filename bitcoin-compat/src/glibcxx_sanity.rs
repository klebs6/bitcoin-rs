// ---------------- [ File: bitcoin-compat/src/glibcxx_sanity.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/compat/glibcxx_sanity.cpp]

/**
  | trigger: use ctype<char>::widen to trigger
  | ctype<char>::_M_widen_init().
  |
  | test: convert a char from narrow to wide and
  |   back. Verify that the result matches the
  |   original.
  */
pub fn sanity_test_widen(testchar: u8) -> bool {
    
    todo!();
        /*
        const std::ctype<char>& test(std::use_facet<std::ctype<char> >(std::locale()));
        return test.narrow(test.widen(testchar), 'b') == testchar;
        */
}

/**
  | trigger: use list::push_back and list::pop_back
  |   to trigger _M_hook and _M_unhook.
  |
  | test: Push a sequence of integers into
  | a list. Pop them off and verify that they match
  | the original sequence.
  */
pub fn sanity_test_list(size: u32) -> bool {
    
    todo!();
        /*
        std::list<unsigned int> test;
        for (unsigned int i = 0; i != size; ++i)
            test.push_back(i + 1);

        if (test.size() != size)
            return false;

        while (!test.empty()) {
            if (test.back() != test.size())
                return false;
            test.pop_back();
        }
        return true;
        */
}

/**
  | trigger: string::at(x) on an empty string to
  | trigger __throw_out_of_range_fmt.
  |
  | test: force std::string to throw an
  |   out_of_range exception. Verify that it's
  |   caught correctly.
  */
pub fn sanity_test_range_fmt() -> bool {
    
    todo!();
        /*
            std::string test;
        try {
            test.at(1);
        } catch (const std::out_of_range&) {
            return true;
        } catch (...) {
        }
        return false;
        */
}

pub fn glibcxx_sanity_test() -> bool {
    
    todo!();
        /*
            return sanity_test_widen('a') && sanity_test_list(100) && sanity_test_range_fmt();
        */
}
