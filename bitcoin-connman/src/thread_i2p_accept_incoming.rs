// ---------------- [ File: bitcoin-connman/src/thread_i2p_accept_incoming.rs ]
crate::ix!();

pub fn subroutine_thread_i2p_accept_incoming(connman: Arc<Connman>)  {
    
    pub const err_wait_begin: Duration = Duration::seconds(1);
    pub const err_wait_cap:   Duration = Duration::minutes(5);

    let mut err_wait = err_wait_begin;

    let mut advertising_listen_addr: bool = false;

    let mut conn = Connection::default();

    while !connman.interrupt_net.get().as_bool() {

        if !connman.i2p_sam_session
            .get_mut()
            .listen(&mut conn) 
        {
            if advertising_listen_addr && conn.me.base.is_valid() {
                remove_local(&conn.me);
                advertising_listen_addr = false;
            }

            connman.interrupt_net.get_mut().sleep_for(err_wait);

            if err_wait < err_wait_cap {
                err_wait *= 2_i32;
            }

            continue;
        }

        if !advertising_listen_addr {
            add_local(&conn.me, Some(LOCAL_MANUAL.try_into().unwrap()));
            advertising_listen_addr = true;
        }

        if !connman.i2p_sam_session
            .get_mut()
            .accept(&mut conn) {
            continue;
        }

        let mut sock = (*conn.sock).release();

        connman.create_node_from_accepted_socket(
            &mut sock, 
            NetPermissionFlags::None,
            &Address::new(conn.me.clone(),  ServiceFlags::NODE_NONE),
            &Address::new(conn.peer.clone(),ServiceFlags::NODE_NONE)
        );
    }
}
