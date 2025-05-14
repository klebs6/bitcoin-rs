// ---------------- [ File: bitcoin-addrman/src/resolve.rs ]
crate::ix!();

impl AddrManInner {

    pub unsafe fn resolve_collisions(&mut self,
        n_key:            &u256,
        asmap:            &Vec<bool>,
        tried_collisions: *mut HashSet<i32>,
    )  {

        (*tried_collisions).retain(|id_new| {

            let mut erase_collision: bool = false;

            // If id_new not found in mapInfo
            // remove it from m_tried_collisions
            if !self.map_info.contains_key(id_new) {
                erase_collision = true;

            } else {

                let info_new: *mut AddrInfo = self.map_info.get_mut(&id_new).unwrap() as *mut AddrInfo;

                unsafe {

                    // Which tried bucket to move the
                    // entry to.
                    let tried_bucket:     usize = (*info_new).get_tried_bucket(n_key, asmap).try_into().unwrap();
                    let tried_bucket_pos: usize = (*info_new).get_bucket_position(n_key, false, tried_bucket).try_into().unwrap();

                    if !(*info_new).source.is_valid() {

                        // id_new may no longer map to
                        // a valid address
                        erase_collision = true;

                    } else if self.vv_tried[tried_bucket][tried_bucket_pos] != -1 {

                        // The position in the tried
                        // bucket is not empty
                        //
                        // Get the to-be-evicted
                        // address that is being
                        // tested
                        let id_old:   i32 = self.vv_tried[tried_bucket][tried_bucket_pos];;
                        let info_old: *mut AddrInfo = self.map_info.get_mut(&id_old).unwrap() as *mut AddrInfo;

                        //  Has successfully connected in last X hours
                        if get_adjusted_time() - (*info_old).n_last_success < (ADDRMAN_REPLACEMENT_HOURS * (60 * 60)).try_into().unwrap() {
                            erase_collision = true;

                        } else if get_adjusted_time() - (*info_old).n_last_try < (ADDRMAN_REPLACEMENT_HOURS * (60 * 60)).try_into().unwrap() {

                            // attempted to connect
                            // and failed in last
                            // X hours
                            //
                            // Give address at least
                            // 60 seconds to
                            // successfully connect
                            if get_adjusted_time() - (*info_old).n_last_try > 60 {

                                log_print!(
                                    bc_log::addrman, 
                                    "Replacing %s with %s in tried table\n", 
                                    (*info_old).to_string(), 
                                    (*info_new).to_string()
                                );

                                // Replaces an
                                // existing address
                                // already in the
                                // tried table with
                                // the new address
                                self.good(
                                    &(*info_new).address.service,
                                    false,
                                    get_adjusted_time(),
                                    n_key,
                                    asmap,
                                    tried_collisions
                                );

                                erase_collision = true;

                            }

                        } else if get_adjusted_time() - (*info_new).n_last_success > ADDRMAN_TEST_WINDOW.try_into().unwrap() {

                            // If the collision hasn't
                            // resolved in some
                            // reasonable amount of
                            // time, just evict the
                            // old entry -- we must
                            // not be able to connect
                            // to it for some reason.
                            log_print!(
                                bc_log::addrman,
                                "Unable to test; replacing %s with %s in tried table anyway\n",
                                (*info_old).to_string(),
                                (*info_new).to_string()
                            );

                            self.good(
                                &(*info_new).address.service, 
                                false, 
                                get_adjusted_time(),
                                n_key,
                                asmap,
                                tried_collisions
                            );

                            erase_collision = true;
                        }

                    } else {

                        // Collision is not actually
                        // a collision anymore
                        self.good(
                            &(*info_new).address.service,
                            false,
                            get_adjusted_time(),
                            n_key,
                            asmap,
                            tried_collisions
                        );

                        erase_collision = true;
                    }
                }
            }

            !erase_collision
        })
    }
}
