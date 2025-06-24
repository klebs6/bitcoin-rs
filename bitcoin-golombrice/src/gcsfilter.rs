// ---------------- [ File: bitcoin-golombrice/src/gcsfilter.rs ]
crate::ix!();

/// Compact probabilistic set (BIP‑158 Golomb‑coded filter).
///
/// This implements a Golomb-coded set as defined in BIP 158. It is a compact,
/// probabilistic data structure for testing set membership.
/// 
#[derive(Builder,Debug, Clone, Getters, Default)]
#[getset(get = "pub")]
#[builder(setter(into))]
pub struct GCSFilter {
    params:  GcsFilterParams,
    /// Number of elements in the filter
    n:       u32,
    /// Range of element hashes, F = N * M
    f:       u64,
    encoded: Vec<u8>,
}

impl From<Option<GcsFilterParams>> for GCSFilter {
    fn from(params: Option<GcsFilterParams>) -> Self {
        let params = params.unwrap_or_default();

        // Serialise CompactSize(0) with a `VectorWriter`.
        let buf_rc = std::rc::Rc::new(std::cell::RefCell::new(Vec::<u8>::new()));
        {
            let mut vw = VectorWriter::new(
                GCS_SER_TYPE as i32,
                GCS_SER_VERSION as i32,
                buf_rc.clone(),
                0,
            );
            write_compact_size(&mut vw, 0);
        }

        let x = Self { params, n: 0, f: 0, encoded: buf_rc.borrow().clone() };

        x
    }
}

impl GCSFilter {

    pub fn getn(&self) -> u32 {
        self.n
    }
    
    pub fn get_params(&self) -> &GcsFilterParams {
        &self.params
    }
    
    pub fn get_encoded(&self) -> &Vec<u8> {
        &self.encoded
    }

    /**
      | Hash a data element to an integer in the
      | range [0, N * M).
      |
      */
    pub fn hash_to_range(&self, element: &GcsFilterElement) -> u64 {
        let mut hasher = SipHasher::new_with_keys(
            *self.params.siphash_k0(),
            *self.params.siphash_k1(),
        );
        hasher.write(element);
        let hash = hasher.finish();
        map_into_range(hash, self.f)
    }
    
    pub fn build_hashed_set(&self, elements: &GcsFilterElementSet) -> Vec<u64> {
        
        let mut hashed_elements = Vec::<u64>::default();

        hashed_elements.reserve(elements.len());

        for element in elements.iter() {
            hashed_elements.push(self.hash_to_range(element));
        }

        hashed_elements.sort();

        hashed_elements
    }

    /**
      | Checks if the element may be in the set.
      | False positives are possible with probability
      | 1/M.
      |
      */
    pub fn match_(&self, element: &GcsFilterElement) -> bool {
        
        let query: u64 = self.hash_to_range(element);
        self.match_internal(&query, 1)
    }
    
    /**
      | Checks if any of the given elements may be
      | in the set. 
      |
      | False positives are possible with
      | probability 1/M per element checked.
      |
      | This is more efficient that checking Match
      | on multiple elements separately.
      |
      */
    pub fn match_any(&self, elements: &GcsFilterElementSet) -> bool {
        
        let queries: Vec<u64> = self.build_hashed_set(elements);
        self.match_internal(queries.as_ptr(), queries.len())
    }
}

#[cfg(test)]
mod gcsfilter_behaviour_tests {
    use super::*;
    use std::iter::FromIterator;

    fn sample_params() -> GcsFilterParams {
        GcsFilterParams::new(None, None, Some(19), Some(784_931))
    }

    fn bytevec(b: &[u8]) -> GcsFilterElement {
        Vec::from(b)
    }

    #[traced_test]
    fn empty_filter_matches_nothing() {
        let filter = GCSFilter::from(Some(sample_params()));
        assert!(!filter.match_(&bytevec(b"anything")),
            "empty filter must never match");
    }

    #[traced_test]
    fn single_element_roundtrip() {
        let params = sample_params();
        let elements = GcsFilterElementSet::from_iter(
            [bytevec(b"satoshi")].into_iter());

        let filter = GCSFilter::new_with_element_set(&params, &elements);
        assert_eq!(filter.getn(), 1);

        // Encode / decode cycle.
        let decoded =
            GCSFilter::new_with_encoded_filter(&params, filter.get_encoded().clone());
        assert!(decoded.match_(&bytevec(b"satoshi")));
        assert!(!decoded.match_(&bytevec(b"nakamoto")));
    }

    #[traced_test]
    fn match_any_efficiency_check() {
        let params = sample_params();
        let elements = GcsFilterElementSet::from_iter(
            [bytevec(b"a"), bytevec(b"b"), bytevec(b"c")].into_iter());

        let filter = GCSFilter::new_with_element_set(&params, &elements);

        // Query two present, one absent.
        let queries = GcsFilterElementSet::from_iter(
            [bytevec(b"c"), bytevec(b"d")].into_iter());

        assert!(filter.match_any(&queries),
            "should match at least one element");
        assert!(!filter.match_(&bytevec(b"d")),
            "specific non‑member must not match deterministically");
    }
}
