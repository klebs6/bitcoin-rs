// ---------------- [ File: bitcoin-connman/src/calculate_keyed_net_group.rs ]
crate::ix!();

impl Connman {

    pub fn calculate_keyed_net_group(&self, ad: &Address) -> u64 {

        let vch_net_group: Vec::<u8> 
        = ad.service.base.get_group(self.addrman.get().get_asmap());

        let mut randomizer 
        = self.get_deterministic_randomizer(RANDOMIZER_ID_NETGROUP);

        randomizer.write(vch_net_group.as_slice());

        randomizer.finish()
    }
}
