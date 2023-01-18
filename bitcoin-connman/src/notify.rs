crate::ix!();

impl Connman {

    pub fn notify_num_connections_changed(&self)  {
        
        let nodes_size: u32 = self.cs_v_nodes.get().nodes.len().try_into().unwrap();

        if nodes_size != self.n_prev_node_count.load(atomic::Ordering::Relaxed) {

            self.n_prev_node_count.store(nodes_size, atomic::Ordering::Relaxed);

            self.client_interface.get_mut()
                .notify_num_connections_changed(nodes_size.try_into().unwrap());
        }
    }
}
