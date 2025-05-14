// ---------------- [ File: bitcoin-argsman/src/parse.rs ]
crate::ix!();

impl ArgsManagerInner {
    
    pub fn parse_parameters(
        &mut self, 
        argv:  &Vec<String>,
        error: &mut String) -> bool {

        let argc = argv.len();

        //LOCK(cs_args);
        
        self.settings.command_line_options.clear();

        for mut i in 1..argc {

            let mut key: String = argv[i as usize].to_string();

            #[cfg(MAC_OSX)]
            {
                // At the first time when a user
                // gets the "App downloaded from
                // the internet" warning, and
                // clicks the Open button, macOS
                // passes a unique process serial
                // number (PSN) as
                // -psn_... command-line argument,
                // which we filter out.
                if key.substr(0, 5) == "-psn_" {
                    continue;
                }
            }

            // bitcoin-tx using stdin
            if key == "-" {
                break;
            }

            let mut val = String::default();

            if let Some(is_index) = key.find('=') {

                val = key[is_index + 1..].to_string();

                key.remove(is_index);
            }

            #[cfg(WIN32)]
            {
                key = to_lower(key);

                if key[0] == '/' {
                    key[0] = '-';
                }
            }

            if key.chars().nth(0) != Some('-') {

                if !self.accept_any_command && self.command.is_empty() {

                    // The first non-dash arg is a registered command
                    let flags: Option::<u32> = self.get_arg_flags(&key);

                    if flags.is_none() || (flags.unwrap() & ArgsManagerFlags::COMMAND.bits()) == 0 {

                        *error = format!{
                            "Invalid command '{}'",
                            argv[i as usize]
                        };

                        return false;
                    }
                }

                self.command.push(key);

                while {
                    i += 1;
                    i
                } < argc{
                    //  The remaining args are command args
                    self.command.push(argv[i as usize].to_string());
                }

                break;
            }

            // Transform --foo to -foo
            if key.len() > 1 && key.chars().nth(1).unwrap() == '-' {
                key = key[1..].to_string();
            }

            //  Transform -foo to foo
            key = key[1..].to_string();

            let mut section = String::default();

            let arg = format!{"-{}",key};

            let value: SettingsValue = interpret_option(&mut section,&mut key,&val);

            let flags: Option::<u32> = self.get_arg_flags(&arg);

            // Unknown command line options and
            // command line options with dot
            // characters (which are returned from
            // InterpretOption with nonempty
            // section strings) are not valid.
            if flags.is_none() || section.is_empty() {
                *error = format!{"Invalid parameter {:?}",argv[i as usize]};
                return false;
            }

            if !check_valid(&key,&value,flags.unwrap(),error) {
                return false;
            }

            self.settings
                .command_line_options
                .get_mut(&key)
                .unwrap()
                .push(value);
        }

        // we do not allow -includeconf from
        // command line, only -noincludeconf
        let includes = self.settings.command_line_options.get( "includeconf");

        if includes.is_some() {

            let values: SettingsSpan = SettingsSpan::from(includes.unwrap());

            //  Range may be empty if -noincludeconf was passed
            if !values.empty() {

                *error = format!{
                    "-includeconf cannot be used from commandline; -includeconf={}", 
                    unsafe { (*values.begin()).0.write(None, None) }
                };

                // pick first value as example
                return false;
            }
        }

        true
    }
}
