crate::ix!();

impl AddrManInner {

    /**
      | Clear a position in a "new" table. This is
      | the only place where entries are actually
      | deleted.
      */
    pub fn clear_new(&mut self, 
        n_ubucket:     usize,
        n_ubucket_pos: usize) {

        // if there is an entry in the specified
        // bucket, delete it.
        if self.vv_new[n_ubucket][n_ubucket_pos] != -1 {

            let n_id_delete: i32 
            = self.vv_new[n_ubucket][n_ubucket_pos];

            let info_delete: &mut AddrInfo 
            = self.map_info.get_mut(&n_id_delete).unwrap();

            assert!(info_delete.n_ref_count > 0);

            info_delete.n_ref_count -= 1;
            self.vv_new[n_ubucket][n_ubucket_pos] = -1;

            log_print!(
                bc_log::addrman, 
                "Removed %s from new[%i][%i]\n", 
                info_delete.to_string(), 
                n_ubucket, 
                n_ubucket_pos
            );

            if info_delete.n_ref_count == 0 {
                self.delete(n_id_delete);
            }
        }
    }
}
