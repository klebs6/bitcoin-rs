crate::ix!();

pub struct ConnmanAddrFetches {
    pub addr_fetches: VecDeque<String>,
}

impl Connman {

    pub fn add_addr_fetch(&self, str_dest: &str)  {
        
        let mut guard = self
            .addr_fetches_mutex
            .get_mut();

        guard.addr_fetches.push_back(str_dest.to_string());
    }
}
