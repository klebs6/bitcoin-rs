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
    fn millis_to_timeval(&self/*n_timeout*/) -> libc::timeval {
        
        todo!();
            /*
                struct timeval timeout;
            timeout.tv_sec  = nTimeout / 1000;
            timeout.tv_usec = (nTimeout % 1000) * 1000;
            return timeout;
            */
    }
}

impl MillisToTimeval for libc::timeval {
    fn millis_to_timeval(&self /*ms*/) -> libc::timeval {
        
        todo!();
            /*
                return MillisToTimeval(count_milliseconds(ms));
            */
    }
}
