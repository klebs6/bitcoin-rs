// ---------------- [ File: bitcoinleveldb-slice/src/range.rs ]
crate::ix!();

/**
   A range of keys
  */
#[derive(Default)]
pub struct Range {

    /**
       Included in the range
      */
    start: Slice,

    /**
       Not included in the range
      */
    limit: Slice,
}

impl Range {

    pub fn new(
        s: &Slice,
        l: &Slice) -> Self {
    
        todo!();
        /*
        : start(s),
        : limit(l),

        
        */
    }
}
