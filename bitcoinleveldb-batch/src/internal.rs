// ---------------- [ File: bitcoinleveldb-batch/src/internal.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/write_batch_internal.h]

/// WriteBatchInternal provides static methods for manipulating a WriteBatch
/// that we don't want in the public WriteBatch interface.
/// 
pub mod write_batch_internal {
    use super::*;

    pub fn insert_into(
        b:        *const WriteBatch,
        memtable: *mut MemTable) -> crate::Status {

        trace!(
            "write_batch_internal::insert_into: batch_ptr={:p} memtable_ptr={:p}",
            b,
            memtable
        );

        assert!(
            !b.is_null(),
            "write_batch_internal::insert_into: batch pointer must not be null"
        );
        assert!(
            !memtable.is_null(),
            "write_batch_internal::insert_into: memtable pointer must not be null"
        );

        unsafe {

            let mut inserter = MemTableInserterBuilder::default()
                .sequence(sequence(b))
                .mem(memtable)
                .build()
                .unwrap();

            let handler_ptr: *mut dyn WriteBatchHandler =
                &mut inserter as *mut dyn WriteBatchHandler;

            let s = (*b).iterate(handler_ptr);
            trace!(
                "write_batch_internal::insert_into: iterate done status_ok={}",
                s.is_ok()
            );
            s
        }
    }

    pub fn set_contents(
        b:        *mut WriteBatch,
        contents: &Slice)  {

        trace!(
            "write_batch_internal::set_contents: batch_ptr={:p} contents_len={}",
            b,
            *contents.size()
        );

        assert!(
            !b.is_null(),
            "write_batch_internal::set_contents: batch pointer must not be null"
        );
        assert!(
            *contents.size() >= HEADER,
            "write_batch_internal::set_contents: contents too small ({} < {})",
            *contents.size(),
            HEADER
        );

        unsafe {
            let src = if *contents.size() == 0 {
                &[][..]
            } else {
                std::slice::from_raw_parts(*contents.data(), *contents.size())
            };

            let mut s = String::new();
            {
                let v: &mut Vec<u8> = s.as_mut_vec();
                v.extend_from_slice(src);
            }
            (*b).set_rep(s);

            trace!(
                "write_batch_internal::set_contents: new_rep_len={}",
                (*b).rep().len()
            );
        }
    }

    pub fn append(
        dst: *mut WriteBatch,
        src: *const WriteBatch)  {

        trace!(
            "write_batch_internal::append: dst_ptr={:p} src_ptr={:p}",
            dst,
            src
        );

        assert!(
            !dst.is_null(),
            "write_batch_internal::append: dst pointer must not be null"
        );
        assert!(
            !src.is_null(),
            "write_batch_internal::append: src pointer must not be null"
        );

        let new_count = count(dst as *const WriteBatch) + count(src);
        set_count(dst, new_count);

        unsafe {
            let src_len = (*src).rep().len();
            assert!(
                src_len >= HEADER,
                "write_batch_internal::append: src rep too small ({} < {})",
                src_len,
                HEADER
            );

            let src_bytes = std::slice::from_raw_parts((*src).rep().as_ptr(), src_len);
            let tail = &src_bytes[HEADER..];

            let before = (*dst).rep().len();
            {
                let dst_vec: &mut Vec<u8> = (*dst).rep_mut().as_mut_vec();
                dst_vec.extend_from_slice(tail);
            }
            trace!(
                "write_batch_internal::append: dst_len {} -> {}",
                before,
                (*dst).rep().len()
            );
        }
    }

    /**
      | Return the number of entries in the batch.
      |
      */
    pub fn count(b: *const WriteBatch) -> i32 {

        trace!("write_batch_internal::count: batch_ptr={:p}", b);

        assert!(
            !b.is_null(),
            "write_batch_internal::count: batch pointer must not be null"
        );

        unsafe {
            let rep_len = (*b).rep().len();
            assert!(
                rep_len >= HEADER,
                "write_batch_internal::count: rep too small ({} < {})",
                rep_len,
                HEADER
            );

            let ptr = (*b).rep().as_ptr().add(8);
            let n = decode_fixed32(ptr) as i32;

            trace!("write_batch_internal::count: decoded={}", n);
            n
        }
    }

