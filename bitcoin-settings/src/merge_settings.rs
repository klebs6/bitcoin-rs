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
pub fn merge_settings<F>(
    settings: &Settings,
    section:  &String,
    name:     &String,
    mut fn_:  F,
)
where
    F: FnMut(SettingsSpan, Source),
{
    trace!(
        "merge_settings: begin for name='{}' section='{}'",
        name,
        section
    );

    // Merge in the forced settings
    if let Some(value) = settings.forced_settings().get(name) {
        debug!("merge_settings: applying FORCED");
        fn_(SettingsSpan::from(value), Source::FORCED);
    }

    // Merge in the command-line options
    if let Some(values) = settings.command_line_options().get(name) {
        debug!("merge_settings: applying COMMAND_LINE ({} entrie(s))", values.len());
        fn_(SettingsSpan::from(values), Source::COMMAND_LINE);
    }

    // Merge in the read-write settings
    if let Some(value) = settings.rw_settings().get(name) {
        debug!("merge_settings: applying RW_SETTINGS");
        fn_(SettingsSpan::from(value), Source::RW_SETTINGS);
    }

    // Merge in the network-specific section of the config file
    if !section.is_empty() {
        if let Some(map) = settings.ro_config().get(section) {
            if let Some(values) = map.get(name) {
                debug!(
                    "merge_settings: applying CONFIG_FILE_NETWORK_SECTION ({} entrie(s))",
                    values.len()
                );
                fn_(SettingsSpan::from(values), Source::CONFIG_FILE_NETWORK_SECTION);
            }
        }
    }

    // Merge in the default section of the config file
    if let Some(map) = settings.ro_config().get("") {
        if let Some(values) = map.get(name) {
            debug!(
                "merge_settings: applying CONFIG_FILE_DEFAULT_SECTION ({} entrie(s))",
                values.len()
            );
            fn_(SettingsSpan::from(values), Source::CONFIG_FILE_DEFAULT_SECTION);
        }
    }

    trace!("merge_settings: end for name='{}'", name);
}

#[cfg(test)]
mod merge_settings_integration_spec {

    use super::*;

    fn base_settings() -> Settings {
        SettingsBuilder::default()
            .forced_settings(HashMap::new())
            .command_line_options(HashMap::new())
            .rw_settings(HashMap::new())
            .ro_config(HashMap::new())
            .build()
            .unwrap()
    }

    #[traced_test]
    fn merge_callback_invocation_order_and_sources() {
        // (merge_settings_integration_spec) updated to avoid requiring Debug for Source
        info!("Verifying callback order and sources for a fully-populated Settings");

        let mut s = SettingsBuilder::default()
            .forced_settings(HashMap::new())
            .command_line_options(HashMap::new())
            .rw_settings(HashMap::new())
            .ro_config(HashMap::new())
            .build()
            .unwrap();

        {
            let forced = s.forced_settings_mut();
            forced.insert("k".into(), sv_json("\"F\""));
            let cli = s.command_line_options_mut();
            cli.insert("k".into(), vec![sv_json("\"C1\""), sv_json("\"C2\"")]);
            let rw = s.rw_settings_mut();
            rw.insert("k".into(), sv_json("\"RW\""));

            let mut net_map = HashMap::new();
            net_map.insert("k".into(), vec![sv_json("\"N1\""), sv_json("\"N2\"")]);

            let mut def_map = HashMap::new();
            def_map.insert("k".into(), vec![sv_json("\"D1\"")]);

            let ro = s.ro_config_mut();
            ro.insert("main".into(), net_map);
            ro.insert("".into(), def_map);
        }

        let mut seen: Vec<(Source, Vec<String>, bool)> = Vec::new();

        merge_settings(&s, &"main".into(), &"k".into(), |span, source| {
            unsafe {
                let begin = span.begin();
                let end = span.end();
                let len = end.offset_from(begin);
                let eff = if len <= 0 {
                    vec![]
                } else {
                    std::slice::from_raw_parts(begin, len as usize)
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                };
                seen.push((source, eff, span.last_negated()));
            }
        });

        // Log a human-readable summary without requiring Debug on Source.
        let src_to_str = |s: &Source| match s {
            Source::FORCED => "FORCED",
            Source::COMMAND_LINE => "COMMAND_LINE",
            Source::RW_SETTINGS => "RW_SETTINGS",
            Source::CONFIG_FILE_NETWORK_SECTION => "CONFIG_FILE_NETWORK_SECTION",
            Source::CONFIG_FILE_DEFAULT_SECTION => "CONFIG_FILE_DEFAULT_SECTION",
        };
        let summary = seen
            .iter()
            .map(|(src, vals, last)| format!("({}:{:?}, last_neg={})", src_to_str(src), vals, last))
            .collect::<Vec<_>>()
            .join(", ");
        debug!("Seen summary: [{}]", summary);

        // Expected exact order
        assert!(matches!(seen[0].0, Source::FORCED));
        assert!(matches!(seen[1].0, Source::COMMAND_LINE));
        assert!(matches!(seen[2].0, Source::RW_SETTINGS));
        assert!(matches!(seen[3].0, Source::CONFIG_FILE_NETWORK_SECTION));
        assert!(matches!(seen[4].0, Source::CONFIG_FILE_DEFAULT_SECTION));

        // Check effective values captured at each stage
        assert_eq!(seen[0].1, vec!["\"F\""]);
        assert_eq!(seen[1].1, vec!["\"C1\"", "\"C2\""]);
        assert_eq!(seen[2].1, vec!["\"RW\""]);
        assert_eq!(seen[3].1, vec!["\"N1\"", "\"N2\""]);
        assert_eq!(seen[4].1, vec!["\"D1\""]);
    }

    #[traced_test]
    fn merge_skips_network_section_when_section_is_empty() {
        info!("Verifying that network-specific section is skipped when empty section name is provided");

        let mut s = base_settings();
        {
            s.command_line_options_mut()
                .insert("x".into(), vec![sv_json("\"cli\"")]);
            let mut net_map = HashMap::new();
            net_map.insert("x".into(), vec![sv_json("\"net\"")]);
            s.ro_config_mut().insert("main".into(), net_map);

            let mut def_map = HashMap::new();
            def_map.insert("x".into(), vec![sv_json("\"def\"")]);
            s.ro_config_mut().insert("".into(), def_map);
        }

        let mut sources = Vec::new();
        merge_settings(&s, &"".into(), &"x".into(), |_, src| sources.push(src));

        // With empty section, only FORCED (none here), CLI, RW (none), and DEFAULT_SECTION should fire.
        // In our setup: CLI + DEFAULT_SECTION are expected.
        assert!(sources.iter().any(|s| matches!(s, Source::COMMAND_LINE)));
        assert!(sources.iter().any(|s| matches!(s, Source::CONFIG_FILE_DEFAULT_SECTION)));
        assert!(!sources.iter().any(|s| matches!(s, Source::CONFIG_FILE_NETWORK_SECTION)));
    }
}
