// ---------------- [ File: bitcoin-peerman/src/peer_has_header.rs ]
crate::ix!();

#[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
pub fn peer_has_header_with_arc(
    state:  &NodeState,
    pindex: Arc<BlockIndex>) -> bool 
{
    unsafe {

        if (*state).pindex_best_known_block.is_some() 
        && pindex == {

            let best_known_block = (*state).pindex_best_known_block.as_ref().unwrap();
            let ancestor         = best_known_block.clone().get_ancestor(pindex.n_height);

            ancestor.as_ref().unwrap().clone()
        }
        {
            return true;
        }

        if (*state).pindex_best_header_sent.is_some() 
        && pindex == {

            let best_header_sent = (*state).pindex_best_header_sent.as_ref().unwrap();
            let ancestor         = best_header_sent.clone().get_ancestor(pindex.n_height);

            ancestor.as_ref().unwrap().clone()
        }
        {
            return true;
        }

        false
    }
}

#[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
pub fn peer_has_header_with_amo(
    state:  &NodeState,
    pindex: Arc<BlockIndex>) -> bool 
{
    unsafe {

        if (*state).pindex_best_known_block.is_some() 
        && pindex == {

            let best_known_block = (*state).pindex_best_known_block.as_ref().unwrap();

            best_known_block.clone().get_ancestor(pindex.n_height).unwrap().clone()
        }
        {
            return true;
        }

        if let Some(ref header) = (*state).pindex_best_header_sent {

            let ancestor = header.clone().get_ancestor(pindex.n_height);

            if pindex == *ancestor.as_ref().unwrap() {
                return true;
            }
        }

        false
    }
}

