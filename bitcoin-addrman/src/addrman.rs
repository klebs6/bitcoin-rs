crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/addrman.h]

/** 
 | Stochastic address manager
 |
 | Design goals:
 |
 |  - Keep the address tables in-memory, and
 |    asynchronously dump the entire table to
 |    peers.dat.
 |
 |  - Make sure no (localized) attacker can fill
 |    the entire table with his nodes/addresses.
 |
 | To that end:
 |
 |  Addresses are organized into buckets that can
 |  each store up to 64 entries.
 |
 |    ---------------------------------------
 |    Addresses to which our node has not
 |    successfully connected go into 1024 "new"
 |    buckets.
 |
 |      - Based on the address range (/16 for
 |        IPv4) of the source of information, or
 |        if an asmap is provided, the AS it
 |        belongs to (for IPv4/IPv6), 64 buckets
 |        are selected at random.
 |
 |      - The actual bucket is chosen from one of
 |        these, based on the range in which the
 |        address itself is located.
 |
 |      - The position in the bucket is chosen
 |        based on the full address.
 |
 |      - One single address can occur in up to
 |        8 different buckets to increase
 |        selection chances for addresses that are
 |        seen frequently. The chance for
 |        increasing this multiplicity decreases
 |        exponentially.
 |
 |      - When adding a new address to an occupied
 |        position of a bucket, it will not
 |        replace the existing entry unless that
 |        address is also stored in another bucket
 |        or it doesn't meet one of several
 |        quality criteria (see IsTerrible for
 |        exact criteria).
 |
 |    ---------------------------------------
 |    Addresses of nodes that are known to be
 |    accessible go into 256 "tried" buckets.
 |
 |      - Each address range selects at random
 |        8 of these buckets.
 |
 |      - The actual bucket is chosen from one of
 |        these, based on the full address.
 |
 |      - When adding a new good address to an
 |        occupied position of a bucket, a FEELER
 |        connection to the old address is
 |        attempted. The old entry is only
 |        replaced and moved back to the "new"
 |        buckets if this attempt was
 |        unsuccessful.
 |
 |    ---------------------------------------
 |    Bucket selection is based on cryptographic
 |    hashing, using a randomly-generated 256-bit
 |    key, which should not be observable by
 |    adversaries.
 |
 |    ---------------------------------------
 |    Several indexes are kept for high
 |    performance. Setting m_consistency_check_ratio 
 |    with the -checkaddrman configuration option will
 |    introduce (expensive) consistency checks for
 |    the entire data structure.
 */
pub struct AddrMan {
    impl_: Box<AddrManImpl>,
}

//-------------------------------------------[.cpp/bitcoin/src/addrman.cpp]
impl AddrMan {

    pub fn new(
        asmap:                   Vec<bool>,
        deterministic:           bool,
        consistency_check_ratio: i32) -> Self {
        Self {
            impl_: Box::new(AddrManImpl::new(
                           asmap,
                           deterministic,
                           consistency_check_ratio))
        }
    }
    
    pub fn serialize<Stream: GetVersion + GetType>(&self, s: &mut Stream)  {
        self.impl_.serialize(s);
    }
    
    pub fn unserialize<Stream: GetVersion + GetType>(&mut self, s: &mut Stream)  {
        self.impl_.unserialize(s);
    }
    
    /**
      | Return the number of (unique) addresses
      | in all tables.
      |
      */
    pub fn len(&self) -> usize {
        self.impl_.len()
    }
    
    /**
      | Add addresses to addrman's new table.
      |
      */
    pub fn add(&mut self, 
        addr:           &Vec<Address>,
        source:         &NetAddr,
        n_time_penalty: Option<i64>) -> bool {

        let n_time_penalty: i64 = n_time_penalty.unwrap_or(0);

        self.impl_.add(addr,source,n_time_penalty)
    }
    
    /**
      | Mark an entry as accessible, possibly
      | moving it from "new" to "tried".
      |
      */
    pub fn good(&mut self, 
        addr:   &Service,
        n_time: Option<i64>)  {

        let n_time: i64 = n_time.unwrap_or(get_adjusted_time());

        self.impl_.good(addr,n_time);
    }
    
    /**
      | Mark an entry as connection attempted
      | to.
      |
      */
    pub fn attempt(&mut self, 
        addr:          &Service,
        count_failure: bool,
        n_time:        Option<i64>)  {

        let n_time: i64 = n_time.unwrap_or(get_adjusted_time());
        
        self.impl_.attempt(addr, count_failure, n_time);
    }
    
    /**
      | See if any to-be-evicted tried table
      | entries have been tested and if so resolve
      | the collisions.
      |
      */
    pub fn resolve_collisions(&mut self)  {
        
        self.impl_.resolve_collisions();
    }
    
    /**
      | Randomly select an address in the tried
      | table that another address is attempting
      | to evict.
      | 
      | -----------
      | @return
      | 
      | CAddress The record for the selected
      | tried peer. int64_t The last time we
      | attempted to connect to that peer.
      |
      */
    pub fn select_tried_collision(&mut self) -> (Address,i64) {
        self.impl_.select_tried_collision()
    }
    
    /**
      | Choose an address to connect to.
      | 
      | -----------
      | @param[in] newOnly
      | 
      | Whether to only select addresses from
      | the new table.
      | 
      | -----------
      | @return
      | 
      | CAddress The record for the selected
      | peer. int64_t The last time we attempted
      | to connect to that peer.
      |
      */
    pub fn select(&self, new_only: Option<bool>) -> (Address,i64) {

        let new_only: bool = new_only.unwrap_or(false);
        
        self.impl_.select(new_only)
    }
    
    /**
      | Return all or many randomly selected
      | addresses, optionally by network.
      | 
      | -----------
      | @param[in] max_addresses
      | 
      | Maximum number of addresses to return
      | (0 = all).
      | ----------
      | @param[in] max_pct
      | 
      | Maximum percentage of addresses to
      | return (0 = all).
      | ----------
      | @param[in] network
      | 
      | Select only addresses of this network
      | (nullopt = all).
      | 
      | -----------
      | @return
      | 
      | A vector of randomly selected addresses
      | from vRandom.
      |
      */
    pub fn get_addr(&self, 
        max_addresses: usize,
        max_pct:       usize,
        network:       Option<Network>) -> Vec<Address> {
        
        self.impl_.get_addr(max_addresses, max_pct, network)
    }
    
    /**
      | We have successfully connected to this
      | peer. Calling this function updates
      | the CAddress's nTime, which is used
      | in our IsTerrible() decisions and gossiped
      | to peers. Callers should be careful
      | that updating this information doesn't
      | leak topology information to network
      | spies. net_processing calls this function
      | when it *disconnects* from a peer to
      | not leak information about currently
      | connected peers.
      | 
      | -----------
      | @param[in] addr
      | 
      | The address of the peer we were connected
      | to
      | ----------
      | @param[in] nTime
      | 
      | The time that we were last connected
      | to this peer
      |
      */
    pub fn connected(&mut self, 
        addr:   &Service,
        n_time: Option<i64>)  {

        let n_time: i64 = n_time.unwrap_or(get_adjusted_time());
        
        self.impl_.connected(addr,n_time)
    }
    
    /**
      | Update an entry's service bits.
      |
      */
    pub fn set_services(&mut self, 
        addr:       &Service,
        n_services: ServiceFlags)  {
        
        self.impl_.set_services(addr,n_services)
    }
    
    pub fn get_asmap(&self) -> &Vec<bool> {
        
        self.impl_.get_asmap()
    }
}
