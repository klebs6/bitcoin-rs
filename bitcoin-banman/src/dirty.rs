// ---------------- [ File: bitcoin-banman/src/dirty.rs ]
crate::ix!();

impl BanMan {

    pub fn banned_set_is_dirty(&mut self) -> bool {
        self.cs_banned.lock().is_dirty
    }
    
    /**
      | set the "dirty" flag for the banlist
      |
      */
    pub fn set_banned_set_dirty(&mut self, dirty: Option<bool>)  {

        let dirty: bool = dirty.unwrap_or(true);
        
        // reuse m_banned lock for the m_is_dirty flag
        self.cs_banned.get_mut().is_dirty = dirty;
    }
}