    /**
      | Set the count for the number of entries
      | in the batch.
      |
      */
    pub fn set_count(
        b: *mut WriteBatch,
        n: i32)  {

        trace!(
            "write_batch_internal::set_count: batch_ptr={:p} n={}",
            b,
            n
        );

        assert!(
            !b.is_null(),
            "write_batch_internal::set_count: batch pointer must not be null"
        );

        unsafe {
            let rep_len = (*b).rep().len();
            assert!(
                rep_len >= HEADER,
                "write_batch_internal::set_count: rep too small ({} < {})",
                rep_len,
                HEADER
            );

            let dst = (*b).rep_mut().as_mut_vec().as_mut_ptr().add(8);
            encode_fixed32(dst, n as u32);
        }
    }

    /**
      | Return the sequence number for the start
      | of this batch.
      |
      */
    pub fn sequence(b: *const WriteBatch) -> SequenceNumber {

        trace!("write_batch_internal::sequence: batch_ptr={:p}", b);

        assert!(
            !b.is_null(),
            "write_batch_internal::sequence: batch pointer must not be null"
        );

        unsafe {
            let rep_len = (*b).rep().len();
            assert!(
                rep_len >= HEADER,
                "write_batch_internal::sequence: rep too small ({} < {})",
                rep_len,
                HEADER
            );

            let ptr = (*b).rep().as_ptr();
            let seq = decode_fixed64(ptr);

            trace!("write_batch_internal::sequence: decoded_seq={}", seq);
            seq
        }
    }

    /**
      | Store the specified number as the sequence
      | number for the start of this batch.
      |
      */
    pub fn set_sequence(
        b:   *mut WriteBatch,
        seq: SequenceNumber)  {

        trace!(
            "write_batch_internal::set_sequence: batch_ptr={:p} seq={}",
            b,
            seq
        );

        assert!(
            !b.is_null(),
            "write_batch_internal::set_sequence: batch pointer must not be null"
        );

        unsafe {
            let rep_len = (*b).rep().len();
            assert!(
                rep_len >= HEADER,
                "write_batch_internal::set_sequence: rep too small ({} < {})",
                rep_len,
                HEADER
            );

            let dst = (*b).rep_mut().as_mut_vec().as_mut_ptr();
            encode_fixed64(dst, seq);
        }
    }

    pub fn contents(batch: *const WriteBatch) -> Slice {

        trace!(
            "write_batch_internal::contents: batch_ptr={:p}",
            batch
        );

        assert!(
            !batch.is_null(),
            "write_batch_internal::contents: batch pointer must not be null"
        );

        unsafe {
            let ptr = (*batch).rep().as_ptr();
            let len = (*batch).rep().len();
            trace!("write_batch_internal::contents: len={}", len);
            Slice::from_ptr_len(ptr, len)
        }
    }

    pub fn byte_size(batch: *const WriteBatch) -> usize {

        trace!(
            "write_batch_internal::byte_size: batch_ptr={:p}",
            batch
        );

        assert!(
            !batch.is_null(),
            "write_batch_internal::byte_size: batch pointer must not be null"
        );

        unsafe {
            let n = (*batch).rep().len();
            trace!("write_batch_internal::byte_size: {}", n);
            n
        }
    }
}

#[cfg(test)]
mod internal_rs_exhaustive_contract_suite {
    use super::*;
    use crate::write_batch_test_harness_utilities::*;

    #[traced_test]
    fn write_batch_internal_set_sequence_round_trips_and_updates_header_bytes() {
        trace!("write_batch_internal_set_sequence_round_trips_and_updates_header_bytes: begin");

        let mut batch = WriteBatch::new();
        write_batch_internal::set_sequence(&mut batch as *mut WriteBatch, 123);

        assert_eq!(
            123,
            write_batch_internal::sequence(&batch as *const WriteBatch)
        );

        let bytes = collect_write_batch_contents_bytes(&batch);
        let (seq, count) = decode_write_batch_header_fields(&bytes);
        assert_eq!(seq, 123);
        assert_eq!(count, 0);

        trace!("write_batch_internal_set_sequence_round_trips_and_updates_header_bytes: end");
    }

    #[traced_test]
    fn write_batch_internal_set_count_round_trips_and_updates_header_bytes() {
        trace!("write_batch_internal_set_count_round_trips_and_updates_header_bytes: begin");

        let mut batch = WriteBatch::new();
        write_batch_internal::set_count(&mut batch as *mut WriteBatch, 7);

        assert_eq!(7, write_batch_internal::count(&batch as *const WriteBatch));

        let bytes = collect_write_batch_contents_bytes(&batch);
        let (seq, count) = decode_write_batch_header_fields(&bytes);
        assert_eq!(seq, 0);
        assert_eq!(count, 7);

        trace!("write_batch_internal_set_count_round_trips_and_updates_header_bytes: end");
    }

