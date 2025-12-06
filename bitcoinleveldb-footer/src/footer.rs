// ---------------- [ File: bitcoinleveldb-footer/src/footer.rs ]
crate::ix!();

/**
  | Footer encapsulates the fixed information
  | stored at the tail end of every table
  | file.
  |
  */
#[derive(Default,Getters,MutGetters,Setters)]
#[getset(get="pub",set="pub",get_mut="pub")]
pub struct Footer {
    metaindex_handle: BlockHandle,
    index_handle:     BlockHandle,
}

/**
  | Note:
  | 
  | The serialization of a Footer will always
  | occupy exactly this many bytes. It consists
  | of two block handles and a magic number.
  |
  */
pub const FOOTER_ENCODED_LENGTH: usize = 2 * BLOCK_HANDLE_MAX_ENCODED_LENGTH + 8;

#[cfg(test)]
mod footer_struct_accessors_and_default_state_tests {
    use super::*;

    fn make_block_handle_for_footer_tests(offset: u64, size: u64) -> BlockHandle {
        let mut h = BlockHandle::default();
        h.set_offset(offset);
        h.set_size(size);
        h
    }

    #[traced_test]
    fn footer_default_initializes_block_handles_to_zero() {
        let footer = Footer::default();

        assert_eq!(
            footer.metaindex_handle().offset(),
            0,
            "footer_default_initializes_block_handles_to_zero: default metaindex offset must be 0"
        );
        assert_eq!(
            footer.metaindex_handle().size(),
            0,
            "footer_default_initializes_block_handles_to_zero: default metaindex size must be 0"
        );
        assert_eq!(
            footer.index_handle().offset(),
            0,
            "footer_default_initializes_block_handles_to_zero: default index offset must be 0"
        );
        assert_eq!(
            footer.index_handle().size(),
            0,
            "footer_default_initializes_block_handles_to_zero: default index size must be 0"
        );
    }

    #[traced_test]
    fn footer_setting_metaindex_handle_does_not_affect_index_handle() {
        let metaindex = make_block_handle_for_footer_tests(100, 200);
        let mut footer = Footer::default();

        footer.set_metaindex_handle(metaindex);

        assert_eq!(
            footer.metaindex_handle().offset(),
            metaindex.offset(),
            "footer_setting_metaindex_handle_does_not_affect_index_handle: metaindex offset must match"
        );
        assert_eq!(
            footer.metaindex_handle().size(),
            metaindex.size(),
            "footer_setting_metaindex_handle_does_not_affect_index_handle: metaindex size must match"
        );

        assert_eq!(
            footer.index_handle().offset(),
            0,
            "footer_setting_metaindex_handle_does_not_affect_index_handle: index offset must remain default 0"
        );
        assert_eq!(
            footer.index_handle().size(),
            0,
            "footer_setting_metaindex_handle_does_not_affect_index_handle: index size must remain default 0"
        );
    }

    #[traced_test]
    fn footer_setting_index_handle_does_not_affect_metaindex_handle() {
        let index = make_block_handle_for_footer_tests(300, 400);
        let mut footer = Footer::default();

        footer.set_index_handle(index);

        assert_eq!(
            footer.index_handle().offset(),
            index.offset(),
            "footer_setting_index_handle_does_not_affect_metaindex_handle: index offset must match"
        );
        assert_eq!(
            footer.index_handle().size(),
            index.size(),
            "footer_setting_index_handle_does_not_affect_metaindex_handle: index size must match"
        );

        assert_eq!(
            footer.metaindex_handle().offset(),
            0,
            "footer_setting_index_handle_does_not_affect_metaindex_handle: metaindex offset must remain default 0"
        );
        assert_eq!(
            footer.metaindex_handle().size(),
            0,
            "footer_setting_index_handle_does_not_affect_metaindex_handle: metaindex size must remain default 0"
        );
    }

    #[traced_test]
    fn footer_setting_both_handles_roundtrip_through_accessors() {
        let metaindex = make_block_handle_for_footer_tests(1111, 2222);
        let index     = make_block_handle_for_footer_tests(3333, 4444);

        let mut footer = Footer::default();
        footer.set_metaindex_handle(metaindex);
        footer.set_index_handle(index);

        assert_eq!(
            footer.metaindex_handle().offset(),
            metaindex.offset()
        );
        assert_eq!(
            footer.metaindex_handle().size(),
            metaindex.size()
        );
        assert_eq!(
            footer.index_handle().offset(),
            index.offset()
        );
        assert_eq!(
            footer.index_handle().size(),
            index.size()
        );
    }
}
