crate::ix!();

/**
  | Reply from Tor, can be single or multi-line
  |
  */
pub struct TorControlReply {
    code:  i32,
    lines: Vec<String>,
}

impl Default for TorControlReply {
    
    fn default() -> Self {
        todo!();
        /*
            Clear();
        */
    }
}

impl TorControlReply {
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            code = 0;
            lines.clear();
        */
    }
}
