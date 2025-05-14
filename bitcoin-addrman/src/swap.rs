// ---------------- [ File: bitcoin-addrman/src/swap.rs ]
crate::ix!();

impl AddrManInner {

    /**
      | Swap two elements in vRandom.
      |
      */
    pub fn swap_random(&self, 
        n_rnd_pos1: usize,
        n_rnd_pos2: usize)  {

        if n_rnd_pos1 == n_rnd_pos2 {
            return;
        }

        let random_len: usize = self.random.borrow().len();

        assert!(
            n_rnd_pos1 < random_len
            && n_rnd_pos2 < random_len
        );

        let n_id1: i32 = self.random.borrow()[n_rnd_pos1];
        let n_id2: i32 = self.random.borrow()[n_rnd_pos2];

        let it_1 = self.map_info.get(&n_id1);
        let it_2 = self.map_info.get(&n_id2);

        assert!(it_1.is_some());
        assert!(it_2.is_some());

        it_1.unwrap().n_random_pos.replace(n_rnd_pos2.try_into().unwrap());
        it_2.unwrap().n_random_pos.replace(n_rnd_pos1.try_into().unwrap());

        self.random.borrow_mut()[n_rnd_pos1] = n_id2;
        self.random.borrow_mut()[n_rnd_pos2] = n_id1;
    }
}
