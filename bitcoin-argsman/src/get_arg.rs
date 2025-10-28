// ---------------- [ File: bitcoin-argsman/src/get_arg.rs ]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn inner_with_options_map() -> ArgsManagerInner {
        let mut inner = ArgsManagerInner::default();
        inner.available_args.insert(OptionsCategory::OPTIONS, HashMap::new());
        inner
    }

    #[test]
    fn get_arg_int_and_bool_from_forced_settings() {
        let mut inner = inner_with_options_map();

        // Force-set a string numeric value
        inner.force_set_arg("-blocks", "42");
        assert_eq!(inner.get_int_arg("-blocks", 0), 42);
        assert_eq!(inner.get_arg("-blocks", "x"), "42");

        // Boolean handling: empty string => true (matches interpret_bool)
        inner.force_set_arg("-fast", "");
        assert!(inner.get_bool_arg("-fast", false));
        inner.force_set_arg("-fast", "0");
        assert!(!inner.get_bool_arg("-fast", true));
        inner.force_set_arg("-fast", "1");
        assert!(inner.get_bool_arg("-fast", false));
    }

    #[test]
    fn get_arg_flags_sees_inserted_args() {
        let mut inner = inner_with_options_map();
        // Register an option
        let d = ArgDescriptor {
            name:     "-try=<n>",
            help:     "count".into(),
            flags:    ArgsManagerFlags::ALLOW_INT,
            category: OptionsCategory::OPTIONS
        };
        inner.add_arg(&d);
        assert_eq!(inner.get_arg_flags("-try"), Some(ArgsManagerFlags::ALLOW_INT.bits()));
    }
}
