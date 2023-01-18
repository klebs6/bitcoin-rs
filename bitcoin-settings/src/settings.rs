crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/settings.h]

/**
  | Settings value type
  | (string/integer/boolean/null variant).
  |
  | @note UniValue is used here for convenience
  |       and because it can be easily serialized
  |       in a readable format. But any other
  |       variant type that can be assigned
  |       strings, int64_t, and bool values and
  |       has get_str(), get_int64(), get_bool(),
  |       isNum(), isBool(), isFalse(), isTrue()
  |       and
  |       isNull() methods can be substituted if
  |       there's a need to move away from
  |       UniValue. (An implementation with
  |       boost::variant was posted at
  |       https://github.com/bitcoin/bitcoin/pull/15934/files#r337691812)
  */
pub struct SettingsValue(pub UniValue);

impl PartialEq<SettingsValue> for SettingsValue {
    
    fn eq(&self, other: &SettingsValue) -> bool {
        todo!();
        /*
            return a.write() == b.write();
        */
    }
}

impl Eq for SettingsValue {}

impl std::fmt::Display for SettingsValue {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
        /*
            os << value.write();
        return os;
        */
    }
}

pub struct SettingsTuple((String,SettingsValue));

impl std::fmt::Display for SettingsTuple {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
        /*
            SettingsValue out(SettingsValue::VOBJ);
        out.__pushKV(kv.first, kv.second);
        os << out.write();
        return os;
        */
    }
}

/**
  | Stored settings. This struct combines settings
  | from the command line, a read-only
  | configuration file, and a read-write runtime
  | settings file.
  */
#[derive(Default)]
pub struct Settings {

    /**
      | Map of setting name to forced setting
      | value.
      |
      */
    pub forced_settings:      HashMap<String,SettingsValue>,

    /**
      | Map of setting name to list of command
      | line values.
      |
      */
    pub command_line_options: HashMap<String,Vec<SettingsValue>>,

    /**
      | Map of setting name to read-write file
      | setting value.
      |
      */
    pub rw_settings:          HashMap<String,SettingsValue>,

    /**
      | Map of config section name and setting
      | name to list of config file values.
      |
      */
    pub ro_config:            HashMap<String,HashMap<String,Vec<SettingsValue>>>,
}

/**
  | Accessor for list of settings that skips
  | negated values when iterated over.
  |
  | The last boolean `false` value in the list and
  | all earlier values are considered negated.
  */
pub struct SettingsSpan {
    pub data: *const SettingsValue,
    pub size: usize,
}

impl Default for SettingsSpan {
    fn default() -> Self {
        Self {
            data: null(),
            size: 0,
        }
    }
}

impl From<&SettingsValue> for SettingsSpan {
    fn from(value: &SettingsValue) -> Self {
    
        todo!();
        /*
        : settings_span(&value, 1),

        
        */
    }
}

impl From<&Vec<SettingsValue>> for SettingsSpan {
    fn from(vec: &Vec<SettingsValue>) -> Self {
    
        todo!();
        /*
        : settings_span(vec.data(), vec.size()),

        
        */
    }
}

impl SettingsSpan {

    pub fn new(
        data: *const SettingsValue,
        size: usize) -> Self {
    
        todo!();
        /*
        : data(data),
        : size(size),

        
        */
    }
    
    /**
      | Pointer to first non-negated value.
      |
      */
    pub fn begin(&self) -> *const SettingsValue {
        
        todo!();
        /*
            return data + negated();
        */
    }
    
    /**
      | Pointer to end of values.
      |
      */
    pub fn end(&self) -> *const SettingsValue {
        
        todo!();
        /*
            return data + size;
        */
    }
    
    /**
      | True if there are any non-negated values.
      |
      */
    pub fn empty(&self) -> bool {
        
        todo!();
        /*
            return size == 0 || last_negated();
        */
    }
    
    /**
      | True if the last value is negated.
      |
      */
    pub fn last_negated(&self) -> bool {
        
        todo!();
        /*
            return size > 0 && data[size - 1].isFalse();
        */
    }
    
