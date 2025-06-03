// ---------------- [ File: bitcoinleveldb-filter/src/filter_block_test.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/filter_block_test.cc]

#[cfg(test)]
mod test_filter_block {
    use super::*;

    // A simple test filter: "one 32-bit hash per key"
    #[derive(Default)]
    pub struct TestHashFilter;

    // Now implement all subtraits:
    impl Name for TestHashFilter {
        fn name(&self) -> *const u8 {
            static NAME: &str = "TestHashFilter";
            NAME.as_ptr()
        }
    }

    impl CreateFilter for TestHashFilter {
        fn create_filter(&self, keys: *const Slice, n: i32, dst: &mut Vec<u8>) {
            for i in 0..n {
                let sl = unsafe { &*keys.add(i as usize) };
                let h = test_hash(*sl.data() as *const u8, *sl.size(), 1);
                put_fixed32(dst, h);
            }
        }
    }

    impl KeyMayMatch for TestHashFilter {
        fn key_may_match(&self, key: &Slice, filter: &Slice) -> bool {
            let h = test_hash(*key.data() as *const u8, *key.size(), 1);
            let data = unsafe {
                std::slice::from_raw_parts(*filter.data() as *const u8, *filter.size())
            };
            let mut idx = 0;
            while idx + 4 <= data.len() {
                let v = decode_fixed32(&data[idx..idx + 4]);
                if v == h {
                    return true;
                }
                idx += 4;
            }
            false
        }
    }

    // Finally, implement the supertrait FilterPolicy
    impl FilterPolicy for TestHashFilter {}

    /// A simple hash function. 
    fn test_hash(data: *const u8, n: usize, seed: u32) -> u32 {
        // For brevity, a small sum-hash:
        let bytes = unsafe { std::slice::from_raw_parts(data, n) };
        let mut hash_val = seed;
        for &b in bytes {
            hash_val = hash_val.wrapping_mul(16777619) ^ (b as u32);
        }
        hash_val
    }

    fn escape_string(slice: &Slice) -> String {
        let data = unsafe {
            std::slice::from_raw_parts(*slice.data() as *const u8, *slice.size())
        };
        let mut out = String::new();
        for &b in data {
            if (b.is_ascii_graphic() || b == b' ') {
                out.push(b as char);
            } else {
                out.push_str(&format!("\\x{:02x}", b));
            }
        }
        out
    }

    // A small struct grouping everything
    #[derive(Default)]
    pub struct FilterBlockTest {
        policy: TestHashFilter,
    }

    #[traced_test]
    fn filter_block_test_empty_builder() {
        let test = FilterBlockTest::default();
        let mut builder = FilterBlockBuilder::new(Box::new(test.policy));
        let block = builder.finish();
        let escaped = escape_string(&block);
        // Expect \x00\x00\x00\x00\x0b for an empty block
        assert_eq!(escaped, "\\x00\\x00\\x00\\x00\\x0b");

        let reader = FilterBlockReader::new(Box::new(TestHashFilter), &block);
        // Should treat empty as "maybe match" except offset array says no actual filters
        // Implementation might treat as "match" if invalid or no offset => true
        assert!(reader.key_may_match(0, &Slice::from_ptr_len(b"foo".as_ptr(), 3)));
        assert!(reader.key_may_match(100000, &Slice::from_ptr_len(b"foo".as_ptr(), 3)));
    }

