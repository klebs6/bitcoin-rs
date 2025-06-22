// ---------------- [ File: bitcoin-univalue/src/get.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/univalue/lib/univalue_get.cpp]

impl UniValue {

    pub fn get_obj_map(&self, kv: &mut HashMap<String,UniValue>)  {
        
        todo!();
        /*
            if (typ != VOBJ)
            return;

        kv.clear();
        for (size_t i = 0; i < keys.size(); i++)
            kv[keys[i]] = values[i];
        */
    }

    pub fn get_type(&self) -> uni_value::VType {

        todo!();
        /*
            return typ;
        */
    }
    
    pub fn get_val_str(&self) -> &String {
        
        todo!();
        /*
            return val;
        */
    }
   
    /**
      | Strict type-specific getters, these
      | throw std::runtime_error if the value
      | is of unexpected type
      |
      */
    pub fn get_keys<'a>(&'a self) -> Result<&'a Vec<String>,StdException> {
        
        if self.typ != uni_value::VType::VOBJ {
            return Err(runtime_error("JSON value is not an object as expected"));
        }

        Ok(&self.keys)
    }
    
    pub fn get_values<'a>(&'a self) -> Result<&'a Vec<UniValue>,StdException> {
        
        if self.typ != uni_value::VType::VOBJ 
        && self.typ != uni_value::VType::VARR 
        {
            return Err(runtime_error("JSON value is not an object or array as expected"));
        }

        Ok(&self.values)
    }
    
    pub fn get_bool(&self) -> bool {
        
        todo!();
        /*
            if (typ != VBOOL)
            throw std::runtime_error("JSON value is not a boolean as expected");
        return isTrue();
        */
    }
    
    pub fn get_str(&self) -> &str {
        
        todo!();
        /*
            if (typ != VSTR)
            throw std::runtime_error("JSON value is not a string as expected");
        return getValStr();
        */
    }

    pub fn get_str_mut(&mut self) -> &mut str {
        
        todo!();
        /*
            if (typ != VSTR)
            throw std::runtime_error("JSON value is not a string as expected");
        return getValStr();
        */
    }
    
    pub fn get_int(&self) -> i32 {
        
        todo!();
        /*
        if (typ != VNUM)
            throw std::runtime_error("JSON value is not an integer as expected");
        int32_t retval;
        if (!ParseInt32(getValStr(), &retval))
            throw std::runtime_error("JSON integer out of range");
        return retval;
        */
    }
    
    pub fn get_int64(&self) -> i64 {
        
        todo!();
        /*
        if (typ != VNUM)
            throw std::runtime_error("JSON value is not an integer as expected");
        int64_t retval;
        if (!ParseInt64(getValStr(), &retval))
            throw std::runtime_error("JSON integer out of range");
        return retval;
        */
    }
    
    pub fn get_real(&self) -> f64 {
        
        todo!();
        /*
        if (typ != VNUM)
            throw std::runtime_error("JSON value is not a number as expected");
        double retval;
        if (!ParseDouble(getValStr(), &retval))
            throw std::runtime_error("JSON double out of range");
        return retval;
        */
    }
    
    pub fn get_obj(&self) -> &UniValue {
        
        todo!();
        /*
        if (typ != VOBJ)
            throw std::runtime_error("JSON value is not an object as expected");
        return *this;
        */
    }
    
    pub fn get_array(&self) -> &UniValue {
        
        todo!();
        /*
        if (typ != VARR)
            throw std::runtime_error("JSON value is not an array as expected");
        return *this;
        */
    }
}
