crate::ix!();

impl AddrManImpl {

    #[EXCLUSIVE_LOCKS_REQUIRED(!cs)]
    pub fn unserialize<Stream: GetType + GetVersion>(&mut self, s: &mut Stream) -> Result<(), StdException>  {
    
        let mut inner = self.cs.lock();

        assert!(inner.random.borrow().is_empty());

        let mut format = AddrManFormat::default();

        todo!();
        /*
        s_ >> Using<CustomUintFormatter<1>>::new(format);
        */

        let mut stream_version: i32 = s.get_version();

        if format >= AddrManFormat::V3_BIP155 {

            // Add ADDRV2_FORMAT to the version so
            // that the CNetAddr and CAddress
            // unserialize methods know that an
            // address in addrv2 format is coming.
            stream_version |= ADDRV2_FORMAT;
        }

        let s: OverrideStream::<Stream> = OverrideStream::<Stream>::new(
            &mut *s, 
            s.get_type(), 
            stream_version
        );

        let mut compat = 0;

        s.stream_into(&mut compat);

        let lowest_compatible: u8 = compat - ADDR_MAN_INCOMPATIBILITY_BASE;

        if lowest_compatible > ADDR_MAN_FILE_FORMAT as u8 {

            let error = format!{
                "Unsupported format of addrman database: {:?}. 
                It is compatible with formats >={}, but the 
                maximum supported by this version of {} is {}.",
                format,
                lowest_compatible,
                PACKAGE_NAME,
                ADDR_MAN_FILE_FORMAT as u8
            };

            return Err(
                ios_base_failure(&error)
            );
        }

        s.stream_into(&mut self.n_key);
        s.stream_into(&mut inner.n_new);
        s.stream_into(&mut inner.n_tried);

        let mut n_ubuckets: i32 = 0;

        s.stream_into(&mut n_ubuckets);

        if format >= AddrManFormat::V1_DETERMINISTIC {
            n_ubuckets ^= (1 << 30);
        }


        if inner.n_new > (ADDRMAN_NEW_BUCKET_COUNT * ADDRMAN_BUCKET_SIZE).try_into().unwrap() || inner.n_new < 0 {

            let error = format!(
                "Corrupt AddrMan serialization: n_new={}, should be in [0, {}]",
                inner.n_new, 
                ADDRMAN_NEW_BUCKET_COUNT * ADDRMAN_BUCKET_SIZE
            );

            return Err(ios_base_failure(&error));
        }

        if inner.n_new > (ADDRMAN_NEW_BUCKET_COUNT * ADDRMAN_BUCKET_SIZE).try_into().unwrap() || inner.n_new < 0 {

            let error = format!(
                "Corrupt AddrMan serialization: n_new={}, should be in [0, {}]",
                inner.n_new,
                ADDRMAN_NEW_BUCKET_COUNT * ADDRMAN_BUCKET_SIZE
            );

            return Err(ios_base_failure(&error));
        }

        if inner.n_tried > (ADDRMAN_TRIED_BUCKET_COUNT * ADDRMAN_BUCKET_SIZE).try_into().unwrap() || inner.n_tried < 0 {

            let error = format!(
                "Corrupt AddrMan serialization: nTried={}, should be in [0, {}]",
                inner.n_tried,
                ADDRMAN_TRIED_BUCKET_COUNT * ADDRMAN_BUCKET_SIZE
            );

            return Err(ios_base_failure(&error));
        }

        // Deserialize entries from the new table.
        for n in 0..inner.n_new {

            let mut info: *mut AddrInfo = inner.map_info.get_mut(&n).unwrap();

            unsafe {

                s.stream_into(&mut *info);

                inner.map_addr.insert((*info).address.service.clone(), n);

                let new_pos = inner.random.borrow().len().try_into().unwrap();

                (*info).n_random_pos.replace(new_pos);
            }

            inner.random.borrow_mut().push(n);
        }

        inner.n_id_count = inner.n_new;

        // Deserialize entries from the tried
        // table.
        let mut n_lost: i32 = 0;;


        for n in 0..inner.n_tried {

            let mut info = AddrInfo::default();

            s.stream_into(&mut info);

            let n_kbucket:     usize = info.get_tried_bucket(&self.n_key, &self.asmap).try_into().unwrap();
            let n_kbucket_pos: usize = info.get_bucket_position(&self.n_key, false, n_kbucket).try_into().unwrap();

            if info.source.is_valid() && inner.vv_tried[n_kbucket][n_kbucket_pos] == -1 {

                let new_pos: i32 = inner.random.borrow_mut().len().try_into().unwrap();

                info.n_random_pos.replace(new_pos);
                info.in_tried     = true;

                let n_id_count = inner.n_id_count;

                inner.random.borrow_mut().push(n_id_count);

                inner.map_info.insert(n_id_count,info.clone());

                inner.map_addr.insert(info.address.service.clone(), n_id_count);

                inner.vv_tried[n_kbucket][n_kbucket_pos] = n_id_count;

                inner.n_id_count += 1;

            } else {

                n_lost += 1;
            }
        }

        inner.n_tried -= n_lost;

        // Store positions in the new table
        // buckets to apply later (if possible).
        //
        // An entry may appear in up to
        // ADDRMAN_NEW_BUCKETS_PER_ADDRESS
        // buckets, so we store all
        // bucket-entry_index pairs to iterate
        // through later.
        let mut bucket_entries = Vec::<(i32,i32)>::default();

        for bucket in 0..n_ubuckets {

            let mut num_entries: i32 = 0;

            s.stream_into(&mut num_entries);

            for n in 0..num_entries {

                let mut entry_index: i32 = 0;

                s.stream_into(&mut entry_index);

                if entry_index >= 0 && entry_index < inner.n_new {
                    bucket_entries.push((bucket, entry_index));
                }
            }
        }

        // If the bucket count and asmap checksum
        // haven't changed, then attempt to
        // restore the entries to the
        // buckets/positions they were in before
        // serialization.
        let mut supplied_asmap_checksum = u256::default();;

        if self.asmap.len() != 0 {
            supplied_asmap_checksum = serialize_hash(&self.asmap, None, None);
        }

        let mut serialized_asmap_checksum = u256::default();

        if format >= AddrManFormat::V2_ASMAP {
            s.stream_into(&mut serialized_asmap_checksum);
        }

        let restore_bucketing: bool = 
        n_ubuckets == ADDRMAN_NEW_BUCKET_COUNT.try_into().unwrap() 
        && 
        serialized_asmap_checksum == supplied_asmap_checksum;

        if !restore_bucketing {
            log_print!(
                bc_log::addrman, 
                "Bucketing method was updated, re-bucketing addrman entries from disk\n"
            );
        }

        for bucket_entry in bucket_entries.iter() {

            let mut bucket:      usize = bucket_entry.0.try_into().unwrap();
            let entry_index: i32 = bucket_entry.1;

            let info: *mut AddrInfo = inner.map_info.get_mut(&entry_index).unwrap();

            unsafe {

                // Don't store the entry in the new
                // bucket if it's not a valid address
                // for our addrman
                if !(*info).source.is_valid() {
                    continue;
                }

                // The entry shouldn't appear in more
                // than ADDRMAN_NEW_BUCKETS_PER_ADDRESS. 
                // If it has already, just skip this
                // bucket_entry.
                if (*info).n_ref_count >= ADDRMAN_NEW_BUCKETS_PER_ADDRESS.try_into().unwrap() {
                    continue;
                }

                let mut bucket_position: usize 
                = (*info).get_bucket_position(
                    &self.n_key, 
                    true, 
                    bucket.try_into().unwrap()
                ).try_into().unwrap();

                if restore_bucketing && inner.vv_new[bucket][bucket_position] == -1 {

                    // Bucketing has not changed,
                    // using existing bucket positions
                    // for the new table
                    inner.vv_new[bucket][bucket_position] = entry_index;

                    (*info).n_ref_count += 1;

                } else {

                    // In case the new table data
                    // cannot be used (bucket count
                    // wrong or new asmap), try to
                    // give them a reference based on
                    // their primary source address.
                    bucket = (*info).get_new_bucket(&self.n_key, &self.asmap)
                        .try_into().unwrap();

                    bucket_position = (*info).get_bucket_position(
                        &self.n_key, 
                        true, 
                        bucket.try_into().unwrap()
                    ).try_into().unwrap();

                    if inner.vv_new[bucket][bucket_position] == -1 {

                        inner.vv_new[bucket][bucket_position] = entry_index;

                        (*info).n_ref_count += 1;
                    }
                }
            }
        }

        // Prune new entries with refcount 0 (as
        // a result of collisions or invalid
        // address).
        let mut n_lost_unk: i32 = 0;

        inner.map_info.retain(|_, info| {

            let delete: bool 
            = info.in_tried == false && info.n_ref_count == 0;

            if delete {
                n_lost_unk += 1;
            }

            !delete
        });

        if n_lost + n_lost_unk > 0 {
            log_print!(
                bc_log::addrman, 
                "addrman lost %i new and %i tried addresses due to collisions or invalid addresses\n", 
                n_lost_unk, 
                n_lost
            );
        }

        let check_code: i32 = inner.force_check_addrman(&self.n_key, &self.asmap);

        if check_code != 0 {

            let msg = format!( "Corrupt data. Consistency check failed with code {}", check_code);

            return Err(ios_base_failure(&msg));
        }

        Ok(())
    }
}