    #[traced_test]
    fn filter_block_test_single_chunk() {
        let test = FilterBlockTest::default();
        let mut builder = FilterBlockBuilder::new(Box::new(test.policy));

        builder.start_block(100);
        builder.add_key(&Slice::from_ptr_len(b"foo".as_ptr(), 3));
        builder.add_key(&Slice::from_ptr_len(b"bar".as_ptr(), 3));
        builder.add_key(&Slice::from_ptr_len(b"box".as_ptr(), 3));

        builder.start_block(200);
        builder.add_key(&Slice::from_ptr_len(b"box".as_ptr(), 3));

        builder.start_block(300);
        builder.add_key(&Slice::from_ptr_len(b"hello".as_ptr(), 5));

        let block = builder.finish();
        let reader = FilterBlockReader::new(Box::new(TestHashFilter), &block);

        assert!(reader.key_may_match(100, &Slice::from_ptr_len(b"foo".as_ptr(), 3)));
        assert!(reader.key_may_match(100, &Slice::from_ptr_len(b"bar".as_ptr(), 3)));
        assert!(reader.key_may_match(100, &Slice::from_ptr_len(b"box".as_ptr(), 3)));
        // "hello" was added at offset=300, but we do a simple test hash. It might collide
        assert!(reader.key_may_match(100, &Slice::from_ptr_len(b"hello".as_ptr(), 5)));

        assert!(!reader.key_may_match(100, &Slice::from_ptr_len(b"missing".as_ptr(), 7)));
        assert!(!reader.key_may_match(100, &Slice::from_ptr_len(b"other".as_ptr(), 5)));
    }

    #[traced_test]
    fn filter_block_test_multi_chunk() {
        let test = FilterBlockTest::default();
        let mut builder = FilterBlockBuilder::new(Box::new(test.policy));

        // Offsets [0..2047] => first filter
        builder.start_block(0);
        builder.add_key(&Slice::from_ptr_len(b"foo".as_ptr(), 3));

        builder.start_block(2000);
        builder.add_key(&Slice::from_ptr_len(b"bar".as_ptr(), 3));

        // Offsets [2048..4095] => second filter
        builder.start_block(3100);
        builder.add_key(&Slice::from_ptr_len(b"box".as_ptr(), 3));

        // [4096..6143] => third filter (empty)
        builder.start_block(4100);

        // [8192..10239] => fourth filter
        builder.start_block(9000);
        builder.add_key(&Slice::from_ptr_len(b"box".as_ptr(), 3));
        builder.add_key(&Slice::from_ptr_len(b"hello".as_ptr(), 5));

        let block = builder.finish();
        let reader = FilterBlockReader::new(Box::new(TestHashFilter), &block);

        // For the first filter => offset=0 => "foo", offset=2000 => "bar"
        assert!(reader.key_may_match(0, &Slice::from_ptr_len(b"foo".as_ptr(), 3)));
        assert!(reader.key_may_match(2000, &Slice::from_ptr_len(b"bar".as_ptr(), 3)));
        assert!(!reader.key_may_match(0, &Slice::from_ptr_len(b"box".as_ptr(), 3)));
        assert!(!reader.key_may_match(0, &Slice::from_ptr_len(b"hello".as_ptr(), 5)));

        // second filter => offset=3100 => "box"
        assert!(reader.key_may_match(3100, &Slice::from_ptr_len(b"box".as_ptr(), 3)));
        assert!(!reader.key_may_match(3100, &Slice::from_ptr_len(b"foo".as_ptr(), 3)));
        assert!(!reader.key_may_match(3100, &Slice::from_ptr_len(b"bar".as_ptr(), 3)));
        assert!(!reader.key_may_match(3100, &Slice::from_ptr_len(b"hello".as_ptr(), 5)));

        // third filter => offset=4100 => empty
        assert!(!reader.key_may_match(4100, &Slice::from_ptr_len(b"foo".as_ptr(), 3)));
        assert!(!reader.key_may_match(4100, &Slice::from_ptr_len(b"bar".as_ptr(), 3)));
        assert!(!reader.key_may_match(4100, &Slice::from_ptr_len(b"box".as_ptr(), 3)));
        assert!(!reader.key_may_match(4100, &Slice::from_ptr_len(b"hello".as_ptr(), 5)));

        // fourth filter => offset=9000 => "box","hello"
        assert!(reader.key_may_match(9000, &Slice::from_ptr_len(b"box".as_ptr(), 3)));
        assert!(reader.key_may_match(9000, &Slice::from_ptr_len(b"hello".as_ptr(), 5)));
        assert!(!reader.key_may_match(9000, &Slice::from_ptr_len(b"foo".as_ptr(), 3)));
        assert!(!reader.key_may_match(9000, &Slice::from_ptr_len(b"bar".as_ptr(), 3)));
    }
}
