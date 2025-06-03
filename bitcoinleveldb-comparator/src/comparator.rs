// ---------------- [ File: bitcoinleveldb-comparator/src/comparator.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/comparator.h]

/// A bytewise comparator that sorts slices by
/// lexicographic ordering (the default for
/// LevelDB).
#[derive(Debug)]
pub struct BytewiseComparatorImpl {
    // No fields needed; logic is purely functional.
}

impl Default for BytewiseComparatorImpl {
    fn default() -> Self {
        info!("Creating BytewiseComparatorImpl by default");
        Self {}
    }
}

impl SliceComparator for BytewiseComparatorImpl {
    fn bytewise_comparator(&self) -> *const dyn SliceComparator {
        trace!("Returning global bytewise_comparator pointer");
        bytewise_comparator()
    }
}

impl FindShortestSeparator for BytewiseComparatorImpl {
    fn find_shortest_separator(&self, start: &mut Vec<u8>, limit: &[u8]) {
        trace!(
            "BytewiseComparatorImpl::find_shortest_separator called with start={:?}, limit={:?}",
            start,
            limit
        );

        // 1. Find length of common prefix
        let min_length = std::cmp::min(start.len(), limit.len());
        let mut diff_index = 0;
        while diff_index < min_length {
            if start[diff_index] != limit[diff_index] {
                break;
            }
            diff_index += 1;
        }

        if diff_index < min_length {
            let diff_byte = start[diff_index];
            let limit_byte = limit[diff_index];

            // 2. If we can increment diff_byte without crossing limit_byte,
            //    do so and truncate.
            if diff_byte < 0xFF && (diff_byte + 1) < limit_byte {
                start[diff_index] = diff_byte + 1;
                // Truncate after diff_index + 1
                start.truncate(diff_index + 1);
                debug!("Shortened separator -> {:?}", start);
            }
        }
        // else they share the entire prefix up to min_length or
        // one is a prefix of the other, so do nothing
    }
}

impl FindShortSuccessor for BytewiseComparatorImpl {

    fn find_short_successor(&self, key: &mut Vec<u8>) {
        trace!(
            "BytewiseComparatorImpl::find_short_successor called with key={:?}",
            key
        );

        // 1. Find first character that can be incremented
        for i in 0..key.len() {
            if key[i] != 0xFF {
                key[i] = key[i] + 1; // increment it
                key.truncate(i + 1); // truncate the rest
                debug!("Short successor -> {:?}", key);
                return;
            }
        }
        // If entire key is 0xFF, do nothing
        trace!("Key is a run of 0xFF bytes; leaving unchanged");
    }
}

// -------------------------------------
// Name trait implementation
// -------------------------------------
impl Name for BytewiseComparatorImpl {
    fn name(&self) -> *const u8 {
        info!("Returning the name of BytewiseComparatorImpl");
        static NAME: &str = "leveldb.BytewiseComparator";
        NAME.as_ptr()
    }
}

// -------------------------------------
// Compare trait implementation
// -------------------------------------
impl Compare for BytewiseComparatorImpl {
    fn compare(&self, a: &Slice, b: &Slice) -> i32 {
        trace!("BytewiseComparatorImpl::compare invoked");
        let cmp = a.compare(b);
        trace!("compare result: {}", cmp);
        cmp
    }
}

/// Return a pointer to a global, bytewise comparator.
/// This replicates the C++ pattern of `static NoDestructor<BytewiseComparatorImpl>`.
pub fn bytewise_comparator() -> *const dyn SliceComparator {

    static BYTEWISE_COMPARATOR: OnceLock<BytewiseComparatorImpl> = OnceLock::new();

    trace!("bytewise_comparator() invoked");
    let reference = BYTEWISE_COMPARATOR.get_or_init(|| {
        info!("Initializing BytewiseComparatorImpl singleton");
        BytewiseComparatorImpl::default()
    });
    // We cast &BytewiseComparatorImpl -> *const dyn SliceComparator
    reference as *const BytewiseComparatorImpl as *const dyn SliceComparator
}

#[cfg(test)]
mod test_comparator {
    use super::*;

    #[traced_test]
    fn test_compare_basic() {
        let cmp = BytewiseComparatorImpl::default();
        let s1 = Slice::from_ptr_len(b"abc".as_ptr(), 3);
        let s2 = Slice::from_ptr_len(b"abd".as_ptr(), 3);
        let s3 = Slice::from_ptr_len(b"abc".as_ptr(), 3);

        assert!(cmp.compare(&s1, &s2) < 0, "abc < abd");
        assert!(cmp.compare(&s2, &s1) > 0, "abd > abc");
        assert_eq!(cmp.compare(&s1, &s3), 0, "abc == abc");
    }

    #[traced_test]
    fn test_find_shortest_separator() {
        let cmp = BytewiseComparatorImpl::default();
        let limit = b"abcxyz".to_vec();

        // If prefix matches entirely, do nothing
        let mut start1 = b"abc".to_vec();
        cmp.find_shortest_separator(&mut start1, &limit);
        assert_eq!(start1, b"abc");

        // Diverge at index=2
        let mut start2 = b"abaxxx".to_vec(); // 'a' < 'c'
        cmp.find_shortest_separator(&mut start2, &limit);
        // Expect "abb" => 'a'(0x61), 'b'(0x62), second char is 'a'(0x61) < 'c'(0x63),
        // so 0x61 + 1=0x62 => 'b'
        // => [0x61, 0x62, 0x62] -> "abb"
        assert_eq!(start2, b"abb");
    }

    #[traced_test]
    fn test_find_short_successor() {
        let cmp = BytewiseComparatorImpl::default();
        
        // Key #1: "abz" => first non-0xFF is 'a' -> 'b' (0x61 -> 0x62), 
        // truncate after i=0 => "b".
        let mut key1 = b"abz".to_vec();
        cmp.find_short_successor(&mut key1);
        assert_eq!(key1, b"b", "Expect the official LevelDB left-to-right behavior");

        // Key #2: entire key is 0xFF => do nothing
        let mut key2 = vec![0xFF, 0xFF, 0xFF];
        cmp.find_short_successor(&mut key2);
        assert_eq!(key2, vec![0xFF, 0xFF, 0xFF]);

        // Key #3: [ 'a', 0xFF, ... ] => first is 'a', not 0xFF => increment to 'b', truncate => "b"
        let mut key3 = vec![b'a', 0xFF, b'x', b'y', b'z'];
        cmp.find_short_successor(&mut key3);
        assert_eq!(key3, b"b", "Same logic as above");
    }

    #[traced_test]
    fn test_bytewise_comparator_singleton() {
        let ptr1 = bytewise_comparator();
        assert!(!ptr1.is_null());
        let ptr2 = bytewise_comparator();
        assert_eq!(ptr1, ptr2, "Singleton pointer must not change");
    }
}
