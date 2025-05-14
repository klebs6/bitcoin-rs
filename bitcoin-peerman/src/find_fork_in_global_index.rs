// ---------------- [ File: bitcoin-peerman/src/find_fork_in_global_index.rs ]
crate::ix!();

/**
  | Find the last common block between the
  | parameter chain and a locator.
  |
  */
#[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
pub fn find_fork_in_global_index(
    blockman: &mut BlockManager,
    chain:    &dyn ChainInterface,
    locator:  &BlockLocator) -> Option<Arc<BlockIndex>> {
    
    todo!();
    /*
        AssertLockHeld(cs_main);

    // Find the latest block common to locator and chain - we expect that
    // locator.vHave is sorted descending by height.
    for (const uint256& hash : locator.vHave) {
        CBlockIndex* pindex = LookupBlockIndex(hash);
        if (pindex) {
            if (chain.Contains(pindex))
                return pindex;
            if (pindex->GetAncestor(chain.Height()) == chain.Tip()) {
                return chain.Tip();
            }
        }
    }
    return chain.Genesis();
    */
}
