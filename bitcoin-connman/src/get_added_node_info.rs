crate::ix!();

impl Connman {

    pub fn get_added_node_info(&self) -> Vec<AddedNodeInfo> {

        let mut ret = Vec::<AddedNodeInfo>::default();

        let mut l_addresses: Vec::<String> = Vec::<String>::new();

        {
            let mut guard = self.cs_v_added_nodes.get();

            ret.reserve(guard.added_nodes.len());

            l_addresses.extend(guard.added_nodes.clone());
        }
        
        // Build a map of all already connected
        // addresses (by IP:port and by name) to
        // inbound/outbound and resolved CService
        let mut map_connected         = HashMap::<Service,bool>::default();;
        let mut map_connected_by_name = HashMap::<String,(bool,Service)>::default();

        {
            let mut guard = self.cs_v_nodes.get();

            for pnode in guard.nodes.iter() {

                let node = pnode.get();

                if node.service().base.is_valid() {
                    map_connected.insert(node.service().clone(), node.is_inbound_conn());
                }

                let addr_name: String = node.addr_name().to_string();

                if !addr_name.is_empty() {
                    map_connected_by_name.insert(
                        addr_name, 
                        (
                            node.is_inbound_conn(),
                            node.service().clone()
                        )
                    );
                }
            }
        }

        for str_add_node in l_addresses.iter() {

            let service: Service = Service::from(
                lookup_numeric(
                    str_add_node,
                    Some(params().get_default_port_from_addr(str_add_node)),
                    None
                )
            );

            let mut added_node = AddedNodeInfo {
                str_added_node:   str_add_node.to_string(),
                resolved_address: Service::default(),
                connected:        false,
                inbound:          false
            };

            if service.base.is_valid() {

                // strAddNode is an IP:port
                let it = map_connected.get(&service);

                if it != None {
                    added_node.resolved_address = service;
                    added_node.connected = true;
                    added_node.inbound = *it.unwrap();
                }

            } else {

                // strAddNode is a name
                let it = map_connected_by_name.get(str_add_node);;

                if it != None {
                    added_node.resolved_address = it.unwrap().1.clone();
                    added_node.connected = true;
                    added_node.inbound = it.unwrap().0;
                }
            }

            ret.push(added_node); // want move
        }

        ret
    }
}
