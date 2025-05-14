// ---------------- [ File: bitcoin-argsman/src/inner.rs ]
crate::ix!();

#[derive(Default)]
pub struct ArgsManagerInner {
    pub settings:                    Settings,
    pub command:                     Vec<String>,
    pub network:                     Option<String>,
    pub network_only_args:           HashSet<String>,
    pub available_args:              HashMap<OptionsCategory,HashMap<String,ArgsManagerArg>>,
    pub accept_any_command:          bool, // default = { true }
    pub config_sections:             LinkedList<SectionInfo>,
    pub cached_blocks_path:          Option<Box<Path>>,
    pub cached_datadir_path:         Option<Box<Path>>,
    pub cached_network_datadir_path: Option<Box<Path>>,
}
