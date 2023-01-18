crate::ix!();

pub struct NodeVProcessMsg {

    pub process_msg: Vec<NetMessage>,
}

impl Default for NodeVProcessMsg {

    fn default() -> Self {
        Self {
            process_msg: Vec::<NetMessage>::default(),
        }
    }
}
