// ---------------- [ File: bitcoinleveldb-slice/src/slice.rs ]
crate::ix!();

use crate::_core::intrinsics::compare_bytes;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/slice.h]

/// A simple structure containing a pointer into some external storage and a size.
/// The user of a Slice must ensure that the referenced data is valid
/// for the lifetime of this struct.
#[derive(Copy,Clone,Debug, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Slice {
    /// Pointer to the data (not owned here).
    data: *const u8,

    /// The length of the data (in bytes).
    size: usize,
}

impl Default for Slice {

    fn default() -> Self {
        info!("Creating a default Slice (empty)");
        Self {
            data: b"".as_ptr(),
            size: 0,
        }
    }
}

impl Slice {
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self::from_ptr_len(bytes.as_ptr(), bytes.len())
    }

    #[inline]
    pub fn from_str(s: &str) -> Self {
        Self::from_ptr_len(s.as_ptr(), s.len())
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        let len = *self.size();
        let ptr = *self.data();

        if len == 0 {
            if ptr.is_null() {
                warn!(
                    data_ptr = ?ptr,
                    "Slice::as_bytes: empty Slice has null data pointer; returning empty slice defensively"
                );
            } else {
                trace!(
                    data_ptr = ?ptr,
                    "Slice::as_bytes: empty Slice; returning empty slice"
                );
            }
            return &[];
        }

        assert!(
            !ptr.is_null(),
            "Slice::as_bytes: non-empty Slice requires non-null data pointer (len={})",
            len
        );

        unsafe { core::slice::from_raw_parts(ptr, len) }
    }
}

impl Index<usize> for Slice {
    type Output = u8;

    /// Return the i-th byte in the referenced data.
    /// REQUIRES: i < size()
    fn index(&self, i: usize) -> &Self::Output {
        debug!("Indexing into Slice at position {}", i);
        assert!(
            i < self.size,
            "Index out of range: i={} >= size={}",
            i,
            self.size
        );

        assert!(
            !self.data.is_null(),
            "Indexing into Slice requires non-null data pointer for non-empty slice (size={})",
            self.size
        );

        unsafe { &*self.data.add(i) }
    }
}

impl PartialEq for Slice {
    fn eq(&self, other: &Self) -> bool {
        trace!("Checking equality of two Slices");

        if self.size != other.size {
            return false;
        }

        if self.size == 0 {
            return true;
        }

        assert!(
            !self.data.is_null() && !other.data.is_null(),
            "Slice::eq: non-empty slices require non-null data pointers (len={})",
            self.size
        );

        unsafe {
            // compare_bytes returns an i32 (< 0, == 0, or > 0),
            // so check if result == 0 for equality
            let cmp = compare_bytes(self.data, other.data, self.size);
            cmp == 0
        }
    }
}

impl Eq for Slice {}

impl From<&String> for Slice {

    /// Create a slice that refers to the contents of the given `String`.
    fn from(s: &String) -> Self {
        info!("Creating a Slice from &String");
        Self {
            data: s.as_ptr(),
            size: s.len(),
        }
    }
}

impl From<&str> for Slice {

    /// Create a slice that refers to the contents of the given `String`.
    fn from(s: &str) -> Self {
        info!("Creating a Slice from &str");
        Self {
            data: s.as_ptr(),
            size: s.len(),
        }
    }
}

impl From<*const u8> for Slice {

    /// Create a slice that refers to `s[0..strlen(s)-1]`.
    /// Interprets `s` as a C-style null-terminated string.
    fn from(s: *const u8) -> Self {
        info!("Creating a Slice from a raw C-string pointer");
        if s.is_null() {
            warn!("Received a null pointer in Slice::from(*const u8); returning empty slice");
            return Self::default();
        }
        unsafe {
            let c_str = CStr::from_ptr(s as *const i8);
            let bytes = c_str.to_bytes();
            Self {
                data: bytes.as_ptr(),
                size: bytes.len(),
            }
        }
    }
}

impl From<&[u8]> for Slice {

