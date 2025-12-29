// ---------------- [ File: bitcoinleveldb-batch/src/write_batch.rs ]
crate::ix!();

/**
  | WriteBatch header has an 8-byte sequence
  | number followed by a 4-byte count.
  |
  */
pub const HEADER: usize = 12;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/write_batch.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/write_batch.cc]

/**
  | WriteBatch holds a collection of updates to
  | apply atomically to a DB.
  |
  | The updates are applied in the order in which
  | they are added to the WriteBatch.  For example,
  | the value of "key" will be "v3" after the
  | following batch is written:
  |
  |    batch.Put("key", "v1");
  |    batch.Delete("key");
  |    batch.Put("key", "v2");
  |    batch.Put("key", "v3");
  |
  | Multiple threads can invoke const methods on
  | a WriteBatch without external synchronization,
  | but if any of the threads may call a non-const
  | method, all threads accessing the same
  | WriteBatch must use external synchronization.
  ---------------------------
  | WriteBatch::rep_ :=
  |    sequence: fixed64
  |    count: fixed32
  |    data: record[count]
  | record :=
  |    kTypeValue varstring varstring         |
  |    kTypeDeletion varstring
  | varstring :=
  |    len: varint32
  |    data: uint8[len]
  */
#[derive(Setters,Getters,MutGetters)]
#[getset(set="pub",get="pub",get_mut="pub")]
pub struct WriteBatch {

    /**
      | See comment in write_batch.cc for the
      | format of rep_
      |
      */
    rep: String,
}

impl WriteBatch {

    pub fn new() -> Self {

        trace!("WriteBatch::new");

        let mut batch = WriteBatch {
            rep: String::new(),
        };
        batch.clear();
        batch
    }

    /**
      | Clear all updates buffered in this batch.
      |
      */
    pub fn clear(&mut self)  {

        trace!(
            "WriteBatch::clear: rep_len_before={} rep_cap_before={}",
            self.rep().len(),
            self.rep().capacity()
        );

        self.rep_mut().clear();
        unsafe {
            let v: &mut Vec<u8> = self.rep_mut().as_mut_vec();
            v.resize(HEADER, 0u8);
        }

        trace!(
            "WriteBatch::clear: rep_len_after={} rep_cap_after={}",
            self.rep().len(),
            self.rep().capacity()
        );
    }

    /**
      | The size of the database changes caused by
      | this batch.
      |
      | This number is tied to implementation
      | details, and may change across releases. It
      | is intended for LevelDB usage metrics.
      */
    pub fn approximate_size(&self) -> usize {

        trace!(
            "WriteBatch::approximate_size: rep_len={}",
            self.rep().len()
        );

        self.rep().len()
    }

    /**
      | Copies the operations in "source" to this
      | batch.
      |
      | This runs in O(source size) time. However,
      | the constant factor is better than calling
      | Iterate() over the source batch with
      | a Handler that replicates the operations into
      | this batch.
      */
    pub fn append(&mut self, source: &WriteBatch)  {

        trace!(
            "WriteBatch::append: dst_len={} src_len={}",
            self.rep().len(),
            source.rep.len()
        );

        write_batch_internal::append(self as *mut WriteBatch, source as *const WriteBatch);

        trace!(
            "WriteBatch::append: dst_len_after={} new_count={}",
            self.rep().len(),
            write_batch_internal::count(self as *const WriteBatch)
        );
    }
}

#[cfg(test)]
mod write_batch_rs_exhaustive_contract_suite {
    use super::*;
    use crate::write_batch_test_harness_utilities::*;

    #[traced_test]
    fn new_produces_zeroed_header_with_zero_count_and_size_equals_header() {
        trace!("new_produces_zeroed_header_with_zero_count_and_size_equals_header: begin");

        let batch = WriteBatch::new();
        assert_eq!(
            0,
            write_batch_internal::sequence(&batch as *const WriteBatch)
        );
        assert_eq!(0, write_batch_internal::count(&batch as *const WriteBatch));
        assert_eq!(HEADER, batch.approximate_size());

        trace!("new_produces_zeroed_header_with_zero_count_and_size_equals_header: end");
    }

