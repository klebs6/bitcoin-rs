// ---------------- [ File: bitcoin-univalue/src/univalue.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/univalue/include/univalue.h]

#[derive(Getters,Clone,Debug)]
#[getset(get="pub")]
pub struct UniValue {

    typ:    uni_value::VType,

    /// numbers are stored as strings
    val:    String,
    keys:   Vec<String>,
    values: Vec<UniValue>,
}

//-------------------------------------------[.cpp/bitcoin/src/univalue/lib/univalue.cpp]

lazy_static!{
    pub static ref NULL_UNI_VALUE: UniValue = UniValue::default();
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
    /// A default `UniValue` is the JSON literal **null**.
    fn default() -> Self {
        Self {
            typ:    uni_value::VType::VNULL,
            val:    String::new(),
            keys:   Vec::new(),
            values: Vec::new(),
        }
    }
}

impl UniValue {

    /// Create a new `UniValue` with the provided type
    /// and (optionally) initial string value.
    #[instrument(level = "trace", skip_all)]
    pub fn new(initial_type: uni_value::VType, initial_str: Option<&str>) -> Self {
        let mut uv = Self::default();
        match initial_type {
            uni_value::VType::VNULL => { /* nothing */ }
            uni_value::VType::VBOOL => {
                uv.set_bool(initial_str == Some("1"));
            }
            uni_value::VType::VSTR => {
                uv.set_str(initial_str.unwrap_or_default());
            }
            uni_value::VType::VNUM => {
                uv.val = initial_str.unwrap_or_default().to_owned();
                uv.typ = uni_value::VType::VNUM;
            }
            uni_value::VType::VARR => {
                uv.typ = uni_value::VType::VARR;
            }
            uni_value::VType::VOBJ => {
                uv.typ = uni_value::VType::VOBJ;
            }
        }
        uv
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
    
    /// Clear the current value, turning it into **null**.
    #[instrument(level = "trace", skip(self))]
    pub fn clear(&mut self) {
        self.typ    = uni_value::VType::VNULL;
        self.val.clear();
        self.keys.clear();
        self.values.clear();
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
