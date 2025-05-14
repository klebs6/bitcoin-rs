// ---------------- [ File: bitcoin-addrman/src/good.rs ]
crate::ix!();

impl AddrManInner {

    pub unsafe fn good(&mut self, 
        addr:              &Service,
        test_before_evict: bool,
        n_time:            i64,
        n_key:             &u256,
        asmap:             &Vec<bool>,
        tried_collisions:  *mut HashSet<i32>) {
            
        unsafe {

            let mut n_id = i32::default();

            self.n_last_good = n_time;

            let info: *mut AddrInfo = self.find(addr,Some(&mut n_id as *mut _));

            //  if not found, bail out
            if info == std::ptr::null_mut() {
                return;
            }

            //  update info
            (*info).n_last_success = n_time;
            (*info).n_last_try     = n_time;
            (*info).n_attempts     = 0;

            // n_time is not updated here, to avoid
            // leaking information about
            // currently-connected peers.

            // if it is already in the tried set,
            // don't do anything else
            if (*info).in_tried {
                return;
            }

            // if it is not in new, something bad
            // happened
            if !assume!((*info).n_ref_count > 0) {
                return;
            }

            //  which tried bucket to move the entry to
            let tried_bucket:     usize = (*info).get_tried_bucket(n_key, asmap).try_into().unwrap();
            let tried_bucket_pos: usize = (*info).get_bucket_position(n_key, false, tried_bucket).try_into().unwrap();

            //  Will moving this address into tried evict another entry?
            if test_before_evict && (self.vv_tried[tried_bucket][tried_bucket_pos] != -1) {

                if (*tried_collisions).len() < ADDRMAN_SET_TRIED_COLLISION_SIZE {
                    (*tried_collisions).insert(n_id);
                }

                //  Output the entry we'd be colliding with, for debugging purposes
                let colliding_entry = self.map_info.get(&self.vv_tried[tried_bucket][tried_bucket_pos]);

                log_print!(
                    bc_log::addrman, 
                    "Collision with %s while attempting to move %s to tried table. Collisions=%d\n", 
                    match colliding_entry.is_some() {
                        true   => (*colliding_entry).to_string(),
                        false  => ""
                    }, 
                    addr.to_string(), 
                    (*tried_collisions).len()
                );

            } else {

                //  move nId to the tried tables
                self.make_tried(&mut *info,n_id,n_key,asmap);

                log_print!(
                    bc_log::addrman, 
                    "Moved %s mapped to AS%i to tried[%i][%i]\n", 
                    addr.to_string(), 
                    addr.get_mappedas(asmap), 
                    tried_bucket, 
                    tried_bucket_pos
                );
            }
        }
    }
}
