crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/blockfilter.h]
//-------------------------------------------[.cpp/bitcoin/src/blockfilter.cpp]

/**
  | Complete block filter struct as defined
  | in BIP 157. Serialization matches payload
  | of "cfilter" messages.
  |
  */
pub struct BlockFilter {
    filter_type: BlockFilterType,
    block_hash:  u256,
    filter:      GCSFilter,
}

impl Default for BlockFilter {
    fn default() -> Self {
        Self {
            filter_type: BlockFilterType::INVALID,
            ..Default::default()
        }
    }
}

impl BlockFilter {

    pub fn get_filter_type(&self) -> BlockFilterType {
        self.filter_type.clone()
    }
    
    pub fn get_block_hash(&self) -> &u256 {
        &self.block_hash
    }
    
    pub fn get_filter(&self) -> &GCSFilter {
        &self.filter
    }
    
    pub fn get_encoded_filter(&self) -> &Vec<u8> {
        self.filter.get_encoded()
    }

    pub fn serialize<Stream: StreamItems>(&self, s: &mut Stream)  {
    
        s.stream(self.filter_type.clone() as u8);
        s.stream(self.block_hash.clone());
        s.stream(self.filter.get_encoded());
    }
    
    pub fn unserialize<Stream: StreamInto>(&mut self, s: &mut Stream) -> Result<(), StdException> {
    
        let mut encoded_filter = vec![];

        let mut filter_type = 0;

        s.stream_into(&mut filter_type);
        s.stream_into(&mut self.block_hash);
        s.stream_into(&mut encoded_filter);

        self.filter_type = match filter_type {
            0   => BlockFilterType::BASIC,
            255 => BlockFilterType::INVALID,
            _   => panic!("invalid filter_type"),
        };

        let mut params = gcs_filter::Params::default();

        if !self.build_params(&mut params) {
            return Err(ios_base_failure("unknown filter_type"));
        }

        self.filter = GCSFilter::new_with_encoded_filter(&params,encoded_filter);

        Ok(())
    }

    /**
      | Reconstruct a BlockFilter from parts.
      |
      */
    pub fn new_from_parts(
        filter_type: BlockFilterType,
        block_hash:  &u256,
        filter:      Vec<u8>) -> Result<Self,StdException> {

        let mut x: Self = unsafe { std::mem::zeroed() };
        x.filter_type = filter_type;
        x.block_hash  = block_hash.clone();
    
        let mut params = gcs_filter::Params::default();

        if !x.build_params(&mut params) {
            return Err(invalid_argument("unknown filter_type"));
        }

        x.filter = GCSFilter::new_with_encoded_filter(
            &params,
            filter
        );

        Ok(x)
    }
    
    /**
      | Construct a new BlockFilter of the specified
      | type from a block.
      |
      */
    pub fn new_from_block(
        filter_type: BlockFilterType,
        block:       &Block,
        block_undo:  &BlockUndo) -> Result<Self,StdException> {
    
        let mut x: Self = unsafe { std::mem::zeroed() };
        x.filter_type = filter_type;
        x.block_hash  = block.header.get_hash();

        let mut params = gcs_filter::Params::default();

        if !x.build_params(&mut params) {
            return Err(invalid_argument("unknown filter_type"));
        }

        x.filter = GCSFilter::new_with_element_set(
            &params,
            &basic_filter_elements(block,block_undo)
        );

        Ok(x)
    }
    
    pub fn build_params(&self, params: &mut gcs_filter::Params) -> bool {
        
        match self.filter_type {
            BlockFilterType::BASIC  => {
                params.siphash_k0 = self.block_hash.blob.get_u64(0);
                params.siphash_k1 = self.block_hash.blob.get_u64(1);
                params.p = BASIC_FILTER_P;
                params.m = BASIC_FILTER_M;
                return true;
            },

            BlockFilterType::INVALID  => {
                return false;
            },
        }

        false
    }
    
    /**
      | Compute the filter hash.
      |
      */
    pub fn get_hash(&self) -> u256 {
        
        let data: &Vec::<u8> = self.get_encoded_filter();

        let mut result = u256::default();

        Hash256::default().write(data.as_slice()).finalize(
            unsafe {
                std::slice::from_raw_parts_mut(
                    result.blob.data_mut(), 
                    result.blob.size().try_into().unwrap()
                )
            }
        );

        result
    }
    
    /**
      | Compute the filter header given the
      | previous one.
      |
      */
    pub fn compute_header(&self, prev_header: &u256) -> u256 {
        
        let mut filter_hash: u256 = self.get_hash();

        let mut result = u256::default();

        Hash256::default()
            .write(filter_hash.as_slice())
            .write(prev_header.as_slice())
            .finalize(result.as_slice());

        result
    }
}
