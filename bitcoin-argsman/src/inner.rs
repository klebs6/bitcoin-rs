// ---------------- [ File: bitcoin-argsman/src/inner.rs ]
crate::ix!();

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

impl Default for ArgsManagerInner {
    fn default() -> Self {
        Self {
            settings:                    Settings::default(),
            command:                     Vec::new(),
            network:                     None,
            network_only_args:           HashSet::new(),
            available_args:              HashMap::new(),
            accept_any_command:          true,     // <-- important
            config_sections:             LinkedList::new(),
            cached_blocks_path:          None,
            cached_datadir_path:         None,
            cached_network_datadir_path: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_empty() {
        let i = ArgsManagerInner::default();
        assert!(i.command.is_empty());
        assert!(i.available_args.is_empty());
        assert!(i.network.is_none());
        assert!(i.cached_blocks_path.is_none());
    }
}
