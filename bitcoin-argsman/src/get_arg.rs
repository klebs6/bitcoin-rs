crate::ix!();

impl ArgsManagerInner {

    pub fn get_arg_flags(&self, name: &str) -> Option<u32> {
        
        for arg_map in self.available_args.iter() {

            let search = arg_map.1.get(name);

            if search.is_some() {
                return Some(search.unwrap().flags);
            }
        }

        None
    }

    pub fn get_args(&self, str_arg: &str) -> Vec<String> {
        
        let mut result = Vec::<String>::default();

        for value in self.get_settings_list(str_arg).iter() {
            result.push(
                match value.0.is_false() {
                    true   => "0",
                    false  => match value.0.is_true() {
                        true   => "1",
                        false  => value.0.get_str()
                    }
                }.to_string()
            );
        }

        result

    }
    
    pub fn get_arg(&self, 
        str_arg:     &str,
        str_default: &str) -> String {
        
        let value: SettingsValue = self.get_setting(str_arg);

        match value.0.is_null() {
            true   => str_default,
            false  => match value.0.is_false() {
                true   => "0",
                false  => match value.0.is_true() {
                    true   => "1",
                    false  => value.0.get_str()
                }
            }
        }.to_string()
    }

    /**
      | Return integer argument or default
      | value
      | 
      | -----------
      | @param strArg
      | 
      | Argument to get (e.g. "-foo")
      | ----------
      | @param nDefault
      | 
      | (e.g. 1)
      | 
      | -----------
      | @return
      | 
      | command-line argument (0 if invalid
      | number) or default value
      |
      */
    pub fn get_int_arg(&self, 
        str_arg:   &str,
        n_default: i64) -> i64 {
        
        let value: SettingsValue = self.get_setting(str_arg);

        match value.0.is_null() {
            true   => n_default,
            false  => match value.0.is_false() {
                true   => 0,
                false  => match value.0.is_true() {
                    true   => 1,
                    false  => match value.0.is_num() {
                        true   => value.0.get_int64(),
                        false  => locale_independent_atoi::<i64>(value.0.get_str())
                    }
                }
            }
        }
    }
    
    /**
      | Return boolean argument or default
      | value
      | 
      | -----------
      | @param strArg
      | 
      | Argument to get (e.g. "-foo")
      | ----------
      | @param fDefault
      | 
      | (true or false)
      | 
      | -----------
      | @return
      | 
      | command-line argument or default value
      |
      */
    pub fn get_bool_arg(&self, 
        str_arg: &str,
        default: bool) -> bool {
        
        let value: SettingsValue = self.get_setting(str_arg);

        match value.0.is_null() {
            true   => default,
            false  => match value.0.is_bool() {
                true   => value.0.get_bool(),
                false  => interpret_bool(value.0.get_str())
            }
        }
    }
}
