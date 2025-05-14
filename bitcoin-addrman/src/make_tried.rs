// ---------------- [ File: bitcoin-addrman/src/make_tried.rs ]
crate::ix!();

impl AddrManInner {

    /**
      | Move an entry from the "new" table(s)
      | to the "tried" table
      |
      */
    pub fn make_tried(&mut self, 
        info: &mut AddrInfo,
        n_id: i32,
        n_key: &u256,
        asmap: &Vec<bool>
    )  {
        
        // remove the entry from all new buckets
        let start_bucket: usize = info.get_new_bucket(n_key, asmap).try_into().unwrap();

        for n in 0..ADDRMAN_NEW_BUCKET_COUNT {

            let bucket: usize = (start_bucket + n) % ADDRMAN_NEW_BUCKET_COUNT;
            let pos:    usize = info.get_bucket_position(n_key, true, bucket).try_into().unwrap();

            if self.vv_new[bucket][pos] == n_id {

                self.vv_new[bucket][pos] = -1;

                info.n_ref_count -= 1;

                if info.n_ref_count == 0 {
                    break;
                }
            }
        }

        self.n_new -= 1;

        assert!(info.n_ref_count == 0);

        //  which tried bucket to move the entry to
        let n_kbucket:     usize = info.get_tried_bucket(n_key, asmap).try_into().unwrap();
        let n_kbucket_pos: usize = info.get_bucket_position(n_key, false, n_kbucket).try_into().unwrap();

        //  first make space to add it (the
        //  existing tried entry there is moved to
        //  new, deleting whatever is there).
        if self.vv_tried[n_kbucket][n_kbucket_pos] != -1 {

            //  find an item to evict
            let n_id_evict: i32 = self.vv_tried[n_kbucket][n_kbucket_pos];;

            assert!(self.map_info.contains_key(&n_id_evict));

            unsafe {

                let info_old: *mut AddrInfo = self.map_info.get_mut(&n_id_evict).unwrap() as *mut AddrInfo;

                //  Remove the to-be-evicted item from the tried set.
                (*info_old).in_tried = false;

                self.vv_tried[n_kbucket][n_kbucket_pos] = -1;

                self.n_tried -= 1;

                //  find which new bucket it belongs to
                let n_ubucket:     usize = (*info_old).get_new_bucket(n_key, asmap).try_into().unwrap();
                let n_ubucket_pos: usize = (*info_old).get_bucket_position(n_key, true, n_ubucket).try_into().unwrap();

                self.clear_new(n_ubucket, n_ubucket_pos);

                assert!(self.vv_new[n_ubucket][n_ubucket_pos] == -1);

                //  Enter it into the new set again.
                (*info_old).n_ref_count = 1;

                self.vv_new[n_ubucket][n_ubucket_pos] = n_id_evict;

                self.n_new += 1;

                log_print!(
                    bc_log::addrman, 
                    "Moved %s from tried[%i][%i] to new[%i][%i] to make space\n", 
                    (*info_old).to_string(), 
                    n_kbucket, 
                    n_kbucket_pos, 
                    n_ubucket, 
                    n_ubucket_pos
                );
            }
        }

        assert!(self.vv_tried[n_kbucket][n_kbucket_pos] == -1);

        self.vv_tried[n_kbucket][n_kbucket_pos] = n_id;

        self.n_tried += 1;

        info.in_tried = true;
    }
}
