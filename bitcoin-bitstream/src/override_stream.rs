crate::ix!();

pub struct OverrideStream<Stream> {
    stream:    *mut Stream,
    n_type:    i32,
    n_version: i32,
}

impl<Stream> StreamInto for OverrideStream<Stream> {
    
    #[inline] fn stream_into<Item>(&self, rhs: &mut Item) {
        todo!();
        /*
            // Serialize to this stream
            ::Serialize(*this, obj);
            return (*this);
        */
    }
}

impl<Stream> StreamItems for OverrideStream<Stream> {
    
    #[inline] fn stream<Item>(&mut self, x: Item) {
        todo!();
        /*
            // Unserialize from this stream
            ::Unserialize(*this, obj);
            return (*this);
        */
    }
}

//------------------------------
impl<Stream> GetType for OverrideStream<Stream> {

    fn get_type(&self) -> i32 {
        
        todo!();
        /*
            return nType;
        */
    }
}

impl<Stream> GetVersion for OverrideStream<Stream> {

    fn get_version(&self) -> i32 {
        
        todo!();
        /*
            return nVersion;
        */
    }
}

impl<Stream> OverrideStream<Stream> {
    
    pub fn new(
        stream:    *mut Stream,
        n_type:    i32,
        n_version: i32) -> Self {
    
        todo!();
        /*
        : stream(stream_),
        : n_type(nType_),
        : n_version(nVersion_),

        
        */
    }
    
    pub fn write(&mut self, 
        pch:    *const u8,
        n_size: usize)  {
        
        todo!();
        /*
            stream->write(pch, nSize);
        */
    }
    
    pub fn read(&mut self, 
        pch:    *mut u8,
        n_size: usize)  {
        
        todo!();
        /*
            stream->read(pch, nSize);
        */
    }
    
    
    pub fn size(&self) -> usize {
        
        todo!();
        /*
            return stream->size();
        */
    }
    
    pub fn ignore(&mut self, size: usize)  {
        
        todo!();
        /*
            return stream->ignore(size);
        */
    }
}
