// ---------------- [ File: bitcoin-serialize/src/unserialize.rs ]
crate::ix!();

/// Replace the previous definition (which took `&self`) with a
/// mutable‐reference API so implementations can actually write the
/// value that has just been read from the stream.
pub trait Unserialize<Stream> {
    /// Populate `self` with data read from `s`.
    fn unserialize(&mut self, s: &mut Stream);
}

impl<Stream> Unserialize<Stream> for i8
where
    Stream: Read,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        let v = ser_readdata8(s) as i8;
        trace!(value = v, "unserialize i8");
        *self = v;
    }
}

impl<Stream> Unserialize<Stream> for u8
where
    Stream: Read,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        let v = ser_readdata8(s);
        trace!(value = v, "unserialize u8");
        *self = v;
    }
}

impl<Stream> Unserialize<Stream> for i16
where
    Stream: Read,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        let v = ser_readdata16(s) as i16;
        trace!(value = v, "unserialize i16");
        *self = v;
    }
}

impl<Stream> Unserialize<Stream> for u16
where
    Stream: Read,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        let v = ser_readdata16(s);
        trace!(value = v, "unserialize u16");
        *self = v;
    }
}

impl<Stream> Unserialize<Stream> for i32
where
    Stream: Read,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        let v = ser_readdata32(s) as i32;
        trace!(value = v, "unserialize i32");
        *self = v;
    }
}

impl<Stream> Unserialize<Stream> for u32
where
    Stream: Read,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        let v = ser_readdata32(s);
        trace!(value = v, "unserialize u32");
        *self = v;
    }
}

impl<Stream> Unserialize<Stream> for i64
where
    Stream: Read,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        let v = ser_readdata64(s) as i64;
        trace!(value = v, "unserialize i64");
        *self = v;
    }
}

impl<Stream> Unserialize<Stream> for u64
where
    Stream: Read,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        let v = ser_readdata64(s);
        trace!(value = v, "unserialize u64");
        *self = v;
    }
}

impl<Stream, const N: usize> Unserialize<Stream> for [u8; N]
where
    Stream: Read,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        trace!(len = N, "unserialize [u8; N]");
        s.read_exact(self)
            .expect("I/O error while reading fixed‑length byte array");
    }
}

impl<Stream> Unserialize<Stream> for bool
where
    Stream: Read,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        let v = ser_readdata8(s) != 0;
        trace!(value = v, "unserialize bool");
        *self = v;
    }
}

impl<Stream> Unserialize<Stream> for &[u8]
where
    Stream: Read,
{
    /// Read **exactly** `self.len()` bytes from `s` into the existing slice.
    ///
    /// This mirrors the “const‑cast” logic of Bitcoin Core’s
    /// `Span<unsigned char>` deserialiser: we temporarily treat the slice’s
    /// data as mutable, even though the reference itself is `&[u8]`.
    /// The caller is responsible for guaranteeing that the backing memory
    /// is writable (e.g. it was originally borrowed as `&mut [u8]` and only
    /// viewed through an immutable lens here).
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        let len = self.len();
        trace!(len, "unserialize &[u8]");

        // SAFETY:
        //  * `ptr` was obtained from a valid slice and is properly aligned.
        //  * It lives as long as `*self`.
        //  * We will not read or write out‑of‑bounds.
        //  * The caller upholds the aliasing contract (no concurrent mutable
        //    borrows of the same memory).
        let ptr = self.as_ptr() as *mut u8;
        let dst: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };

        s.read_exact(dst)
            .expect("I/O error while reading raw byte span");
    }
}

impl<Stream> Unserialize<Stream> for String
where
    Stream: Read,
{
    fn unserialize(&mut self, is: &mut Stream) {
        let n_size = crate::read_compact_size(is, Some(true)) as usize;
        trace!(len = n_size, "unserialize String");
        self.clear();
        self.reserve(n_size);
        if n_size != 0 {
            unsafe { self.as_mut_vec().set_len(n_size) };
            is.read_exact(unsafe { self.as_mut_vec() })
                .expect("I/O error while reading String");
        }
    }
}