    #[traced_test]
    fn clear_removes_all_records_and_resets_header_fields() {
        trace!("clear_removes_all_records_and_resets_header_fields: begin");

        let mut batch = WriteBatch::new();
        batch.put(&Slice::from("a"), &Slice::from("va"));
        batch.delete(&Slice::from("b"));
        write_batch_internal::set_sequence(&mut batch as *mut WriteBatch, 123);

        assert!(batch.approximate_size() > HEADER);
        assert_eq!(2, write_batch_internal::count(&batch as *const WriteBatch));
        assert_eq!(
            123,
            write_batch_internal::sequence(&batch as *const WriteBatch)
        );

        batch.clear();

        assert_eq!(HEADER, batch.approximate_size());
        assert_eq!(0, write_batch_internal::count(&batch as *const WriteBatch));
        assert_eq!(
            0,
            write_batch_internal::sequence(&batch as *const WriteBatch)
        );

        trace!("clear_removes_all_records_and_resets_header_fields: end");
    }

    #[traced_test]
    fn approximate_size_matches_internal_byte_size_for_all_states() {
        trace!("approximate_size_matches_internal_byte_size_for_all_states: begin");

        let mut batch = WriteBatch::new();
        assert_eq!(
            batch.approximate_size(),
            write_batch_internal::byte_size(&batch as *const WriteBatch)
        );

        batch.put(&Slice::from("foo"), &Slice::from("bar"));
        assert_eq!(
            batch.approximate_size(),
            write_batch_internal::byte_size(&batch as *const WriteBatch)
        );

        batch.delete(&Slice::from("box"));
        assert_eq!(
            batch.approximate_size(),
            write_batch_internal::byte_size(&batch as *const WriteBatch)
        );

        trace!("approximate_size_matches_internal_byte_size_for_all_states: end");
    }

    #[traced_test]
    fn append_noop_when_source_is_empty_and_does_not_change_count_or_state() {
        trace!(
            "append_noop_when_source_is_empty_and_does_not_change_count_or_state: begin"
        );

        let mut dst = WriteBatch::new();
        let src = WriteBatch::new();

        dst.put(&Slice::from("a"), &Slice::from("va"));
        write_batch_internal::set_sequence(&mut dst as *mut WriteBatch, 50);

        let before_count = write_batch_internal::count(&dst as *const WriteBatch);
        let before_state = format_memtable_state_for_batch(&mut dst as *mut WriteBatch);

        dst.append(&src);

        let after_count = write_batch_internal::count(&dst as *const WriteBatch);
        let after_state = format_memtable_state_for_batch(&mut dst as *mut WriteBatch);

        assert_eq!(before_count, after_count);
        assert_eq!(before_state, after_state);

        trace!(
            "append_noop_when_source_is_empty_and_does_not_change_count_or_state: end"
        );
    }

    #[traced_test]
    fn append_combines_records_and_insertion_sequence_starts_at_destination_header_sequence() {
        trace!(
            "append_combines_records_and_insertion_sequence_starts_at_destination_header_sequence: begin"
        );

        let mut dst = WriteBatch::new();
        let mut src = WriteBatch::new();

        write_batch_internal::set_sequence(&mut dst as *mut WriteBatch, 200);
        write_batch_internal::set_sequence(&mut src as *mut WriteBatch, 999);

        dst.put(&Slice::from("foo"), &Slice::from("bar"));
        src.put(&Slice::from("baz"), &Slice::from("boo"));

        dst.append(&src);

        assert_eq!(
            2,
            write_batch_internal::count(&dst as *const WriteBatch)
        );
        assert_eq!(
            200,
            write_batch_internal::sequence(&dst as *const WriteBatch)
        );

        // Record order in `dst` is: Put(foo) then Put(baz).
        // Sequences assigned by MemTableInserter are: foo@200, baz@201.
        // Iteration order is by user key: baz then foo.
        let state = format_memtable_state_for_batch(&mut dst as *mut WriteBatch);
        assert_eq!(state, "Put(baz, boo)@201Put(foo, bar)@200");

        trace!(
            "append_combines_records_and_insertion_sequence_starts_at_destination_header_sequence: end"
        );
    }
}