    /// Create a slice referencing the given byte slice.
    ///
    /// This does **not** copy data. The caller must ensure
    /// the referenced bytes remain valid for the duration
    /// of the Slice.
    fn from(bytes: &[u8]) -> Self {
        info!(
            "Creating a Slice from &[u8] (len = {})",
            bytes.len()
        );

        if bytes.is_empty() {
            trace!("Received empty byte slice; returning empty Slice");
            return Slice {
                data: b"".as_ptr(),
                size: 0,
            };
        }

        let ptr = bytes.as_ptr();
        let len = bytes.len();

        trace!(
            "Slice::from(&[u8]) -> data={:p}, size={}",
            ptr,
            len
        );

        Slice {
            data: ptr,
            size: len,
        }
    }
}

impl Slice {
    /// Create a slice that refers to `d[0..n-1]`.
    pub fn from_ptr_len(d: *const u8, n: usize) -> Self {
        info!(
            "Creating a Slice from pointer {:p} with length {}",
            d, n
        );

        if n == 0 {
            if d.is_null() {
                warn!(
                    "Slice::from_ptr_len: received null pointer with zero length; canonicalizing to empty slice"
                );
                return Self {
                    data: b"".as_ptr(),
                    size: 0,
                };
            }

            trace!(
                "Slice::from_ptr_len: zero-length slice with non-null pointer {:p}",
                d
            );

            return Self { data: d, size: 0 };
        }

        assert!(
            !d.is_null(),
            "Slice::from_ptr_len: pointer must be non-null when length is non-zero (len={})",
            n
        );

        Self { data: d, size: n }
    }

    /// Return true iff the length of the referenced data is zero.
    pub fn empty(&self) -> bool {
        let is_empty = self.size == 0;
        trace!("Checking if Slice is empty -> {}", is_empty);
        is_empty
    }

    /// Change this slice to refer to an empty array.
    pub fn clear(&mut self) {
        info!("Clearing Slice to become empty");
        self.data = b"".as_ptr();
        self.size = 0;
    }

    /// Drop the first `n` bytes from this slice.
    pub fn remove_prefix(&mut self, n: usize) {
        info!("Removing prefix of length {} from Slice", n);

        if n == 0 {
            trace!(
                "Slice::remove_prefix: n=0 no-op (size={})",
                self.size
            );
            return;
        }

        assert!(
            n <= self.size,
            "remove_prefix({}) out of range for size {}",
            n,
            self.size
        );

        assert!(
            !self.data.is_null(),
            "Slice::remove_prefix: data pointer must not be null when removing a non-zero prefix (n={}, size={})",
            n,
            self.size
        );

        unsafe {
            self.data = self.data.add(n);
        }

        self.size -= n;
        trace!("New size after remove_prefix = {}", self.size);
    }

    /// Return a `String` that contains a copy of the referenced data.
    pub fn to_string(&self) -> String {
        trace!("Converting Slice to a Rust String");
        if self.size == 0 {
            String::new()
        } else {
            assert!(
                !self.data.is_null(),
                "Slice::to_string: non-empty Slice requires non-null data pointer (len={})",
                self.size
            );

            unsafe {
                let bytes = std::slice::from_raw_parts(self.data, self.size);
                String::from_utf8_lossy(bytes).into_owned()
            }
        }
    }

    /// Return true iff `x` is a prefix of `self`.
    pub fn starts_with(&self, x: &Slice) -> bool {
        trace!("Checking if Slice starts with another Slice prefix");

        if x.size == 0 {
            return true;
        }

        if self.size < x.size {
            return false;
        }

        assert!(
            !self.data.is_null() && !x.data.is_null(),
            "Slice::starts_with: non-empty slices require non-null data pointers (self_len={}, prefix_len={})",
            self.size,
            x.size
        );

        unsafe {
            // compare_bytes returns i32;
            // prefix is equal if cmp == 0
            let cmp = compare_bytes(self.data, x.data, x.size);
            cmp == 0
        }
    }

