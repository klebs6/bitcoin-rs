crate::ix!();

impl Connman {

    pub fn find_node_with_ip(&self, ip: &NetAddr) -> Amo<Box<dyn NodeInterface>> {
        self.cs_v_nodes.get().find_node_with_ip(ip)
    }
    
    pub fn find_node_with_subnet(&self, subnet: &SubNet) -> Amo<Box<dyn NodeInterface>> {
        self.cs_v_nodes.get().find_node_with_subnet(subnet)
    }
    
    pub fn find_node_with_addr_name(&self, addr_name: &str) -> Amo<Box<dyn NodeInterface>> {
        self.cs_v_nodes.get().find_node_with_addr_name(addr_name)
    }
    
    pub fn find_node_with_addr(&self, addr: &Service) -> Amo<Box<dyn NodeInterface>> {
        self.cs_v_nodes.get().find_node_with_addr(addr)
    }
}

impl ConnmanNodes {

    pub fn find_node_with_ip(&self, ip: &NetAddr) -> Amo<Box<dyn NodeInterface>> {
        
        for pnode in self.nodes.iter() {

            let node = pnode.get();

            unsafe {
                if node.service().base == *ip {
                    return pnode.clone();
                }
            }
        }

        Amo::<Box<dyn NodeInterface>>::none()
    }
    
    pub fn find_node_with_subnet(&self, sub_net: &SubNet) -> Amo<Box<dyn NodeInterface>> {
        
        for pnode in self.nodes.iter() {

            let node = pnode.get();

            unsafe {
                if sub_net.match_(&node.service().base) {
                    return pnode.clone();
                }
            }
        }

        Amo::<Box<dyn NodeInterface>>::none()
    }
    
    pub fn find_node_with_addr_name(&self, addr_name: &str) -> Amo<Box<dyn NodeInterface>> {
        
        for pnode in self.nodes.iter() {

            let node = pnode.get();

            unsafe {
                if node.addr_name() == addr_name {
                    return pnode.clone();
                }
            }
        }

        Amo::<Box<dyn NodeInterface>>::none()
    }
    
    pub fn find_node_with_addr(&self, addr: &Service) -> Amo<Box<dyn NodeInterface>> {
        
        for pnode in self.nodes.iter() {

            let node = pnode.get();

            unsafe {
                if node.service() == addr {
                    return pnode.clone();
                }
            }
        }

        Amo::<Box<dyn NodeInterface>>::none()
    }
}

