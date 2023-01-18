crate::ix!();

impl Connman {

    pub fn delete_node(&self, pnode: Amo<Box<dyn NodeInterface>>) {

        //assert!(pnode != null_mut());

        unsafe {

            {
                let mut node = pnode.get_mut();

                self.msgproc
                    .get_mut()
                    .finalize_node(&mut node);
            }

            //libc::free(pnode as *mut c_void); 

            pnode.take();

            /*
            //NOTE: no guarantees that this method
            //works.  there could still be
            //somebody else holding an Arc here 
            //
            //it is probably still better than
            //the C, though
            let ptr = Arc::into_raw(pnode.inner());

            Arc::decrement_strong_count(ptr);
            */
        }
    }
    
    pub fn add_node(&mut self, str_node: &str) -> bool {

        let mut guard = self.cs_v_added_nodes.get_mut();

        for it in guard.added_nodes.iter() {
            if str_node == it {
                return false;
            }
        }

        guard.added_nodes.push(str_node.to_string());

        true
    }
    
    pub fn remove_added_node(&mut self, str_node: &String) -> bool {
        
        let mut guard = self.cs_v_added_nodes.get_mut();

        let mut deleted: bool = false;

        guard.added_nodes.retain(|item: &String| {

            let delete = str_node == item;

            if delete {
                deleted = true;
            }

            !delete
        });

        deleted
    }
}

