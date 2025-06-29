// ---------------- [ File: bitcoin-settings/src/get_setting.rs ]
crate::ix!();

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
