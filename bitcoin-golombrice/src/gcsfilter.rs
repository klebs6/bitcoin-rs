// ---------------- [ File: bitcoin-golombrice/src/gcsfilter.rs ]
crate::ix!();

/**
  | SerType used to serialize parameters
  | in GCS filter encoding.
  |
  */
pub const GCS_SER_TYPE: usize = SER_NETWORK as usize;

/**
  | Protocol version used to serialize
  | parameters in GCS filter encoding.
  |
  */
pub const GCS_SER_VERSION: usize = 0;

/// Compact probabilistic set (BIP‑158 Golomb‑coded filter).
///
/// This implements a Golomb-coded set as defined in BIP 158. It is a compact,
/// probabilistic data structure for testing set membership.
/// 
#[derive(Debug, Clone, Getters, Default)]
#[getset(get = "pub")]
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

        // Encode CompactSize(0) so the empty filter is fully serialised.
        let mut encoded = Vec::<u8>::new();
        {
            let mut w = VectorWriter::new(GCS_SER_TYPE, GCS_SER_VERSION, &mut encoded, 0);
            write_compact_size(&mut w, 0);
        }

        trace!(target: "gcsfilter", "initialised empty GCSFilter");
        Self {
            params,
            n: 0,
            f: 0,
            encoded,
        }
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

    pub fn new_with_encoded_filter(
        params: &GcsFilterParams,
        encoded_filter: Vec<u8>,
    ) -> Self {
        info!(target: "gcsfilter", bytes = encoded_filter.len(), "decoding GCSFilter");

        let mut stream = VectorReader::new(
            GCS_SER_TYPE.try_into().unwrap(),
            GCS_SER_VERSION.try_into().unwrap(),
            &encoded_filter,
            0,
        );

        let n_u64 = read_compact_size(&mut stream, None);
        let n = u32::try_from(n_u64).expect("N must be < 2^32");
        let f = n as u64 * *params.m() as u64;

        let mut bitreader = BitStreamReader::<VectorReader>::new(&mut stream);
        for _ in 0..n {
            let _ = golomb_rice_decode(&mut bitreader, *params.p());
        }
        if !stream.empty() {
            error!(target: "gcsfilter", "encoded_filter contains excess data");
            panic!("encoded_filter contains excess data");
        }

        Self {
            params: params.clone(),
            n,
            f,
            encoded: encoded_filter,
        }
    }

    /// Build a filter from a concrete element set.
    pub fn new_with_element_set(
        params: &GcsFilterParams,
        elements: &GcsFilterElementSet,
    ) -> Self {
        let n = u32::try_from(elements.len()).expect("N must be < 2^32");
        let f = n as u64 * *params.m() as u64;

        // Serialise CompactSize‑encoded N.
        let mut encoded = Vec::<u8>::new();
        let mut stream = VectorWriter::new(
            GCS_SER_TYPE,
            GCS_SER_VERSION,
            &mut encoded,
            0,
        );
        write_compact_size(&mut stream, n.into());

        if elements.is_empty() {
            return Self { params: params.clone(), n, f, encoded };
        }

        // Hash, sort, encode deltas.
        let mut hashed: Vec<u64> = elements
            .iter()
            .map(|e| {
                let mut h = SipHasher::new_with_keys(*params.siphash_k0(), *params.siphash_k1());
                h.write(e);
                map_into_range(h.finish(), f)
            })
            .collect();
        hashed.sort_unstable();

        let mut bw = BitStreamWriter::<VectorWriter>::new(&mut stream);
        let mut last = 0u64;
        for v in hashed {
            let delta = v - last;
            golomb_rice_encode(&mut bw, *params.p(), delta);
            last = v;
        }
        bw.flush();

        debug!(target: "gcsfilter", n, bytes = encoded.len(), "built GCSFilter");

        Self { params: params.clone(), n, f, encoded }
    }

    /**
      | Helper method used to implement Match
      | and MatchAny
      |
      */
    pub fn match_internal(
        &self,
        element_hashes: *const u64,
        size:           usize,
    ) -> bool {
        let mut stream = VectorReader::new(
            GCS_SER_TYPE.try_into().unwrap(),
            GCS_SER_VERSION.try_into().unwrap(),
            &self.encoded,
            0,
        );

        let n = read_compact_size(&mut stream, None);
        debug_assert_eq!(n as u32, self.n);

        let mut br = BitStreamReader::<VectorReader>::new(&mut stream);
        let mut value = 0u64;
        let mut idx = 0usize;

        for _ in 0..self.n {
            let delta = golomb_rice_decode(&mut br, *self.params.p());
            value += delta;

            loop {
                if idx == size {
                    return false;
                }
                unsafe {
                    let query = *element_hashes.add(idx);
                    if query == value {
                        return true;
                    }
                    if query > value {
                        break;
                    }
                }
                idx += 1;
            }
        }
        false
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
