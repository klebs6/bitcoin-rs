// ---------------- [ File: bitcoin-argsman/src/select_network.rs ]
crate::ix!();

impl ArgsManagerInner {

    /**
      | Select the network in use
      |
      */
    pub fn select_config_network(&mut self, network: &str)  {
        
        self.network = Some(network.to_string());
    }
}
