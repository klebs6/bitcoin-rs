// ---------------- [ File: bitcoin-univalue/src/univalue.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/univalue/include/univalue.h]

#[derive(Clone,Debug)]
pub struct UniValue {

    pub typ:    uni_value::VType,

    /**
      | numbers are stored as C++ strings
      |
      */
    pub val:    String,

    pub keys:   Vec<String>,
    pub values: Vec<UniValue>,
}

pub mod uni_value {

    #[derive(PartialEq,Eq,Clone,Debug)]
    pub enum VType { 
        VNULL, 
        VOBJ, 
        VARR, 
        VSTR, 
        VNUM, 
        VBOOL, 
    }
}

impl Default for UniValue {
    fn default() -> Self {
        todo!();
        /*
            typ = VNULL;
        */
    }
}

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

impl UniValue {

    pub fn new(
        initial_type: uni_value::VType,
        initial_str:  Option<&str>) -> Self {
        let initial_str: &str = initial_str.unwrap_or("");
        todo!();
        /*


            typ = initialType;
            val = initialStr;
        */
    }
    
    pub fn set_i32(&mut self, val: i32) -> bool {
        
        todo!();
        /*
            return setInt((int64_t)val_);
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
    
    pub fn empty(&self) -> bool {
        
        todo!();
        /*
            return (values.size() == 0);
        */
    }
    
    pub fn size(&self) -> usize {
        
        todo!();
        /*
            return values.size();
        */
    }
    
    pub fn exists(&self, key: &String) -> bool {
        
        todo!();
        /*
            size_t i; return findKey(key, i);
        */
    }
    
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
    
    pub fn ty(&self) -> uni_value::VType {
        
        todo!();
        /*
            return getType();
        */
    }
    
    pub fn find_value(&mut self, 
        obj:  &UniValue,
        name: &str) -> &UniValue {
        
        todo!();
        /*
        
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            typ = VNULL;
        val.clear();
        keys.clear();
        values.clear();
        */
    }
    
    pub fn set_null(&mut self) -> bool {
        
        todo!();
        /*
            clear();
        return true;
        */
    }
    
    pub fn set_bool(&mut self, val: bool) -> bool {
        
        todo!();
        /*
            clear();
        typ = VBOOL;
        if (val_)
            val = "1";
        return true;
        */
    }
    
    pub fn set_num_str(&mut self, val: &String) -> bool {
        
        todo!();
        /*
            if (!validNumStr(val_))
            return false;

        clear();
        typ = VNUM;
        val = val_;
        return true;
        */
    }
    
    pub fn set_int<T: Integer>(&mut self, val: T) -> bool {
        
        todo!();
        /*
            std::ostringstream oss;

        oss << val_;

        return setNumStr(oss.str());
        */
    }
    
    pub fn set_float(&mut self, val: f64) -> bool {
        
        todo!();
        /*
            std::ostringstream oss;

        oss << std::setprecision(16) << val_;

        bool ret = setNumStr(oss.str());
        typ = VNUM;
        return ret;
        */
    }
    
    pub fn set_str(&mut self, val: &String) -> bool {
        
        todo!();
        /*
            clear();
        typ = VSTR;
        val = val_;
        return true;
        */
    }
    
    pub fn set_array(&mut self) -> bool {
        
        todo!();
        /*
            clear();
        typ = VARR;
        return true;
        */
    }
    
    pub fn set_object(&mut self) -> bool {
        
        todo!();
        /*
            clear();
        typ = VOBJ;
        return true;
        */
    }
    
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
    
    pub fn find_key(&self, 
        key:     &String,
        ret_idx: &mut usize) -> bool {
        
        todo!();
        /*
            for (size_t i = 0; i < keys.size(); i++) {
            if (keys[i] == key) {
                retIdx = i;
                return true;
            }
        }

        return false;
        */
    }
    
