crate::ix!();

/** 
 | The various states a (txhash,peer) pair can be
 | in.
 |
 | Note that CANDIDATE is split up into
 | 3 substates (DELAYED, BEST, READY), allowing
 | more efficient implementation.  Also note that
 | the sorting order of ByTxHashView relies on the
 | specific order of values in this enum.
 |
 | Expected behaviour is:
 |
 |   - When first announced by a peer, the state
 |     is CANDIDATE_DELAYED until reqtime is
 |     reached.
 |
 |   - Announcements that have reached their
 |     reqtime but not been requested will be
 |     either CANDIDATE_READY or
 |     CANDIDATE_BEST. Neither of those has an
 |     expiration time; they remain in that state
 |     until they're requested or no longer
 |     needed. CANDIDATE_READY announcements are
 |     promoted to CANDIDATE_BEST when they're the
 |     best one left.
 |
 |   - When requested, an announcement will be in
 |     state REQUESTED until expiry is reached.
 |
 |   - If expiry is reached, or the peer replies
 |     to the request (either with NOTFOUND or the
 |     tx), the state becomes COMPLETED.
 */
#[repr(u8)]
#[derive(Clone,PartialEq,Eq)]
pub enum State {

    /**
      | A CANDIDATE announcement whose reqtime
      | is in the future.
      |
      */
    CANDIDATE_DELAYED = 0,

    /**
      | A CANDIDATE announcement that's not
      | CANDIDATE_DELAYED or CANDIDATE_BEST.
      |
      */
    CANDIDATE_READY,

    /**
      | The best CANDIDATE for a given txhash;
      | only if there is no REQUESTED announcement
      | already for that txhash.
      | 
      | The CANDIDATE_BEST is the highest-priority
      | announcement among all CANDIDATE_READY
      | (and _BEST) ones for that txhash.
      |
      */
    CANDIDATE_BEST,

    /**
      | A REQUESTED announcement.
      |
      */
    REQUESTED,

    /**
      | A COMPLETED announcement.
      |
      */
    COMPLETED,
}

impl TryFrom<u8> for State {

    type Error = &'static str;

    fn try_from(x: u8) -> Result<State, Self::Error> {
        match x {
            0 => Ok(State::CANDIDATE_DELAYED),
            2 => Ok(State::CANDIDATE_READY),
            3 => Ok(State::CANDIDATE_BEST),
            4 => Ok(State::REQUESTED),
            5 => Ok(State::COMPLETED),
            _ => Err("bad input"),
        }
    }
}