    /// Three-way comparison. Returns value:
    ///   < 0 iff `self` <  `b`
    ///   == 0 iff `self` == `b`
    ///   > 0 iff `self` >  `b`
    #[inline]
    pub fn compare(&self, b: &Slice) -> i32 {
        debug!("Comparing two Slices via compare()");
        let min_len = if self.size < b.size { self.size } else { b.size };

        if min_len == 0 {
            if self.size < b.size {
                return -1;
            } else if self.size > b.size {
                return 1;
            } else {
                return 0;
            }
        }

        assert!(
            !self.data.is_null() && !b.data.is_null(),
            "Slice::compare: non-empty slices require non-null data pointers (a_len={}, b_len={}, min_len={})",
            self.size,
            b.size,
            min_len
        );

        unsafe {
            // compare_bytes returns negative if self < b,
            // 0 if equal, positive if self > b, up to min_len
            let cmp = compare_bytes(self.data, b.data, min_len);
            if cmp != 0 {
                // If the initial min_len bytes differ, cmp is already
                // an i32 less than, equal to, or greater than zero.
                return cmp;
            } else {
                // The first min_len bytes are identical;
                // decide by comparing lengths
                if self.size < b.size {
                    -1
                } else if self.size > b.size {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[cfg(test)]
mod test_slice_interface {
    use super::*;

    #[traced_test]
    fn check_slice_defaults_and_clear() {
        let mut slice = Slice::default();
        assert!(slice.empty(), "Expected default slice to be empty");
        assert_eq!(*slice.size(), 0);
        slice.clear();
        assert!(slice.empty(), "Clear on an empty slice remains empty");
    }

    #[traced_test]
    fn check_slice_from_string() {
        let s = String::from("hello");
        let slice = Slice::from(&s);
        assert_eq!(*slice.size(), 5);
        assert!(!slice.empty(), "Should not be empty if length is > 0");
        assert_eq!(slice.to_string(), "hello");
    }

    #[traced_test]
    fn check_slice_remove_prefix() {
        let mut slice = Slice::from_ptr_len(b"abcdefgh".as_ptr(), 8);
        assert_eq!(*slice.size(), 8);
        slice.remove_prefix(3);
        assert_eq!(*slice.size(), 5);
        assert_eq!(slice.to_string(), "defgh");
    }

    #[traced_test]
    fn check_slice_starts_with() {
        let main_str = Slice::from_ptr_len(b"abcdef".as_ptr(), 6);
        let prefix = Slice::from_ptr_len(b"abc".as_ptr(), 3);
        let not_prefix = Slice::from_ptr_len(b"bbc".as_ptr(), 3);

        assert!(main_str.starts_with(&prefix));
        assert!(!main_str.starts_with(&not_prefix));
    }

    #[traced_test]
    fn check_slice_compare() {
        let slice_a = Slice::from_ptr_len(b"abc".as_ptr(), 3);
        let slice_b = Slice::from_ptr_len(b"abcd".as_ptr(), 4);
        let slice_c = Slice::from_ptr_len(b"abc".as_ptr(), 3);

        assert!(slice_a.compare(&slice_b) < 0, "Expected a < b");
        assert!(slice_b.compare(&slice_a) > 0, "Expected b > a");
        assert_eq!(slice_a.compare(&slice_c), 0, "Expected a == c");
    }

    #[traced_test]
    fn check_slice_equality() {
        let slice_a = Slice::from_ptr_len(b"abc".as_ptr(), 3);
        let slice_b = Slice::from_ptr_len(b"abc".as_ptr(), 3);
        let slice_c = Slice::from_ptr_len(b"abd".as_ptr(), 3);

        assert_eq!(slice_a, slice_b, "Slices with same data/size should be equal");
        assert_ne!(slice_a, slice_c, "Slices with different data should not be equal");
    }

    #[traced_test]
    fn check_slice_cstring() {
        // Demonstrate using From<*const u8> for a null-terminated string
        let c_str = b"Hello\0";
        let slice = Slice::from(c_str.as_ptr());
        assert_eq!(*slice.size(), 5);
        assert_eq!(slice.to_string(), "Hello");
    }
}

#[cfg(test)]
mod slice_interface_contract_suite {
    use super::*;
    use crate::Range;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    fn expect_panic_from_contract_violation<F>(context: &'static str, f: F)
    where
        F: FnOnce(),
    {
        info!("Expecting panic for contract violation: {}", context);

        let result = catch_unwind(AssertUnwindSafe(f));
        assert!(
            result.is_err(),
            "Expected panic but closure completed successfully: {}",
            context
        );

        trace!("Observed expected panic for: {}", context);
    }

    fn assert_slice_len_and_emptiness(slice: &Slice, expected_len: usize, context: &'static str) {
        debug!(
            "Asserting Slice len/empty contract (context='{}'): expected_len={}, actual_len={}, empty={}",
            context,
            expected_len,
            *slice.size(),
            slice.empty()
        );

        assert_eq!(
            *slice.size(),
            expected_len,
            "Unexpected Slice size (context='{}')",
            context
        );
        assert_eq!(
            slice.empty(),
            expected_len == 0,
            "Unexpected Slice empty() (context='{}')",
            context
        );
    }

    fn assert_slice_bytes_equal(slice: &Slice, expected: &[u8], context: &'static str) {
        trace!(
            "Asserting Slice bytes equality (context='{}'): expected_len={}, actual_len={}",
            context,
            expected.len(),
            *slice.size()
        );

        assert_slice_len_and_emptiness(slice, expected.len(), context);

        let actual = slice.as_bytes();
        debug!(
            "Slice.as_bytes() (context='{}'): actual_ptr={:p}, actual_len={}",
            context,
            actual.as_ptr(),
            actual.len()
        );

        assert_eq!(
            actual, expected,
            "Slice bytes mismatch (context='{}')",
            context
        );
    }

    #[traced_test]
    fn validate_default_is_empty_and_clear_is_idempotent() {
        info!("Validating Slice::default() and Slice::clear() contract");

        let mut s = Slice::default();

        assert_slice_len_and_emptiness(&s, 0, "default");
        assert_slice_bytes_equal(&s, b"", "default as_bytes");
        assert_eq!(s.to_string(), "", "default to_string should be empty");

        info!("Calling clear() on default slice");
        s.clear();

        assert_slice_len_and_emptiness(&s, 0, "after clear");
        assert_slice_bytes_equal(&s, b"", "after clear as_bytes");
        assert_eq!(s.to_string(), "", "cleared to_string should be empty");
    }

    #[traced_test]
    fn validate_from_str_and_from_trait_for_str_are_consistent_views() {
        info!("Validating Slice::from_str and Slice::from(&str) view consistency");

        let lit: &'static str = "hello";
        let a = Slice::from_str(lit);
        let b = Slice::from(lit);

        debug!(
            "Pointers: from_str={:p}, from_trait={:p}, lit={:p}",
            *a.data(),
            *b.data(),
            lit.as_ptr()
        );

        assert_slice_bytes_equal(&a, b"hello", "from_str bytes");
        assert_slice_bytes_equal(&b, b"hello", "from(&str) bytes");
        assert_eq!(a, b, "Expected both constructors to yield equal slices");
        assert_eq!(a.to_string(), "hello");
        assert_eq!(b.to_string(), "hello");
    }

    #[traced_test]
    fn validate_from_bytes_and_from_slice_are_consistent_views() {
        info!("Validating Slice::from_bytes and Slice::from(&[u8]) view consistency");

        let buf: [u8; 4] = [0u8, 1u8, 2u8, 255u8];

        let a = Slice::from_bytes(&buf);
        let b = Slice::from(&buf[..]);

        debug!(
            "Pointers: from_bytes={:p}, from(&[u8])={:p}, buf={:p}",
            *a.data(),
            *b.data(),
            buf.as_ptr()
        );

        assert_eq!(*a.data(), buf.as_ptr(), "from_bytes should view buf");
        assert_eq!(*b.data(), buf.as_ptr(), "from(&[u8]) should view buf");

        assert_slice_bytes_equal(&a, &buf[..], "from_bytes bytes");
        assert_slice_bytes_equal(&b, &buf[..], "from(&[u8]) bytes");
        assert_eq!(a, b, "Expected both constructors to yield equal slices");
    }

    #[traced_test]
    fn validate_view_semantics_track_mutations_in_backing_buffer() {
        info!("Validating that Slice is a non-owning view over mutable backing storage");

        let mut buf: [u8; 3] = [b'a', b'b', b'c'];
        let s = Slice::from_bytes(&buf);

        assert_slice_bytes_equal(&s, b"abc", "initial bytes");
        assert_eq!(s[1], b'b', "initial index read");

        info!("Mutating backing buffer and verifying Slice observes the mutation");
        buf[1] = b'Z';

        debug!("Post-mutation backing buf = {:?}", buf);
        assert_eq!(s[1], b'Z', "Slice should observe mutated backing storage");
        assert_slice_bytes_equal(&s, &[b'a', b'Z', b'c'], "post-mutation bytes");
    }

    #[traced_test]
    fn validate_indexing_bounds_and_panics() {
        info!("Validating Slice indexing contract and out-of-range panics");

        let s = Slice::from_str("abc");

        assert_eq!(s[0], b'a');
        assert_eq!(s[1], b'b');
        assert_eq!(s[2], b'c');

        expect_panic_from_contract_violation("index == size", || {
            let _ = s[3];
        });

        expect_panic_from_contract_violation("index > size", || {
            let _ = s[999];
        });

        let empty = Slice::default();
        expect_panic_from_contract_violation("indexing empty slice", || {
            let _ = empty[0];
        });
    }

    #[traced_test]
    fn validate_remove_prefix_full_edge_matrix() {
        info!("Validating Slice::remove_prefix across edge cases (0, size, >size)");

        let mut s = Slice::from_str("abcdef");
        assert_slice_bytes_equal(&s, b"abcdef", "initial");

        info!("remove_prefix(0) should be a no-op");
        s.remove_prefix(0);
        assert_slice_bytes_equal(&s, b"abcdef", "after remove_prefix(0)");

        info!("remove_prefix(3) should drop first 3 bytes");
        s.remove_prefix(3);
        assert_slice_bytes_equal(&s, b"def", "after remove_prefix(3)");

        info!("remove_prefix(size) should yield empty slice");
        let mut t = Slice::from_str("xyz");
        t.remove_prefix(3);
        assert_slice_len_and_emptiness(&t, 0, "after remove_prefix(size)");
        assert_slice_bytes_equal(&t, b"", "after remove_prefix(size) as_bytes");
        assert_eq!(t.to_string(), "", "to_string should be empty after full prefix removal");

        info!("remove_prefix on empty slice: n=0 ok, n>0 panics");
        let mut empty = Slice::default();
        empty.remove_prefix(0);
        assert_slice_len_and_emptiness(&empty, 0, "empty after remove_prefix(0)");

        expect_panic_from_contract_violation("remove_prefix(1) on empty", move || {
            let mut e = Slice::default();
            e.remove_prefix(1);
        });

        expect_panic_from_contract_violation("remove_prefix(size+1)", || {
            let mut u = Slice::from_str("hi");
            u.remove_prefix(3);
        });
    }

    #[traced_test]
    fn validate_as_bytes_and_to_string_for_utf8_and_non_utf8() {
        info!("Validating Slice::as_bytes and Slice::to_string behavior (utf8 + non-utf8)");

        let utf8 = Slice::from_str("hello");
        assert_slice_bytes_equal(&utf8, b"hello", "utf8 as_bytes");
        assert_eq!(utf8.to_string(), "hello", "utf8 to_string roundtrip");

        let empty = Slice::default();
        assert_slice_bytes_equal(&empty, b"", "empty as_bytes");
        assert_eq!(empty.to_string(), "", "empty to_string");

        info!("Validating that non-UTF8 bytes do not panic and produce replacement chars");
        let bad: [u8; 3] = [0xFF, 0xFE, 0xFD];
        let non_utf8 = Slice::from_bytes(&bad);

        assert_slice_bytes_equal(&non_utf8, &bad[..], "non-utf8 as_bytes");

        let rendered = non_utf8.to_string();
        debug!("Rendered lossy string from non-utf8 bytes: {:?}", rendered);

        assert!(
            !rendered.is_empty(),
            "Expected lossy conversion to produce a non-empty string"
        );
        assert!(
            rendered.contains('\u{FFFD}'),
            "Expected lossy conversion to contain the replacement character"
        );
    }

    #[traced_test]
    fn validate_starts_with_edge_cases() {
        info!("Validating Slice::starts_with across key edge cases");

        let main = Slice::from_str("abcdef");
        let prefix = Slice::from_str("abc");
        let equal = Slice::from_str("abcdef");
        let longer = Slice::from_str("abcdefg");
        let mismatch = Slice::from_str("abD");

        let empty_prefix = Slice::from_str("");

        assert!(main.starts_with(&prefix), "Expected prefix match");
        assert!(main.starts_with(&equal), "Expected self starts_with self");
        assert!(!main.starts_with(&longer), "Expected false when prefix longer than self");
        assert!(!main.starts_with(&mismatch), "Expected false for mismatching prefix");
        assert!(main.starts_with(&empty_prefix), "Expected empty prefix to match");
        assert!(Slice::default().starts_with(&empty_prefix), "Empty slice should start with empty");
        assert!(
            !Slice::default().starts_with(&Slice::from_str("x")),
            "Empty slice should not start with non-empty prefix"
        );
    }

    #[traced_test]
    fn validate_compare_lexicographic_and_length_tiebreakers() {
        info!("Validating Slice::compare lexicographic ordering and length tiebreaks");

        let a = Slice::from_str("abc");
        let b = Slice::from_str("abd");
        let c = Slice::from_str("ab");
        let d = Slice::from_str("");
        let e = Slice::from_str("");

        debug!("Comparisons: a vs b, a vs c, c vs a, d vs e");
        assert!(a.compare(&b) < 0, "Expected 'abc' < 'abd'");
        assert!(b.compare(&a) > 0, "Expected 'abd' > 'abc'");

        assert!(a.compare(&c) > 0, "Expected 'abc' > 'ab' (length tiebreak)");
        assert!(c.compare(&a) < 0, "Expected 'ab' < 'abc' (length tiebreak)");

        assert_eq!(d.compare(&e), 0, "Expected empty compare empty == 0");
    }

    #[traced_test]
    fn validate_equality_is_consistent_with_compare_zero() {
        info!("Validating Slice PartialEq/Eq consistency with compare() == 0");

        let x1_buf: [u8; 3] = [b'x', b'y', b'z'];
        let x2_buf: [u8; 3] = [b'x', b'y', b'z'];
        let y_buf: [u8; 3] = [b'x', b'y', b'0'];

        let x1 = Slice::from_bytes(&x1_buf);
        let x2 = Slice::from_bytes(&x2_buf);
        let y = Slice::from_bytes(&y_buf);

        debug!(
            "Pointers: x1={:p}, x2={:p}, y={:p}",
            *x1.data(),
            *x2.data(),
            *y.data()
        );

        assert_eq!(x1, x2, "Equal content should be equal even if backing differs");
        assert_eq!(x1.compare(&x2), 0, "Equal slices should compare == 0");

        assert_ne!(x1, y, "Different content should not be equal");
        assert!(x1.compare(&y) != 0, "Different slices should compare != 0");
    }

    #[traced_test]
    fn validate_from_cstring_null_pointer_yields_empty() {
        info!("Validating Slice::from(*const u8) handles null pointers by returning empty");

        let s = Slice::from(core::ptr::null::<u8>());
        assert_slice_len_and_emptiness(&s, 0, "null cstring");
        assert_slice_bytes_equal(&s, b"", "null cstring as_bytes");
        assert_eq!(s.to_string(), "", "null cstring to_string should be empty");
    }

    #[traced_test]
    fn validate_from_cstring_stops_at_first_nul_terminator() {
        info!("Validating Slice::from(*const u8) stops at first NUL terminator");

        let c_buf = b"Hello\0World\0";
        let s = Slice::from(c_buf.as_ptr());

        assert_slice_bytes_equal(&s, b"Hello", "cstring to bytes");
        assert_eq!(*s.size(), 5, "Expected length up to first NUL");
        assert_eq!(s.to_string(), "Hello", "Expected to_string up to first NUL");
    }

    #[traced_test]
    fn validate_range_new_and_default_getters() {
        info!("Validating Range::new, Range::default, and getter behavior");

        let r0 = Range::default();
        debug!(
            "Default Range: start.size={}, limit.size={}",
            *r0.start().size(),
            *r0.limit().size()
        );
        assert!(r0.start().empty(), "Default range start should be empty");
        assert!(r0.limit().empty(), "Default range limit should be empty");

        let start = Slice::from_str("a");
        let limit = Slice::from_str("z");
        let r = Range::new(start, limit);

        assert_eq!(r.start().to_string(), "a", "Range start getter should match");
        assert_eq!(r.limit().to_string(), "z", "Range limit getter should match");
    }

    #[traced_test]
    fn validate_from_ptr_len_null_pointer_with_zero_length_yields_empty_and_is_safe_to_inspect() {
        info!(
            "Validating Slice::from_ptr_len(null, 0) yields an empty slice that is safe to inspect"
        );

        let s = Slice::from_ptr_len(core::ptr::null::<u8>(), 0);

        assert_slice_len_and_emptiness(&s, 0, "from_ptr_len(null,0)");
        assert_slice_bytes_equal(&s, b"", "from_ptr_len(null,0) as_bytes");
        assert_eq!(
            s.to_string(),
            "",
            "from_ptr_len(null,0) to_string should be empty"
        );

        let mut t = Slice::from_ptr_len(core::ptr::null::<u8>(), 0);
        t.remove_prefix(0);

        assert_slice_len_and_emptiness(&t, 0, "remove_prefix(0) after from_ptr_len(null,0)");
        assert_slice_bytes_equal(&t, b"", "remove_prefix(0) after from_ptr_len(null,0) as_bytes");
    }

    #[traced_test]
    fn validate_from_ptr_len_null_pointer_with_nonzero_length_panics_to_prevent_ub() {
        info!(
            "Validating Slice::from_ptr_len(null, nonzero) panics to prevent undefined behavior"
        );

        expect_panic_from_contract_violation("from_ptr_len(null, 1)", || {
            let _ = Slice::from_ptr_len(core::ptr::null::<u8>(), 1);
        });
    }

    #[traced_test]
    fn validate_as_bytes_empty_slice_is_safe_even_if_data_pointer_is_null_via_public_setter() {
        info!(
            "Validating Slice::as_bytes on an empty slice is safe even if data pointer is set to null via public setter"
        );

        let mut s = Slice::default();
        s.set_data(core::ptr::null::<u8>());
        s.set_size(0);

        assert_slice_len_and_emptiness(&s, 0, "setter-null empty slice");
        assert_slice_bytes_equal(&s, b"", "setter-null empty slice as_bytes");
        assert_eq!(
            s.to_string(),
            "",
            "setter-null empty slice to_string should be empty"
        );
    }

    #[traced_test]
    fn validate_empty_slice_equality_and_prefix_logic_do_not_depend_on_data_pointer_value() {
        info!(
            "Validating empty Slice equality/prefix/compare do not depend on data pointer value"
        );

        let mut a = Slice::default();
        let mut b = Slice::default();

        a.set_data(core::ptr::null::<u8>());
        b.set_data(core::ptr::null::<u8>());
        a.set_size(0);
        b.set_size(0);

        assert_eq!(a, b, "empty slices should compare equal");
        assert!(a.starts_with(&b), "empty slice should start with empty prefix");
        assert_eq!(a.compare(&b), 0, "empty slices should compare == 0");
    }
}
