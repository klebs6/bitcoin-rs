// ---------------- [ File: bitcoin-settings/src/only_has_default_section_setting.rs ]
crate::ix!();

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
