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

