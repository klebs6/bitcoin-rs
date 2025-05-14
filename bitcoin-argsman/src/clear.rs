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
