// ---------------- [ File: bitcoin-addrman/src/pimpl.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/addrman_impl.h]

pub struct AddrManImpl {

    /**
      | A mutex to protect the inner data structures.
      |
      */
    pub cs:            Mutex<AddrManInner>,

    /**
      | secret key to randomize bucket select
      | with
      |
      */
    pub n_key:         u256,

    /**
      | Holds addrs inserted into tried table that
      | collide with existing
      | entries. Test-before-evict discipline used
      | to resolve these collisions.
      */
    pub tried_collisions:        HashSet<i32>,

    /**
      | Perform consistency checks every
      | m_consistency_check_ratio operations (if
      | non-zero).
      |
      */
    pub consistency_check_ratio: i32,

    /**
      | Compressed IP->ASN mapping, loaded from
      | a file when a node starts.
      |
      | Should be always empty if no file was
      | provided.
      |
      | This mapping is then used for bucketing
      | nodes in Addrman.
      |
      | If asmap is provided, nodes will be
      | bucketed by AS they belong to, in order to
      | make impossible for a node to connect to
      | several nodes hosted in a single AS.
      |
      | This is done in response to Erebus attack,
      | but also to generally diversify the
      | connections every node creates, especially
      | useful when a large fraction of nodes
      | operate under a couple of cloud providers.
      |
      | If a new asmap was provided, the existing
      | records would be re-bucketed accordingly.
      */
    pub asmap:                   Vec<bool>,
}

impl Drop for AddrManImpl {
    fn drop(&mut self) {
        self.n_key.blob.set_null();
    }
}

impl AddrManImpl {

    pub fn new(
        asmap:                   Vec<bool>,
        deterministic:           bool,
        consistency_check_ratio: i32) -> Self {

        let mut insecure_rand 
            = FastRandomContext::new(deterministic);

        let n_key: u256 = match deterministic {
            true  => u256::from(1),
            false => insecure_rand.rand256(),
        };

        Self {
            cs:                      Mutex::new(AddrManInner::new(deterministic, insecure_rand)),
            n_key:                   n_key,
            tried_collisions:        HashSet::<i32>::default(),
            consistency_check_ratio: consistency_check_ratio,
            asmap:                   asmap,
        }
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(!cs)]
    pub fn len(&self) -> usize {
        
        let inner = self.cs.lock();

        // TODO: Cache this in an atomic to avoid
        // this overhead
        let result = inner.random.borrow().len();

        result
    }

    #[EXCLUSIVE_LOCKS_REQUIRED(!cs)]
    pub fn add(&mut self, 
        addrs:          &Vec<Address>,
        source:         &NetAddr,
        n_time_penalty: i64) -> bool {

        let mut inner = self.cs.lock();

        let mut n_add: i32 = 0;

        inner.check(self.consistency_check_ratio, &self.n_key, &self.asmap);

        for addr in addrs.iter() {

            n_add += match inner.add(
                addr,
                source,
                &self.n_key,
                &self.asmap,
                n_time_penalty) {
                true  => 1,
                false => 0,
            };
        }

        inner.check(self.consistency_check_ratio, &self.n_key, &self.asmap);

        if n_add != 0 {
            log_print!(
                LogFlags::ADDRMAN, 
                format!{
                    "Added {} addresses from {}: {} tried, {} new\n", 
                    n_add, 
                    source.ToString(), 
                    n_tried, 
                    n_new
                }
            );
        }

        return n_add > 0;

        panic!("could not get lock");
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(!cs)]
    pub fn good(&mut self, 
        addr:   &Service,
        n_time: i64)  {

        let test_before_evict = true;

        let mut inner = self.cs.lock();

        inner.check(self.consistency_check_ratio, &self.n_key, &self.asmap);

        unsafe {
            inner.good(addr, test_before_evict, n_time, &self.n_key, &self.asmap, &mut self.tried_collisions);
        }

        inner.check(self.consistency_check_ratio, &self.n_key, &self.asmap);
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(!cs)]
    pub fn attempt(&mut self, 
        addr:          &Service,
        count_failure: bool,
        n_time:        i64)  {

        let mut inner = self.cs.lock();

        inner.check(self.consistency_check_ratio, &self.n_key, &self.asmap);

        inner.attempt(addr, count_failure, n_time);

        inner.check(self.consistency_check_ratio, &self.n_key, &self.asmap);
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(!cs)]
    pub fn resolve_collisions(&mut self)  {
        
        let mut inner = self.cs.lock();

        inner.check(self.consistency_check_ratio, &self.n_key, &self.asmap);

        //I think this is probably unsafe because
        //we iterate over the set, removing
        //certain items which don't match
        //a criteria, but then we try to mutably
        //borrow this same set within the removal
        //procedure, updating and possibly
        //inserting items with good.  in any case,
        //this routine (and the routine "good")
        //needs to be checked for bugs
        unsafe {
            inner.resolve_collisions(&self.n_key, &self.asmap, &mut self.tried_collisions);
        }

        inner.check(self.consistency_check_ratio, &self.n_key, &self.asmap);
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(!cs)]
    pub fn select_tried_collision(&mut self) -> (Address,i64) {
        
        let mut inner = self.cs.lock();

        inner.check(self.consistency_check_ratio, &self.n_key, &self.asmap);

        let ret = inner.select_tried_collision(&self.n_key, &self.asmap, &mut self.tried_collisions);

        inner.check(self.consistency_check_ratio, &self.n_key, &self.asmap);

        ret
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn select(&self, new_only: bool) -> (Address,i64) {
        
        let inner = self.cs.lock();

        inner.check(self.consistency_check_ratio, &self.n_key, &self.asmap);

        let addr_ret = inner.select(new_only);

        inner.check(self.consistency_check_ratio, &self.n_key, &self.asmap);

        addr_ret
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(!cs)]
    pub fn get_addr(&self, 
        max_addresses: usize,
        max_pct:       usize,
        network:       Option<Network>) -> Vec<Address> {
        
        let inner = self.cs.lock();

        inner.check(self.consistency_check_ratio, &self.n_key, &self.asmap);

        let addresses = inner.get_addr(max_addresses,max_pct,network);

        inner.check(self.consistency_check_ratio, &self.n_key, &self.asmap);

        addresses
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(!cs)]
    pub fn connected(&mut self, 
        addr:   &Service,
        n_time: i64)  {

        let mut inner = self.cs.lock();
        
        inner.check(self.consistency_check_ratio, &self.n_key, &self.asmap);

        inner.connected(addr, n_time);

        inner.check(self.consistency_check_ratio, &self.n_key, &self.asmap);
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(!cs)]
    pub fn set_services(&mut self, 
        addr:       &Service,
        n_services: ServiceFlags)  {
        
        let mut inner = self.cs.lock();

        inner.check(self.consistency_check_ratio, &self.n_key, &self.asmap);

        inner.set_services(addr, n_services);

        inner.check(self.consistency_check_ratio, &self.n_key, &self.asmap);
    }
    
    pub fn get_asmap(&self) -> &Vec<bool> {
        
        &self.asmap
    }
}
