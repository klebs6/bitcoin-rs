// ---------------- [ File: bitcoin-serialize/src/vector_formatter.rs ]
crate::ix!();

/// Format a collection (`Vec<T>` or any [`Vec`]‑like wrapper) whose **elements**
/// must themselves be (de)serialised with a dedicated formatter `F`.
///
/// This mirrors Bitcoin Core’s original implementation, including the 5 MiB
/// incremental allocation safeguard against OOM‑DoS vectors.
pub struct VectorFormatter<F> {
    _marker: PhantomData<fn() -> F>,
}

impl<F> Default for VectorFormatter<F> {
    #[inline]
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<F, T> ValueFormatter<Vec<T>> for VectorFormatter<F>
where
    F: ValueFormatter<T> + Default,
    T: Default,
{
    fn ser<S: Write>(&mut self, s: &mut S, v: &Vec<T>) {
        trace!(len = v.len(), "VectorFormatter::ser");
        write_compact_size(s, v.len() as u64);

        let mut fmt = F::default();
        for elem in v {
            fmt.ser(s, elem);
        }

    }
    fn unser<S: Read>(&mut self, s: &mut S, v: &mut Vec<T>) {
        v.clear();
        let total = read_compact_size(s, Some(true)) as usize;
        trace!(expected = total, "VectorFormatter::unser → begin");

        let mut fmt = F::default();
        let mut read_elems = 0usize;
        while read_elems < total {
            // Allocate in ≤ 5 MiB blocks to avoid unbounded allocations.
            let elems_per_block =
                1 + (MAX_VECTOR_ALLOCATE as usize - 1) / std::mem::size_of::<T>();
            let blk = std::cmp::min(total - read_elems, elems_per_block);

            v.reserve(blk);
            for _ in 0..blk {
                let mut elem = T::default();
                fmt.unser(s, &mut elem);
                v.push(elem);
            }
            read_elems += blk;
        }

        trace!(actual = v.len(), "VectorFormatter::unser → done");
    }
}

#[cfg(test)]
mod vector_formatter_tests {
    use super::*;
    use std::io::Cursor;

    #[derive(Clone, PartialEq, Eq, Debug)]
    struct KB([u8; 1024]);

    impl Default for KB {
        fn default() -> Self { KB([0u8; 1024]) }
    }

    impl<Stream: Write> crate::serialize::BtcSerialize<Stream> for KB {
        fn serialize(&self, s: &mut Stream) {
            s.write_all(&self.0)
                .expect("I/O error while writing KB");
        }
    }

    impl<Stream: Read> crate::unserialize::BtcUnserialize<Stream> for KB {
        fn unserialize(&mut self, s: &mut Stream) {
            s.read_exact(&mut self.0)
                .expect("I/O error while reading KB");
        }
    }

    #[traced_test]
    fn roundtrip_empty_vector() {
        let v: Vec<u32> = Vec::new();
        let mut buf     = Cursor::new(Vec::<u8>::new());

        VectorFormatter::<DefaultFormatter>::default().ser(&mut buf, &v);
        buf.set_position(0);

        let mut decoded: Vec<u32> = Vec::new();
        VectorFormatter::<DefaultFormatter>::default().unser(&mut buf, &mut decoded);

        assert!(decoded.is_empty());
    }

    #[traced_test]
    fn roundtrip_small_vector() {
        let v: Vec<u16> = (0..10_000).map(|i| i as u16).collect();
        let mut buf     = Cursor::new(Vec::<u8>::new());

        VectorFormatter::<DefaultFormatter>::default().ser(&mut buf, &v);
        buf.set_position(0);

        let mut decoded: Vec<u16> = Vec::new();
        VectorFormatter::<DefaultFormatter>::default().unser(&mut buf, &mut decoded);

        assert_eq!(decoded, v);
    }

    /// A > 5 MiB payload forces the  “block”  path (allocation in chunks).
    #[traced_test]
    fn roundtrip_large_vector_multiple_blocks() {
        // `elem` is 1 KiB, so each block holds ≈ 4 883 elements.
        type Elem = KB;

        let blocks         = 2;
        let elems_per_blk  =
            1 + ((crate::constants::MAX_VECTOR_ALLOCATE as usize - 1)
                 / std::mem::size_of::<Elem>());
        let total_elems    = elems_per_blk * blocks + 7;   // cross boundary

        let v: Vec<Elem> = vec![KB([0u8; 1024]); total_elems];

        let mut buf = Cursor::new(Vec::<u8>::new());
        VectorFormatter::<DefaultFormatter>::default().ser(&mut buf, &v);
        buf.set_position(0);

        let mut decoded: Vec<Elem> = Vec::new();
        VectorFormatter::<DefaultFormatter>::default().unser(&mut buf, &mut decoded);

        assert_eq!(decoded.len(), v.len());
        assert_eq!(decoded, v);
    }
}
