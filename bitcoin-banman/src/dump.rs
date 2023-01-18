crate::ix!();

impl BanMan {

    pub fn dump_banlist(&mut self)  {
        
        // clean unused entries (if bantime has
        // expired)
        self.sweep_banned();

        if !self.banned_set_is_dirty() {
            return;
        }

        let n_start = Instant::now();

        let mut banmap = BanMap::default();

        self.get_banned(&mut banmap);

        if self.ban_db.write(&mut banmap) {
            self.set_banned_set_dirty(Some(false));
        }

        log_print!(
            bc_log::net, 
            "Flushed %d banned node addresses/subnets to disk  %dms\n", 
            banmap.size(), 
            Instant::now() - n_start
        );
    }
}
