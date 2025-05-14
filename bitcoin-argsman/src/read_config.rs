// ---------------- [ File: bitcoin-argsman/src/read_config.rs ]
crate::ix!();

/**
  | Most paths passed as configuration
  | arguments are treated as relative to
  | the datadir if they are not absolute.
  | 
  | -----------
  | @param path
  | 
  | The path to be conditionally prefixed
  | with datadir.
  | ----------
  | @param net_specific
  | 
  | Use network specific datadir variant
  | 
  | -----------
  | @return
  | 
  | The normalized path.
  |
  */
pub fn abs_path_for_config_val(
        path:         &Path,
        net_specific: Option<bool>) -> PathBuf {

    let net_specific: bool = net_specific.unwrap_or(true);
    
    if path.is_absolute() {
        return path.to_path_buf();
    }

    let base = match net_specific {
        true  => G_ARGS.lock().cs_args.lock().get_data_dir_net(),
        false => G_ARGS.lock().cs_args.lock().get_data_dir_base(),
    };

    let mut builder = PathBuf::new();
    builder.push(canonicalize(base).unwrap());
    builder.push(canonicalize(path).unwrap());

    builder
}

pub fn get_config_file(conf_path: &str) -> PathBuf {
    
    let path = Path::new(conf_path);

    abs_path_for_config_val(&path,Some(false))
}

/**
  | Return true if -datadir option points
  | to a valid directory or is not specified.
  |
  */
pub fn check_data_dir_option() -> bool {
    
    let datadir: String = G_ARGS
        .lock()
        .cs_args
        .lock()
        .get_arg("-datadir", "");

    let path = Path::new(&datadir);

    datadir.is_empty() || std::fs::canonicalize(path).unwrap().is_dir()
}

/**
  | Check settings value validity according
  | to flags.
  | 
  | TODO: Add more meaningful error checks
  | here in the future
  | 
  | See "here's how the flags are meant to
  | behave" in https://github.com/bitcoin/bitcoin/pull/16097#issuecomment-514627823
  |
  */
pub fn check_valid(
    key:   &str,
    val:   &SettingsValue,
    flags: u32,
    error: &mut String) -> bool {

    if val.0.is_bool() 
    && (flags & ArgsManagerFlags::ALLOW_BOOL.bits()) == 0 
    {
        *error = format!{"Negating of -{} is meaningless and therefore forbidden",key};
        return false;
    }

    true
}

pub fn get_config_options<R: std::io::Read>(
    stream:   &mut std::io::BufReader<R>,
    filepath: &str,
    error:    &mut String,
    options:  &mut Vec<(String,String)>,
    sections: &mut LinkedList<SectionInfo>) -> bool {

    let mut str_:   String = String::default();
    let mut prefix: String = String::default();
    let mut pos:    Option<usize> = None;
    let mut linenr: i32 = 1;

    while stream.read_line(&mut str_).is_ok() {

        let mut used_hash: bool = false;

        pos = str_.find('#');

        if pos != None {
            str_ = str_[0..pos.unwrap()].to_string();
            used_hash = true;
        }

        lazy_static!{
            static ref pattern: String = " \t\r\n".to_string();
        }

        str_ = trim_string(&str_,Some(pattern.as_str()));

        if str_.len() != 0 {

            if str_.chars().nth(0).unwrap() == '[' 
            && str_.chars().nth(str_.len() - 1).unwrap() == ']' {

                let section: String = str_[1..str_.len() - 2].to_string();

                let info = SectionInfo::new(
                    &section,
                    filepath,
                    linenr
                );

                sections.push_back(info);

                prefix = format!{"{}.", section};

            } else {

                if str_.chars().nth(0).unwrap() == '-' {

                    *error = format!{
                        "parse error on line {}: {}, options in configuration file must be specified without leading -",
                        linenr,
                        str_
                    };

                    return false;

                } else {

                    pos = str_.find('=');

                    if pos != None {

                        let name:  String = format!{
                            "{}{}", 
                            prefix, 
                            &trim_string(
                                &str_[0..pos.unwrap()].to_string(),
                                Some(&*pattern)
                            )
                        };

                        let value: String = trim_string(

                            &str_
                                .chars()
                                .nth(pos.unwrap() + 1)
                                .unwrap()
                                .to_string(),

                            Some(&*pattern)
                        );

                        if used_hash && name.find("rpcpassword") != None {

                            *error = format!{
                                "parse error on line {}, using # in rpcpassword can be ambiguous and should be avoided",
                                linenr
                            };

                            return false;
                        }

                        options.push((name.clone(), value));

                        pos = name.rfind('.');

                        if pos != None && prefix.len() <= pos.unwrap() {

                            let info = SectionInfo::new(&name[0..pos.unwrap()],filepath,linenr);

                            sections.push_back(info);
                        }

                    } else {

                        *error = format!{
                            "parse error on line {}: {}",
                            linenr,
                            str_
                        };

                        if str_.len() >= 2 && &str_[0..2] == "no" {
                            *error = format!{
                                "{}, if you intended to specify a negated option, use {}=1 instead",
                                error,
                                str_
                            };
                        }

                        return false;
                    }
                }
            }
        }

        linenr += 1;
    }

    true
}

impl ArgsManagerInner {

