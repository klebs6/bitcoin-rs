crate::ix!();

pub struct ConnmanAddedNodes {
    pub added_nodes:  Vec<String>,
}

unsafe impl Send for ConnmanAddedNodes {}
unsafe impl Sync for ConnmanAddedNodes {}


//-------------------------------
pub struct ConnmanNodes {
    pub nodes: Vec<Amo<Box<dyn NodeInterface>>>,
}

unsafe impl Send for ConnmanNodes {}
unsafe impl Sync for ConnmanNodes {}

//-------------------------------
pub type ConnmanNodeFn       = dyn Fn(Amo<Box<dyn NodeInterface>>) -> ();
pub type ConnmanForNodeFn    = dyn Fn(Amo<Box<dyn NodeInterface>>) -> bool;

pub type ConnmanNodeFnMut    = dyn FnMut(Amo<Box<dyn NodeInterface>>) -> ();
pub type ConnmanForNodeFnMut = dyn FnMut(Amo<Box<dyn NodeInterface>>) -> bool;

impl Connman {
    
    pub fn for_node<'a>(&'a mut self, 
        id:   NodeId,
        func: &'a ConnmanForNodeFn) -> bool {

        self.cs_v_nodes.get_mut().for_node(id,func)
    }

    pub fn for_each_node<'a>(&'a mut self, func: &'a ConnmanNodeFn)  {

        self.cs_v_nodes.get_mut().for_each_node(func)
    }

    pub fn for_node_mut<'a>(&'a mut self, 
        id:   NodeId,
        func: &'a mut ConnmanForNodeFnMut) -> bool {

        self.cs_v_nodes.get_mut().for_node_mut(id,func)
    }

    pub fn for_each_node_mut<'a>(&'a mut self, func: &'a mut ConnmanNodeFnMut)  {

        self.cs_v_nodes.get_mut().for_each_node_mut(func)
    }
}

/**
  | Whether the node should be passed out
  | in
  | 
  | ForEach* callbacks
  |
  */
pub fn node_fully_connected(pnode: Amo<Box<dyn NodeInterface>>) -> bool {
    
    let node = pnode.get();

    unsafe {
        node.successfully_connected() 
        && !node.marked_for_disconnect()
    }
}

macro_rules! for_node {
    ($self:ident, $id:ident, $func:ident) => {

        let mut found: Amo<Box<dyn NodeInterface>> = Amo::<Box<dyn NodeInterface>>::none();

        for pnode in $self.nodes.iter() {

            let node = pnode.get();

            unsafe {

                if node.get_id() == $id {
                    found = pnode.into();
                    break;
                }
            }
        }

        match found.is_some() {
            true  => node_fully_connected(found.clone()) && $func(found.clone()),
            false => false,
        }
    }
}

macro_rules! for_each_node {
    ($self:ident,$func:ident) => {

        for pnode in $self.nodes.iter() {
            if node_fully_connected(pnode.clone()) {
                $func(pnode.clone());
            }
        }
    }
}

impl ConnmanNodes {

    pub fn for_node<'a>(&'a mut self, 
        id:   NodeId,
        func: &'a ConnmanForNodeFn) -> bool {

        for_node!{self, id, func}
    }

    pub fn for_node_mut<'a>(&'a mut self, 
        id:   NodeId,
        func: &'a mut ConnmanForNodeFnMut) -> bool {

        for_node!{self, id, func}
    }

    pub fn for_each_node<'a>(&'a mut self, func: &'a ConnmanNodeFn)  {

        for_each_node!{self, func}
    }

    pub fn for_each_node_mut<'a>(&'a mut self, func: &'a mut ConnmanNodeFnMut)  {

        for_each_node!{self, func}
    }
}
