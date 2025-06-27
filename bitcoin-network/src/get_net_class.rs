crate::ix!();

impl NetAddr {
    
    pub fn get_net_class(&self) -> Network {
        
        todo!();
        /*
            // Make sure that if we return NET_IPV6, then IsIPv6() is true. The callers expect that.

        // Check for "internal" first because such addresses are also !IsRoutable()
        // and we don't want to return NET_UNROUTABLE in that case.
        if (IsInternal()) {
            return NET_INTERNAL;
        }
        if (!IsRoutable()) {
            return NET_UNROUTABLE;
        }
        if (HasLinkedIPv4()) {
            return NET_IPV4;
        }
        return m_net;
        */
    }
}
