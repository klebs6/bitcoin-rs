// ---------------- [ File: bitcoin-addrman/src/get.rs ]
crate::ix!();

impl AddrManInner {

    pub fn get_addr(&self, 
        max_addresses: usize,
        max_pct:       usize,
        network:       Option<Network>) -> Vec<Address> {
        
        let mut n_nodes: usize = self.random.borrow().len();

        if max_pct != 0 {
            n_nodes = max_pct * n_nodes / 100
        }

        if max_addresses != 0 {
            n_nodes = min(n_nodes,max_addresses);
        }

        //  gather a list of random nodes, skipping those of low quality
        let now: i64 = get_adjusted_time();

        let mut addresses = Vec::<Address>::default();

        for n in 0..self.random.borrow().len() {

            if addresses.len() >= n_nodes {
                break;
            }

            let n_rnd_pos: i32 = {

                let p0: u64 = (self.random.borrow().len() - n).try_into().unwrap();

                let x0: u64 = self.insecure_rand.borrow_mut().randrange(p0).try_into().unwrap();
                let x1: u64 = n.try_into().unwrap();

                (x0 + x1).try_into().unwrap()
            };

            self.swap_random(n, n_rnd_pos.try_into().unwrap());

            let it = self.map_info.get(&self.random.borrow()[n]);

            assert!(it.is_some());

            let ai: &AddrInfo = it.unwrap();

            //  Filter by network (optional)
            if network.is_some() {
                if &ai.source.get_net_class() != network.as_ref().unwrap() {
                    continue;
                }
            }

            //  Filter for quality
            if ai.is_terrible(Some(now)) {
                continue;
            }

            addresses.push(ai.address.clone());
        }

        log_print!(
            bc_log::addrman, 
            "GetAddr returned %d random addresses\n", 
            addresses.size()
        );

        addresses
    }
}
