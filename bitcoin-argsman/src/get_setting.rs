crate::ix!();

impl ArgsManagerInner {

    /**
      | Get settings file path, or return false
      | if read-write settings were disabled
      | with -nosettings.
      |
      */
    pub fn get_settings_path(&self, 
        filepath: Option<&mut Box<Path>>,
        temp:     Option<bool>) -> bool {

        let temp: bool = temp.unwrap_or(false);

        if self.is_arg_negated("-settings") {
            return false;
        }
        
        if filepath.is_some() {

            let settings: String 
            = self.get_arg("-settings",BITCOIN_SETTINGS_FILENAME);

            let p2 = match temp {
                true   => settings + ".tmp",
                false  => settings
            }.to_string();

            if let Some(filepath) = filepath {

                let mut buf = PathBuf::new();

                buf.push(std::fs::canonicalize(self.get_data_dir_net()).unwrap());
                buf.push(std::fs::canonicalize(p2).unwrap());

                *filepath = buf.into_boxed_path();
            }
        }

        true
    }

    /**
      | Get setting value.
      | 
      | Result will be null if setting was unset,
      | true if "-setting" argument was passed
      | false if "-nosetting" argument was
      | passed, and a string if a "-setting=value"
      | argument was passed.
      |
      */
    pub fn get_setting(&self, arg: &str) -> SettingsValue {

        let get_chain_name = false;

        get_setting(
            &self.settings,
            self.network.as_ref().unwrap(),
            &setting_name(arg),
            !self.use_default_section(arg),
            get_chain_name
        )
    }

    /**
      | Get list of setting values.
      |
      */
    pub fn get_settings_list(&self, arg: &str) -> Vec<SettingsValue> {
        
        get_settings_list(
            &self.settings,
            self.network.as_ref().unwrap(),
            &setting_name(arg),
            !self.use_default_section(arg)
        )
    }
}
