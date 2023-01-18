crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/util/str.h]
//-------------------------------------------[.cpp/bitcoin/src/test/util/str.cpp]

pub fn case_insensitive_equal(
        s1: &String,
        s2: &String) -> bool {
    
    todo!();
        /*
            if (s1.size() != s2.size()) return false;
        for (size_t i = 0; i < s1.size(); ++i) {
            char c1 = s1[i];
            if (c1 >= 'A' && c1 <= 'Z') c1 -= ('A' - 'a');
            char c2 = s2[i];
            if (c2 >= 'A' && c2 <= 'Z') c2 -= ('A' - 'a');
            if (c1 != c2) return false;
        }
        return true;
        */
}

/**
  | Increment a string. Useful to enumerate
  | all fixed length strings with characters
  | in [min_char, max_char].
  |
  */
pub fn next_string<CharType, const StringLength: usize>(
        string:   &mut [CharType; StringLength],
        min_char: CharType,
        max_char: CharType) -> bool {

    todo!();
        /*
            for (CharType& elem : string) {
            bool has_next = elem != max_char;
            elem = elem < min_char || elem >= max_char ? min_char : CharType(elem + 1);
            if (has_next) return true;
        }
        return false;
        */
}

/**
  | Iterate over string values and call
  | function for each string without successive
  | duplicate characters.
  |
  */
pub fn for_each_no_dup<CharType, Fn, const StringLength: usize>(
        string:   &mut [CharType; StringLength],
        min_char: CharType,
        max_char: CharType,
        fn_:      Fn)  {

    todo!();
        /*
            for (bool has_next = true; has_next; has_next = NextString(string, min_char, max_char)) {
            int prev = -1;
            bool skip_string = false;
            for (CharType c : string) {
                if (c == prev) skip_string = true;
                if (skip_string || c < min_char || c > max_char) break;
                prev = c;
            }
            if (!skip_string) fn();
        }
        */
}