    pub fn read_config_stream<R: std::io::Read>(&mut self, 
        stream:              &mut std::io::BufReader<R>,
        filepath:            &str,
        error:               &mut String,
        ignore_invalid_keys: Option<bool>) -> bool {

        let ignore_invalid_keys: bool = ignore_invalid_keys.unwrap_or(false);

        let mut options = Vec::<(String,String)>::default();

        if !get_config_options(
            stream,
            filepath,
            error,
            &mut options,
            &mut self.config_sections) 
        {
            return false;
        }

        for option in options.iter() {

            let mut section = String::default();

            let mut key: String = option.0.to_string();

            let value: SettingsValue = interpret_option(
                &mut section,
                &mut key,
                &option.1
            );

            let arg = format!{"-{}",key};

            let flags: Option::<u32> = self.get_arg_flags(&arg);

            if flags.is_some() {

                if !check_valid(&key,&value,flags.unwrap(),error) {
                    return false;
                }

                self.settings
                    .ro_config
                    .get_mut(&section)
                    .unwrap()
                    .get_mut(&key)
                    .unwrap()
                    .push(value);

            } else {

                if ignore_invalid_keys {

                    log_printf!(
                        "Ignoring unknown configuration value %s\n", 
                        option.0
                    );

                } else {

                    *error = format!{
                        "Invalid configuration value {}",
                        option.0
                    };

                    return false;
                }
            }
        }

        true
    }
    
    pub fn read_config_files(&mut self, 
        error:               &mut String,
        ignore_invalid_keys: Option<bool>) -> bool {

        let ignore_invalid_keys: bool = ignore_invalid_keys.unwrap_or(false);

        self.settings.ro_config.clear();
        self.config_sections.clear();

        let conf_path: String = self.get_arg("-conf", BITCOIN_CONF_FILENAME);

        let mut file: Result<File,_> = File::open(get_config_file(&conf_path));

        // not ok to have a config file specified
        // that cannot be opened
        if self.is_arg_set("-conf") && !file.is_ok() {

            *error = format!{
                "specified config file \"{}\" could not be opened.",
                conf_path
            };

            return false;
        }

        // ok to not have a config file
        if file.is_ok() {

            let mut stream = BufReader::new(file.unwrap());

            if !self.read_config_stream(
                &mut stream,
                &conf_path,
                error,
                Some(ignore_invalid_keys)) 
            {
                return false;
            }

            // `-includeconf` cannot be included
            // in the command line arguments
            // except as `-noincludeconf` (which
            // indicates that no included conf
            // file should be used).
            let mut use_conf_file: bool = true;;

            //LOCK(cs_args);

            let includes = self.settings.command_line_options.get("includeconf");

            if includes.is_some() {

                // ParseParameters() fails if
                // a non-negated -includeconf is
                // passed on the command-line
                assert!(
                    SettingsSpan::from(includes.unwrap()).last_negated()
                );

                use_conf_file = false;
            }

            if use_conf_file {

                let chain_id: String = self.get_chain_name().unwrap();

                let mut conf_file_names = Vec::<String>::default();

                let mut add_includes = |
                    network:         &str, 
                    skip:            Option::<usize>, 
                    conf_file_names: &mut Vec<String>, 
                    settings:        &Settings
                | {

                    let skip = skip.unwrap_or(0);

                    let mut num_values: usize = 0;

                    //LOCK(cs_args);

                    let section = settings.ro_config.get(network);

                    if section.is_some() {

                        let values = section.unwrap().get("includeconf");

                        if values.is_some() {

                            let span = SettingsSpan::from(values.unwrap());
                            let vlen = values.as_ref().unwrap().len();

                            for i in max(skip,span.negated())..vlen {
                                conf_file_names.push(values.unwrap()[i].to_string());
                            }

                            num_values = values.unwrap().len();
                        }
                    }

                    return num_values;
                };

                // We haven't set m_network yet
                // (that happens in
                // SelectParams()), so manually
                // check for network.includeconf
                // args.
                let chain_includes:   usize = add_includes(&chain_id, None, &mut conf_file_names, &self.settings);
                let default_includes: usize = add_includes("",        None, &mut conf_file_names, &self.settings);

                for conf_file_name in conf_file_names.iter() {

                    let mut conf_file_stream: Result<File,_> = File::open(get_config_file(conf_file_name));

                    if conf_file_stream.is_ok() {

                        let mut reader = BufReader::new(conf_file_stream.unwrap());

                        if !self.read_config_stream(
                            &mut reader,
                            conf_file_name,
                            error,
                            Some(ignore_invalid_keys)) 
                        {
                            return false;
                        }

                        log_printf!(
                            "Included configuration file {}\n", 
                            conf_file_name
                        );

                    } else {

                        *error = format!{
                            "Failed to include configuration file {}", 
                            conf_file_name
                        };

                        return false;
                    }
                }

                //  Warn about recursive -includeconf
                conf_file_names.clear();

                add_includes(&chain_id, /* skip= */ Some(chain_includes),   &mut conf_file_names, &self.settings);
                add_includes("",        /* skip= */ Some(default_includes), &mut conf_file_names, &self.settings);

                let chain_id_final: String = self.get_chain_name().unwrap();

                if chain_id_final != chain_id {

                    // Also warn about recursive
                    // includeconf for the chain
                    // that was specified in one
                    // of the includeconfs
                    add_includes(&chain_id_final, None, &mut conf_file_names, &self.settings);
                }

                for conf_file_name in conf_file_names.iter() {
                    eprintln!(
                        "warning: -includeconf cannot be used from included files; ignoring -includeconf={}\n", 
                        conf_file_name
                    );
                }
            }
        }

        // If datadir is changed in .conf file:
        G_ARGS
            .lock()
            .cs_args
            .lock()
            .clear_path_cache();

        if !check_data_dir_option() {

            *error = format!{
                "specified data directory \"{}\" does not exist.",
                self.get_arg("-datadir","")
            };

            return false;
        }

        true
    }
}
