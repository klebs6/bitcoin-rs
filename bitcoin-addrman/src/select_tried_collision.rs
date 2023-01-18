crate::ix!();

impl AddrManInner {

    pub fn select_tried_collision(&mut self, 
        n_key:            &u256, 
        asmap:            &Vec<bool>,
        tried_collisions: &mut HashSet<i32>) -> (Address,i64) 
    {
        if tried_collisions.is_empty() {
            return Default::default();
        }

        let id_new: i32 = {

            //  Selects a random element from m_tried_collisions
            let max: u64 = tried_collisions.len().try_into().unwrap();

            let random_idx: usize = self
                .insecure_rand
                .borrow_mut()
                .randrange(max)
                .try_into()
                .unwrap();

            *tried_collisions.iter().nth(random_idx).unwrap()
        };

        // If id_new not found in mapInfo remove
        // it from m_tried_collisions
        if !self.map_info.contains_key(&id_new) {
            tried_collisions.remove(&id_new);
            return Default::default();
        }

        let new_info: &AddrInfo = &self.map_info[&id_new];

        //  which tried bucket to move the entry to
        let tried_bucket: usize = new_info.get_tried_bucket(
            n_key, 
            asmap
        ).try_into().unwrap();

        let tried_bucket_pos: usize = new_info.get_bucket_position(
            n_key, 
            false, 
            tried_bucket.try_into().unwrap()
        ).try_into().unwrap();

        let info_old: &AddrInfo 
        = &self.map_info[&self.vv_tried[tried_bucket][tried_bucket_pos]];

        (info_old.address.clone(), info_old.n_last_try)
    }
}
