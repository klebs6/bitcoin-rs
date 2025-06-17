// ---------------- [ File: bitcoin-log/src/defaults.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/logging.h]

pub const DEFAULT_LOGTIMEMICROS:      bool = false;
pub const DEFAULT_LOGIPS:             bool = false;
pub const DEFAULT_LOGTIMESTAMPS:      bool = true;
pub const DEFAULT_LOGTHREADNAMES:     bool = false;
pub const DEFAULT_LOGSOURCELOCATIONS: bool = false;

pub const DEFAULT_DEBUGLOGFILE: &'static str = "debug.log";

lazy_static!{
    pub static ref LOG_IPS: bool = DEFAULT_LOGIPS;
}
