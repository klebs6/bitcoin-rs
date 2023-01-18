crate::ix!();

lazy_static!{
    pub static ref CS_MAIN: ScopedRawMutex = Default::default();
}

/**
  | This function is used for testing the
  | stale tip eviction logic, see denialofservice_tests.cpp
  |
  */
pub fn update_last_block_announce_time(
    node:            NodeId,
    time_in_seconds: i64)
{
    
    let guard = CS_MAIN.lock();

    let state = create_state(node);

    if state.is_some() {
        state.get_mut().last_block_announcement = OffsetDateTime::from_unix_timestamp(time_in_seconds).ok();
    }
}
