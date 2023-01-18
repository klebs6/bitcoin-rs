crate::ix!();

impl Connman {

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
      */
    pub fn get_addresses(&self, 
        max_addresses: usize,
        max_pct:       usize,
        network:       Option<Network>) -> Vec<Address> {
        
        let mut addresses: Vec::<Address> 
        = self.addrman
            .get()
            .get_addr(max_addresses, max_pct, network);

        let banman = self.banman.get();

        addresses.retain(|item: &Address| {

            let is_discouraged: bool = banman.is_discouraged(&item.service.base);
            let is_banned:      bool = banman.is_netaddr_banned(&item.service.base);

            let delete = is_discouraged || is_banned;

            !delete
        });

        addresses
    }
    
    /**
      | Cache is used to minimize topology leaks,
      | so it should be used for all non-trusted
      | calls, for example, p2p.
      | 
      | A non-malicious call (from RPC or a peer
      | with addr permission) should call the
      | function without a parameter to avoid
      | using the cache.
      |
      */
    pub fn get_addresses_with_requestor(
        &mut self, 
        requestor:     &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        max_addresses: usize,
        max_pct:       usize) -> Vec<Address> {

        let local_socket_bytes = requestor.addr_bind()
            .service
            .base
            .get_addr_bytes();

        let mut randomizer = 
        self.get_deterministic_randomizer(RANDOMIZER_ID_ADDRCACHE);

        randomizer.write_u8(requestor.service().base.get_network() as u8);

        let slice = unsafe {
            std::slice::from_raw_parts(
                local_socket_bytes.as_ptr(), 
                local_socket_bytes.len()
            )
        };

        randomizer.write(slice);

        let cache_id: u64 = randomizer.finish();
        
        let current_time = Instant::now();

        let mut r = self.addr_response_caches.insert(cache_id, ConnmanCachedAddrResponse::default());

        let cache_entry: &mut ConnmanCachedAddrResponse = r.as_mut().unwrap();

        if cache_entry.cache_entry_expiration.unwrap() < current_time {

            //  If emplace() added new one it has expiration 0.
            cache_entry.addrs_response_cache 
                = self.get_addresses(
                    max_addresses,
                    max_pct,
                    /* network */ None
                );

            //  Choosing a proper cache lifetime
            //  is a trade-off between the privacy
            //  leak minimization and the
            //  usefulness of ADDR responses to
            //  honest users.
            //
            //  Longer cache lifetime makes it
            //  more difficult for an attacker to
            //  scrape enough AddrMan data to
            //  maliciously infer something
            //  useful.
            //
            //  By the time an attacker scraped
            //  enough AddrMan records, most of
            //  the records should be old enough
            //  to not leak topology info by
            //  e.g. analyzing real-time changes
            //  in timestamps.
            //
            //  It takes only several hundred
            //  requests to scrape everything from
            //  an AddrMan containing 100,000
            //  nodes, so ~24 hours of cache
            //  lifetime indeed makes the data
            //  less inferable by the time most of
            //  it could be scraped (considering
            //  that timestamps are updated via
            //  ADDR self-announcements and when
            //  nodes communicate).
            //
            //  We also should be robust to those
            //  attacks which may not require
            //  scraping *full* victim's AddrMan
            //  (because even several timestamps
            //  of the same handful of nodes may
            //  leak privacy).
            //
            //  On the other hand, longer cache
            //  lifetime makes ADDR responses
            //  outdated and less useful for an
            //  honest requestor, e.g. if most
            //  nodes in the ADDR response are no
            //  longer active.
            //
            //  However, the churn in the network
            //  is known to be rather low. Since
            //  we consider nodes to be "terrible"
            //  (see IsTerrible()) if the
            //  timestamps are older than 30 days,
            //  max. 
            //
            //  24 hours of "penalty" due to cache
            //  shouldn't make any meaningful
            //  difference in terms of the
            //  freshness of the response.
            cache_entry.cache_entry_expiration 
                = Some(
                    current_time 
                    + Duration::hours(21) 
                    + get_random_duration(Duration::hours(6))
                );
        }

        cache_entry.addrs_response_cache.clone()
    }
}
