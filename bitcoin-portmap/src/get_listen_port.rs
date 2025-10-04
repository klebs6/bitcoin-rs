// ---------------- [ File: bitcoin-portmap/src/get_listen_port.rs ]
crate::ix!();

pub fn get_listen_port() -> u16 {
    
    todo!();
        /*
            return static_cast<uint16_t>(gArgs.GetIntArg("-port", Params().GetDefaultPort()));
        */
}

/// Return the TCP port the node should bind to for incoming connections.
///
/// Mirrors the C++ call  
/// `gArgs.GetIntArg("-port", Params().GetDefaultPort())`.
///
/// 1. Fetches the chain‑specific default via `Params().GetDefaultPort()`.  
/// 2. Uses `ArgsManager::get_int_arg("-port", default)` to honour any
///    `-port=<n>` command‑line override supplied by the user.
///
/// Robust `tracing` instrumentation is provided for both the *resolved* port
/// and the *default* port used in the lookup.
pub fn get_listen_port() -> u16 {
    let default_port = bitcoinchain_params::params().get_default_port();
    let port = G_ARGS
        .lock()
        .get_int_arg("-port", default_port as i64) as u16;

    trace!(
        target = "portmap",
        port,
        default_port,
        "get_listen_port resolved"
    );
    port
}

#[cfg(test)]
mod test_get_listen_port {
    use super::*;

    #[traced_test]
    fn default_when_no_cli_override() {
        G_ARGS.lock().clear_args();
        assert_eq!(get_listen_port(), 8333);
    }
}
