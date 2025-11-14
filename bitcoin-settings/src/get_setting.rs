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
    get_chain_name:                bool,
) -> SettingsValue {

    debug!(
        "get_setting: section='{section}', name='{name}', ignore_default_section_config={ignore_default_section_config}, get_chain_name={get_chain_name}"
    );

    let mut result: Option<SettingsValue> = None;
    let mut done = false; // Done merging any more settings sources.

    // The merge_settings API expects &String.
    let section_s = section.to_owned();
    let name_s    = name.to_owned();

    merge_settings(settings, &section_s, &name_s, |span: SettingsSpan, source: Source| {
        let source_str = match source {
            Source::FORCED                      => "FORCED",
            Source::COMMAND_LINE                => "COMMAND_LINE",
            Source::RW_SETTINGS                 => "RW_SETTINGS",
            Source::CONFIG_FILE_NETWORK_SECTION => "CONFIG_FILE_NETWORK_SECTION",
            Source::CONFIG_FILE_DEFAULT_SECTION => "CONFIG_FILE_DEFAULT_SECTION",
        };

        // Weird behavior preserved for backwards compatibility: Apply negated
        // setting even if non-negated setting would be ignored. A negated
        // value in the default section is applied to network specific options,
        // even though normal non-negated values there would be ignored.
        let never_ignore_negated_setting = span.last_negated();

        // Weird behavior preserved for backwards compatibility: Take first
        // assigned value instead of last. In general, later settings take
        // precedence over early settings, but for backwards compatibility in
        // the config file the precedence is reversed for all settings except
        // chain name settings.
        let reverse_precedence = 
            matches!(source, Source::CONFIG_FILE_NETWORK_SECTION | Source::CONFIG_FILE_DEFAULT_SECTION) 
            && !get_chain_name;

        // Weird behavior preserved for backwards compatibility: Negated
        // -regtest and -testnet arguments which you would expect to override
        // values set in the configuration file are currently accepted but
        // silently ignored. It would be better to apply these just like other
        // negated values, or at least warn they are ignored.
        let skip_negated_command_line = get_chain_name;

        if done {
            trace!("get_setting: already decided (done=true), skipping source={source_str}");
            return;
        }

        // Ignore settings in default config section if requested.
        if ignore_default_section_config
            && matches!(source, Source::CONFIG_FILE_DEFAULT_SECTION)
            && !never_ignore_negated_setting
        {
            debug!("get_setting: ignoring default-section values for name='{name}' (except final negation) from source={source_str}");
            return;
        }

        // Skip negated command line settings.
        if skip_negated_command_line && span.last_negated() {
            debug!("get_setting: skipping negated command-line value for name='{name}'");
            return;
        }

        if !span.empty() {
            // Select value according to precedence quirk.
            let chosen = unsafe {
                if reverse_precedence {
                    // first effective value
                    (*span.begin()).clone()
                } else {
                    // last effective value
                    (*span.end().offset(-1)).clone()
                }
            };
            trace!(
                "get_setting: selecting value (reverse_precedence={reverse_precedence}) from source={source_str}: {}",
                chosen
            );
            result = Some(chosen);
            done = true;
        } else if span.last_negated() {
            trace!(
                "get_setting: span empty but last value is explicit negation; returning false (source={source_str})"
            );
            result = Some(SettingsValue::from(false));
            done = true;
        } else {
            trace!(
                "get_setting: span empty and not negated for source={source_str}; continuing"
            );
        }
    });

    let out = result.unwrap_or_else(|| {
        debug!("get_setting: no value found; returning null");
        SettingsValue(UniValue::null())
    });
    info!("get_setting: section='{section}', name='{name}' -> {}", out);
    out
}

#[cfg(test)]
mod get_setting_behavior_spec {

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
    fn ensure_forced_value_wins_get_setting() {
        info!("Verifying that a FORCED value wins and short-circuits further merging");
        let mut forced = HashMap::new();
        forced.insert("opt".into(), sv_json("\"forced\""));

        let mut cli = HashMap::new();
        cli.insert("opt".into(), vec![sv_json("\"cmd1\"")]);

        let mut net_map = HashMap::new();
        net_map.insert("opt".into(), vec![sv_json("\"confnet\"")]);

        let mut ro = HashMap::new();
        ro.insert("main".into(), net_map);

        let settings = build_settings(forced, cli, HashMap::new(), ro);
        let out = get_setting(&settings, "main", "opt", false, false);
        debug!("Result: {}", out);
        assert_eq!(out, sv_json("\"forced\""));
    }

    #[traced_test]
    fn respect_reverse_precedence_in_config_sections_get_setting() {
        info!("Verifying reverse precedence in config file sections when not querying chain name");
        let mut net_map = HashMap::new();
        net_map.insert(
            "alpha".into(),
            vec![sv_json("\"first\""), sv_json("\"second\"")],
        );

        let mut ro = HashMap::new();
        ro.insert("main".into(), net_map);

        let settings = build_settings(HashMap::new(), HashMap::new(), HashMap::new(), ro);

        let out = get_setting(&settings, "main", "alpha", false, false);
        debug!("Result: {}", out);
        assert_eq!(out, sv_json("\"first\""), "reverse precedence should pick first");
    }