    /**
      | Number of negated values.
      |
      */
    pub fn negated(&self) -> usize {
        
        todo!();
        /*
            for (size_t i = size; i > 0; --i) {
            if (data[i - 1].isFalse()) return i; // Return number of negated values (position of last false value)
        }
        return 0;
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/util/settings.cpp]

pub enum Source {
   FORCED,
   COMMAND_LINE,
   RW_SETTINGS,
   CONFIG_FILE_NETWORK_SECTION,
   CONFIG_FILE_DEFAULT_SECTION
}

/**
  | Merge settings from multiple sources in
  | precedence order:
  |
  | Forced config > command line > read-write
  | settings file > config file network-specific
  | section > config file default section
  |
  | This function is provided with a callback
  | function fn that contains specific logic for
  | how to merge the sources.
  */

pub fn merge_settings<Fn>(
    settings: &Settings,
    section:  &String,
    name:     &String,
    fn_:      Fn)  {

    todo!();
        /*
            // Merge in the forced settings
        if (auto* value = FindKey(settings.forced_settings, name)) {
            fn(SettingsSpan(*value), Source::FORCED);
        }
        // Merge in the command-line options
        if (auto* values = FindKey(settings.command_line_options, name)) {
            fn(SettingsSpan(*values), Source::COMMAND_LINE);
        }
        // Merge in the read-write settings
        if (const SettingsValue* value = FindKey(settings.rw_settings, name)) {
            fn(SettingsSpan(*value), Source::RW_SETTINGS);
        }
        // Merge in the network-specific section of the config file
        if (!section.empty()) {
            if (auto* map = FindKey(settings.ro_config, section)) {
                if (auto* values = FindKey(*map, name)) {
                    fn(SettingsSpan(*values), Source::CONFIG_FILE_NETWORK_SECTION);
                }
            }
        }
        // Merge in the default section of the config file
        if (auto* map = FindKey(settings.ro_config, "")) {
            if (auto* values = FindKey(*map, name)) {
                fn(SettingsSpan(*values), Source::CONFIG_FILE_DEFAULT_SECTION);
            }
        }
        */
}

/**
  | Read settings file.
  |
  */
pub fn read_settings(
    path:   &Path,
    values: &mut HashMap<String,SettingsValue>,
    errors: &mut Vec<String>) -> bool {
    
    todo!();
        /*
            values.clear();
        errors.clear();

        // Ok for file to not exist
        if (!fs::exists(path)) return true;

        fsbridge::ifstream file;
        file.open(path);
        if (!file.is_open()) {
          errors.emplace_back(strprintf("%s. Please check permissions.", fs::PathToString(path)));
          return false;
        }

        SettingsValue in;
        if (!in.read(std::string{std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>()})) {
            errors.emplace_back(strprintf("Unable to parse settings file %s", fs::PathToString(path)));
            return false;
        }

        if (file.fail()) {
            errors.emplace_back(strprintf("Failed reading settings file %s", fs::PathToString(path)));
            return false;
        }
        file.close(); // Done with file descriptor. Release while copying data.

        if (!in.isObject()) {
            errors.emplace_back(strprintf("Found non-object value %s in settings file %s", in.write(), fs::PathToString(path)));
            return false;
        }

        const std::vector<std::string>& in_keys = in.getKeys();
        const std::vector<SettingsValue>& in_values = in.getValues();
        for (size_t i = 0; i < in_keys.size(); ++i) {
            auto inserted = values.emplace(in_keys[i], in_values[i]);
            if (!inserted.second) {
                errors.emplace_back(strprintf("Found duplicate key %s in settings file %s", in_keys[i], fs::PathToString(path)));
            }
        }
        return errors.empty();
        */
}

/**
  | Write settings file.
  |
  */
pub fn write_settings(
    path:   &Path,
    values: &HashMap<String,SettingsValue>,
    errors: &mut Vec<String>) -> bool {
    
    todo!();
        /*
            SettingsValue out(SettingsValue::VOBJ);
        for (const auto& value : values) {
            out.__pushKV(value.first, value.second);
        }
        fsbridge::ofstream file;
        file.open(path);
        if (file.fail()) {
            errors.emplace_back(strprintf("Error: Unable to open settings file %s for writing", fs::PathToString(path)));
            return false;
        }
        file << out.write(/* prettyIndent= */ 1, /* indentLevel= */ 4) << std::endl;
        file.close();
        return true;
        */
}

/**
  | Get settings value from combined sources:
  | forced settings, command line arguments,
  | runtime read-write settings, and the
  | read-only config file.
  | 
  | -----------
  | @param ignore_default_section_config
  | 
  | - ignore values in the default section
  | of the config file (part before any [section]
  | keywords)
  | ----------
  | @param get_chain_name
  | 
  | - enable special backwards compatible
  | behavior for GetChainName
  |
  */
pub fn get_setting(
    settings:                      &Settings,
    section:                       &str,
    name:                          &str,
    ignore_default_section_config: bool,
    get_chain_name:                bool) -> SettingsValue {
    
    todo!();
        /*
            SettingsValue result;
        bool done = false; // Done merging any more settings sources.
        MergeSettings(settings, section, name, [&](SettingsSpan span, Source source) {
            // Weird behavior preserved for backwards compatibility: Apply negated
            // setting even if non-negated setting would be ignored. A negated
            // value in the default section is applied to network specific options,
            // even though normal non-negated values there would be ignored.
            const bool never_ignore_negated_setting = span.last_negated();

            // Weird behavior preserved for backwards compatibility: Take first
            // assigned value instead of last. In general, later settings take
            // precedence over early settings, but for backwards compatibility in
            // the config file the precedence is reversed for all settings except
            // chain name settings.
            const bool reverse_precedence =
                (source == Source::CONFIG_FILE_NETWORK_SECTION || source == Source::CONFIG_FILE_DEFAULT_SECTION) &&
                !get_chain_name;

            // Weird behavior preserved for backwards compatibility: Negated
            // -regtest and -testnet arguments which you would expect to override
            // values set in the configuration file are currently accepted but
            // silently ignored. It would be better to apply these just like other
            // negated values, or at least warn they are ignored.
            const bool skip_negated_command_line = get_chain_name;

            if (done) return;

            // Ignore settings in default config section if requested.
            if (ignore_default_section_config && source == Source::CONFIG_FILE_DEFAULT_SECTION &&
                !never_ignore_negated_setting) {
                return;
            }

            // Skip negated command line settings.
            if (skip_negated_command_line && span.last_negated()) return;

            if (!span.empty()) {
                result = reverse_precedence ? span.begin()[0] : span.end()[-1];
                done = true;
            } else if (span.last_negated()) {
                result = false;
                done = true;
            }
        });
        return result;
        */
}

/**
  | Get combined setting value similar to
  | GetSetting(), except if setting was specified
  | multiple times, return a list of all the
  | values specified.
  */
pub fn get_settings_list(
    settings:                      &Settings,
    section:                       &String,
    name:                          &String,
    ignore_default_section_config: bool) -> Vec<SettingsValue> {
    
    todo!();
        /*
            std::vector<SettingsValue> result;
        bool done = false; // Done merging any more settings sources.
        bool prev_negated_empty = false;
        MergeSettings(settings, section, name, [&](SettingsSpan span, Source source) {
            // Weird behavior preserved for backwards compatibility: Apply config
            // file settings even if negated on command line. Negating a setting on
            // command line will ignore earlier settings on the command line and
            // ignore settings in the config file, unless the negated command line
            // value is followed by non-negated value, in which case config file
            // settings will be brought back from the dead (but earlier command
            // line settings will still be ignored).
            const bool add_zombie_config_values =
                (source == Source::CONFIG_FILE_NETWORK_SECTION || source == Source::CONFIG_FILE_DEFAULT_SECTION) &&
                !prev_negated_empty;

            // Ignore settings in default config section if requested.
            if (ignore_default_section_config && source == Source::CONFIG_FILE_DEFAULT_SECTION) return;

            // Add new settings to the result if isn't already complete, or if the
            // values are zombies.
            if (!done || add_zombie_config_values) {
                for (const auto& value : span) {
                    if (value.isArray()) {
                        result.insert(result.end(), value.getValues().begin(), value.getValues().end());
                    } else {
                        result.push_back(value);
                    }
                }
            }

            // If a setting was negated, or if a setting was forced, set
            // done to true to ignore any later lower priority settings.
            done |= span.negated() > 0 || source == Source::FORCED;

            // Update the negated and empty state used for the zombie values check.
            prev_negated_empty |= span.last_negated() && result.empty();
        });
        return result;
        */
}

/**
  | Return true if a setting is set in the default
  | config file section, and not overridden by
  | a higher priority command-line or network
  | section value.
  |
  | This is used to provide user warnings about
  | values that might be getting ignored
  | unintentionally.
  */
pub fn only_has_default_section_setting(
    settings: &Settings,
    section:  &str,
    name:     &str) -> bool {
    
    todo!();
        /*
            bool has_default_section_setting = false;
        bool has_other_setting = false;
        MergeSettings(settings, section, name, [&](SettingsSpan span, Source source) {
            if (span.empty()) return;
            else if (source == Source::CONFIG_FILE_DEFAULT_SECTION) has_default_section_setting = true;
            else has_other_setting = true;
        });
        // If a value is set in the default section and not explicitly overwritten by the
        // user on the command line or in a different section, then we want to enable
        // warnings about the value being ignored.
        return has_default_section_setting && !has_other_setting;
        */
}

