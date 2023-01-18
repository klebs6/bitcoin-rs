crate::ix!();

/**
  | Descriptor with some wallet metadata
  |
  */
pub struct WalletDescriptor {

    descriptor:    Arc<dyn Descriptor>,
    creation_time: u64, // default = 0

    /**
      | First item in range; start of range,
      | inclusive, i.e. [range_start, range_end).
      | This never changes.
      |
      */
    range_start:   i32, // default = 0

    /**
      | Item after the last; end of range, exclusive,
      | i.e. [range_start, range_end). This
      | will increment with each TopUp()
      |
      */
    range_end:     i32, // default = 0

    /**
      | Position of the next item to generate
      |
      */
    next_index:    i32, // default = 0

    cache:         DescriptorCache,
}

lazy_static!{
    /*
    SERIALIZE_METHODS(WalletDescriptor, obj)
        {
            std::string descriptor_str;
            SER_WRITE(obj, descriptor_str = obj.descriptor->ToString());
            READWRITE(descriptor_str, obj.creation_time, obj.next_index, obj.range_start, obj.range_end);
            SER_READ(obj, obj.DeserializeDescriptor(descriptor_str));
        }
    */
}

impl WalletDescriptor {
    
    pub fn deserialize_descriptor(&mut self, str_: &String)  {
        
        todo!();
        /*
            std::string error;
            FlatSigningProvider keys;
            descriptor = Parse(str, keys, error, true);
            if (!descriptor) {
                throw std::ios_base::failure("Invalid descriptor: " + error);
            }
        */
    }
    
    pub fn new(
        descriptor:    Arc<dyn Descriptor>,
        creation_time: u64,
        range_start:   i32,
        range_end:     i32,
        next_index:    i32) -> Self {
    
        todo!();
        /*
        : descriptor(descriptor),
        : creation_time(creation_time),
        : range_start(range_start),
        : range_end(range_end),
        : next_index(next_index),

        
        */
    }
}
