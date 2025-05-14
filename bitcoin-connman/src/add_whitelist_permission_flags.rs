// ---------------- [ File: bitcoin-connman/src/add_whitelist_permission_flags.rs ]
crate::ix!();

impl Connman {

    pub fn add_whitelist_permission_flags(&self, 
        flags: &mut NetPermissionFlags,
        addr:  &NetAddr)  {

        for subnet in self.whitelisted_range.get().iter() {

            if subnet.subnet.match_(addr) {

                NetPermissions::add_flag(flags, subnet.base.flags);
            }
        }
    }
}
