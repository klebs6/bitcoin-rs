crate::ix!();

pub struct BlockIndexWorkComparator { }

impl BlockIndexWorkComparator {

    pub fn invoke(&self, 
        pa: *const BlockIndex,
        pb: *const BlockIndex) -> bool {
        
        unsafe {

            //  First sort by most total work, ...
            if (*pa).n_chain_work > (*pb).n_chain_work {
                return false;
            }

            if (*pa).n_chain_work < (*pb).n_chain_work {
                return true;
            }

            //  ... then by earliest time received, ...
            if (*pa).n_sequence_id < (*pb).n_sequence_id {
                return false;
            }

            if (*pa).n_sequence_id > (*pb).n_sequence_id {
                return true;
            }

            // Use pointer address as tie breaker
            // (should only happen with blocks loaded
            // from disk, as those all have id 0).
            if pa < pb {
                return false;
            }

            if pa > pb {
                return true;
            }
        }

        // Identical blocks.
        false
    }
}
