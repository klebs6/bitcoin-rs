// ---------------- [ File: bitcoin-banman/src/clear.rs ]
crate::ix!();

impl BanMan {

    pub fn clear_banned(&mut self)  {
        
        self.cs_banned
            .get_mut()
            .clear_banned();

        // store banlist to disk
        self.dump_banlist();

        if self.client_interface.is_some() {
            self.client_interface
                .get_mut()
                .banned_list_changed();
        }
    }
}
