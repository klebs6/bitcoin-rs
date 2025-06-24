// ---------------- [ File: bitcoin-golombrice/src/new_with_encoded_filter.rs ]
crate::ix!();

impl GCSFilter {

    pub fn new_with_encoded_filter(
        params:        &GcsFilterParams,
        encoded_filter: Vec<u8>,
    ) -> Self {
        use std::{cell::RefCell, rc::Rc, sync::Arc};

        let data_arc = Arc::new(encoded_filter.clone());
        let mut rdr  = VectorReader::new(
            GCS_SER_TYPE as i32,
            GCS_SER_VERSION as i32,
            data_arc.clone(),
            0,
        );

        // 1. element count
        let n_u64 = read_compact_size(&mut rdr, None);
        let n     = u32::try_from(n_u64).expect("N must be < 2^32");
        let f     = n as u64 * *params.m() as u64;

        // 2. verify length by decoding
        let rc_rdr     = Rc::new(RefCell::new(rdr));
        let mut br     = BitStreamReader::<VectorReader>::new(rc_rdr.clone());
        for _ in 0..n {
            let _ = golomb_rice_decode(&mut br, *params.p());
        }
        if rc_rdr.borrow().size() != 0 {
            panic!("encoded_filter contains excess data");
        }

        GCSFilterBuilder::default()
            .params(params.clone())
            .n(n)
            .f(f)
            .encoded(encoded_filter)
            .build()
            .unwrap()
    }
}

#[cfg(test)]
mod gcsfilter_new_with_encoded_filter_tests {
    use super::*;
    use std::iter::FromIterator;

    fn params() -> GcsFilterParams { GcsFilterParams::default() }
    fn bv(s: &[u8]) -> GcsFilterElement { s.to_vec() }

    #[traced_test]
    fn encode_then_decode_consistency() {
        let elems = GcsFilterElementSet::from_iter(
            [bv(b"a"), bv(b"b"), bv(b"c")].into_iter());
        let built  = GCSFilter::new_with_element_set(&params(), &elems);
        let encoded = built.get_encoded().clone();

        let decoded = GCSFilter::new_with_encoded_filter(&params(), encoded);
        for e in &elems {
            assert!(decoded.match_(e));
        }
        assert_eq!(decoded.getn(), built.getn());
    }

    #[test]
    #[should_panic(expected = "encoded_filter contains excess data")]
    fn rejects_trailing_garbage() {
        let elems = GcsFilterElementSet::with_hasher(ByteVectorHash::default());      // empty filter encodes as 1 byte 0x00
        let filter = GCSFilter::new_with_element_set(&params(), &elems);
        let mut bad = filter.get_encoded().clone();
        bad.extend_from_slice(&[0xFF, 0xEE]);        // append garbage
        // This must panic.
        let _ = GCSFilter::new_with_encoded_filter(&params(), bad);
    }
}
