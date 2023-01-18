crate::ix!();

/*
/**
  | Helper to count the seconds of a duration.
  | 
  | All durations should be using std::chrono
  | and calling this should generally be
  | avoided in code. Though, it is still
  | preferred to an inline t.count() to
  | protect against a reliance on the exact
  | type of t.
  | 
  | This helper is used to convert durations
  | before passing them over an interface
  | that doesn't support std::chrono (e.g.
  | RPC, debug log, or the GUI)
  |
  */
pub fn count_seconds(t: Seconds) -> i64 {
    
    todo!();
        /*
            return t.count();
        */
}

pub fn count_milliseconds(t: Milliseconds) -> i64 {
    
    todo!();
        /*
            return t.count();
        */
}

pub fn count_microseconds(t: Microseconds) -> i64 {
    
    todo!();
        /*
            return t.count();
        */
}

pub type SecondsDouble = Seconds<u64>;

/**
  | Helper to count the seconds in any duration
  | type
  |
  */
#[inline] pub fn count_seconds_double(t: SecondsDouble) -> f64 {
    
    todo!();
        /*
            return t.count();
        */
}
*/
