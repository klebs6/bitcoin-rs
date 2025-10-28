// ---------------- [ File: bitcoin-argsman/src/options_category.rs ]
crate::ix!();

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum OptionsCategory {
    OPTIONS,
    CONNECTION,
    WALLET,
    WALLET_DEBUG_TEST,
    ZMQ,
    DEBUG_TEST,
    CHAINPARAMS,
    NODE_RELAY,
    BLOCK_CREATION,
    RPC,
    GUI,
    COMMANDS,
    REGISTER_COMMANDS,

    /**
      | Always the last option to avoid printing
      | these in the help
      |
      */
    HIDDEN, 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn categories_are_orderable() {
        // Just make sure Ord/Eq compile and behave deterministically
        let mut v = vec![
            OptionsCategory::RPC,
            OptionsCategory::OPTIONS,
            OptionsCategory::CHAINPARAMS,
        ];
        v.sort();
        assert_eq!(v[0], OptionsCategory::OPTIONS);
        assert_eq!(v[1], OptionsCategory::CHAINPARAMS);
    }
}
