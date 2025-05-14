// ---------------- [ File: bitcoin-univalue/src/get.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/univalue/lib/univalue_get.cpp]

pub fn parse_prechecks(x: &String) -> bool {
    
    if x.is_empty() {
        //  No empty string allowed
        return false;
    }

    let first  = x.chars().nth(0).unwrap() as i32;
    let last   = x.chars().nth(x.len() - 1).unwrap() as i32;

    let padded = json_isspace(first) || json_isspace(last);

    if x.len() >= 1 && padded {
        //  No padding allowed
        return false;
    }

    if x.len() != unsafe { libc::strlen(x.as_ptr() as *const i8) } {
        //  No embedded NUL characters allowed
        return false;
    }

    true
}

pub fn parse_int32(
        str_: &String,
        out:  *mut i32) -> bool {
    
    todo!();
        /*
            if (!ParsePrechecks(str))
            return false;
        char *endp = nullptr;
        errno = 0; // strtol will not set errno if valid
        long int n = strtol(str.c_str(), &endp, 10);
        if(out) *out = (int32_t)n;
        // Note that strtol returns a *long int*, so even if strtol doesn't report an over/underflow
        // we still have to check that the returned value is within the range of an *int32_t*. On 64-bit
        // platforms the size of these types may be different.
        return endp && *endp == 0 && !errno &&
            n >= std::numeric_limits<int32_t>::min() &&
            n <= std::numeric_limits<int32_t>::max();
        */
}

pub fn parse_int64(
        str_: &String,
        out:  *mut i64) -> bool {
    
    todo!();
        /*
            if (!ParsePrechecks(str))
            return false;
        char *endp = nullptr;
        errno = 0; // strtoll will not set errno if valid
        long long int n = strtoll(str.c_str(), &endp, 10);
        if(out) *out = (int64_t)n;
        // Note that strtoll returns a *long long int*, so even if strtol doesn't report a over/underflow
        // we still have to check that the returned value is within the range of an *int64_t*.
        return endp && *endp == 0 && !errno &&
            n >= std::numeric_limits<int64_t>::min() &&
            n <= std::numeric_limits<int64_t>::max();
        */
}

pub fn parse_double(
        x:   &String,
        out: *mut f64) -> bool {
    
    if !parse_prechecks(x) {
        return false;
    }

    let first  = x.chars().nth(0).unwrap();
    let second = x.chars().nth(1).unwrap();

    if x.len() >= 2 && first == '0' && second == 'x' {

        //  No hexadecimal floats allowed
        return false;
    }

    let result: Result<f64,_> = x.parse::<f64>();

    if out != std::ptr::null_mut() {

        unsafe {

            if let Ok(result) = result {
                *out = result;

            } else {
                *out = 0.0;
            }
        }
    }

    result.is_ok()
}

impl UniValue {
    
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
