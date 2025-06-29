// ---------------- [ File: bitcoin-settings/src/merge_settings.rs ]
crate::ix!();

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
