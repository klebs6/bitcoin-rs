crate::ix!();

impl UniValue {

    pub fn is_null(&self) -> bool {
        
        todo!();
        /*
            return (typ == VNULL);
        */
    }
    
    pub fn is_true(&self) -> bool {
        
        todo!();
        /*
            return (typ == VBOOL) && (val == "1");
        */
    }
    
    pub fn is_false(&self) -> bool {
        
        todo!();
        /*
            return (typ == VBOOL) && (val != "1");
        */
    }
    
    pub fn is_bool(&self) -> bool {
        
        todo!();
        /*
            return (typ == VBOOL);
        */
    }
    
    pub fn is_str(&self) -> bool {
        
        todo!();
        /*
            return (typ == VSTR);
        */
    }
    
    pub fn is_num(&self) -> bool {
        
        todo!();
        /*
            return (typ == VNUM);
        */
    }
    
    pub fn is_array(&self) -> bool {
        
        todo!();
        /*
            return (typ == VARR);
        */
    }
    
    pub fn is_object(&self) -> bool {
        
        todo!();
        /*
            return (typ == VOBJ);
        */
    }
}
