// ---------------- [ File: bitcoinchain-interface/src/state.rs ]
crate::ix!();

pub trait ChainStateInterface: 
CoinsTip 
+ ChainHeight
+ IsInitialBlockDownload
+ ActivateBestChain
{}

pub trait CoinsTip {
    fn coins_tip(&mut self) -> &mut CoinsViewCache;
}

pub trait ActivateBestChain {

    fn activate_best_chain(
        &mut self, 
        state:  &mut BlockValidationState,
        pblock: Amo<Block>) -> bool;
}
