crate::ix!();

impl AddrManInner {

    pub fn select(&self, new_only: bool) -> (Address,i64) {
        
        if self.random.borrow().is_empty() {
            return Default::default();
        }

        if new_only && self.n_new == 0 {
            return Default::default();
        }

        // Use a 50% chance for choosing between
        // tried and new table entries.
        if !new_only && (self.n_tried > 0 && (self.n_new == 0 || self.insecure_rand.borrow_mut().randbool() == false)) {

            //  use a tried node
            let mut chance_factor: f64 = 1.0;

            loop {

                // Pick a tried bucket, and an
                // initial position in that
                // bucket.
                let n_kbucket:     usize = self.insecure_rand.borrow_mut().randrange(ADDRMAN_TRIED_BUCKET_COUNT as u64).try_into().unwrap();
                let n_kbucket_pos: usize = self.insecure_rand.borrow_mut().randrange(ADDRMAN_BUCKET_SIZE as u64).try_into().unwrap();

                // Iterate over the positions of
                // that bucket, starting at the
                // initial one, and looping
                // around.
                let mut i = 0;

                for i in 0..ADDRMAN_BUCKET_SIZE {

                    if self.vv_tried[n_kbucket][(n_kbucket_pos + i) % ADDRMAN_BUCKET_SIZE] != -1 {
                        break;
                    }
                }

                // If the bucket is entirely
                // empty, start over with
                // a (likely) different one.
                if i == ADDRMAN_BUCKET_SIZE {
                    continue;
                }

                //  Find the entry to return.
                let n_id: i32 = self.vv_tried[n_kbucket][(n_kbucket_pos + i) % ADDRMAN_BUCKET_SIZE];;

                let it_found = self.map_info.get(&n_id);

                assert!(it_found.is_some());

                let info: &AddrInfo = it_found.unwrap();

                // With probability GetChance()
                // * fChanceFactor, return the
                // entry.
                if (self.insecure_rand.borrow_mut().randbits(30) as f64) < chance_factor * info.get_chance(None) * ((1 << 30) as f64) {

                    log_print!(
                        bc_log::addrman, 
                        "Selected %s from tried\n", 
                        info.to_string()
                    );

                    return (info.address.clone(), info.n_last_try);
                }

                // Otherwise start over with
                // a (likely) different bucket,
                // and increased chance factor.
                chance_factor *= 1.2;
            }
        } else {

            //  use a new node
            let mut chance_factor: f64 = 1.0;;

            loop {

                // Pick a new bucket, and an
                // initial position in that
                // bucket.
                let n_ubucket:     usize = self.insecure_rand.borrow_mut().randrange(ADDRMAN_NEW_BUCKET_COUNT as u64).try_into().unwrap();
                let n_ubucket_pos: usize = self.insecure_rand.borrow_mut().randrange(ADDRMAN_BUCKET_SIZE as u64).try_into().unwrap();

                // Iterate over the positions of
                // that bucket, starting at the
                // initial one, and looping
                // around.
                let mut i = 0;

                for i in 0..ADDRMAN_BUCKET_SIZE {
                    if self.vv_new[n_ubucket][(n_ubucket_pos + i) % ADDRMAN_BUCKET_SIZE] != -1 {
                        break;
                    }
                }

                // If the bucket is entirely
                // empty, start over with
                // a (likely) different one.
                if i == ADDRMAN_BUCKET_SIZE {
                    continue;
                }

                //  Find the entry to return.
                let n_id: usize = {
                    let idx: usize = (n_ubucket_pos + i) % ADDRMAN_BUCKET_SIZE;
                    self.vv_new[n_ubucket][idx].try_into().unwrap()
                };

                let it_found = self.map_info.get(&n_id.try_into().unwrap());

                assert!(it_found.is_some());

                let info: &AddrInfo = it_found.unwrap();

                // With probability GetChance()
                // * fChanceFactor, return the
                // entry.
                if (self.insecure_rand.borrow_mut().randbits(30) as f64) < chance_factor * info.get_chance(None) * ((1 << 30) as f64) {

                    log_print!(
                        bc_log::addrman, 
                        "Selected %s from new\n", 
                        info.to_string()
                    );

                    return (info.address.clone(), info.n_last_try);
                }

                // Otherwise start over with
                // a (likely) different bucket,
                // and increased chance factor.
                chance_factor *= 1.2;
            }
        }
    }
}
