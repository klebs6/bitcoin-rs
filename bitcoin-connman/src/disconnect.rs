crate::ix!();

/**
  | Close socket and set hSocket to INVALID_SOCKET
  |
  */
pub fn close_socket(h_socket: &mut CSocket) -> bool {
    
    todo!();
        /*
            if (hSocket == INVALID_SOCKET)
            return false;
    #ifdef WIN32
        int ret = closesocket(hSocket);
    #else
        int ret = close(hSocket);
    #endif
        if (ret) {
            LogPrintf("Socket close failed: %d. Error: %s\n", hSocket, NetworkErrorString(WSAGetLastError()));
        }
        hSocket = INVALID_SOCKET;
        return ret != SOCKET_ERROR;
        */
}

impl Connman {

    pub fn disconnect_node_with_str(&mut self, str_node: &String) -> bool {

        let guard = self.cs_v_nodes.get_mut();

        let pnode = guard.find_node_with_addr_name(str_node);

        match pnode.is_some() {
            true => {

                let node = pnode.get();

                log_print!(LogFlags::NET, 
                    "disconnect by address%s matched peer=%d; disconnecting\n", 
                    match LOG_IPS {
                        true   => format!("={}",str_node),
                        false  => ""
                    }, 
                    node.get_id()
                );

                node.mark_for_disconnect();

                true
            },
            false => {
                false
            }
        }
    }
    
    pub fn disconnect_node_with_subnet(&mut self, subnet: &SubNet) -> bool {

        let mut disconnected: bool = false;

        let guard = self.cs_v_nodes.get();

        for pnode in guard.nodes.iter() {

            let node = pnode.get();

            unsafe {

                if subnet.match_(&node.service().base) {

                    log_print!(
                        LogFlags::NET, 
                        "disconnect by subnet{} matched peer={}; disconnecting\n", 
                        match LOG_IPS {
                            true   => format!("={}",subnet.to_string()),
                            false  => ""
                        }, 
                        node.get_id()
                    );

                    node.mark_for_disconnect();

                    disconnected = true;
                }
            }
        }

        disconnected
    }
    
    pub fn disconnect_node_with_netaddr(&mut self, addr: &NetAddr) -> bool {
        
        let subnet = SubNet::new_from_net_addr(addr,None);

        self.disconnect_node_with_subnet(&subnet)
    }
    
    pub fn disconnect_node_with_id(&mut self, id: NodeId) -> bool {
        
        let guard = self.cs_v_nodes.get();

        for pnode in guard.nodes.iter() {

            let node = pnode.get();

            unsafe {

                if id == node.get_id() {

                    log_print!(
                        LogFlags::NET, 
                        "disconnect by id peer={}; disconnecting\n", 
                        node.get_id()
                    );

                    node.mark_for_disconnect();

                    return true;
                }
            }
        }

        false
    }

    pub fn disconnect_nodes(&self)  {

        {
            let mut guard = self.cs_v_nodes.get_mut();

            if !self.network_active.load(atomic::Ordering::Relaxed) {

                // Disconnect any connected nodes
                for pnode in guard.nodes.iter() {

                    let node = pnode.get();

                    if !node.marked_for_disconnect() {

                        log_print!(
                            bc_log::NET, 
                            "Network not active, dropping peer={}\n", 
                            node.get_id()
                        );

                        node.mark_for_disconnect();
                    }
                }
            }

            // Disconnect unused nodes
            let nodes_copy: Vec::<Amo<Box<dyn NodeInterface>>> = guard.nodes.clone();;

            for pnode in nodes_copy.iter() {

                let mut node = pnode.get_mut();

                if node.marked_for_disconnect() {

                    //  remove from vNodes
                    guard.nodes.retain(|item| {

                        let delete = item.get().get_id() == node.get_id();

                        !delete
                    });

                    //  release outbound grant (if any)
                    node.release_grant_outbound();

                    //  close socket and cleanup
                    node.close_socket_disconnect();

                    //  hold in disconnected pool until all refs are released
                    node.release();

                    self.nodes_disconnected.get_mut().push(pnode.clone());
                }
            }
        }

        {
            //  Delete disconnected nodes
            let nodes_disconnected_copy: 
            Vec::<Amo<Box<dyn NodeInterface>>> = self.nodes_disconnected.get().clone();

            for pnode in nodes_disconnected_copy.iter() {

                let node = pnode.get();

                // Destroy the object only after
                // other threads have stopped
                // using it.
                if node.get_ref_count() <= 0 {

                    self.nodes_disconnected.get_mut().retain(|item| {
                        item.get().get_id() != node.get_id()
                    });

                    self.delete_node(pnode.clone());
                }
            }
        }
    }
}
