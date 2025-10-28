// ---------------- [ File: bitcoin-argsman/src/interpret.rs ]
crate::ix!();

/**
  | Interpret a string argument as a boolean.
  | 
  | The definition of LocaleIndependentAtoi<int>()
  | requires that non-numeric string values
  | like "foo", return 0. This means that
  | if a user unintentionally supplies
  | a non-integer argument here, the return
  | value is always false. This means that
  | 
  | -foo=false does what the user probably
  | expects, but -foo=true is well defined
  | but does not do what they probably expected.
  | 
  | The return value of LocaleIndependentAtoi<int>(...)
  | is zero when given input not representable
  | as an int.
  | 
  | For a more extensive discussion of this
  | topic (and a wide range of opinions on
  | the Right Way to change this code), see
  | PR12713.
  |
  */
pub fn interpret_bool(str_value: &str) -> bool {

    if str_value.is_empty() {
        return true;
    }

    locale_independent_atoi::<i32>(str_value) != 0
}

/**
  | Interpret -nofoo as if the user supplied
  | -foo=0.
  | 
  | This method also tracks when the -no
  | form was supplied, and if so, checks
  | whether there was a double-negative
  | (-nofoo=0 -> -foo=1).
  | 
  | If there was not a double negative, it
  | removes the "no" from the key and returns
  | false.
  | 
  | If there was a double negative, it removes
  | "no" from the key, and returns true.
  | 
  | If there was no "no", it returns the string
  | value untouched.
  | 
  | Where an option was negated can be later
  | checked using the
  | 
  | IsArgNegated() method. One use case
  | for this is to have a way to disable options
  | that are not normally boolean (e.g.
  | using -nodebuglogfile to request that
  | debug log output is not sent to any file
  | at all).
  |
  */
pub fn interpret_option(
        section: &mut String,
        key:     &mut String,
        value:   &String) -> SettingsValue {
    
    // Split section name from key name for keys
    // like "testnet.foo" or "regtest.bar"
    if let Some(option_index) = key.find('.') {
        *section = key[0..option_index].to_string();
        key.replace_range((0..option_index + 1), "");
    }

    if &key[0..2] == "no" {

        key.replace_range((0..2), "");

        // Double negatives like -nofoo=0 are
        // supported (but discouraged)
        if !interpret_bool(value) {
            log_printf!(
                "Warning: parsed potentially confusing double-negative -{}={}\n", 
                key, 
                value
            );
            return SettingsValue(UniValue::from(true));
        }

        return SettingsValue(UniValue::from(false));
    }

    SettingsValue(UniValue::from(value.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpret_bool_rules() {
        assert!(interpret_bool(""));     // empty => true
        assert!(interpret_bool("1"));
        assert!(!interpret_bool("0"));
        assert!(!interpret_bool("foo")); // atoi("foo") => 0
    }

    #[test]
    fn interpret_option_handles_no_and_double_negative() {
        let mut section = "".to_string();
        let mut key = "nofoo".to_string();
        let v = interpret_option(&mut section, &mut key, &"1".to_string());
        assert_eq!(key, "foo");
        assert!(v.0.is_false());

        let mut section = "".to_string();
        let mut key = "nobar".to_string();
        let v = interpret_option(&mut section, &mut key, &"0".to_string()); // double negative
        assert_eq!(key, "bar");
        assert!(v.0.is_true());
    }

    #[test]
    fn interpret_option_splits_section_dot_key() {
        let mut section = "".to_string();
        let mut key = "testnet.rpcport".to_string();
        let v = interpret_option(&mut section, &mut key, &"18332".to_string());
        assert_eq!(section, "testnet");
        assert_eq!(key, "rpcport");
        assert_eq!(v.0.get_str(), "18332");
    }
}
