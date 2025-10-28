// ---------------- [ File: bitcoin-argsman/src/get_setting.rs ]
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
        let key = setting_name(arg);

        // 1) Command-line options (last value wins, consistent with Core’s behavior).
        if let Some(values) = self.settings.command_line_options().get(&key) {
            if let Some(last) = values.last() {
                return last.clone();
            }
        }

        // 2) Forced settings (used by force_set_arg). Prefer last-like behavior if present.
        if let Some(v) = self.settings.forced_settings().get(&key) {
            return v.clone();
        }

        // 3) Nothing set => null
        SettingsValue(UniValue::default()) // default is a null UniValue
    }

    /**
      | Get list of setting values.
      |
      */
    pub fn get_settings_list(&self, arg: &str) -> Vec<SettingsValue> {
        let key = setting_name(arg);

        if let Some(values) = self.settings.command_line_options().get(&key) {
            return values.clone();
        }
        if let Some(v) = self.settings.forced_settings().get(&key) {
            return vec![v.clone()];
        }
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn get_settings_path_respects_nosettings() {
        let mut inner = ArgsManagerInner::default();
        // Simulate -nosettings by setting -settings=false
        inner.force_set_arg("-settings", "0");
        let mut out: Option<Box<Path>> = None;
        let ok = inner.get_settings_path(out.as_mut(), None);
        assert!(!ok, "should return false when -nosettings is in effect");
    }
}
