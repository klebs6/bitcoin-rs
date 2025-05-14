// ---------------- [ File: bitcoin-argsman/src/log.rs ]
crate::ix!();

impl ArgsManagerInner {

    /**
      | Helper function for LogArgs().
      |
      */
    pub fn log_args_prefix(&self, 
        prefix:  &str,
        section: &str,
        args:    &HashMap<String,Vec<SettingsValue>>)  {
        
        let section_str: String = match section.is_empty() {
            true   => "".to_string(),
            false  => format!{"[{}] ", section}
        };

        for arg in args.iter() {
            for value in arg.1.iter() {

                let flags: Option::<u32> = {
                    let x = format!{"-{}", arg.0};
                    self.get_arg_flags(&x)
                };

                if flags.is_some() {

                    let value_str: String = match (flags.unwrap() & ArgsManagerFlags::SENSITIVE.bits()) != 0 {
                        true   => "****".to_string(),
                        false  => value.0.write(None,None)
                    };

                    log_printf!(
                        "%s %s%s=%s\n", 
                        prefix, 
                        section_str, 
                        arg.0, 
                        &value_str
                    );
                }
            }
        }
    }
    
    /**
      | Log the config file options and the command
      | line arguments, useful for troubleshooting.
      |
      */
    pub fn log_args(&self)  {
        
        for section in self.settings.ro_config.iter() {
            self.log_args_prefix("Config file arg:", section.0, section.1);
        }

        for setting in self.settings.rw_settings.iter() {
            log_printf!(
                "Setting file arg: %s = %s\n", 
                setting.0, 
                setting.1.write()
            );
        }

        self.log_args_prefix(
            "Command-line arg:", 
            "", 
            &self.settings.command_line_options
        );
    }
}
