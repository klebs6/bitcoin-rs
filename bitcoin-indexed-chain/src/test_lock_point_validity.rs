crate::ix!();

/**
  | Test whether the LockPoints height
  | and time are still valid on the current
  | chain
  |
  */
#[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
pub fn test_lock_point_validity(
        active_chain: &mut Chain,
        lp:           Arc<LockPoints>) -> bool {
    
    todo!();
        /*
            AssertLockHeld(cs_main);
        assert(lp);
        // If there are relative lock times then the maxInputBlock will be set
        // If there are no relative lock times, the LockPoints don't depend on the chain
        if (lp->maxInputBlock) {
            // Check whether active_chain is an extension of the block at which the LockPoints
            // calculation was valid.  If not LockPoints are no longer valid
            if (!active_chain.Contains(lp->maxInputBlock)) {
                return false;
            }
        }

        // LockPoints still valid
        return true;
        */
}

