// ---------------- [ File: bitcoinleveldb-options/src/compression_type.rs ]
crate::ix!();

/// DB contents are stored in a set of blocks, each of which holds a sequence of
/// key,value pairs.
///
/// Each block may be compressed before being stored in a file.  The following
/// enum describes which compression method (if any) is used to compress
/// a block.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompressionType {

    /// @note
    /// 
    /// do not change the values of existing entries, as these are part of the
    /// persistent format on disk.
    /// 
    None   = 0x0,
    Snappy = 0x1
}

#[cfg(test)]
mod compression_type_persistent_encoding_suite {
    use super::*;
    use tracing::{debug, info, trace};

    #[traced_test]
    fn compression_type_discriminants_match_on_disk_format() {
        trace!("compression_type_persistent_encoding_suite: start");

        let none_v = CompressionType::None as u8;
        let snappy_v = CompressionType::Snappy as u8;

        info!(none_v, snappy_v, "compression type discriminants");

        assert_eq!(none_v, 0x0);
        assert_eq!(snappy_v, 0x1);

        trace!("compression_type_persistent_encoding_suite: done");
    }

    #[traced_test]
    fn compression_type_is_copy_and_debug_is_stable() {
        trace!("compression_type_persistent_encoding_suite: start");

        let a = CompressionType::Snappy;
        let b = a;

        assert_eq!(a, b);

        let dbg = format!("{:?}", a);
        debug!(dbg = %dbg, "debug formatting");

        assert!(dbg.contains("Snappy"));

        trace!("compression_type_persistent_encoding_suite: done");
    }
}
