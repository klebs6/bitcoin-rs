crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/script/keyorigin.h]

#[derive(Default,Clone)]
pub struct KeyOriginInfo {

    /**
      | First 32 bits of the Hash160 of the public
      | key at the root of the path
      |
      */
    fingerprint: [u8; 4],
    path:        Vec<u32>,
}

impl PartialEq<KeyOriginInfo> for KeyOriginInfo {
    
    #[inline] fn eq(&self, other: &KeyOriginInfo) -> bool {
        todo!();
        /*
            return std::equal(std::begin(a.fingerprint), std::end(a.fingerprint), std::begin(b.fingerprint)) && a.path == b.path;
        */
    }
}

impl Eq for KeyOriginInfo {}

lazy_static!{
    /*
    SERIALIZE_METHODS(KeyOriginInfo, obj) { 
        READWRITE(obj.fingerprint, obj.path); 
    }
    */
}

impl KeyOriginInfo {
    
    pub fn clear(&mut self)  {
        
        let ptr = self.fingerprint.as_mut_ptr() as *mut _ as *mut libc::c_void;

        unsafe { libc::memset(ptr, 0, 4) };

        self.path.clear();
    }
}
