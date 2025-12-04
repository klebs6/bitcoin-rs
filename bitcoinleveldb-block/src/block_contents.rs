// ---------------- [ File: bitcoinleveldb-block/src/block_contents.rs ]
crate::ix!();

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

impl BlockContents {

    pub fn data(&self) -> &Slice {
        trace!("BlockContents::data called (len={})", *self.data.size());
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Slice {
        trace!(
            "BlockContents::data_mut called (len before={})",
            *self.data.size()
        );
        &mut self.data
    }

    pub fn is_cachable(&self) -> bool {
        trace!(
            "BlockContents::is_cachable called => {}",
            self.cachable
        );
        self.cachable
    }

    pub fn is_heap_allocated(&self) -> bool {
        trace!(
            "BlockContents::is_heap_allocated called => {}",
            self.heap_allocated
        );
        self.heap_allocated
    }

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
        assert!(contents.is_cachable());
        assert!(!contents.is_heap_allocated());
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
