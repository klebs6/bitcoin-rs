crate::ix!();

pub struct KeyMetadata {

    n_version:      i32,

    /**
      | 0 means unknown
      |
      */
    n_create_time:  i64,

    /**
      | optional HD/bip32 keypath. Still used
      | to determine whether a key is a seed.
      | Also kept for backwards compatibility
      |
      */
    hd_keypath:     String,

    /**
      | id of the HD seed used to derive this key
      |
      */
    hd_seed_id:     KeyID,

    /**
      | Key origin info with path and fingerprint
      |
      */
    key_origin:     KeyOriginInfo,

    /**
      | Whether the key_origin is useful
      |
      */
    has_key_origin: bool, // default = false
}

pub mod key_metadata {
    pub const VERSION_BASIC:           i32 = 1;
    pub const VERSION_WITH_HDDATA:     i32 = 10;
    pub const VERSION_WITH_KEY_ORIGIN: i32 = 12;
    pub const CURRENT_VERSION:         i32 = VERSION_WITH_KEY_ORIGIN;
}

lazy_static!{
    /*
    SERIALIZE_METHODS(CKeyMetadata, obj)
        {
            READWRITE(obj.nVersion, obj.nCreateTime);
            if (obj.nVersion >= VERSION_WITH_HDDATA) {
                READWRITE(obj.hdKeypath, obj.hd_seed_id);
            }
            if (obj.nVersion >= VERSION_WITH_KEY_ORIGIN)
            {
                READWRITE(obj.key_origin);
                READWRITE(obj.has_key_origin);
            }
        }
    */
}

impl Default for KeyMetadata {
    
    fn default() -> Self {
        todo!();
        /*


            SetNull();
        */
    }
}

impl KeyMetadata {

    pub fn new(n_create_time: i64) -> Self {
    
        todo!();
        /*


            SetNull();
            nCreateTime = nCreateTime_;
        */
    }
    
    pub fn set_null(&mut self)  {
        
        todo!();
        /*
            nVersion = CKeyMetadata::CURRENT_VERSION;
            nCreateTime = 0;
            hdKeypath.clear();
            hd_seed_id.SetNull();
            key_origin.clear();
            has_key_origin = false;
        */
    }
}
