crate::ix!();

impl From<Instant> for UniValue {
    fn from(val: Instant) -> Self {
        todo!();
    }
}

impl From<u64> for UniValue {
    fn from(val: u64) -> Self {
    
        todo!();
        /*
            setInt(val_);
        */
    }
}

impl From<usize> for UniValue {
    fn from(val: usize) -> Self {
    
        todo!();
        /*
            setInt(val_);
        */
    }
}

impl From<uni_value::VType> for UniValue {
    fn from(val: uni_value::VType) -> Self {
    
        todo!();
        /*
            setInt(val_);
        */
    }
}

impl From<i64> for UniValue {
    fn from(val: i64) -> Self {
    
        todo!();
        /*
            setInt(val_);
        */
    }
}

impl From<i32> for UniValue {
    fn from(val: i32) -> Self {
    
        todo!();
        /*
            setInt(val_);
        */
    }
}

impl From<bool> for UniValue {
    fn from(val: bool) -> Self {
    
        todo!();
        /*
            setBool(val_);
        */
    }
}

impl From<f64> for UniValue {
    fn from(val: f64) -> Self {
    
        todo!();
        /*
            setFloat(val_);
        */
    }
}

impl From<&str> for UniValue {
    fn from(val: &str) -> Self {
    
        todo!();
        /*
            setStr(val_);
        */
    }
}

impl From<*const u8> for UniValue {
    fn from(val: *const u8) -> Self {
    
        todo!();
        /*
            std::string s(val_);
            setStr(s);
        */
    }
}
