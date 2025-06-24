// ---------------- [ File: bitcoin-golombrice/src/config.rs ]
crate::ix!();

/**
  | SerType used to serialize parameters
  | in GCS filter encoding.
  |
  */
pub const GCS_SER_TYPE: usize = SER_NETWORK as usize;

/**
  | Protocol version used to serialize
  | parameters in GCS filter encoding.
  |
  */
pub const GCS_SER_VERSION: usize = 0;
