use tracing::{debug,info,trace};

pub fn detect_o_cloexec() -> bool {
    debug!("detect_o_cloexec: start");

    #[cfg(unix)]
    {
        debug!("detect_o_cloexec: unix target detected; assuming O_CLOEXEC support via libc");
        info!("detect_o_cloexec: O_CLOEXEC is supported on this platform");
        return true;
    }

    #[cfg(not(unix))]
    {
        debug!("detect_o_cloexec: non-unix target detected; O_CLOEXEC is not available");
        info!("detect_o_cloexec: O_CLOEXEC is NOT supported on this platform");
        return false;
    }
}

fn main() {
    bitcoin_cfg::setup();
    if detect_o_cloexec() {
        println!("cargo::rustc-cfg=have_o_cloexec");
    }
}
