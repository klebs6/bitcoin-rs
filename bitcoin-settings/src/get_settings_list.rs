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
    ignore_default_section_config: bool,
) -> Vec<SettingsValue> {
    debug!(
        "get_settings_list: section='{}', name='{}', ignore_default_section_config={}",
        section, name, ignore_default_section_config
    );

    let mut result: Vec<SettingsValue> = Vec::new();
    let mut done = false; // Done merging any more settings sources.
    let mut prev_negated_empty = false;

    merge_settings(settings, section, name, |span: SettingsSpan, source: Source| {
        let source_str = match source {
            Source::FORCED => "FORCED",
            Source::COMMAND_LINE => "COMMAND_LINE",
            Source::RW_SETTINGS => "RW_SETTINGS",
            Source::CONFIG_FILE_NETWORK_SECTION => "CONFIG_FILE_NETWORK_SECTION",
            Source::CONFIG_FILE_DEFAULT_SECTION => "CONFIG_FILE_DEFAULT_SECTION",
        };

        // Weird behavior preserved for backwards compatibility: Apply config
        // file settings even if negated on command line. Negating a setting on
        // command line will ignore earlier settings on the command line and
        // ignore settings in the config file, unless the negated command line
        // value is followed by non-negated value, in which case config file
        // settings will be brought back from the dead (but earlier command
        // line settings will still be ignored).
        let add_zombie_config_values 
            = matches!(source, Source::CONFIG_FILE_NETWORK_SECTION | Source::CONFIG_FILE_DEFAULT_SECTION) 
            && !prev_negated_empty;

        // Ignore settings in default config section if requested.
        if ignore_default_section_config && matches!(source, Source::CONFIG_FILE_DEFAULT_SECTION) {
            debug!("get_settings_list: ignoring default-section values for name='{}'", name);
            return;
        }

        // Add new settings to the result if it isn't already complete, or if the
        // values are zombies.
        if !done || add_zombie_config_values {
            unsafe {
                let begin = span.begin();
                let end   = span.end();
                let len   = end.offset_from(begin);
                let len   = if len <= 0 { 0 } else { len as usize };
                let eff   = std::slice::from_raw_parts(begin, len);

                for value in eff.iter() {
                    if value.0.is_array() {
                        let inner = value.0.get_values();
                        debug!(
                            "get_settings_list: flattening array with {} element(s) from source={source_str}",
                            inner.len()
                        );
                        for u in inner {
                            result.push(SettingsValue(u.clone()));
                        }
                    } else {
                        result.push(value.clone());
                    }
                }
            }
        } else {
            trace!(
                "get_settings_list: already complete (done=true) and not adding zombies from source={source_str}"
            );
        }

        // If a setting was negated, or if a setting was forced, set
        // done to true to ignore any later lower priority settings.
        if span.negated() > 0 || matches!(source, Source::FORCED) {
            done = true;
            trace!(
                "get_settings_list: marking as done (negated={} forced={})",
                span.negated() > 0,
                matches!(source, Source::FORCED)
            );
        }

        // Update the negated and empty state used for the zombie values check.
        prev_negated_empty |= span.last_negated() && result.is_empty();
        trace!(
            "get_settings_list: prev_negated_empty set to {}",
            prev_negated_empty
        );
    });

    info!(
        "get_settings_list: collected {} value(s) for '{}'",
        result.len(),
        name
    );
    result
}

#[cfg(test)]
mod accumulate_settings_list_spec {

    use super::*;

    fn build_settings(
        forced: HashMap<String, SettingsValue>,
        cli: HashMap<String, Vec<SettingsValue>>,
        rw: HashMap<String, SettingsValue>,
        ro: HashMap<String, HashMap<String, Vec<SettingsValue>>>,
    ) -> Settings {
        SettingsBuilder::default()
            .forced_settings(forced)
            .command_line_options(cli)
            .rw_settings(rw)
            .ro_config(ro)
            .build()
            .unwrap()
    }

    #[traced_test]
    fn flatten_arrays_and_scalars_in_list_accumulation() {
        info!("Verifying flattening of array values alongside scalar values");

        // Command line: an array ["x","y"] and a scalar "z"
        let mut cli = HashMap::new();
        cli.insert(
            "k".into(),
            vec![sv_json("[\"x\",\"y\"]"), sv_json("\"z\"")],
        );

        let settings = build_settings(HashMap::new(), cli, HashMap::new(), HashMap::new());
        let out = get_settings_list(&settings, &"main".into(), &"k".into(), false);
        debug!("Collected list: {:?}", out.iter().map(|v| v.to_string()).collect::<Vec<_>>());
        assert_eq!(out, vec![sv_json("\"x\""), sv_json("\"y\""), sv_json("\"z\"")]);
    }

