// ---------------- [ File: bitcoin-vectorstream/src/vector_reader.rs ]
crate::ix!();

/**
  | Minimal stream for reading from an existing
  | vector by reference
  |
  */
#[derive(Debug, Getters, Builder)]
pub struct VectorReader {
    /// Serialization type
    #[getset(get = "pub")]
    ty:      i32,

    /// Serialization version (and flags)
    #[getset(get = "pub")]
    version: i32,

    /// Shared buffer we are reading from
    data:    Arc<Vec<u8>>,

    /// Current cursor position (0‑based)
    pos:     usize,
}

impl<T> Shr<&mut T> for VectorReader
where
    T: ?Sized,
{
    type Output = VectorReader;

    /// Deserialize `rhs` from this stream.
    #[inline]
    fn shr(mut self, rhs: &mut T) -> Self::Output {
        trace!("VectorReader >> {:?}", std::any::type_name::<T>());
        Serialize::unserialize(&mut self, rhs);
        self
    }
}

impl VectorReader {

    /// @param[in] type
    /// 
    /// Serialization Type
    /// ----------
    /// @param[in] version
    /// 
    /// Serialization Version (including
    /// any flags)
    /// ----------
    /// @param[in] data
    /// 
    /// Referenced byte vector to overwrite/append
    /// ----------
    /// @param[in] pos
    /// 
    /// Starting position. Vector index where
    /// reads should start.
    /// 
    /// Create a new `VectorReader`.
    ///
    /// # Panics
    ///
    /// Panics when `pos` is greater than `data.len()`.
    pub fn new(
        ty:      i32,
        version: i32,
        data:    Arc<Vec<u8>>,
        pos:     usize,
    ) -> Self {
        trace!(ty, version, pos, len = data.len(), "VectorReader::new");

        if pos > data.len() {
            error!(
                pos,
                len = data.len(),
                "VectorReader(...): end of data (pos > data.len())"
            );
            panic!("VectorReader(...): end of data (pos > data.len())");
        }

        Self { ty, version, data, pos }
    }

    /// (other params same as above)
    /// 
    /// -----------
    /// @param[in] args
    /// 
    /// A list of items to deserialize starting
    /// at pos.
    /// 
    ////
    /// Create a new `VectorReader` and immediately
    /// deserialize a sequence of objects.
    pub fn new_with_args<Args>(
        ty:      i32,
        version: i32,
        data:    Arc<Vec<u8>>,
        pos:     usize,
        args:    Args,
    ) -> Self
    where
        Args: SerializeMany,
    {
        let mut reader = Self::new(ty, version, data, pos);
        SerializeMany::unserialize_many(&mut reader, args);
        reader
    }
    
    /// Remaining unread bytes.
    pub fn size(&self) -> usize {
        self.data.len() - self.pos
    }

    /// `true` when all data have been consumed.
    pub fn empty(&self) -> bool {
        self.pos == self.data.len()
    }
    
    /// Read `n` bytes into `dst`.
    ///
    /// # Panics
    ///
    /// Panics if the read would go past the end of the buffer.
    pub fn read(&mut self, dst: *mut u8, n: usize) {
        if n == 0 {
            return;
        }

        let next_pos = self.pos + n;
        if next_pos > self.data.len() {
            error!(
                next_pos,
                len = self.data.len(),
                "VectorReader::read(): end of data"
            );
            panic!("VectorReader::read(): end of data");
        }

        trace!(bytes = n, from = self.pos, to = next_pos, "VectorReader::read");

        // SAFETY: bounds checked above and the regions do not overlap.
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.data.as_ptr().add(self.pos),
                dst,
                n,
            );
        }

        self.pos = next_pos;
    }
}

#[cfg(test)]
mod vector_reader_exhaustive_suite {
    use super::*;
    use std::{panic, ptr, sync::Arc};

    #[traced_test]
    fn vector_reader_roundtrip_position_advances() {
        let bytes = Arc::new(vec![0xAAu8, 0xBB, 0xCC, 0xDD]);
        let mut rdr = VectorReader::new(0, 0, bytes.clone(), 1);

        let mut buf = [0u8; 2];
        rdr.read(buf.as_mut_ptr(), 2);

        assert_eq!(buf, [0xBB, 0xCC]);
        assert_eq!(rdr.size(), 1);
        assert!(!rdr.empty());
    }


    /// Verify constructor rejects an initial position that
    /// exceeds the backing buffer’s length.
    #[traced_test]
    fn constructor_rejects_out_of_bounds_pos() {
        let data = Arc::new(vec![0x00u8; 4]);
        let result = panic::catch_unwind(|| {
            VectorReader::new(0, 0, data.clone(), 5);
        });
        assert!(result.is_err(), "expected panic for pos > len");
    }

    /// Reading `0` bytes must not advance the cursor.
    #[traced_test]
    fn read_zero_bytes_is_noop() {
        let data = Arc::new(vec![1u8, 2, 3]);
        let mut rdr = VectorReader::new(0, 0, data, 0);

        let remaining_before = rdr.size();
        rdr.read(ptr::null_mut(), 0);
        let remaining_after = rdr.size();

        assert_eq!(remaining_before, remaining_after);
        assert!(!rdr.empty());
    }

    /// Reading exactly the remaining bytes should leave the reader empty.
    #[traced_test]
    fn read_exact_consumes_buffer() {
        let data = Arc::new(vec![10u8, 20, 30]);
        let mut rdr = VectorReader::new(0, 0, data, 0);

        let mut dst = [0u8; 3];
        rdr.read(dst.as_mut_ptr(), dst.len());

        assert_eq!(dst, [10, 20, 30]);
        assert_eq!(rdr.size(), 0);
        assert!(rdr.empty());
    }

    /// Attempting to read past the end of the buffer must panic.
    #[traced_test]
    fn read_past_end_panics() {
        let data = Arc::new(vec![0xAAu8; 2]);
        let mut rdr = VectorReader::new(0, 0, data, 1);

        let mut dst = [0u8; 2];
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            rdr.read(dst.as_mut_ptr(), 2);
        }));
        assert!(result.is_err(), "expected panic when reading past end");
    }

    /// Verify that size/empty advance correctly after partial reads.
    #[traced_test]
    fn size_and_empty_track_progress() {
        let data = Arc::new((0u8..10).collect::<Vec<_>>());
        let mut rdr = VectorReader::new(0, 0, data, 0);

        let half = rdr.size() / 2;
        let mut buf = vec![0u8; half];
        rdr.read(buf.as_mut_ptr(), half);

        assert_eq!(rdr.size(), half);
        assert!(!rdr.empty());

        rdr.read(buf.as_mut_ptr(), half);
        assert_eq!(rdr.size(), 0);
        assert!(rdr.empty());
    }
}
