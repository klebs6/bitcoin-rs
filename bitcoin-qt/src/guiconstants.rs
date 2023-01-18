crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/guiconstants.h]

/**
  | Milliseconds between model updates
  |
  */
pub const MODEL_UPDATE_DELAY: i32 = 250;

/**
  | AskPassphraseDialog -- Maximum passphrase
  | length
  |
  */
pub const MAX_PASSPHRASE_SIZE: i32 = 1024;

/**
  | BitcoinGUI -- Size of icons in status
  | bar
  |
  */
pub const STATUSBAR_ICONSIZE: i32 = 16;

pub const DEFAULT_SPLASHSCREEN: bool = true;

/**
  | Invalid field background style
  |
  */
pub const STYLE_INVALID: &'static str = "background:#FF8080";

/**
  | Transaction list -- unconfirmed transaction
  |
  */
pub fn color_unconfirmed() -> CppBox<QColor> { unsafe { QColor::from_3_int(128, 128, 128) } }

/**
  | Transaction list -- negative amount
  |
  */
pub fn color_negative() -> CppBox<QColor> { unsafe { QColor::from_3_int(255, 0, 0) } }

/**
  | Transaction list -- bare address (without
  | label)
  |
  */
pub fn color_bareaddress() -> CppBox<QColor> { unsafe { QColor::from_3_int(140, 140, 140) } }

/**
  | Transaction list -- TX status decoration
  | - open until date
  |
  */
pub fn color_tx_status_openuntildate() -> CppBox<QColor> { unsafe { QColor::from_3_int(64, 64, 255) } }

/**
  | Transaction list -- TX status decoration
  | - danger, tx needs attention
  |
  */
pub fn color_tx_status_danger() -> CppBox<QColor> { unsafe { QColor::from_3_int(200, 100, 100) } }

/**
  | Transaction list -- TX status decoration
  | - default color
  |
  */
pub fn color_black() -> CppBox<QColor> { unsafe { QColor::from_3_int(0, 0, 0) } }

/**
  | Tooltips longer than this (in characters)
  | are converted into rich text, so that
  | they can be word-wrapped.
  |
  */
pub const TOOLTIP_WRAP_THRESHOLD: i32 = 80;

/**
  | Number of frames in spinner animation
  |
  */
pub const SPINNER_FRAMES: usize = 36;

pub const QAPP_ORG_NAME:         &'static str = "Bitcoin";
pub const QAPP_ORG_DOMAIN:       &'static str = "bitcoin.org";
pub const QAPP_APP_NAME_DEFAULT: &'static str = "Bitcoin-Qt";
pub const QAPP_APP_NAME_TESTNET: &'static str = "Bitcoin-Qt-testnet";
pub const QAPP_APP_NAME_SIGNET:  &'static str = "Bitcoin-Qt-signet";
pub const QAPP_APP_NAME_REGTEST: &'static str = "Bitcoin-Qt-regtest";

/**
  | One gigabyte (GB) in bytes
  |
  */
pub const GB_BYTES: u64 = 1000000000;

/**
  | Default prune target displayed in GUI.
  |
  */
pub const DEFAULT_PRUNE_TARGET_GB: i32 = 2;
