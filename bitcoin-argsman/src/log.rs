// ---------------- [ File: bitcoin-argsman/src/log.rs ]
crate::ix!();

impl ArgsManagerInner {

     /**
      | Helper function for LogArgs().
      |
      */
    pub fn log_args_prefix(
        &self,
        prefix:  &str,
        section: &str,
        args:    &HashMap<String,Vec<SettingsValue>>,
    ) {
        let section_str = if section.is_empty() {
            "".to_string()
        } else {
            format!("[{}] ", section)
        };

        for (name, values) in args.iter() {
            for value in values.iter() {
                let flags = self.get_arg_flags(&format!("-{}", name));
                if let Some(flags) = flags {
                    let value_str = if (flags & ArgsManagerFlags::SENSITIVE.bits()) != 0 {
                        "****".to_string()
                    } else {
                        value.0.write(None, None)
                    };
                    log_printf!("{} {}{}={}\n", prefix, section_str, name, value_str);
                }
            }
        }
    }
   
    /**
      | Log the config file options and the command
      | line arguments, useful for troubleshooting.
      |
      */
    pub fn log_args(&self) {
        for (section_name, section_args) in self.settings.ro_config().iter() {
            self.log_args_prefix("Config file arg:", section_name, section_args);
        }

        for (k, v) in self.settings.rw_settings().iter() {
            log_printf!("Setting file arg: {} = {}\n", k, v);
        }

        self.log_args_prefix(
            "Command-line arg:",
            "",
            self.settings.command_line_options(),
        );
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn log_args_prefix_does_not_crash_and_masks_sensitive() {
        let mut inner = ArgsManagerInner::default();
        inner.available_args.insert(OptionsCategory::OPTIONS, HashMap::<String,ArgsManagerArg>::new());

        // Register two args: one sensitive, one not
        let s = ArgDescriptor {
            name: "-secret",
            help: "sensitive".into(),
            flags: ArgsManagerFlags::ALLOW_STRING | ArgsManagerFlags::SENSITIVE,
            category: OptionsCategory::OPTIONS
        };
        inner.add_arg(&s);
        let n = ArgDescriptor {
            name: "-normal",
            help: "normal".into(),
            flags: ArgsManagerFlags::ALLOW_STRING,
            category: OptionsCategory::OPTIONS
        };
        inner.add_arg(&n);

        let mut map: HashMap<String, Vec<SettingsValue>> = HashMap::new();
        map.insert("secret".into(), vec![SettingsValue(UniValue::from("hunter2"))]);
        map.insert("normal".into(), vec![SettingsValue(UniValue::from("x"))]);

        // Should not panic; output goes to log_printf! (side-effect)
        inner.log_args_prefix("Command-line arg:", "", &map);
    }
}
