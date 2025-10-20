// ---------------- [ File: bitcoin-addrman/src/add.rs ]
crate::ix!();

impl AddrManInner {

    pub fn add(&mut self, 
        addr:               &Address,
        source:             &NetAddr,
        n_key:              &u256,
        asmap:              &Vec<bool>,
        mut n_time_penalty: i64) -> bool {

        if !addr.service.base.is_routable() {
            return false;
        }

        let mut f_new:  bool = false;
        let mut n_id:   i32  = 0;
        let mut p_info: *mut AddrInfo = self.find(&addr.service, Some(&mut n_id));

        // Do not set a penalty for a source's self-announcement
        if addr.service.base == *source {
            n_time_penalty = 0;
        }

        if p_info != null_mut() {

            let info = unsafe{ &mut *p_info };

            // periodically update n_time
            let f_currently_online: bool = get_adjusted_time() - (addr.n_time as i64) < 24 * 60 * 60;

            let n_update_interval: i64 = match f_currently_online {
                true  => 60 * 60,
                false => 24 * 60 * 60,
            };

            let info_address_ntime:       i64 = i64::from(info.address.n_time);
            let positive_addr_ntime:     bool = addr.n_time != 0;
            let info_address_ntime_eq_0: bool = info.address.n_time == 0;
            let addr_ntime:               i64 = addr.n_time as i64;

            if positive_addr_ntime && 
                (info_address_ntime_eq_0 || info_address_ntime < addr_ntime - n_update_interval - n_time_penalty) {

                    info.address.n_time = std::cmp::max(0, (addr.n_time as i64) - n_time_penalty) as u32;
            }

            // add services
            info.address.n_services 
                = ServiceFlags::from(info.address.n_services.bits() | addr.n_services.bits());


            // do not update if no new
            // information is present
            if addr.n_time == 0 || (info.address.n_time != 0 && addr.n_time <= info.address.n_time) {
                return false;
            }

            // do not update if the entry was
            // already in the "tried" table
            if info.in_tried {
                return false;
            }

            // do not update if the max
            // reference count is reached
            if info.n_ref_count == ADDRMAN_NEW_BUCKETS_PER_ADDRESS.try_into().unwrap() {
                return false;
            }

            // stochastic test: previous
            // nRefCount == N: 2^N times
            // harder to increase it
            let mut n_factor: i32 = 1;

            for n in 0..info.n_ref_count {
                n_factor *= 2;
            }

            if n_factor > 1 && (self.insecure_rand.borrow_mut().randrange(n_factor.try_into().unwrap()) != 0) {
                return false;
            }

        } else {

            unsafe { 

                p_info = self.create(addr, source, Some(&mut n_id));

                (*p_info).address.n_time 
                    = std::cmp::max(
                        0, 
                        ((*p_info).address.n_time as u32) - (n_time_penalty as u32)
                    );

                self.n_new += 1;
                f_new = true;
            }
        }

        let info = unsafe{ &mut *p_info };

        let n_ubucket: usize = info.get_new_bucket_given_source(
            n_key, 
            source, 
            asmap
        ).try_into().unwrap();

        let n_ubucket_pos: usize = info.get_bucket_position(
            n_key, 
            true, 
            n_ubucket
        ).try_into().unwrap();

        if self.vv_new[n_ubucket][n_ubucket_pos] != n_id {

            let mut f_insert: bool = self.vv_new[n_ubucket][n_ubucket_pos] == -1;

            if !f_insert {

                let infoExisting: &AddrInfo = &self.map_info[&self.vv_new[n_ubucket][n_ubucket_pos]];

                if infoExisting.is_terrible(None)
                    || (infoExisting.n_ref_count > 1 && info.n_ref_count == 0) 
                {
                    // Overwrite the existing new table entry.
                    f_insert = true;
                }
            }

            if f_insert {

                self.clear_new(n_ubucket, n_ubucket_pos);

                info.n_ref_count += 1;

                self.vv_new[n_ubucket][n_ubucket_pos] = n_id;

                log_print!{
                    LogFlags::ADDRMAN, 
                    "Added %s mapped to AS%i to new[%i][%i]\n",
                    addr.ToString(), 
                    addr.get_mapped_as(asmap), 
                    n_ubucket, 
                    n_ubucket_pos
                };

            } else {

                if info.n_ref_count == 0 {
                    self.delete(n_id);
                }
            }
        }

        f_new
    }
}
