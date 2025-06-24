// ---------------- [ File: bitcoin-vectorstream/src/vector_writer.rs ]
crate::ix!();

/**
  | Minimal stream for overwriting and/or
  | appending to an existing byte vector.
  |
  | The referenced vector will grow as necessary.
  */
#[derive(Debug, Getters, Builder)]
pub struct VectorWriter {
    /// Serialization type
    #[getset(get = "pub")]
    n_type:    i32,

    /// Serialization version (and flags)
    #[getset(get = "pub")]
    n_version: i32,

    /// Shared, mutable buffer
    vch_data:  Rc<RefCell<Vec<u8>>>,

    /// Current cursor position (0‑based)
    n_pos:     usize,
}

impl Default for VectorWriter {
    fn default() -> Self {
        // Empty buffer held inside an `Rc<RefCell<…>>`, matching the
        // interior‑mutability pattern used by `BitStreamWriter`.
        Self::new(0, 0, std::rc::Rc::new(std::cell::RefCell::new(Vec::new())), 0)
    }
}

impl<T> Shl<&T> for VectorWriter
where
    T: bitcoin_serialize::BtcSerialize<VectorWriter> + ?Sized,
{
    type Output = VectorWriter;

    #[inline]
    fn shl(mut self, rhs: &T) -> Self::Output {
        trace!("VectorWriter << {:?}", std::any::type_name::<T>());
        rhs.serialize(&mut self); // trait call
        self
    }
}

impl Write for VectorWriter {

    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // call inherent pointer‑based writer
        VectorWriter::write(self, buf.as_ptr(), buf.len());
        Ok(buf.len())
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

impl VectorWriter {

    /// Create a new `VectorWriter`.
    ///
    /// @param[in] nTypeIn
    /// 
    /// Serialization Type
    /// ----------
    /// @param[in] nVersionIn
    /// 
    /// Serialization Version (including
    /// any flags)
    /// ----------
    /// @param[in] vchDataIn
    /// 
    /// Referenced byte vector to overwrite/append
    /// ----------
    /// @param[in] nPosIn
    /// 
    /// Starting position. Vector index where
    /// writes should start. The vector will
    /// initially grow as necessary to max(nPosIn,
    /// vec.size()). So to append, use vec.size().
    /// 
    pub fn new(
        n_type:    i32,
        n_version: i32,
        vch_data:  Rc<RefCell<Vec<u8>>>,
        n_pos:     usize,
    ) -> Self {
        trace!(n_type, n_version, n_pos, "VectorWriter::new");

        {
            let mut vec_ref = vch_data.borrow_mut();
            if n_pos > vec_ref.len() {
                debug!(
                    "resizing buffer from {} to {} bytes to satisfy n_pos",
                    vec_ref.len(),
                    n_pos
                );
                vec_ref.resize(n_pos, 0);
            }
        }

        Self { n_type, n_version, vch_data, n_pos }
    }

    /// Create a new `VectorWriter` and immediately serialize a sequence of
    /// objects supplied in `args`.
    pub fn new_with_args<Args>(
        n_type:    i32,
        n_version: i32,
        vch_data:  Rc<RefCell<Vec<u8>>>,
        n_pos:     usize,
        args:      Args,
    ) -> Self
    where
        Args: bitcoin_serialize::SerializeMany<VectorWriter>,
    {
        let mut writer = Self::new(n_type, n_version, vch_data, n_pos);
        args.serialize_many(&mut writer);
        writer
    }

    /// Write `n_size` bytes from `pch` into the stream.
    pub fn write(&mut self, pch: *const u8, n_size: usize) {
        trace!(bytes = n_size, pos_before = self.n_pos, "VectorWriter::write");

        let mut vec_ref = self.vch_data.borrow_mut();
        assert!(
            self.n_pos <= vec_ref.len(),
            "n_pos ({}) exceeds vec.len() ({})",
            self.n_pos,
            vec_ref.len()
        );

        let overwrite = min(n_size, vec_ref.len() - self.n_pos);

        // SAFETY: regions checked for bounds; they do not overlap.
        unsafe {
            if overwrite != 0 {
                std::ptr::copy_nonoverlapping(
                    pch,
                    vec_ref.as_mut_ptr().add(self.n_pos),
                    overwrite,
                );
            }

            if overwrite < n_size {
                let extra = n_size - overwrite;
                let start_ptr = pch.add(overwrite);
                vec_ref.reserve(extra);
                let len_before = vec_ref.len();
                vec_ref.set_len(len_before + extra);
                std::ptr::copy_nonoverlapping(
                    start_ptr,
                    vec_ref.as_mut_ptr().add(len_before),
                    extra,
                );
            }
        }

        self.n_pos += n_size;
        trace!(pos_after = self.n_pos, "VectorWriter::write finished");
    }
}

#[cfg(test)]
mod vector_writer_exhaustive_suite {
    use super::*;
    use std::{
        panic::{self, AssertUnwindSafe},
        ptr,
        rc::Rc,
        cell::RefCell,
    };

