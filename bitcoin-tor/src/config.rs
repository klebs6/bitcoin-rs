// ---------------- [ File: bitcoin-tor/src/config.rs ]
crate::ix!();

/**
  | Functionality for communicating with
  | Tor.
  |
  */
lazy_static!{
    /*
    extern const std::string DEFAULT_TOR_CONTROL;
    */
}

pub const DEFAULT_LISTEN_ONION: bool = true;

/**
  | Default control port
  |
  */
pub const DEFAULT_TOR_CONTROL: &'static str = "127.0.0.1:9051";

/**
  | Tor cookie size (from control-spec.txt)
  |
  */
pub const TOR_COOKIE_SIZE: i32 = 32;

/**
  | Size of client/server nonce for SAFECOOKIE
  |
  */
pub const TOR_NONCE_SIZE: i32 = 32;

/**
  | For computing serverHash in SAFECOOKIE
  |
  */
pub const TOR_SAFE_SERVERKEY: &'static str = "Tor safe cookie authentication server-to-controller hash";

/**
  | For computing clientHash in SAFECOOKIE
  |
  */
pub const TOR_SAFE_CLIENTKEY: &'static str = "Tor safe cookie authentication controller-to-server hash";

/**
  | Exponential backoff configuration
  | - initial timeout in seconds
  |
  */
pub const RECONNECT_TIMEOUT_START: f32 = 1.0;

/**
  | Exponential backoff configuration
  | - growth factor
  |
  */
pub const RECONNECT_TIMEOUT_EXP: f32 = 1.5;

/**
  | Maximum length for lines received on
  | TorControlConnection. tor-control-spec.txt
  | mentions that there is explicitly no
  | limit defined to line length, this is
  | belt-and-suspenders sanity limit
  | to prevent memory exhaustion.
  |
  */
pub const MAX_LINE_LENGTH: i32 = 100000;