impl<Stream, T: Default, const N: usize> Unserialize<Stream> for PreVector<T, N>
where
    Stream: Read,
    T: Clone + crate::unserialize::Unserialize<Stream>,
{
    fn unserialize(&mut self, is: &mut Stream) {
        self.clear();
        let total = crate::read_compact_size(is, Some(true)) as usize;
        trace!(expected = total, "unserialize PreVector");

        let mut read_elems = 0usize;

        while read_elems < total {

            let elems_per_block =
                1 + (crate::constants::MAX_VECTOR_ALLOCATE as usize - 1)
                    / std::mem::size_of::<T>();

            let blk = std::cmp::min(total - read_elems, elems_per_block);

            self.resize(read_elems + blk, T::default()); // safe resize

            for i in read_elems..read_elems + blk {
                crate::unserialize::Unserialize::unserialize(&mut self[i], is);
            }

            read_elems += blk;
        }
    }
}

impl<Stream, T, A> Unserialize<Stream> for Vec<T, A>
where
    Stream: Read,
    T: Default,
    A: std::alloc::Allocator + Clone,
    T: crate::unserialize::Unserialize<Stream>,
{
    fn unserialize(&mut self, is: &mut Stream) {
        self.clear();
        let total = crate::read_compact_size(is, Some(true)) as usize;
        trace!(expected = total, "unserialize Vec");

        let mut read_elems = 0usize;
        while read_elems < total {
            let elems_per_block =
                1 + (crate::constants::MAX_VECTOR_ALLOCATE as usize - 1)
                    / std::mem::size_of::<T>();
            let blk = std::cmp::min(total - read_elems, elems_per_block);

            self.reserve(blk);
            for _ in 0..blk {
                let mut elem = T::default();
                crate::unserialize::Unserialize::unserialize(&mut elem, is);
                self.push(elem);
            }
            read_elems += blk;
        }
    }
}

impl<Stream, K, V> Unserialize<Stream> for (K, V)
where
    Stream: Read,
    K: crate::unserialize::Unserialize<Stream> + Default,
    V: crate::unserialize::Unserialize<Stream> + Default,
{
    fn unserialize(&mut self, is: &mut Stream) {
        self.0.unserialize(is);
        self.1.unserialize(is);
        trace!("unserialize (K,V) tuple");
    }
}

impl<Stream, K, V, S> Unserialize<Stream> for HashMap<K, V, S>
where
    Stream: Read,
    K: crate::unserialize::Unserialize<Stream> + Eq + std::hash::Hash + Default,
    V: crate::unserialize::Unserialize<Stream> + Default,
    S: std::hash::BuildHasher + Default,
{
    fn unserialize(&mut self, is: &mut Stream) {
        self.clear();
        let total = crate::read_compact_size(is, Some(true)) as usize;
        trace!(expected = total, "unserialize HashMap");

        for _ in 0..total {
            let mut k = K::default();
            k.unserialize(is);
            let mut v = V::default();
            v.unserialize(is);
            self.insert(k, v);
        }
    }
}

impl<Stream, K, S> Unserialize<Stream> for HashSet<K, S>
where
    Stream: Read,
    K: crate::unserialize::Unserialize<Stream> + Eq + std::hash::Hash + Default,
    S: std::hash::BuildHasher + Default,
{
    fn unserialize(&mut self, is: &mut Stream) {
        self.clear();
        let total = crate::read_compact_size(is, Some(true)) as usize;
        trace!(expected = total, "unserialize HashSet");

        for _ in 0..total {
            let mut key = K::default();
            key.unserialize(is);
            self.insert(key);
        }
    }
}

impl<Stream, T> Unserialize<Stream> for Box<T>
where
    Stream: Read,
    T: Default + crate::unserialize::Unserialize<Stream>,
{
    fn unserialize(&mut self, is: &mut Stream) {
        let mut tmp = T::default();
        crate::unserialize::Unserialize::unserialize(&mut tmp, is);
        *self = Box::new(tmp);
    }
}

impl<Stream, T> Unserialize<Stream> for std::sync::Arc<T>
where
    Stream: Read,
    T: Default + crate::unserialize::Unserialize<Stream>,
{
    fn unserialize(&mut self, is: &mut Stream) {
        let mut tmp = T::default();
        crate::unserialize::Unserialize::unserialize(&mut tmp, is);
        *self = std::sync::Arc::new(tmp);
    }
}
