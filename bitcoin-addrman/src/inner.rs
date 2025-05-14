// ---------------- [ File: bitcoin-addrman/src/inner.rs ]
crate::ix!();

pub struct AddrManInner {

    /**
      | Source of random numbers for randomization
      | in inner loops
      |
      */
    pub insecure_rand: RefCell<FastRandomContext>,

    /**
      | last used nId
      |
      */
    pub n_id_count:  i32,

    /**
      | table with information about all n_ids
      |
      */
    pub map_info:    HashMap<i32,AddrInfo>,

    /**
      | find an nId based on its network address
      | and port.
      |
      */
    pub map_addr:    HashMap<Service,i32,ServiceHash>,

    /**
      | randomly-ordered vector of all n_ids
      |
      | This is mutable because it is unobservable
      | outside the class, so any changes to it
      | (even in const methods) are also
      | unobservable.
      */
    pub random:      RefCell<Vec<i32>>,

    /**
      | number of "tried" entries
      |
      */
    pub n_tried:     i32,

    /**
      | list of "tried" buckets
      |
      */
    pub vv_tried:    AddrManTriedBucketList,

    /**
      | number of (unique) "new" entries
      |
      */
    pub n_new:       i32,

    /**
      | list of "new" buckets
      |
      */
    pub vv_new:      AddrManNewBucketList,

    /**
      | last time Good was called (memory
      | only). Initially set to 1 so that "never" is
      | strictly worse.
      */
    pub n_last_good: i64,
}

impl AddrManInner {

    pub fn default_new_bucket_list() -> AddrManNewBucketList {
        [[-1; ADDRMAN_NEW_BUCKET_COUNT]; ADDRMAN_BUCKET_SIZE]
    }

    pub fn default_tried_bucket_list() -> AddrManTriedBucketList {
        [[-1; ADDRMAN_TRIED_BUCKET_COUNT]; ADDRMAN_BUCKET_SIZE]
    }

    pub fn new(deterministic: bool, insecure_rand: FastRandomContext) -> Self {

        Self {
            insecure_rand: RefCell::new(insecure_rand),
            n_id_count:    0,
            map_info:      HashMap::<i32,AddrInfo>::default(),
            map_addr:      HashMap::<Service,i32,ServiceHash>::default(),
            random:        RefCell::new(vec![]),
            n_tried:       0,
            vv_tried:      Self::default_tried_bucket_list(),
            n_new:         0,
            vv_new:        Self::default_new_bucket_list(),
            n_last_good:   1,
        }
    }
}
