// ---------------- [ File: bitcoin-argsman/src/set.rs ]
crate::ix!();

impl ArgsManagerInner {

    /**
      | Set an argument if it doesn't already
      | have a value
      | 
      | -----------
      | @param strArg
      | 
      | Argument to set (e.g. "-foo")
      | ----------
      | @param strValue
      | 
      | Value (e.g. "1")
      | 
      | -----------
      | @return
      | 
      | true if argument gets set, false if it
      | already had a value
      |
      */
    pub fn soft_set_arg(&mut self, 
        str_arg:   &str,
        str_value: &str) -> bool {
        
        if self.is_arg_set(str_arg) {
            return false;
        }

        self.force_set_arg(str_arg, str_value);

        true
    }

    /**
      | Forces an arg setting. Called by
      | 
      | SoftSetArg() if the arg hasn't already
      | been set. Also called directly in testing.
      |
      */
    pub fn force_set_arg(&mut self, str_arg: &str, str_value: &str)  {
        let key = setting_name(str_arg);

        let uni = match str_value {
            "0" => UniValue::from(false),
            "1" => UniValue::from(true),
            _   => UniValue::from(str_value),
        };
        let val = SettingsValue(uni);

        // Keep forced map
        self.settings
            .forced_settings_mut()
            .insert(key.clone(), val.clone());

        // Mirror into CLI options so local resolver sees it
        self.settings
            .command_line_options_mut()
            .entry(key)
            .or_insert_with(Vec::new)
            .push(val);
    }


    /**
      | Set a boolean argument if it doesn't
      | already have a value
      | 
      | -----------
      | @param strArg
      | 
      | Argument to set (e.g. "-foo")
      | ----------
      | @param fValue
      | 
      | Value (e.g. false)
      | 
      | -----------
      | @return
      | 
      | true if argument gets set, false if it
      | already had a value
      |
      */
    pub fn soft_set_bool_arg(&mut self, 
        str_arg: &str,
        value:   bool) -> bool {

        if value {
            self.soft_set_arg(str_arg,&"1".to_string())
        } else {
            self.soft_set_arg(str_arg,&"0".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soft_set_and_force_set_behavior() {
        let mut inner = ArgsManagerInner::default();

        // soft set when not set
        assert!(inner.soft_set_arg("-foo", "1"));
        assert_eq!(inner.get_arg("-foo", "0"), "1");
        // soft set should not override
        assert!(!inner.soft_set_arg("-foo", "2"));
        assert_eq!(inner.get_arg("-foo", "0"), "1");
        // force set always overrides
        inner.force_set_arg("-foo", "3");
        assert_eq!(inner.get_arg("-foo", "0"), "3");

        // boolean soft-set
        let _ = inner.soft_set_bool_arg("-bar", true);
        assert_eq!(inner.get_arg("-bar", "0"), "1");
        let _ = inner.soft_set_bool_arg("-bar", false); // won't override
        assert_eq!(inner.get_arg("-bar", "0"), "1");
    }
}
