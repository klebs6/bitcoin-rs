// ---------------- [ File: bitcoin-golombrice/src/match_internal.rs ]
crate::ix!();

impl GCSFilter {

    pub fn match_internal(
        &self,
        element_hashes: *const u64,
        size:           usize,
    ) -> bool {
        use std::{cell::RefCell, rc::Rc, sync::Arc};

        let rdr_arc = Arc::new(self.encoded().clone());
        let mut vr  = VectorReader::new(
            GCS_SER_TYPE as i32,
            GCS_SER_VERSION as i32,
            rdr_arc,
            0,
        );

        let n_read = read_compact_size(&mut vr, None);
        assert_eq!(n_read as u32, *self.n());

        let rc_vr = Rc::new(RefCell::new(vr));
        let mut br = BitStreamReader::<VectorReader>::new(rc_vr.clone());

        let mut value = 0u64;
        let mut idx   = 0usize;

        for _ in 0..*self.n() {
            value += golomb_rice_decode(&mut br, *self.params().p());

            loop {
                if idx == size { return false; }
                unsafe {
                    let q = *element_hashes.add(idx);
                    if q == value { return true; }
                    if q >  value { break; }
                }
                idx += 1;
            }
        }
        false
    }
}

// ----------[ bitcoin-golombrice/src/gcsfilter.rs ]----------
#[cfg(test)]
mod gcsfilter_match_internal_tests {
    use super::*;
    use std::iter::FromIterator;

    fn params() -> GcsFilterParams { GcsFilterParams::default() }
    fn bv(s: &[u8]) -> GcsFilterElement { s.to_vec() }

    /// Build a filter containing two items and
    /// test `match_internal` directly with raw pointers.
    #[traced_test]
    fn pointer_based_match() {
        let elements = GcsFilterElementSet::from_iter(
            [bv(b"k1"), bv(b"k2")].into_iter());

        let filter = GCSFilter::new_with_element_set(&params(), &elements);
        let hash_k1 = filter.hash_to_range(&bv(b"k1"));
        let hash_absent = filter.hash_to_range(&bv(b"zzz"));
        let queries = [hash_k1, hash_absent];

        // SAFETY: we pass a pointer valid for the length argument.
        let found = unsafe { filter.match_internal(queries.as_ptr(), queries.len()) };
        assert!(found, "expected to match one query");

        let absent_only = [hash_absent];
        let found_none = unsafe { filter.match_internal(absent_only.as_ptr(), absent_only.len()) };
        assert!(!found_none, "absent value must not match");
    }
}
