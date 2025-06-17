// ---------------- [ File: bitcoinleveldb-slice/src/slice.rs ]
crate::ix!();

use crate::_core::intrinsics::compare_bytes;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/slice.h]

/// A simple structure containing a pointer into some external storage and a size.
/// The user of a Slice must ensure that the referenced data is valid
/// for the lifetime of this struct.
#[derive(Debug, Getters, Setters)]
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
        unsafe { &*self.data.add(i) }
    }
}

impl PartialEq for Slice {

    fn eq(&self, other: &Self) -> bool {
        trace!("Checking equality of two Slices");
        if self.size != other.size {
            return false;
        }
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

impl Slice {

    /// Create a slice that refers to `d[0..n-1]`.
    pub fn from_ptr_len(d: *const u8, n: usize) -> Self {
        info!(
            "Creating a Slice from pointer {:p} with length {}",
            d, n
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
        assert!(
            n <= self.size,
            "remove_prefix({}) out of range for size {}",
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
            unsafe {
                let bytes = std::slice::from_raw_parts(self.data, self.size);
                String::from_utf8_lossy(bytes).into_owned()
            }
        }
    }

    /// Return true iff `x` is a prefix of `self`.
    pub fn starts_with(&self, x: &Slice) -> bool {
        trace!("Checking if Slice starts with another Slice prefix");
        if self.size < x.size {
            return false;
        }
        unsafe {
            // Again, compare_bytes returns i32;
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
