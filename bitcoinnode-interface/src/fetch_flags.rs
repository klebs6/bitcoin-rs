// ---------------- [ File: bitcoinnode-interface/src/fetch_flags.rs ]
crate::ix!();

#[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
pub fn get_fetch_flags(pfrom: &dyn NodeInterface) -> GetDataMsg {
    
    todo!();
        /*
            uint32_t nFetchFlags = 0;
        if (State(pfrom.GetId())->fHaveWitness) {
            nFetchFlags |= MSG_WITNESS_FLAG;
        }
        return nFetchFlags;
        */
}
