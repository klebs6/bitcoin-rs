// ---------------- [ File: bitcoinleveldb-repair/src/table_info.rs ]
crate::ix!();

#[derive(Default, Getters, MutGetters)]
#[getset(get="pub", get_mut="pub(crate)")]
pub struct RepairerTableInfo {
    meta:         FileMetaData,
    max_sequence: SequenceNumber,
}

#[cfg(test)]
mod repairer_table_info_suite {
    use super::*;
    use tracing::{debug, info, trace, warn};

    #[traced_test]
    fn repairer_table_info_stores_file_metadata_and_sequence() {
        let mut info = RepairerTableInfo {
            meta: FileMetaData::default(),
            max_sequence: 0,
        };

        info.meta.set_number(7);
        info.meta.set_file_size(1234);
        info.max_sequence = 99;

        trace!(
            table_no = *info.meta.number(),
            file_size = *info.meta.file_size(),
            max_seq = info.max_sequence,
            "constructed RepairerTableInfo"
        );

        assert_eq!(*info.meta.number(), 7);
        assert_eq!(*info.meta.file_size(), 1234);
        assert_eq!(info.max_sequence, 99);
    }
}
