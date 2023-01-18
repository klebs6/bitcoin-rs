crate::ix!();

pub enum WaitState {

    /**
      | Used for announcements that need efficient
      | testing of "is their timestamp in the
      | future?".
      |
      */
    FUTURE_EVENT,

    /**
      | Used for announcements whose timestamp
      | is not relevant.
      |
      */
    NO_EVENT,

    /**
      | Used for announcements that need efficient
      | testing of "is their timestamp in the
      | past?".
      |
      */
    PAST_EVENT,
}

pub fn get_wait_state(ann: &Announcement) -> WaitState {
    
    if ann.is_waiting() {
        return WaitState::FUTURE_EVENT;
    }

    if ann.is_selectable() {
        return WaitState::PAST_EVENT;
    }

    WaitState::NO_EVENT
}