    #[traced_test]
    fn skip_negated_command_line_for_chain_name_get_setting() {
        info!("Verifying that negated CLI settings are skipped when get_chain_name=true");
        let mut cli = HashMap::new();
        // CLI provides a final explicit negation which would normally yield false.
        cli.insert("chain".into(), vec![SettingsValue::from(false)]);

        let mut net_map = HashMap::new();
        net_map.insert("chain".into(), vec![sv_json("\"netvalue\"")]);

        let mut ro = HashMap::new();
        ro.insert("main".into(), net_map);

        let settings = build_settings(HashMap::new(), cli, HashMap::new(), ro);
        let out = get_setting(&settings, "main", "chain", false, true);
        debug!("Result: {}", out);
        assert_eq!(out, sv_json("\"netvalue\""), "negated CLI must be skipped for chain name");
    }

    #[traced_test]
    fn ignore_default_section_unless_negated_get_setting() {
        info!("Verifying ignoring non-negated default-section values when requested");
        let mut default_map = HashMap::new();
        default_map.insert("key".into(), vec![sv_json("\"dval\"")]);

        let mut ro = HashMap::new();
        ro.insert("".into(), default_map);

        let settings = build_settings(HashMap::new(), HashMap::new(), HashMap::new(), ro);
        let out = get_setting(&settings, "main", "key", true, false);
        debug!("Result (non-negated default ignored): {}", out);
        assert_eq!(out, SettingsValue(UniValue::null()));

        info!("Verifying that final negation in default section still applies even if ignored flag is set");
        let mut default_map2 = HashMap::new();
        default_map2.insert("flag".into(), vec![SettingsValue::from(false)]);
        let mut ro2 = HashMap::new();
        ro2.insert("".into(), default_map2);

        let settings2 = build_settings(HashMap::new(), HashMap::new(), HashMap::new(), ro2);
        let out2 = get_setting(&settings2, "regtest", "flag", true, false);
        debug!("Result (negated default applied): {}", out2);
        assert_eq!(out2, SettingsValue::from(false));
    }

    #[traced_test]
    fn negated_only_result_is_false_get_setting() {
        info!("Verifying that a lone negation yields boolean false");
        let mut cli = HashMap::new();
        cli.insert("feature".into(), vec![SettingsValue::from(false)]);
        let settings = build_settings(HashMap::new(), cli, HashMap::new(), HashMap::new());
        let out = get_setting(&settings, "main", "feature", false, false);
        debug!("Result: {}", out);
        assert_eq!(out, SettingsValue::from(false));
    }

    #[traced_test]
    fn choose_last_value_for_non_config_sources_get_setting() {
        info!("Verifying that for non-config sources, last assignment wins");
        let mut cli = HashMap::new();
        cli.insert(
            "opt".into(),
            vec![sv_json("\"first\""), sv_json("\"second\"")],
        );
        let settings = build_settings(HashMap::new(), cli, HashMap::new(), HashMap::new());
        let out = get_setting(&settings, "main", "opt", false, false);
        debug!("Result: {}", out);
        assert_eq!(out, sv_json("\"second\""));
    }
}

#[cfg(test)]
mod get_setting_precedence_additional_spec {
    use super::*;

    fn sv_json(j: &str) -> SettingsValue {
        SettingsValue(UniValue::from(j))
    }

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
    fn cli_overrides_rw_and_all_config_sources_get_setting() {
        info!("Ensuring command-line takes precedence over RW and any config sections");
        let mut cli = HashMap::new();
        cli.insert("k".into(), vec![sv_json("\"cli1\""), sv_json("\"cli2\"")]);

        let mut rw = HashMap::new();
        rw.insert("k".into(), sv_json("\"rw\""));

        let mut net_map = HashMap::new();
        net_map.insert("k".into(), vec![sv_json("\"net\"")]);

        let mut def_map = HashMap::new();
        def_map.insert("k".into(), vec![sv_json("\"def\"")]);

        let mut ro = HashMap::new();
        ro.insert("main".into(), net_map);
        ro.insert("".into(), def_map);

        let settings = build_settings(HashMap::new(), cli, rw, ro);
        let out = get_setting(&settings, "main", "k", false, false);
        debug!("Result: {}", out);
        assert_eq!(out, sv_json("\"cli2\""), "last CLI value should win");
    }

    #[traced_test]
    fn get_chain_name_negated_then_nonnegated_cli_is_respected() {
        info!("When get_chain_name=true and CLI ends non-negated, the CLI value is used");
        let mut cli = HashMap::new();
        cli.insert(
            "chain".into(),
            vec![SettingsValue::from(false), sv_json("\"cli\"")],
        );

        let settings = build_settings(HashMap::new(), cli, HashMap::new(), HashMap::new());
        let out = get_setting(&settings, "main", "chain", false, true);
        debug!("Result: {}", out);
        assert_eq!(out, sv_json("\"cli\""));
    }
}