    /* ---------------- fixed tests ---------------- */

    /// Appending via an offset equal to the initial length must extend
    /// the vector without corrupting prior data.
    #[traced_test]
    fn writer_appends_correctly() {
        let buf = Rc::new(RefCell::new(vec![1u8, 2, 3]));
        let start_pos = buf.borrow().len();          // ← borrow ends here
        let mut wtr = VectorWriter::new(0, 0, buf.clone(), start_pos);

        let src = [4u8, 5, 6];
        wtr.write(src.as_ptr(), src.len());

        assert_eq!(&*buf.borrow(), &[1, 2, 3, 4, 5, 6]);
    }

    /// Creating a writer at `pos == len` followed by a write must grow
    /// the buffer, mirroring C++ append semantics.
    #[traced_test]
    fn append_at_end() {
        let buf = Rc::new(RefCell::new(vec![1u8, 2, 3]));
        let start_pos = buf.borrow().len();          // ← borrow ends here
        let mut wtr = VectorWriter::new(0, 0, buf.clone(), start_pos);

        let src = [4u8, 5, 6];
        wtr.write(src.as_ptr(), src.len());

        assert_eq!(&*buf.borrow(), &[1, 2, 3, 4, 5, 6]);
    }

    /* -------------- remaining tests unchanged -------------- */

    #[traced_test]
    fn write_zero_bytes_is_noop() {
        let buf = Rc::new(RefCell::new(vec![1u8, 2, 3]));
        let len_before = buf.borrow().len();

        let mut wtr = VectorWriter::new(0, 0, buf.clone(), 1);
        wtr.write(ptr::null(), 0);

        let vec_after = buf.borrow();
        assert_eq!(vec_after.len(), len_before);
        assert_eq!(&*vec_after, &[1, 2, 3]);
    }

    #[traced_test]
    fn overwrite_in_place() {
        let buf = Rc::new(RefCell::new(vec![0u8; 4]));
        let mut wtr = VectorWriter::new(0, 0, buf.clone(), 1);

        let src = [0xAAu8, 0xBB];
        wtr.write(src.as_ptr(), src.len());

        assert_eq!(&*buf.borrow(), &[0, 0xAA, 0xBB, 0]);
    }

    #[traced_test]
    fn pos_beyond_len_resizes_with_zero_fill() {
        let buf = Rc::new(RefCell::new(vec![1u8, 2]));
        let mut wtr = VectorWriter::new(0, 0, buf.clone(), 5);

        assert_eq!(&*buf.borrow(), &[1, 2, 0, 0, 0]);

        let src = [7u8, 8];
        wtr.write(src.as_ptr(), src.len());

        assert_eq!(&*buf.borrow(), &[1, 2, 0, 0, 0, 7, 8]);
    }

    #[traced_test]
    fn multiple_writes_maintain_integrity() {
        let buf = Rc::new(RefCell::new(Vec::<u8>::new()));
        let mut wtr = VectorWriter::new(0, 0, buf.clone(), 0);

        let first  = [0x10u8, 0x11];
        let second = [0x20u8; 3];
        let third  = [0x30u8];

        wtr.write(first.as_ptr(),  first.len());
        wtr.write(second.as_ptr(), second.len());
        wtr.write(third.as_ptr(),  third.len());

        assert_eq!(&*buf.borrow(), &[0x10, 0x11, 0x20, 0x20, 0x20, 0x30]);
    }

    #[traced_test]
    fn internal_invariant_violation_panics() {
        let buf = Rc::new(RefCell::new(vec![0u8; 2]));
        let mut wtr = VectorWriter::new(0, 0, buf.clone(), 0);

        wtr.n_pos = 10; // intentionally break invariant

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            let b = [1u8];
            wtr.write(b.as_ptr(), 1);
        }));
        assert!(result.is_err(), "expected panic on invariant violation");
    }
}
