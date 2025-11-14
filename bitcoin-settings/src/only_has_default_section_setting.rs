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
    name:     &str,
) -> bool {
    debug!(
        "only_has_default_section_setting: section='{section}', name='{name}'"
    );

    let mut has_default_section_setting = false;
    let mut has_other_setting = false;

    let section_s = section.to_owned();
    let name_s    = name.to_owned();

    merge_settings(settings, &section_s, &name_s, |span: SettingsSpan, source: Source| {
        if span.empty() {
            trace!("only_has_default_section_setting: span empty; skipping");
            return;
        } else if matches!(source, Source::CONFIG_FILE_DEFAULT_SECTION) {
            has_default_section_setting = true;
            trace!("only_has_default_section_setting: found default-section value");
        } else {
            has_other_setting = true;
            trace!("only_has_default_section_setting: found higher-priority value");
        }
    });

    // If a value is set in the default section and not explicitly overwritten by the
    // user on the command line or in a different section, then we want to enable
    // warnings about the value being ignored.
    let res = has_default_section_setting && !has_other_setting;
    info!(
        "only_has_default_section_setting: section='{section}', name='{name}' -> {}",
        res
    );
    res
}

#[cfg(test)]
mod only_default_section_detection_spec {

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
    fn true_when_only_default_section_has_value() {
        info!("Verifying 'only_has_default_section_setting' returns true when only default has value");
        let mut def_map = HashMap::new();
        def_map.insert("opt".into(), vec![sv_json("\"def\"")]);

        let mut ro = HashMap::new();
        ro.insert("".into(), def_map);

        let s = build_settings(HashMap::new(), HashMap::new(), HashMap::new(), ro);
        let r = only_has_default_section_setting(&s, "main", "opt");
        debug!("Result: {}", r);
        assert!(r);
    }

    #[traced_test]
    fn false_when_overridden_by_network_or_cli() {
        info!("Verifying 'only_has_default_section_setting' returns false if overridden");
        // default has value
        let mut def_map = HashMap::new();
        def_map.insert("opt".into(), vec![sv_json("\"def\"")]);
        // network overrides
        let mut net_map = HashMap::new();
        net_map.insert("opt".into(), vec![sv_json("\"net\"")]);

        let mut ro = HashMap::new();
        ro.insert("".into(), def_map);
        ro.insert("main".into(), net_map);

        let mut cli = HashMap::new();
        cli.insert("opt".into(), vec![sv_json("\"cli\"")]);

        let s = build_settings(HashMap::new(), cli, HashMap::new(), ro);
        let r = only_has_default_section_setting(&s, "main", "opt");
        debug!("Result: {}", r);
        assert!(!r);
    }
}
