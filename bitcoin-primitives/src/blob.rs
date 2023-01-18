crate::ix!();

pub const fn base_blob_width<const BITS: usize>() -> usize 
{
    BITS / 8
}

/**
  | Template base class for fixed-sized
  | opaque blobs.
  |
  */
#[derive(Clone,Debug,Hash)]
pub struct BaseBlob<const BITS: usize> 
where [u8; base_blob_width::<BITS>()]:
{
    pub data: [u8; base_blob_width::<BITS>()],
}

//------------------------------
unsafe impl<const BITS: usize> Send for BaseBlob<BITS> 
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{}

unsafe impl<const BITS: usize> Sync for BaseBlob<BITS> 
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{}

//------------------------------
impl<const BITS: usize> PartialEq<BaseBlob<BITS>> for BaseBlob<BITS> 
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{
    fn eq(&self, other: &BaseBlob<BITS>) -> bool {
        self.compare(other) == 0
    }
}

impl<const BITS: usize> Eq for BaseBlob<BITS> 
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{}

impl<const BITS: usize> Ord for BaseBlob<BITS> 
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{
    
    fn cmp(&self, other: &BaseBlob<BITS>) -> Ordering {

        let x = self.compare(other);

        match x {
            _ if x < 0  => Ordering::Less,
            _ if x == 0 => Ordering::Equal,
            _ if x > 0  => Ordering::Greater,
            _ => unreachable![],
        }
    }
}

impl<const BITS: usize> PartialOrd<BaseBlob<BITS>> for BaseBlob<BITS> 
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{
    fn partial_cmp(&self, other: &BaseBlob<BITS>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const BITS: usize> Default for BaseBlob<BITS> 
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{

    /**
      | construct 0 value by default
      |
      */
    fn default() -> Self {
    
        Self {
            data: [0; base_blob_width::<BITS>()],
        }
    }
}

impl<const BITS: usize> From<u8> for BaseBlob<BITS> 
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{

    /**
      | constructor for constants between
      | 1 and 255
      |
      */
    fn from(v: u8) -> Self {
    
        todo!();
        /*
            : m_data{v}
        */
    }
}

impl<const BITS: usize> From<&Vec<u8>> for BaseBlob<BITS> 
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{

    fn from(vch: &Vec<u8>) -> Self {
    
        todo!();
        /*
           assert(vch.size() == sizeof(m_data));
           memcpy(m_data, vch.data(), sizeof(m_data));
        */
    }
}

impl<const BITS: usize> BaseBlob<BITS> 
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{
    pub const ZERO: Self = Self::zero();
    pub const ONE:  Self = Self::one();

    pub const fn zero() -> Self {
        Self {
            data: [0; base_blob_width::<BITS>()]
        }
    }

    pub const fn one()  -> Self { 
        let mut x = Self {
            data: [0; base_blob_width::<BITS>()]
        };
        x.data[0] += 1;
        x
    }

    pub fn is_null(&self) -> bool {
        
        todo!();
        /*
            for (int i = 0; i < WIDTH; i++)
                if (m_data[i] != 0)
                    return false;
            return true;
        */
    }
    
    pub fn set_null(&mut self)  {
        
        todo!();
        /*
            memset(m_data, 0, sizeof(m_data));
        */
    }
    
    #[inline] pub fn compare(&self, other: &BaseBlob<BITS>) -> i32 {
        
        todo!();
        /*
            return memcmp(m_data, other.m_data, sizeof(m_data));
        */
    }
    
    pub fn data(&self) -> *const u8 {
        
        self.data.as_ptr()
    }
    
    pub fn data_mut(&mut self) -> *mut u8 {
        
        self.data.as_mut_ptr()
    }
    
    pub fn begin_mut(&mut self) -> *mut u8 {
        
        &mut self.data[0] as *mut _
    }
    
    pub fn end_mut(&mut self) -> *mut u8 {
        
        todo!();
        /*
            return &m_data[WIDTH];
        */
    }
    
    pub fn begin(&self) -> *const u8 {
        
        todo!();
        /*
            return &m_data[0];
        */
    }
    
    pub fn end(&self) -> *const u8 {
        
        todo!();
        /*
            return &m_data[WIDTH];
        */
    }
    
    pub fn size(&self) -> u32 {
        
        todo!();
        /*
            return sizeof(m_data);
        */
    }
    
    pub fn get_u64(&self, pos: i32) -> u64 {
        
        todo!();
        /*
            const uint8_t* ptr = m_data + pos * 8;
            return ((uint64_t)ptr[0]) | 
                   ((uint64_t)ptr[1]) << 8 | 
                   ((uint64_t)ptr[2]) << 16 | 
                   ((uint64_t)ptr[3]) << 24 | 
                   ((uint64_t)ptr[4]) << 32 | 
                   ((uint64_t)ptr[5]) << 40 | 
                   ((uint64_t)ptr[6]) << 48 | 
                   ((uint64_t)ptr[7]) << 56;
        */
    }
    
    
    pub fn serialize<Stream>(&self, s: &mut Stream)  {
    
        todo!();
        /*
            s.write((char*)m_data, sizeof(m_data));
        */
    }
    
    
    pub fn unserialize<Stream>(&mut self, s: &mut Stream)  {
    
        todo!();
        /*
            s.read((char*)m_data, sizeof(m_data));
        */
    }

    pub fn get_hex(&self) -> String {
        
        todo!();
        /*
        let mut data_rev: [u8; Self::WIDTH] = unsafe { std::mem::zeroed() };

        for i in 0..Self::WIDTH {
            data_rev[i] = self.data[Self::WIDTH - 1 - i]
        }

        hex_str(data_rev)
        */
    }
    
    pub fn set_hex(&mut self, psz: *const u8)  {
        
        todo!();
        /*
            memset(m_data, 0, sizeof(m_data));

        // skip leading spaces
        while (IsSpace(*psz))
            psz++;

        // skip 0x
        if (psz[0] == '0' && ToLower(psz[1]) == 'x')
            psz += 2;

        // hex string to uint
        size_t digits = 0;
        while (::HexDigit(psz[digits]) != -1)
            digits++;
        unsigned char* p1 = (unsigned char*)m_data;
        unsigned char* pend = p1 + WIDTH;
        while (digits > 0 && p1 < pend) {
            *p1 = ::HexDigit(psz[--digits]);
            if (digits > 0) {
                *p1 |= ((unsigned char)::HexDigit(psz[--digits]) << 4);
                p1++;
            }
        }
        */
    }
    
    pub fn set_hex_from_str(&mut self, str_: &str)  {
        
        todo!();
        /*
            SetHex(str.c_str());
        */
    }
    
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return (GetHex());
        */
    }
}

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

/**
  | 160-bit opaque blob.
  | 
  | -----------
  | @note
  | 
  | This type is called u160 for historical
  | reasons only. It is an opaque blob of
  | 160 bits and has no integer operations.
  |
  */
#[derive(Clone,Default,PartialEq,Eq,Hash)]
pub struct u160 {
    pub blob: BaseBlob<160>,
}

impl From<&Vec<u8>> for u160 {
    
    fn from(vch: &Vec<u8>) -> Self {
    
        todo!();
        /*


            : base_blob<160>(vch)
        */
    }
}
