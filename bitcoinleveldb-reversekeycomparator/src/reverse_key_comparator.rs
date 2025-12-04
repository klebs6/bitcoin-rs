// ---------------- [ File: bitcoinleveldb-reversekeycomparator/src/reverse_key_comparator.rs ]
crate::ix!();
//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/table_test.cc]

/**
  | Return reverse of "key".
  | 
  | Used to test non-lexicographic comparators.
  |
  */
pub fn reverse(key_: &Slice) -> String {
    unsafe {
        let ptr = *key_.data();
        let len = *key_.size();
        let bytes = core::slice::from_raw_parts(ptr, len);

        // LevelDB’s tests treat these as byte strings; we preserve byte‑level
        // reversal and then re‑interpret as UTF‑8, matching std::string behavior.
        let mut rev = Vec::with_capacity(len);
        for b in bytes.iter().rev() {
            rev.push(*b);
        }

        String::from_utf8_lossy(&rev).to_string()
    }
}

pub struct ReverseKeyComparator;

impl SliceComparator for ReverseKeyComparator {
    fn bytewise_comparator(
        &self,
    ) -> *const (dyn SliceComparator
              + 'static) {
        bytewise_comparator()
    }
}

impl Named for ReverseKeyComparator {
    fn name(&self) -> Cow<'_, str> {
        Cow::Owned(String::from(
            "leveldb.ReverseBytewiseComparator",
        ))
    }
}

impl Compare for ReverseKeyComparator {
    fn compare(&self, a: &Slice, b: &Slice) -> i32 {
        let ra = reverse(a);
        let rb = reverse(b);

        let ra_slice = Slice::from(ra.as_bytes());
        let rb_slice = Slice::from(rb.as_bytes());

        unsafe {
            let base =
                &*bytewise_comparator();
            let r = base.compare(&ra_slice, &rb_slice);
            trace!(
                "ReverseKeyComparator::compare: a='{}', b='{}', ra='{}', rb='{}', result={}",
                a.to_string(),
                b.to_string(),
                ra,
                rb,
                r
            );
            r
        }
    }
}

impl FindShortestSeparator for ReverseKeyComparator {
    fn find_shortest_separator(
        &self,
        start: &mut Vec<u8>,
        limit: &[u8],
    ) {
        // Work in reversed space, delegate to the bytewise comparator, then
        // reverse back.
        let mut s_rev: Vec<u8> =
            start.iter().rev().copied().collect();
        let l_rev: Vec<u8> =
            limit.iter().rev().copied().collect();

        unsafe {
            let base =
                &*bytewise_comparator();
            base.find_shortest_separator(&mut s_rev, &l_rev);
        }

        *start = s_rev.into_iter().rev().collect();

        trace!(
            "ReverseKeyComparator::find_shortest_separator: updated start to {:?}",
            start
        );
    }
}

impl FindShortSuccessor for ReverseKeyComparator {
    fn find_short_successor(&self, key_: &mut Vec<u8>) {
        let mut k_rev: Vec<u8> =
            key_.iter().rev().copied().collect();

        unsafe {
            let base =
                &*bytewise_comparator();
            base.find_short_successor(&mut k_rev);
        }

        *key_ = k_rev.into_iter().rev().collect();

        trace!(
            "ReverseKeyComparator::find_short_successor: updated key to {:?}",
            key_
        );
    }
}

pub fn increment(
    cmp: Box<dyn SliceComparator>,
    key_: *mut String,
) {
    unsafe {
        assert!(!key_.is_null(), "increment: key pointer is null");
        let key = &mut *key_;

        let cmp_name = cmp.name();

        let bytewise_name = {
            let bw =
                &*bytewise_comparator();
            bw.name()
        };

        if cmp_name.as_ref() == bytewise_name.as_ref() {
            trace!(
                "increment: using bytewise comparator semantics for key='{}'",
                key
            );
            key.push('\0');
        } else {
            trace!(
                "increment: using reverse comparator semantics for key='{}'",
                key
            );
            let mut rev: String = key.chars().rev().collect();
            rev.push('\0');
            *key = rev.chars().rev().collect();
        }
    }
}

#[cfg(test)]
mod reverse_key_comparator_behavior_tests {
    use super::*;

    #[traced_test]
    fn reverse_function_reverses_simple_ascii_keys() {
        let key_bytes = b"abcdef";
        let slice = Slice::from(&key_bytes[..]);

        let reversed = reverse(&slice);
        assert_eq!(reversed, "fedcba");
    }

    #[traced_test]
    fn reverse_key_comparator_matches_bytewise_on_reversed_keys() {
        let cmp = ReverseKeyComparator;

        let a = Slice::from("abc".as_bytes());
        let b = Slice::from("abd".as_bytes());

        let ra = reverse(&a);
        let rb = reverse(&b);

        let ra_slice = Slice::from(ra.as_bytes());
        let rb_slice = Slice::from(rb.as_bytes());

        let base = unsafe {
            &*bytewise_comparator()
        };

        let expected = base.compare(&ra_slice, &rb_slice);
        let actual = cmp.compare(&a, &b);

        assert_eq!(
            actual, expected,
            "reverse_key_comparator_matches_bytewise_on_reversed_keys: comparator result mismatch"
        );
    }
}
