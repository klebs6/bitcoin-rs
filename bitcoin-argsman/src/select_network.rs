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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_config_network_sets_field() {
        let mut inner = ArgsManagerInner::default();
        inner.select_config_network("signet");
        assert_eq!(inner.network.as_deref(), Some("signet"));
    }
}
