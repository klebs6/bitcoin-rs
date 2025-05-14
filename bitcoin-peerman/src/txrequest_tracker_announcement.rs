// ---------------- [ File: bitcoin-peerman/src/txrequest_tracker_announcement.rs ]
crate::ix!();

/**
  | Type alias for sequence numbers.
  |
  */
pub type SequenceNumber = u64;

/**
  | An announcement. This is the data we
  | track for each txid or wtxid that is announced
  | to us by each peer.
  |
  */
#[derive(PartialEq,Debug,Clone)]
pub struct Announcement {

    /**
      | Txid or wtxid that was announced.
      |
      */
    pub txhash: u256,

    /**
      | For CANDIDATE_{DELAYED,BEST,READY}
      | the reqtime; for REQUESTED the expiry.
      |
      */
    pub time:   OffsetDateTime /* micros */,

    /**
      | What peer the request was from.
      |
      */
    pub peer:   NodeId,

    pub bits:   AnnouncementBits,
}

impl Announcement {

    delegate! {
        to self.bits {
            pub fn is_wtxid(&self)  -> u8;
            pub fn preferred(&self) -> u8;
            pub fn sequence(&self)  -> u64;
            pub fn state(&self)     -> u8;
        }
    }
}

#[bitfield]
#[derive(PartialEq,Debug,Clone)]
pub struct AnnouncementBits {

    /**
      | What sequence number this announcement
      | has.
      |
      */
    pub sequence: B59,//SequenceNumber

    /**
      | Whether the request is preferred.
      |
      */
    pub preferred: B1,

    /**
      | Whether this is a wtxid request.
      |
      */
    pub is_wtxid: B1,

    /**
      | What state this announcement is in.
      | 
      | This is a uint8_t instead of a State to
      | silence a GCC warning in versions prior
      | to 8.4 and 9.3.
      | 
      | See https://gcc.gnu.org/bugzilla/show_bug.cgi?id=61414
      |
      */
    pub state: B3,
}

impl Announcement {

    /**
      | Convert m_state to a State enum.
      |
      */
    pub fn get_state(&self) -> State {
        
        self.state().try_into().unwrap()
    }

    /**
      | Convert a State enum to a uint8_t and
      | store it in m_state.
      |
      */
    pub fn set_state(&mut self, state: State)  {
        
        self.set_state(state);
    }

    /**
      | Whether this announcement is selected.
      | There can be at most 1 selected peer per
      | txhash.
      |
      */
    pub fn is_selected(&self) -> bool {
        
        self.get_state() == State::CANDIDATE_BEST || self.get_state() == State::REQUESTED
    }

    /**
      | Whether this announcement is waiting
      | for a certain time to pass.
      |
      */
    pub fn is_waiting(&self) -> bool {
        
        self.get_state() == State::REQUESTED || self.get_state() == State::CANDIDATE_DELAYED
    }

    /**
      | Whether this announcement can feasibly
      | be selected if the current IsSelected()
      | one disappears.
      |
      */
    pub fn is_selectable(&self) -> bool {
        
        self.get_state() == State::CANDIDATE_READY || self.get_state() == State::CANDIDATE_BEST
    }

    /**
      | Construct a new announcement from scratch,
      | initially in CANDIDATE_DELAYED state.
      |
      */
    pub fn new(
        gtxid:     &GenTxId,
        peer:      NodeId,
        preferred: bool,
        reqtime:   OffsetDateTime /* micros */,
        sequence:  SequenceNumber) -> Self {
    
        todo!();
        /*
            :
            m_txhash(gtxid.GetHash()), m_time(reqtime), m_peer(peer), m_sequence(sequence), m_preferred(preferred),
            m_is_wtxid(gtxid.IsWtxid()), m_state(static_cast<uint8_t>(State::CANDIDATE_DELAYED))
        */
    }
}

pub type AnnouncementModifier = Box<dyn FnMut(&mut Announcement) -> ()>;