    #[traced_test]
    fn zombie_config_values_after_negated_then_nonnegated_command_line() {
        info!("Verifying that config values are added back as zombies after a negated CLI followed by non-negated");

        // CLI: first a negation (false), then a non-negated value
        let mut cli = HashMap::new();
        cli.insert(
            "opt".into(),
            vec![SettingsValue::from(false), sv_json("\"cmd\"")],
        );

        // Config (network): two values which should be added as zombies
        let mut net_map = HashMap::new();
        net_map.insert(
            "opt".into(),
            vec![sv_json("\"conf1\""), sv_json("\"conf2\"")],
        );
        let mut ro = HashMap::new();
        ro.insert("main".into(), net_map);

        let settings = build_settings(HashMap::new(), cli, HashMap::new(), ro);
        let out = get_settings_list(&settings, &"main".into(), &"opt".into(), false);

        let got = out.iter().map(|v| v.to_string()).collect::<Vec<_>>();
        debug!("Collected list: {:?}", got);
        assert_eq!(out, vec![sv_json("\"cmd\""), sv_json("\"conf1\""), sv_json("\"conf2\"")]);
    }

    #[traced_test]
    fn forced_short_circuits_non_config_sources_in_list() {
        info!("Verifying that FORCED settings stop accumulation from non-config sources");

        let mut forced = HashMap::new();
        forced.insert("x".into(), sv_json("\"F\""));

        let mut cli = HashMap::new();
        cli.insert("x".into(), vec![sv_json("\"CLI\"")]);

        let settings = build_settings(forced, cli, HashMap::new(), HashMap::new());
        let out = get_settings_list(&settings, &"main".into(), &"x".into(), false);

        let got = out.iter().map(|v| v.to_string()).collect::<Vec<_>>();
        debug!("Collected list: {:?}", got);
        // Command-line value should NOT be appended after a forced value (non-config zombie rule doesn't apply)
        assert_eq!(out, vec![sv_json("\"F\"")]);
    }

    #[traced_test]
    fn ignore_default_section_in_list_when_requested() {
        info!("Verifying that default section values are ignored in list mode when requested");

        let mut net_map = HashMap::new();
        net_map.insert("k".into(), vec![sv_json("\"net\"")]);

        let mut def_map = HashMap::new();
        def_map.insert("k".into(), vec![sv_json("\"def\"")]);

        let mut ro = HashMap::new();
        ro.insert("main".into(), net_map);
        ro.insert("".into(), def_map);

        let settings = build_settings(HashMap::new(), HashMap::new(), HashMap::new(), ro);
        let out = get_settings_list(&settings, &"main".into(), &"k".into(), true);

        let got = out.iter().map(|v| v.to_string()).collect::<Vec<_>>();
        debug!("Collected list: {:?}", got);
        assert_eq!(out, vec![sv_json("\"net\"")], "default section should be ignored");
    }
}

#[cfg(test)]
mod accumulate_settings_list_additional_edges_spec {
    use super::*;

    fn build_settings(
        forced: HashMap<String, SettingsValue>,
        cli: HashMap<String, Vec<SettingsValue>>,
        rw: HashMap<String, SettingsValue>,
        ro: HashMap<String, HashMap<String, Vec<SettingsValue>>>,
    ) -> Settings {
        SettingsBuilder::default()
            .forced_settings(forced)
            .command_line_options(cli)
            .rw_settings(rw)
            .ro_config(ro)
            .build()
            .expect("settings builder")
    }

    #[traced_test]
    fn cli_negated_then_multiple_nonnegated_and_zombie_network_values() {
        info!("Negated CLI followed by non-negated should allow network zombies and include all later non-negated CLI values");
        let mut cli = HashMap::new();
        cli.insert(
            "opt".into(),
            vec![
                SettingsValue::from(false),
                sv_json("\"cmd1\""),
                sv_json("\"cmd2\""),
            ],
        );

        let mut net_map = HashMap::new();
        net_map.insert(
            "opt".into(),
            vec![sv_json("\"conf1\""), sv_json("\"conf2\"")],
        );

        let mut ro = HashMap::new();
        ro.insert("main".into(), net_map);

        let settings = build_settings(HashMap::new(), cli, HashMap::new(), ro);
        let out = get_settings_list(&settings, &"main".into(), &"opt".into(), false);

        let got = out.iter().map(|v| v.to_string()).collect::<Vec<_>>();
        debug!("Collected list: {:?}", got);

        assert_eq!(
            out,
            vec![
                sv_json("\"cmd1\""),
                sv_json("\"cmd2\""),
                sv_json("\"conf1\""),
                sv_json("\"conf2\"")
            ],
            "expect CLI values after negation + network zombies"
        );
    }

    #[traced_test]
    fn top_level_array_is_flattened_but_nested_arrays_are_preserved() {
        info!("Flattening should only occur for the top-level array values, preserving nested arrays");

        // First value is an array [[1,2],3] – top-level flattening yields [ [1,2], 3 ]
        // Second value is a scalar 4
        let mut cli = HashMap::new();
        cli.insert("k".into(), vec![sv_json("[[1,2],3]"), sv_json("4")]);

        let settings = build_settings(HashMap::new(), cli, HashMap::new(), HashMap::new());
        let out = get_settings_list(&settings, &"main".into(), &"k".into(), false);

        let got = out.iter().map(|v| v.to_string()).collect::<Vec<_>>();
        debug!("Collected list: {:?}", got);

        assert_eq!(
            out,
            vec![sv_json("[1,2]"), sv_json("3"), sv_json("4")],
            "expect one level of flattening only"
        );
    }
}
