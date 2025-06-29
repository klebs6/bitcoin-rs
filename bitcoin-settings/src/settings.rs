// ---------------- [ File: bitcoin-settings/src/settings.rs ]
crate::ix!();

/**
  | Stored settings. This struct combines settings
  | from the command line, a read-only
  | configuration file, and a read-write runtime
  | settings file.
  */
#[derive(Builder,Getters,MutGetters,Default)]
#[builder(setter(into))]
#[getset(get="pub")]
pub struct Settings {

    /**
      | Map of setting name to forced setting
      | value.
      |
      */
    forced_settings:      HashMap<String,SettingsValue>,

    /**
      | Map of setting name to list of command
      | line values.
      |
      */
    command_line_options: HashMap<String,Vec<SettingsValue>>,

    /**
      | Map of setting name to read-write file
      | setting value.
      |
      */
    rw_settings:          HashMap<String,SettingsValue>,

    /**
      | Map of config section name and setting
      | name to list of config file values.
      |
      */
    ro_config:            HashMap<String,HashMap<String,Vec<SettingsValue>>>,
}
