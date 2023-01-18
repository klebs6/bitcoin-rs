crate::ix!();

/**
  | Helper class for tests to unify the interface
  | between BlockBuilder/TableBuilder
  | and
  | 
  | Block/Table.
  |
  */
pub struct Constructor {
    data: KVMap,
}

pub trait ConstructorInterface: ConstructorFinishImpl + ConstructorNewIterator {}

pub trait ConstructorFinishImpl {

    /**
      | Construct the data structure from the
      | data in "data"
      |
      */
    fn finish_impl(&mut self, 
        options: &crate::Options,
        data:    &KVMap) -> crate::Status;
}

pub trait ConstructorNewIterator {

    fn new_iterator(&self) -> *mut LevelDBIterator;
}

impl Constructor {

    /*
    /**
      | Overridden in DBConstructor
      |
      */
    fn db(&self) -> *mut dyn DB {
        
        todo!();
        /*
            return nullptr;
        */
    }
    */

    pub fn new(cmp: Box<dyn SliceComparator>) -> Self {
    
        todo!();
        /*
        : data(STLLessThan(cmp)),

        
        */
    }
    
    pub fn add(&mut self, 
        key_:   &String,
        value: &Slice)  {
        
        todo!();
        /*
            data_[key] = value.ToString();
        */
    }

    /**
      | Finish constructing the data structure with
      | all the keys that have been added so far.
      |
      | Returns the keys in sorted order in "*keys"
      | and stores the key/value pairs in "*kvmap"
      */
    pub fn finish(&mut self, 
        options: &Options,
        keys:    *mut Vec<String>,
        kvmap:   *mut KVMap)  {
        
        todo!();
        /*
            *kvmap = data_;
        keys->clear();
        for (const auto& kvp : data_) {
          keys->push_back(kvp.first);
        }
        data_.clear();
        Status s = FinishImpl(options, *kvmap);
        ASSERT_TRUE(s.ok()) << s.ToString();
        */
    }
    
    pub fn data(&self) -> &KVMap {
        
        todo!();
        /*
            return data_;
        */
    }
}


