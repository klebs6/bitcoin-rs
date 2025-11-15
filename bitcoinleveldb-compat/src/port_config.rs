// ---------------- [ File: bitcoinleveldb-compat/src/port_config.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/port/port_config.h.in]

/**
  | Define to true if the platform exposes
  | `fdatasync()` (typically via `<unistd.h>` or
  | an equivalent API).
  |
  | Controlled by the `have_fdatasync` feature.
  */
pub const HAVE_FDATASYNC: bool = cfg!(feature = "have_fdatasync");

/**
  | Define to true if the platform exposes
  | `F_FULLFSYNC` (or an equivalent full‑fsync
  | primitive).
  |
  | Controlled by the `have_fullfsync` feature.
  */
pub const HAVE_FULLFSYNC: bool = cfg!(feature = "have_fullfsync");

/**
  | Define to true if the platform supports
  | `O_CLOEXEC` when opening files.
  |
  | Controlled by the `have_o_cloexec` feature.
  */
pub const HAVE_O_CLOEXEC: bool = cfg!(feature = "have_o_cloexec");

/**
  | Define to true when CRC32C acceleration is
  | available.
  |
  | Controlled by the `have_crc32c` feature.
  */
pub const HAVE_CRC32C: bool = cfg!(feature = "have_crc32c");

/**
  | Define to true when Snappy compression is
  | available.
  |
  | Controlled by the `have_snappy` feature.
  */
pub const HAVE_SNAPPY: bool = cfg!(feature = "have_snappy");

/**
  | True on big‑endian targets, or when the
  | `leveldb_is_big_endian` feature is enabled.
  |
  | This mirrors the original C++ autoconf
  | macro: “1 if the processor stores words
  | with the most significant byte first”.
  */
pub const LEVELDB_IS_BIG_ENDIAN: bool =
    cfg!(feature = "leveldb_is_big_endian") || cfg!(target_endian = "big");

#[cfg(test)]
mod port_config_feature_spec {
    use super::*;

    #[traced_test]
    fn endianness_flags_are_self_consistent() {
        let big = LEVELDB_IS_BIG_ENDIAN;
        let little = crate::port::LITTLE_ENDIAN;
        trace!(big, little, "endianness configuration check");
        assert_eq!(little, !big);
    }

    #[traced_test]
    fn fdatasync_flag_tracks_feature() {
        trace!(have_fdatasync = HAVE_FDATASYNC);
        #[cfg(feature = "have_fdatasync")]
        {
            assert!(HAVE_FDATASYNC);
        }
        #[cfg(not(feature = "have_fdatasync"))]
        {
            assert!(!HAVE_FDATASYNC);
        }
    }

    #[traced_test]
    fn fullfsync_flag_tracks_feature() {
        trace!(have_fullfsync = HAVE_FULLFSYNC);
        #[cfg(feature = "have_fullfsync")]
        {
            assert!(HAVE_FULLFSYNC);
        }
        #[cfg(not(feature = "have_fullfsync"))]
        {
            assert!(!HAVE_FULLFSYNC);
        }
    }

    #[traced_test]
    fn o_cloexec_flag_tracks_feature() {
        trace!(have_o_cloexec = HAVE_O_CLOEXEC);
        #[cfg(feature = "have_o_cloexec")]
        {
            assert!(HAVE_O_CLOEXEC);
        }
        #[cfg(not(feature = "have_o_cloexec"))]
        {
            assert!(!HAVE_O_CLOEXEC);
        }
    }

    #[traced_test]
    fn crc32c_flag_tracks_feature() {
        trace!(have_crc32c = HAVE_CRC32C);
        #[cfg(feature = "have_crc32c")]
        {
            assert!(HAVE_CRC32C);
        }
        #[cfg(not(feature = "have_crc32c"))]
        {
            assert!(!HAVE_CRC32C);
        }
    }

    #[traced_test]
    fn snappy_flag_tracks_feature() {
        trace!(have_snappy = HAVE_SNAPPY);
        #[cfg(feature = "have_snappy")]
        {
            assert!(HAVE_SNAPPY);
        }
        #[cfg(not(feature = "have_snappy"))]
        {
            assert!(!HAVE_SNAPPY);
        }
    }
}
