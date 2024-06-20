crate::ix!();

/**
  | 256-bit opaque blob.
  | 
  | -----------
  | @note
  | 
  | This type is called uint256 for historical
  | reasons only. It is an opaque blob of
  | 256 bits and has no integer operations.
  | Use arith_uint256 if those are required.
  |
  */
#[derive(Default,Debug,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct u256 {
    pub blob: BaseBlob<256>,
}

impl u256 {
    delegate!{
        to self.blob {
            pub fn is_null(&self) -> bool;
            pub fn set_null(&mut self);
            pub fn to_string(&self) -> String;
        }
    }
}

unsafe impl Send for u256 {}
unsafe impl Sync for u256 {}

impl Serialize for u256 {

    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
    {
        todo!();
    }
}

impl<'de> Deserialize<'de> for u256 {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!();
    }
}

impl u256 {

    pub const ONE:  u256 = Self{ blob: BaseBlob::<256>::ONE  };
    pub const ZERO: u256 = Self{ blob: BaseBlob::<256>::ZERO  };

    pub fn byte_len(&mut self) -> usize {
        32
    }

    pub fn as_slice<'a>(&'a self) -> &'a [u8] {

        let data: *const u8 = self.blob.data();
        let size:   usize = self.blob.size().try_into().unwrap();

        unsafe {
            std::slice::from_raw_parts(data,size)
        }
    }

    pub fn as_slice_mut<'a>(&'a mut self) -> &'a mut [u8] {

        let data: *mut u8 = self.blob.data_mut();
        let size:   usize = self.blob.size().try_into().unwrap();

        unsafe {
            std::slice::from_raw_parts_mut(data,size)
        }
    }
}

impl From<&Vec<u8>> for u256 {

    fn from(v: &Vec<u8>) -> Self {
        todo!();
        /*
            : base_blob<256>(vch)
        */
    }
}

impl From<u8> for u256 {
    fn from(v: u8) -> Self {
    
        todo!();
        /*


            : base_blob<256>(v)
        */
    }
}

/**
  | uint256 from const char *.
  | 
  | This is a separate function because
  | the constructor uint256(const char*)
  | can result in dangerously catching
  | uint256(0).
  |
  */
impl From<*const u8> for u256 {

    #[inline] fn from(str_: *const u8) -> Self {
        
        todo!();
            /*
                uint256 rv;
            rv.SetHex(str);
            return rv;
            */
    }
}

/**
  | uint256 from std::string.
  | 
  | This is a separate function because
  | the constructor uint256(const std::string
  | &str) can result in dangerously catching
  | uint256(0) via std::string(const
  | char*).
  |
  */
impl From<&String> for u256 {

    #[inline] fn from(str_: &String) -> Self {
        
        todo!();
            /*
                uint256 rv;
            rv.SetHex(str);
            return rv;
            */
    }
}
