crate::ix!();

impl UniValue {

    pub fn push_back<T: ?Sized>(&mut self, val: &T) -> bool {
        todo!();

        /*
        pub fn push_back(&mut self, val: &UniValue) -> bool {
            
            todo!();
            /*
            
            */
        }
        
        pub fn push_back(&mut self, val: &String) -> bool {
            
            todo!();
            /*
                UniValue tmpVal(VSTR, val_);
                return push_back(tmpVal);
            */
        }
        
        pub fn push_back(&mut self, val: *const u8) -> bool {
            
            todo!();
            /*
                std::string s(val_);
                return push_back(s);
            */
        }
        
        pub fn push_back(&mut self, val: u64) -> bool {
            
            todo!();
            /*
                UniValue tmpVal(val_);
                return push_back(tmpVal);
            */
        }
        
        pub fn push_back(&mut self, val: i64) -> bool {
            
            todo!();
            /*
                UniValue tmpVal(val_);
                return push_back(tmpVal);
            */
        }
        
        pub fn push_back(&mut self, val: bool) -> bool {
            
            todo!();
            /*
                UniValue tmpVal(val_);
                return push_back(tmpVal);
            */
        }
        
        pub fn push_back(&mut self, val: i32) -> bool {
            
            todo!();
            /*
                UniValue tmpVal(val_);
                return push_back(tmpVal);
            */
        }
        
        pub fn push_back(&mut self, val: f64) -> bool {
            
            todo!();
            /*
                UniValue tmpVal(val_);
                return push_back(tmpVal);
            */
        }

        pub fn push_back(&mut self, val: &UniValue) -> bool {
            
            todo!();
            /*
                if (typ != VARR)
                return false;

            values.push_back(val_);
            return true;
            */
        }
        
        pub fn push_backv(&mut self, vec: &Vec<UniValue>) -> bool {
            
            todo!();
            /*
                if (typ != VARR)
                return false;

            values.insert(values.end(), vec.begin(), vec.end());

            return true;
            */
        }
        */
    }
}
