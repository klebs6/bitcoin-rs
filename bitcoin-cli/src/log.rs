// ---------------- [ File: bitcoin-cli/src/log.rs ]
crate::ix!();

/**
  | libevent event log callback
  |
  */
pub extern "C" fn libevent_log_cb(
    severity: i32,
    msg:      *const i8) {

    /*
      | Ignore everything other than errors
      |
      */
    if severity >= EVENT_LOG_ERR.try_into().unwrap() {

        //the c++ threw a runtime error here
        //
        //however since we are in extern "C" for
        //the ffi, we will just log the error
        //
        //return Err(runtime_error(format!("libevent error: {:?}",msg)));
        eprintln!("libevent error: {:?}", msg);
    }
}
