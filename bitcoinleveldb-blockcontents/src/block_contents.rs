// ---------------- [ File: bitcoinleveldb-blockcontents/src/block_contents.rs ]
crate::ix!();

#[derive(Setters,Getters,MutGetters)]
#[getset(get="pub",set="pub",get_mut="pub")]
pub struct BlockContents {

    /**
      | Actual contents of data
      |
      */
    data:           Slice,

    /**
      | True iff data can be cached
      |
      */
    cachable:       bool,

    /**
      | True iff caller should delete[] data.data()
      |
      */
    heap_allocated: bool,
}

impl Default for BlockContents {
    fn default() -> Self {
        let data = Slice::default();

        trace!(
            "BlockContents::default: creating empty non-cached, non-heap-allocated block (len={})",
            *data.size()
        );

        BlockContents {
            data,
            cachable:       false,
            heap_allocated: false,
        }
    }
}

impl BlockContents {

    pub fn new(data: Slice, cachable: bool, heap_allocated: bool) -> Self {
        trace!(
            "BlockContents::new: data_len={}, cachable={}, heap_allocated={}",
            *data.size(),
            cachable,
            heap_allocated
        );

        BlockContents {
            data,
            cachable,
            heap_allocated,
        }
    }
}

#[cfg(test)]
mod block_contents_basic_tests {
    use super::*;

    #[traced_test]
    fn block_contents_new_preserves_flags_and_data() {
        let backing = String::from("block-bytes");
        let slice   = Slice::from(backing.as_bytes());
        let len     = *slice.size();

        let contents = BlockContents::new(slice, true, false);

        trace!("constructed BlockContents in test with len={}", len);
        assert_eq!(*contents.data().size(), len);
        assert!(contents.cachable());
        assert!(!contents.heap_allocated());
    }

    #[traced_test]
    fn block_contents_data_mut_allows_in_place_update() {
        let backing = String::from("abc");
        let slice   = Slice::from(backing.as_bytes());
        let mut contents = BlockContents::new(slice, false, false);

        {
            let data_mut = contents.data_mut();
            let len      = *data_mut.size();
            debug!("before mutation, len={}", len);
            assert_eq!(len, 3);
        }

        let len_after = *contents.data().size();
        assert_eq!(len_after, 3);
    }
}
