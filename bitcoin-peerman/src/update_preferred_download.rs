crate::ix!();

#[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
pub fn update_preferred_download(
    node:  &dyn NodeInterface,
    state: Amo<NodeState>)  {

    unsafe {

        N_PREFERRED_DOWNLOAD.fetch_sub(
            ternary![state.get().preferred_download.load(atomic::Ordering::Relaxed),1,0], 
            atomic::Ordering::Relaxed
        );

        let preferred_download = {
            let b0 = !node.is_inbound_conn();
            let b1 = node.has_permission(NetPermissionFlags::NoBan);
            let b2 = !node.is_addr_fetch_conn();
            let b3 = !node.is_client();

            (b0 || b1) && b2 && b3
        };

        //  Whether this node should be marked as
        //  a preferred download node.
        state.get().preferred_download.store(
            preferred_download, 
            atomic::Ordering::Relaxed
        );

        N_PREFERRED_DOWNLOAD.fetch_add(
            ternary![state.get().preferred_download.load(atomic::Ordering::Relaxed),1,0], 
            atomic::Ordering::Relaxed
        );
    }
}

/**
  | Set the current IBD status in order to
  | figure out the desirable service flags
  |
  */
pub fn set_service_flags_ibd_cache(state: bool)  {
    
    todo!();
        /*
            g_initial_block_download_completed = state;
        */
}
