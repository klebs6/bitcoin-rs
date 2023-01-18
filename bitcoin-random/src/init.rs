crate::ix!();

/**
  | Initialize global RNG state and log
  | any CPU features that are used.
  | 
  | Calling this function is optional.
  | RNG state will be initialized when first
  | needed if it is not called.
  |
  */
pub fn random_init()  {
    
    /*
      | Invoke RNG code to trigger initialization
      | (if not already performed)
      |
      */
    proc_rand(&mut [], 0, RNGLevel::FAST);

    report_hardware_rand();
}

/**
  | Gather entropy from various expensive
  | sources, and feed them to the PRNG state.
  | 
  | Thread-safe.
  |
  */
pub fn rand_add_periodic()  {
    
    proc_rand(&mut [], 0, RNGLevel::PERIODIC);
}

/**
  | Gathers entropy from the low bits of
  | the time at which events occur. Should
  | be called with a uint32_t describing
  | the event at the time an event occurs.
  | 
  | Thread-safe.
  |
  */
pub fn rand_add_event(event_info: u32)  {
    
    G_RNG.lock().add_event(event_info);
}
