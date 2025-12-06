crate::ix!();

pub trait CompactRange {

    /**
      | Compact the underlying storage for the key
      | range [*begin,*end].  In particular, deleted
      | and overwritten versions are discarded, and
      | the data is rearranged to reduce the cost of
      | operations needed to access the data.  This
      | operation should typically only be invoked by
      | users who understand the underlying
      | implementation.
      |
      | begin==nullptr is treated as a key before all
      | keys in the database.  end==nullptr is
      | treated as a key after all keys in the
      | database.  Therefore the following call will
      | compact the entire database:
      | db->CompactRange(nullptr, nullptr);
      */
    fn compact_range(&mut self, 
            begin: *const Slice,
            end:   *const Slice);
}
