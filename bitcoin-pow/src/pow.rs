hello! could you teach me how this crate works?

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/pow.h]
//-------------------------------------------[.cpp/bitcoin/src/pow.cpp]

pub fn get_next_work_required(
        pindex_last: Arc<BlockIndex>,
        pblock:      Arc<BlockHeader>,
        params:      &ChainConsensusParams) -> u32 {
    
    let n_proof_of_work_limit: u32 = uint_to_arith256(&params.pow_limit).get_compact(None);

    //  Only change once per difficulty adjustment interval
    if (((*pindex_last).n_height + 1) as i64) % params.difficulty_adjustment_interval() != 0 {

        if params.pow_allow_min_difficulty_blocks {

            //  Special difficulty rule for
            //  testnet:
            //
            //  If the new block's timestamp is
            //  more than 2* 10 minutes then allow
            //  mining of a min-difficulty block.
            if (*pblock).get_block_time() > (*pindex_last).get_block_time() + params.n_pow_target_spacing * 2 {

                return n_proof_of_work_limit;

            } else {

                // Return the last non-special-min-difficulty-rules-block
                let mut pindex: Arc<BlockIndex> = pindex_last;;

                while (*pindex).pprev.is_some()
                && ((*pindex).n_height as i64) % params.difficulty_adjustment_interval() != 0 
                && (*pindex).n_bits == n_proof_of_work_limit 
                {
                    pindex = (*pindex).pprev.as_ref().unwrap().clone();
                }

                return (*pindex).n_bits;
            }
        }

        return (*pindex_last).n_bits;
    }

    //  Go back by what we want to be 14 days worth of blocks
    let n_height_first: i32 = (((*pindex_last).n_height as i64) - (params.difficulty_adjustment_interval() - 1)).try_into().unwrap();

    assert!(n_height_first >= 0);

    let pindex_first = pindex_last.clone().get_ancestor(n_height_first);

    assert!(pindex_first.is_some());

    calculate_next_work_required(
        pindex_last.clone(),
        pindex_first.unwrap().get_block_time(),
        params
    )
}

pub fn calculate_next_work_required(
        pindex_last:        Arc<BlockIndex>,
        n_first_block_time: i64,
        params:             &ChainConsensusParams) -> u32 {
    
    if params.pow_no_retargeting {
        return (*pindex_last).n_bits;
    }

    //  Limit adjustment step
    let mut n_actual_timespan: i64 = (*pindex_last).get_block_time() - n_first_block_time;;

    if n_actual_timespan < params.n_pow_target_timespan / 4 {
        n_actual_timespan = params.n_pow_target_timespan / 4;
    }

    if n_actual_timespan > params.n_pow_target_timespan * 4 {
        n_actual_timespan = params.n_pow_target_timespan * 4;
    }

    //  Retarget
    let bn_pow_limit: ArithU256 = uint_to_arith256(&params.pow_limit);;

    let mut bn_new = ArithU256::default();

    bn_new.set_compact((*pindex_last).n_bits, null_mut(), null_mut());

    bn_new *= n_actual_timespan;
    bn_new /= params.n_pow_target_timespan;

    if bn_new > bn_pow_limit {
        bn_new = bn_pow_limit;
    }

    bn_new.get_compact(None)
}

/**
  | Check whether a block hash satisfies
  | the proof-of-work requirement specified
  | by nBits
  |
  */
pub fn check_proof_of_work(
        hash:   u256,
        n_bits: u32,
        params: &ChainConsensusParams) -> bool {
    
    let mut negative = bool::default();
    let mut overflow = bool::default();

    let mut bn_target = ArithU256::default();

    bn_target.set_compact(n_bits, &mut negative, &mut overflow);

    //  Check range
    if negative || bn_target == ArithU256::from(0) || overflow || bn_target > uint_to_arith256(&params.pow_limit) {
        return false;
    }

    //  Check proof of work matches claimed amount
    if uint_to_arith256(&hash) > bn_target {
        return false;
    }

    true
}