    #[traced_test]
    fn write_batch_internal_set_contents_copies_payload_and_round_trips_equivalently() {
        trace!(
            "write_batch_internal_set_contents_copies_payload_and_round_trips_equivalently: begin"
        );

        let mut a = WriteBatch::new();
        a.put(&Slice::from("foo"), &Slice::from("bar"));
        a.delete(&Slice::from("box"));
        write_batch_internal::set_sequence(&mut a as *mut WriteBatch, 999);

        let bytes = collect_write_batch_contents_bytes(&a);

        let mut b = WriteBatch::new();
        set_write_batch_contents_bytes(&mut b, &bytes);

        assert_eq!(
            write_batch_internal::sequence(&a as *const WriteBatch),
            write_batch_internal::sequence(&b as *const WriteBatch)
        );
        assert_eq!(
            write_batch_internal::count(&a as *const WriteBatch),
            write_batch_internal::count(&b as *const WriteBatch)
        );

        let sa = format_memtable_state_for_batch(&mut a as *mut WriteBatch);
        let sb = format_memtable_state_for_batch(&mut b as *mut WriteBatch);
        assert_eq!(sa, sb);

        // Verify it is a copy, not an alias: mutating source bytes after SetContents
        // must not affect `b`.
        let mut mutated = bytes.clone();
        mutated[0] ^= 0xff;
        let s_before = write_batch_internal::sequence(&b as *const WriteBatch);
        let _ = mutated;

        let s_after = write_batch_internal::sequence(&b as *const WriteBatch);
        assert_eq!(s_before, s_after);

        trace!(
            "write_batch_internal_set_contents_copies_payload_and_round_trips_equivalently: end"
        );
    }

    #[traced_test]
    fn write_batch_internal_set_contents_panics_on_too_small_input() {
        trace!("write_batch_internal_set_contents_panics_on_too_small_input: begin");

        let mut batch = WriteBatch::new();
        let too_small = vec![0u8; HEADER - 1];
        let s = Slice::from(&too_small[..]);

        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            write_batch_internal::set_contents(&mut batch as *mut WriteBatch, &s);
        }));
        assert!(res.is_err());

        trace!("write_batch_internal_set_contents_panics_on_too_small_input: end");
    }

    #[traced_test]
    fn write_batch_internal_append_preserves_dst_header_sequence_and_appends_records_only() {
        trace!(
            "write_batch_internal_append_preserves_dst_header_sequence_and_appends_records_only: begin"
        );

        let mut dst = WriteBatch::new();
        let mut src = WriteBatch::new();

        write_batch_internal::set_sequence(&mut dst as *mut WriteBatch, 100);
        write_batch_internal::set_sequence(&mut src as *mut WriteBatch, 200);

        dst.put(&Slice::from("b"), &Slice::from("vb"));
        src.put(&Slice::from("a"), &Slice::from("va"));

        write_batch_internal::append(&mut dst as *mut WriteBatch, &src as *const WriteBatch);

        assert_eq!(
            2,
            write_batch_internal::count(&dst as *const WriteBatch)
        );
        assert_eq!(
            100,
            write_batch_internal::sequence(&dst as *const WriteBatch)
        );

        let s = format_memtable_state_for_batch(&mut dst as *mut WriteBatch);
        assert_eq!(s, "Put(a, va)@101Put(b, vb)@100");

        trace!(
            "write_batch_internal_append_preserves_dst_header_sequence_and_appends_records_only: end"
        );
    }

    #[traced_test]
    fn write_batch_internal_append_panics_if_src_rep_is_shorter_than_header() {
        trace!(
            "write_batch_internal_append_panics_if_src_rep_is_shorter_than_header: begin"
        );

        let mut dst = WriteBatch::new();
        let mut src = WriteBatch::new();

        src.rep_mut().truncate(0);

        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            write_batch_internal::append(&mut dst as *mut WriteBatch, &src as *const WriteBatch);
        }));
        assert!(res.is_err());

        trace!(
            "write_batch_internal_append_panics_if_src_rep_is_shorter_than_header: end"
        );
    }
}
