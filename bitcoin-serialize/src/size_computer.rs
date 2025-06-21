// ---------------- [ File: bitcoin-serialize/src/size_computer.rs ]
crate::ix!();

use std::io::{Result as IoResult, Write};

/// Count‑only stream used by `GetSerializeSize`.
///
/// ::GetSerializeSize implementations
/// 
/// Computing the serialized size of objects is done through a special stream object of type
/// CSizeComputer, which only records the number of bytes written to it.
/// 
/// If your Serialize or SerializationOp method has non-trivial overhead for serialization, it may
/// be worthwhile to implement a specialized version for
/// 
/// CSizeComputer, which uses the s.seek() method to record bytes that would be written instead.
/// 
pub struct SizeComputer {
    n_size:    usize,
    n_version: i32,
}

impl SizeComputer {
    pub const fn new(n_version_in: i32) -> Self {
        Self { n_size: 0, n_version: n_version_in }
    }

    /// Bump the byte counter.
    #[inline] pub fn add_bytes(&mut self, n_size: usize) {
        trace!(bytes = n_size, "SizeComputer::add_bytes");
        self.n_size += n_size;
    }

    /// C++ alias that “writes” a raw pointer/len pair.
    #[inline] pub fn write_ptr(&mut self, _psz: *const u8, n_size: usize) {
        self.add_bytes(n_size);
    }

    /// C++ alias that pretends to seek forward `_nSize` bytes.
    #[inline] pub fn seek(&mut self, n_size: usize) {
        self.add_bytes(n_size);
    }

    #[inline] pub fn size(&self) -> usize { self.n_size }
    #[inline] pub fn get_version(&self) -> i32 { self.n_version }
}

/* ---------------- I/O trait glue ---------------- */

impl Write for SizeComputer {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.add_bytes(buf.len());
        Ok(buf.len())
    }
    #[inline] fn flush(&mut self) -> IoResult<()> { Ok(()) }
}

impl<'a, T> std::ops::Shl<&'a T> for SizeComputer
where
    T: BtcSerialize<SizeComputer>,
{
    type Output = SizeComputer;
    #[inline]
    fn shl(mut self, rhs: &'a T) -> Self::Output {
        BtcSerialize::<SizeComputer>::serialize(rhs, &mut self);
        self
    }
}

#[cfg(test)]
mod size_computer_tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn manual_byte_accounting() {
        let mut sc = SizeComputer::new(42);
        assert_eq!(sc.size(),        0);
        assert_eq!(sc.get_version(), 42);

        sc.add_bytes(5);
        sc.write_ptr(std::ptr::null(), 7);
        sc.seek(11);

        assert_eq!(sc.size(), 5 + 7 + 11);
    }

    #[test]
    fn write_trait_counts() {
        let mut sc = SizeComputer::new(0);
        std::io::Write::write_all(&mut sc, &[0u8; 16]).unwrap();
        assert_eq!(sc.size(), 16);
    }

    #[test]
    fn shl_operator_counts() {
        let n: u32 = 0xDEAD_BEEF;          // 4 bytes
        let sc = SizeComputer::new(0) << &n;
        assert_eq!(sc.size(), 4);
    }

    #[test]
    fn size_matches_real_serialisation() {
        // a 2‑tuple (there is an impl for (K,V), but not for 3‑ or 4‑tuples)
        let tpl = (0x11u8, 0x2233u16);

        let actual = {
            let mut cur = Cursor::new(Vec::<u8>::new());
            BtcSerialize::serialize(&tpl, &mut cur);
            cur.into_inner().len()
        };

        let counted = (SizeComputer::new(0) << &tpl).size();
        assert_eq!(counted, actual);
    }
}
