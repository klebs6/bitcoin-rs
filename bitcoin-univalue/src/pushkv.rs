// ---------------- [ File: bitcoin-univalue/src/pushkv.rs ]
crate::ix!();

impl UniValue {

    pub fn push_kvs(&mut self, obj: &UniValue) -> bool {
        
        todo!();
        /*
            if (typ != VOBJ || obj.typ != VOBJ)
            return false;

        for (size_t i = 0; i < obj.keys.size(); i++)
            __pushKV(obj.keys[i], obj.values.at(i));

        return true;
        */
    }

    pub fn pushkv<K,V>(&mut self, k: K, v: V) {

        todo!();

        /*
        pub fn pushkv(&mut self, 
            key: &String,
            val: &UniValue)  {
            
            todo!();
            /*
                keys.push_back(key);
            values.push_back(val_);
            */
        }
        
        pub fn pushkv(&mut self, 
            key: &String,
            val: &UniValue) -> bool {
            
            todo!();
            /*
                if (typ != VOBJ)
                return false;

            size_t idx;
            if (findKey(key, idx))
                values[idx] = val_;
            else
                __pushKV(key, val_);
            return true;
            */
        }
        
        pub fn pushkv(&mut self, 
            key: &String,
            val: &String) -> bool {
            
            todo!();
            /*
                UniValue tmpVal(VSTR, val_);
                return pushKV(key, tmpVal);
            */
        }
        
        pub fn pushkv(&mut self, 
            key: &String,
            val: *const u8) -> bool {
            
            todo!();
            /*
                std::string _val(val_);
                return pushKV(key, _val);
            */
        }
        
        pub fn pushkv(&mut self, 
            key: &String,
            val: i64) -> bool {
            
            todo!();
            /*
                UniValue tmpVal(val_);
                return pushKV(key, tmpVal);
            */
        }
        
        pub fn pushkv(&mut self, 
            key: &String,
            val: u64) -> bool {
            
            todo!();
            /*
                UniValue tmpVal(val_);
                return pushKV(key, tmpVal);
            */
        }
        
        pub fn pushkv(&mut self, 
            key: &String,
            val: bool) -> bool {
            
            todo!();
            /*
                UniValue tmpVal(val_);
                return pushKV(key, tmpVal);
            */
        }
        
        pub fn pushkv(&mut self, 
            key: &String,
            val: i32) -> bool {
            
            todo!();
            /*
                UniValue tmpVal((int64_t)val_);
                return pushKV(key, tmpVal);
            */
        }
        
        pub fn pushkv(&mut self, 
            key: &String,
            val: f64) -> bool {
            
            todo!();
            /*
                UniValue tmpVal(val_);
                return pushKV(key, tmpVal);
            */
        }
        */
    }
}
