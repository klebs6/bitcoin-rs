// ---------------- [ File: bitcoin-golombrice/src/new_with_element_set.rs ]
crate::ix!();

impl GCSFilter {

    /// Build a filter from a concrete element set.
    pub fn new_with_element_set(
        params:   &GcsFilterParams,
        elements: &GcsFilterElementSet,
    ) -> Self {
        use std::{cell::RefCell, rc::Rc};

        let n = u32::try_from(elements.len()).expect("N must be < 2^32");
        let f = n as u64 * *params.m() as u64;

        // Buffer into which the writer serialises.
        let vch_rc = Rc::new(RefCell::new(Vec::<u8>::new()));
        let mut vw = VectorWriter::new(
            GCS_SER_TYPE as i32,
            GCS_SER_VERSION as i32,
            vch_rc.clone(),
            0,
        );
        write_compact_size(&mut vw, n.into());

        // Empty filter – nothing more to do.
        if elements.is_empty() {

            let x = GCSFilterBuilder::default()
                .params(params.clone())
                .n(n)
                .f(f)
                .encoded(vch_rc.borrow().clone())
                .build()
                .unwrap();

            return x;
        }

        // BitStreamWriter over the same VectorWriter (via Rc<RefCell<…>>).
        let vw_rc  = Rc::new(RefCell::new(vw));
        let mut bw = BitStreamWriter::<VectorWriter>::new(vw_rc.clone());

        // hash, sort, encode deltas
        let mut hashed: Vec<u64> = elements
            .iter()
            .map(|e| {
                let mut h = std::hash::SipHasher::new_with_keys(
                    *params.siphash_k0(),
                    *params.siphash_k1(),
                );
                h.write(e);
                map_into_range(h.finish(), f)
            })
            .collect();
        hashed.sort_unstable();

        let mut last = 0u64;
        for v in hashed {
            let delta = v - last;
            golomb_rice_encode(&mut bw, *params.p(), delta);
            last = v;
        }
        bw.flush();

        let x = GCSFilterBuilder::default()
            .params(params.clone())
            .n(n)
            .f(f)
            .encoded(vch_rc.borrow().clone())
            .build()
            .unwrap();

        x
    }
}

#[cfg(test)]
mod gcsfilter_new_with_element_set_tests {
    use super::*;
    use std::iter::FromIterator;

    fn params() -> GcsFilterParams { GcsFilterParams::default() }
    fn bv(s: &[u8]) -> GcsFilterElement { s.to_vec() }

    #[traced_test]
    fn empty_set_produces_header_only() {
        let elements = GcsFilterElementSet::with_hasher(ByteVectorHash::default());
        let filter   = GCSFilter::new_with_element_set(&params(), &elements);

        // CompactSize‑encoded 0 → single byte 0x00
        assert_eq!(filter.get_encoded(), &vec![0x00]);
        assert_eq!(filter.getn(), 0);
        assert_eq!(filter.get_params().p(), &0);
    }

    #[traced_test]
    fn non_empty_roundtrip() {
        let elems = GcsFilterElementSet::from_iter(
            (0..50u8).map(|b| bv(&[b])));
        let filter = GCSFilter::new_with_element_set(&params(), &elems);

        assert_eq!(filter.getn(), 50);
        for e in &elems {
            assert!(filter.match_(e));
        }
    }
}
