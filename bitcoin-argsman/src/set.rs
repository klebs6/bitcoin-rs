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
    pub fn force_set_arg(&mut self, 
        str_arg:   &str,
        str_value: &str)  {
        
        self.settings.forced_settings.insert(
            setting_name(str_arg), 
            SettingsValue(UniValue::from(str_value))
        );
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
