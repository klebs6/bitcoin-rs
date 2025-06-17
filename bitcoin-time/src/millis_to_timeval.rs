// ---------------- [ File: bitcoin-time/src/millis_to_timeval.rs ]
crate::ix!();

/**
  | Convert milliseconds to a struct timeval
  | for e.g. select.
  |
  */
pub trait MillisToTimeval {
    fn millis_to_timeval(&self) -> libc::timeval;
}

impl MillisToTimeval for i64 {

    fn millis_to_timeval(&self) -> libc::timeval {
        let sec  = self / 1_000;
        let usec = (self % 1_000) * 1_000;
        trace!(millis = *self, sec, usec, "millis_to_timeval<i64>");
        libc::timeval {
            tv_sec:  sec as libc::time_t,
            tv_usec: usec as libc::suseconds_t,
        }
    }
}

impl MillisToTimeval for std::time::Duration {
    fn millis_to_timeval(&self) -> libc::timeval {
        ((*self).as_millis() as i64).millis_to_timeval()
    }
}

/*
impl MillisToTimeval for libc::timeval {
    fn millis_to_timeval(&self /*ms*/) -> libc::timeval {
        
        todo!();
            /*
                return MillisToTimeval(count_milliseconds(ms));
            */
    }
}
*/
