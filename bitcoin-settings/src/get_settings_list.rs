// ---------------- [ File: bitcoin-settings/src/get_settings_list.rs ]
crate::ix!();

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
