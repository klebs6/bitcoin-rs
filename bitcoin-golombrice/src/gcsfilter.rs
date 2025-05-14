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

/**
  | This implements a Golomb-coded set
  | as defined in BIP 158. It is a compact,
  | probabilistic data structure for testing
  | set membership.
  |
  */
#[derive(Default)]
pub struct GCSFilter {

    pub params:  gcs_filter::Params,

    /**
      | Number of elements in the filter
      |
      */
    pub n:       u32,

    /**
      | Range of element hashes, F = N * M
      |
      */
    pub f:       u64,

    pub encoded: Vec<u8>,
}

pub mod gcs_filter {

    use super::*;

    pub type Element    = Vec<u8>;
    pub type ElementSet = HashSet<Element,ByteVectorHash>;

    pub struct Params {

        pub siphash_k0: u64,
        pub siphash_k1: u64,

        /**
          | Golomb-Rice coding parameter
          |
          */
        pub p:          u8,

        /**
          | Inverse false positive rate
          |
          */
        pub m:          u32,
    }

    impl Default for Params {
        fn default() -> Self {
            Params::new(None,None,None,None)
        }
    }

    impl Params {

        pub fn new(
            siphash_k0: Option<u64>,
            siphash_k1: Option<u64>,
            p:          Option<u8>,
            m:          Option<u32>) -> Self {

            let siphash_k0: u64 = siphash_k0.unwrap_or(0);
            let siphash_k1: u64 = siphash_k1.unwrap_or(0);

            let p: u8  = p.unwrap_or(0);
            let m: u32 = m.unwrap_or(1);

            todo!();
            /*
            : siphash_k0(siphash_k0),
            : siphash_k1(siphash_k1),
            : p(P),
            : m(M),

            
            */
        }
    }
}

impl From<Option<gcs_filter::Params>> for GCSFilter {

    /**
      | Constructs an empty filter.
      |
      */
    fn from(params: Option<gcs_filter::Params>) -> Self {
        let params: gcs_filter::Params = params.unwrap_or(gcs_filter::Params::default());
    
        todo!();
        /*
            : m_params(params), m_N(0), m_F(0), m_encoded{0}
        */
    }
}

impl GCSFilter {

    pub fn getn(&self) -> u32 {
        self.n
    }
    
    pub fn get_params(&self) -> &gcs_filter::Params {
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
    pub fn hash_to_range(&self, element: &gcs_filter::Element) -> u64 {
        
        let mut hasher = SipHasher::new_with_keys(
            self.params.siphash_k0,
            self.params.siphash_k1
        );

        let slice = unsafe {
            std::slice::from_raw_parts(element.as_ptr(), element.len())
        };

        hasher.write(slice);

        let hash = hasher.finish();

        map_into_range(hash, self.f)
    }
    
    pub fn build_hashed_set(&self, elements: &gcs_filter::ElementSet) -> Vec<u64> {
        
        let mut hashed_elements = Vec::<u64>::default();

        hashed_elements.reserve(elements.len());

        for element in elements.iter() {
            hashed_elements.push(self.hash_to_range(element));
        }

        hashed_elements.sort();

        hashed_elements
    }
    
    /**
      | Reconstructs an already-created filter
      | from an encoding.
      |
      */
    pub fn new_with_encoded_filter(
        params:         &gcs_filter::Params,
        encoded_filter: Vec<u8>) -> Self {
    
        todo!();
        /*


            : m_params(params), m_encoded(std::move(encoded_filter))

        VectorReader stream(GCS_SER_TYPE, GCS_SER_VERSION, m_encoded, 0);

        uint64_t N = ReadCompactSize(stream);
        m_N = static_cast<uint32_t>(N);
        if (m_N != N) {
            throw std::ios_base::failure("N must be <2^32");
        }
        m_F = static_cast<uint64_t>(m_N) * static_cast<uint64_t>(m_params.m_M);

        // Verify that the encoded filter contains exactly N elements. If it has too much or too little
        // data, a std::ios_base::failure exception will be raised.
        BitStreamReader<VectorReader> bitreader(stream);
        for (uint64_t i = 0; i < m_N; ++i) {
            GolombRiceDecode(bitreader, m_params.m_P);
        }
        if (!stream.empty()) {
            throw std::ios_base::failure("encoded_filter contains excess data");
        }
        */
    }
    
    /**
      | Builds a new filter from the params and
      | set of elements.
      |
      */
    pub fn new_with_element_set(
        params:   &gcs_filter::Params,
        elements: &gcs_filter::ElementSet) -> Self {
    
        todo!();
        /*


            : m_params(params)

        size_t N = elements.size();
        m_N = static_cast<uint32_t>(N);
        if (m_N != N) {
            throw invalid_argument("N must be <2^32");
        }
        m_F = static_cast<uint64_t>(m_N) * static_cast<uint64_t>(m_params.m_M);

        CVectorWriter stream(GCS_SER_TYPE, GCS_SER_VERSION, m_encoded, 0);

        WriteCompactSize(stream, m_N);

        if (elements.empty()) {
            return;
        }

        BitStreamWriter<CVectorWriter> bitwriter(stream);

        uint64_t last_value = 0;
        for (uint64_t value : BuildHashedSet(elements)) {
            uint64_t delta = value - last_value;
            GolombRiceEncode(bitwriter, m_params.m_P, delta);
            last_value = value;
        }

        bitwriter.Flush();
        */
    }
    
    /**
      | Helper method used to implement Match
      | and MatchAny
      |
      */
    pub fn match_internal(&self, 
        element_hashes: *const u64,
        size:           usize) -> bool {
        
        let mut stream: VectorReader = VectorReader::new(
            GCS_SER_TYPE.try_into().unwrap(), 
            GCS_SER_VERSION.try_into().unwrap(), 
            &self.encoded, 
            0
        );

        //  Seek forward by size of N
        let N: u64 = read_compact_size(&mut stream, None);

        assert!(N == self.n.into());

        let mut bitreader: BitStreamReader::<VectorReader> 
            = BitStreamReader::<VectorReader>::new(&mut stream);

        let mut value: u64 = 0;
        let mut hashes_index: usize = 0;

        for i in 0..self.n {

            let delta: u64 = golomb_rice_decode(&mut bitreader,self.params.p);
            value += delta;

            while true{

                if hashes_index == size {
                    return false;

                } else {

                    unsafe {
                        if *element_hashes.add(hashes_index) == value {
                            return true;
                        } else {
                            if *element_hashes.add(hashes_index) > value {
                                break;
                            }
                        }
                    }
                }

                hashes_index += 1;
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
    pub fn match_(&self, element: &gcs_filter::Element) -> bool {
        
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
    pub fn match_any(&self, elements: &gcs_filter::ElementSet) -> bool {
        
        let queries: Vec<u64> = self.build_hashed_set(elements);
        self.match_internal(queries.as_ptr(), queries.len())
    }
}
