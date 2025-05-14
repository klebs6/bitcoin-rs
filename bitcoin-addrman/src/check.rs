// ---------------- [ File: bitcoin-addrman/src/check.rs ]
crate::ix!();

impl AddrManInner {

    /**
      | Consistency check, taking into account
      | m_consistency_check_ratio. Will std::abort
      | if an inconsistency is detected.
      */
    pub fn check(&self, consistency_check_ratio: i32, n_key: &u256, asmap: &Vec<bool>)  {

        let consistency_check_ratio: u64 
        = consistency_check_ratio.try_into().unwrap();

        //  Run consistency checks 1 in
        //  m_consistency_check_ratio times if
        //  enabled
        if consistency_check_ratio == 0 {
            return;
        }

        if self.insecure_rand.borrow_mut().randrange(consistency_check_ratio) >= 1 {
            return;
        }
        
        let err: i32 = self.force_check_addrman(n_key, asmap);

        if err != 0 {
            log_printf!("ADDRMAN CONSISTENCY CHECK FAILED!!! err=%i\n", err);
            assert!(false);
        }
    }

    /**
      | Perform consistency check, regardless of
      | m_consistency_check_ratio.
      |
      | @returns an error code or zero.
      */
    pub fn force_check_addrman(&self, n_key: &u256, asmap: &Vec<bool>) -> i32 {

        log_print!(
            bc_log::addrman, 
            "Addrman checks started: new %i, tried %i, total %u\n", 
            n_new, 
            n_tried, 
            random.size()
        );

        let mut set_tried = HashSet::<i32>::default();
        let mut map_new   = HashMap::<i32,i32>::default();

        if self.random.borrow().len() != (self.n_tried + self.n_new) as usize {
            return -7;
        }

        for entry in self.map_info.iter() {

            let n: i32 = *entry.0;

            let info: &AddrInfo = entry.1;

            if info.in_tried {

                if info.n_last_success == 0 {
                    return -1;
                }

                if info.n_ref_count != 0 {
                    return -2;
                }

                set_tried.insert(n);

            } else {

                if info.n_ref_count < 0 || info.n_ref_count > (ADDRMAN_NEW_BUCKETS_PER_ADDRESS as i32) {
                    return -3;
                }

                if info.n_ref_count == 0 {
                    return -4;
                }

                *map_new.get_mut(&n).unwrap() = info.n_ref_count
            }

            let it = self.map_addr.get(&info.address.service);

            if it.is_none() 
            || *it.unwrap() != n 
            {
                return -5;
            }

            let info_nrandom_pos = *info.n_random_pos.borrow();
            let info_nrandom_pos_idx: usize = info_nrandom_pos.try_into().unwrap();

            if info_nrandom_pos < 0 
            || info_nrandom_pos >= self.random.borrow().len().try_into().unwrap() 
            || self.random.borrow()[info_nrandom_pos_idx] != n 
            {
                return -14;
            }

            if info.n_last_try < 0 {
                return -6;
            }

            if info.n_last_success < 0 {
                return -8;
            }
        }

        if set_tried.len() != self.n_tried as usize {
            return -9;
        }

        if map_new.len() != self.n_new as usize {
            return -10;
        }

        for n in 0..ADDRMAN_TRIED_BUCKET_COUNT {
            for i in 0..ADDRMAN_BUCKET_SIZE {

                if self.vv_tried[n][i] != -1 {

                    if !set_tried.contains(&self.vv_tried[n][i]) {
                        return -11;
                    }

                    let it = self.map_info.get(&self.vv_tried[n][i]);

                    if it.is_none() || it.unwrap().get_tried_bucket(n_key, asmap) != n.try_into().unwrap() {
                        return -17;
                    }

                    if it.unwrap().get_bucket_position(n_key, false, n) != i.try_into().unwrap() {
                        return -18;
                    }

                    set_tried.remove(&self.vv_tried[n][i]);
                }
            }
        }

        for n in 0..ADDRMAN_NEW_BUCKET_COUNT {
            for i in 0..ADDRMAN_BUCKET_SIZE {

                if self.vv_new[n][i] != -1 {

                    if !map_new.contains_key(&self.vv_new[n][i]) {
                        return -12;
                    }

                    let it = self.map_info.get(&self.vv_new[n][i]);

                    if it.is_none() || it.unwrap().get_bucket_position(n_key, true, n) != i.try_into().unwrap() {
                        return -19;
                    }

                    if {
                        *map_new.get_mut(&self.vv_new[n][i]).unwrap() -= 1;
                        map_new[&self.vv_new[n][i]]
                    } == 0 {
                        map_new.remove(&self.vv_new[n][i]);
                    }
                }

            }
        }

        if set_tried.len() != 0 {
            return -13;
        }

        if map_new.len() != 0 {
            return -15;
        }

        if n_key.blob.is_null() {
            return -16;
        }

        log_print!(bc_log::addrman, "Addrman checks completed successfully\n");

        0
    }
}
