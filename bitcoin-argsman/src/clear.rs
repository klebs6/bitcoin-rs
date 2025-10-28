// ---------------- [ File: bitcoin-argsman/src/clear.rs ]
crate::ix!();

impl ArgsManagerInner {

    /**
      | Clear available arguments
      |
      */
    pub fn clear_args(&mut self)  {
        
        self.available_args.clear();
        self.network_only_args.clear();
    }

    /**
      | Clear cached directory paths
      |
      */
    pub fn clear_path_cache(&mut self)  {
        
        self.cached_datadir_path         = None;
        self.cached_network_datadir_path = None;
        self.cached_blocks_path          = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn clear_args_empties_available_and_network_only() {
        let mut inner = ArgsManagerInner::default();
        inner.available_args.insert(OptionsCategory::OPTIONS, HashMap::new());
        inner.network_only_args.insert("-rpcport".into());
        inner.clear_args();
        assert!(inner.available_args.is_empty());
        assert!(inner.network_only_args.is_empty());
    }

    #[test]
    fn clear_path_cache_nils_all() {
        let mut inner = ArgsManagerInner::default();
        inner.cached_blocks_path = Some(PathBuf::from("/tmp/whatever/blocks").into_boxed_path());
        inner.cached_datadir_path = Some(PathBuf::from("/tmp/whatever").into_boxed_path());
        inner.cached_network_datadir_path = Some(PathBuf::from("/tmp/whatever/net").into_boxed_path());
        inner.clear_path_cache();
        assert!(inner.cached_blocks_path.is_none());
        assert!(inner.cached_datadir_path.is_none());
        assert!(inner.cached_network_datadir_path.is_none());
    }
}