    pub fn check_object(&self, t: &HashMap<String,uni_value::VType>) -> bool {
        
        todo!();
        /*
            if (typ != VOBJ) {
            return false;
        }

        for (const auto& object: t) {
            size_t idx = 0;
            if (!findKey(object.first, idx)) {
                return false;
            }

            if (values.at(idx).getType() != object.second) {
                return false;
            }
        }

        return true;
        */
    }
}

pub enum JTokenType {
    JTOK_ERR        = -1,
    JTOK_NONE       = 0,                           // eof
    JTOK_OBJ_OPEN,
    JTOK_OBJ_CLOSE,
    JTOK_ARR_OPEN,
    JTOK_ARR_CLOSE,
    JTOK_COLON,
    JTOK_COMMA,
    JTOK_KW_NULL,
    JTOK_KW_TRUE,
    JTOK_KW_FALSE,
    JTOK_NUMBER,
    JTOK_STRING,
}

lazy_static!{
    /*
    extern enum jtokentype getJsonToken(std::string& tokenVal,
                                        unsigned int& consumed, const char *raw, const char *end);
    */
}

lazy_static!{
    /*
    extern const char *uvTypeName(UniValue::VType t);
    */
}

#[inline] pub fn json_token_is_value(jtt: JTokenType) -> bool {
    
    todo!();
        /*
            switch (jtt) {
        case JTOK_KW_NULL:
        case JTOK_KW_TRUE:
        case JTOK_KW_FALSE:
        case JTOK_NUMBER:
        case JTOK_STRING:
            return true;

        default:
            return false;
        }

        // not reached
        */
}

#[inline] pub fn json_isspace(ch: i32) -> bool {
    
    todo!();
        /*
            switch (ch) {
        case 0x20:
        case 0x09:
        case 0x0a:
        case 0x0d:
            return true;

        default:
            return false;
        }

        // not reached
        */
}

//-------------------------------------------[.cpp/bitcoin/src/univalue/lib/univalue.cpp]

lazy_static!{
    pub static ref NULL_UNI_VALUE: UniValue = UniValue::default();
}

impl Index<&str> for UniValue {
    type Output = UniValue;
    
    #[inline] fn index(&self, key: &str) -> &Self::Output {
        todo!();
        /*
            if (typ != VOBJ)
            return NullUniValue;

        size_t index = 0;
        if (!findKey(key, index))
            return NullUniValue;

        return values.at(index);
        */
    }
}

impl Index<usize> for UniValue {
    type Output = UniValue;
    
    #[inline] fn index(&self, index: usize) -> &Self::Output {
        todo!();
        /*
            if (typ != VOBJ && typ != VARR)
            return NullUniValue;
        if (index >= values.size())
            return NullUniValue;

        return values.at(index);
        */
    }
}

pub fn uv_type_name(t: uni_value::VType) -> *const u8 {
    
    todo!();
        /*
            switch (t) {
        case UniValue::VNULL: return "null";
        case UniValue::VBOOL: return "bool";
        case UniValue::VOBJ: return "object";
        case UniValue::VARR: return "array";
        case UniValue::VSTR: return "string";
        case UniValue::VNUM: return "number";
        }

        // not reached
        return nullptr;
        */
}

pub fn find_value<'a>(
        obj:  &'a UniValue,
        name: &'a str) -> &'a UniValue {
    
    todo!();
        /*
            for (unsigned int i = 0; i < obj.keys.size(); i++)
            if (obj.keys[i] == name)
                return obj.values.at(i);

        return NullUniValue;
        */
}

pub fn find_value_mut<'a>(
        obj:  &'a mut UniValue,
        name: &'a str) -> &'a mut UniValue {
    
    todo!();
        /*
            for (unsigned int i = 0; i < obj.keys.size(); i++)
            if (obj.keys[i] == name)
                return obj.values.at(i);

        return NullUniValue;
        */
}

pub fn valid_num_str(s: &str) -> bool {
    
    todo!();
        /*
            std::string tokenVal;
        unsigned int consumed;
        enum jtokentype tt = getJsonToken(tokenVal, consumed, s.data(), s.data() + s.size());
        return (tt == JTOK_NUMBER);
        */
}
