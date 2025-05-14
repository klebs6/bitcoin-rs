// ---------------- [ File: bitcoin-addrman/src/info.rs ]
crate::ix!();

/**
  | Extended statistics about a CAddress
  |
  */
#[derive(Serialize,Deserialize,Clone)]
pub struct AddrInfo {

    pub address:              Address,

    /**
      | last try whatsoever by us (memory only)
      |
      */
    #[serde(skip)]
    pub n_last_try:           i64,

    /**
      | last counted attempt (memory only)
      |
      */
    #[serde(skip)]
    pub n_last_count_attempt: i64,

    /**
      | where knowledge about this address
      | first came from
      |
      */
    pub source:               NetAddr,

    /**
      | last successful connection by us
      |
      */
    pub n_last_success:       i64,

    /**
      | connection attempts since last successful
      | attempt
      |
      */
    pub n_attempts:           i32,

    /**
      | reference count in new sets (memory
      | only)
      |
      */
    #[serde(skip)]
    pub n_ref_count:          i32,

    /**
      | in tried set? (memory only)
      |
      */
    #[serde(skip)]
    pub in_tried:             bool,

    /**
      | position in vRandom
      |
      */
    #[serde(skip)]
    pub n_random_pos:         RefCell<i32>,
}

impl Default for AddrInfo {
    
    fn default() -> Self {
        Self {
            address:              Address::default(),
            n_last_try:           0,
            n_last_count_attempt: 0,
            source:               NetAddr::default(),
            n_last_success:       0,
            n_attempts:           0,
            n_ref_count:          0,
            in_tried:             false,
            n_random_pos:         RefCell::new(-1),
        }
    }
}

impl AddrInfo {

    pub fn new(
        addr_in:     Address,
        addr_source: NetAddr) -> Self {
    
        Self {
            address:              addr_in,
            n_last_try:           0,
            n_last_count_attempt: 0,
            source:               addr_source,
            n_last_success:       0,
            n_attempts:           0,
            n_ref_count:          0,
            in_tried:             false,
            n_random_pos:         RefCell::new(-1),
        }
    }
    
    fn get_writer() -> HashWriter {
        HashWriter::new(SER_GETHASH.try_into().unwrap(), 0)
    }
    
    /**
      | Calculate in which "new" bucket this
      | entry belongs, using its default source
      |
      */
    pub fn get_new_bucket(&self, 
        n_key: &u256,
        asmap: &Vec<bool>) -> i32 {

        self.get_new_bucket_given_source(
            n_key, 
            &self.source, 
            asmap
        )
    }

    /**
      | Calculate in which "tried" bucket this
      | entry belongs
      |
      */
    pub fn get_tried_bucket(&self, 
        n_key: &u256,
        asmap: &Vec<bool>) -> i32 {

        let hash1: u64 = {

            let w       = Self::get_writer();
            let svc_key = self.address.service.get_key();

            (w << n_key << &svc_key).get_cheap_hash()
        };

        let hash2: u64 = {

            let w      = Self::get_writer();
            let group  = self.source.get_group(asmap);
            let h1_mod = hash1 % (ADDRMAN_TRIED_BUCKETS_PER_GROUP as u64);

            (w << n_key << &group << &h1_mod).get_cheap_hash()
        };

        (hash2 % (ADDRMAN_TRIED_BUCKET_COUNT as u64)).try_into().unwrap()
    }

    /**
      | Calculate in which "new" bucket this
      | entry belongs, given a certain source
      |
      */
    pub fn get_new_bucket_given_source(&self, 
        n_key: &u256,
        src:   &NetAddr,
        asmap: &Vec<bool>) -> i32 {
        
        let vch_source_group_key: Vec<u8> = src.get_group(asmap);

        let hash1: u64 = {

            let w = Self::get_writer();

            let g = self.source.get_group(asmap);

            (w << n_key << &g << &vch_source_group_key).get_cheap_hash()
        };


        let hash2: u64 = {

            let w      = Self::get_writer();

            let h1_mod = (hash1 % (ADDRMAN_NEW_BUCKETS_PER_SOURCE_GROUP as u64));

            (w << n_key << &vch_source_group_key << &h1_mod).get_cheap_hash()
        };

        (hash2 % (ADDRMAN_NEW_BUCKET_COUNT as u64)).try_into().unwrap()
    }
    
    /**
      | Calculate in which position of a bucket
      | to store this entry.
      |
      */
    pub fn get_bucket_position(&self, 
        n_key:    &u256,
        f_new:    bool,
        n_bucket: usize) -> i32 {

        let hash1: u64 = {

            let w       = Self::get_writer();
            let tag     = if f_new { 'N' as u8 } else { 'K' as u8 };
            let svc_key = self.address.service.get_key();

            (w << n_key << &tag << &n_bucket << &svc_key).get_cheap_hash()
        };

        (hash1 % (ADDRMAN_BUCKET_SIZE as u64)).try_into().unwrap()
    }
    
    /**
      | Determine whether the statistics about
      | this entry are bad enough so that it can
      | just be deleted
      |
      */
    pub fn is_terrible(&self, n_now: Option<i64>) -> bool {

        let n_now:  i64 = n_now.unwrap_or(get_adjusted_time());
        let n_time: i64 = self.address.n_time as i64;

        if self.n_last_try != 0 && self.n_last_try >= n_now - 60 {
            // never remove things tried in the last minute
            return false;
        }

        if n_time > n_now + 10 * 60 {
            // came in a flying DeLorean
            return true;
        }

        if n_time == 0 
            || n_now - n_time > (ADDRMAN_HORIZON_DAYS * 24 * 60 * 60).try_into().unwrap()  
        {
            // not seen in recent history
            return true;
        }

        if self.n_last_success == 0 
            && self.n_attempts >= (ADDRMAN_RETRIES.try_into().unwrap()) 
        { 
            // tried N times and never a success
            return true;
        }

        if n_now - self.n_last_success > (ADDRMAN_MIN_FAIL_DAYS * 24 * 60 * 60) as i64 
            && self.n_attempts >= (ADDRMAN_MAX_FAILURES as i32) 
        {
            // N successive failures in the last week
            return true;
        }

        false
    }
    
    /**
      | Calculate the relative chance this
      | entry should be given when selecting
      | nodes to connect to
      |
      */
    pub fn get_chance(&self, n_now: Option<i64>) -> f64 {
        let n_now: i64 = n_now.unwrap_or(get_adjusted_time());
        
        let mut chance: f64 = 1.0;

        let n_since_last_try: i64 = std::cmp::max(n_now - self.n_last_try, 0);

        //deprioritize very recent attempts away
        if n_since_last_try < 60 * 10 {
            chance *= 0.01;
        }

        // deprioritize 66% after each failed
        // attempt, but at most 1/28th to avoid
        // the search taking forever or overly
        // penalizing outages.
        chance *= 0.66_f64.powf(std::cmp::min(self.n_attempts, 8) as f64);

        chance
    }
}
