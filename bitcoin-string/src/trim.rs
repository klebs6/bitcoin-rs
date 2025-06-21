// ---------------- [ File: bitcoin-string/src/trim.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/string.h]
//-------------------------------------------[.cpp/bitcoin/src/util/string.cpp]

#[inline]
pub fn trim_string(str_: &String, pattern: Option<&str>) -> String {
    trace!(target: "string_utils", input = %str_, ?pattern, "Starting trim_string");
    //note: removed \f and \v from the c++ default
    //pattern
    let pattern: &str = pattern.unwrap_or(" \n\r\t");
    let trimmed = str_
        .trim_matches(|c| pattern.contains(c))
        .to_string();
    trace!(target: "string_utils", output = %trimmed, "Completed trim_string");
    trimmed
}

#[inline]
pub fn remove_prefix(str_: &String, prefix: &String) -> String {
    trace!(
        target: "string_utils",
        input = %str_,
        prefix = %prefix,
        "Starting remove_prefix"
    );
    let out = if str_.starts_with(prefix) {
        str_[prefix.len()..].to_string()
    } else {
        str_.clone()
    };
    trace!(target: "string_utils", output = %out, "Completed remove_prefix");
    out
}

pub fn join(list: &Vec<String>, separator: &str) -> String {
    trace!(
        target: "string_utils",
        list_len = list.len(),
        separator = %separator,
        "Starting join"
    );
    let out = list.join(separator);
    trace!(target: "string_utils", output = %out, "Completed join");
    out
}

/**
  | Create an unordered multi-line list
  | of items.
  |
  */
#[inline]
pub fn make_unordered_list(items: &Vec<String>) -> String {
    trace!(
        target: "string_utils",
        items_len = items.len(),
        "Starting make_unordered_list"
    );
    let out = items
        .iter()
        .map(|item| format!("- {}", item))
        .collect::<Vec<String>>()
        .join("\n");
    trace!(target: "string_utils", output = %out, "Completed make_unordered_list");
    out
}

/**
  | Check if a string does not contain any
  | embedded NUL (\0) characters
  |
  */
#[inline]
pub fn valid_as_cstring(str_: &str) -> bool {
    trace!(target: "string_utils", input = ?str_, "Starting valid_as_cstring");
    let valid = !str_.as_bytes().contains(&0);
    trace!(target: "string_utils", valid, "Completed valid_as_cstring");
    valid
}

/**
  | Locale-independent version of std::to_string
  |
  */
pub fn to_string<T: std::fmt::Display>(t: &T) -> String {
    trace!(target: "string_utils", "Starting to_string");
    let out = format!("{}", t);
    trace!(target: "string_utils", output = %out, "Completed to_string");
    out
}

/**
  | Check whether a container begins with
  | the given prefix.
  |
  */
#[inline]
pub fn has_prefix<T1, const PREFIX_LEN: usize>(obj: &T1, prefix: &[u8; PREFIX_LEN]) -> bool
where
    T1: AsRef<[u8]>,
{
    let buf = obj.as_ref();
    trace!(
        target: "string_utils",
        obj_len = buf.len(),
        prefix_len = PREFIX_LEN,
        "Starting has_prefix"
    );
    let result = buf.len() >= PREFIX_LEN && &buf[..PREFIX_LEN] == prefix;
    trace!(target: "string_utils", result, "Completed has_prefix");
    result
}

#[cfg(test)]
mod exhaustive_string_utils_tests {
    use super::*;

    #[traced_test]
    fn trim_string_default_pattern() {
        let s = String::from(" \n\r\thello\t\r\n ");
        let expected = "hello".to_string();
        assert_eq!(trim_string(&s, None), expected);
    }

    #[traced_test]
    fn trim_string_custom_pattern() {
        let s = String::from("***abc***");
        let expected = "abc".to_string();
        assert_eq!(trim_string(&s, Some("*")), expected);
    }

    #[traced_test]
    fn remove_prefix_matches() {
        let s = String::from("foobar");
        let prefix = String::from("foo");
        assert_eq!(remove_prefix(&s, &prefix), "bar".to_string());
    }

    #[traced_test]
    fn remove_prefix_no_match() {
        let s = String::from("foobar");
        let prefix = String::from("baz");
        assert_eq!(remove_prefix(&s, &prefix), s);
    }

    #[traced_test]
    fn join_empty_list() {
        let list: Vec<String> = vec![];
        assert_eq!(join(&list, ","), "".to_string());
    }

    #[traced_test]
    fn join_multiple_items() {
        let list = vec!["a".into(), "b".into(), "c".into()];
        assert_eq!(join(&list, "-"), "a-b-c".to_string());
    }

    #[traced_test]
    fn make_unordered_list_basic() {
        let items = vec!["apple".into(), "banana".into()];
        let expected = "- apple\n- banana".to_string();
        assert_eq!(make_unordered_list(&items), expected);
    }

    #[traced_test]
    fn valid_as_cstring_true() {
        let s = "hello";
        assert!(valid_as_cstring(s));
    }

    #[traced_test]
    fn valid_as_cstring_false() {
        let s = "hel\0lo";
        assert!(!valid_as_cstring(s));
    }

    #[traced_test]
    fn to_string_int() {
        let n = 42;
        assert_eq!(to_string(&n), "42".to_string());
    }

    #[traced_test]
    fn to_string_float() {
        let f = 3.14;
        assert_eq!(to_string(&f), "3.14".to_string());
    }

    #[traced_test]
    fn has_prefix_vec() {
        let data: Vec<u8> = vec![1, 2, 3, 4];
        let prefix = [1u8, 2u8];
        assert!(has_prefix(&data, &prefix));
    }

    #[traced_test]
    fn has_prefix_slice_false() {
        let data: &[u8] = &[0, 1, 2];
        let prefix = [1u8, 2u8];
        assert!(!has_prefix(&data, &prefix));
    }
}
