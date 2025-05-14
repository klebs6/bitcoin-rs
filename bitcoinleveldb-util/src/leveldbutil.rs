// ---------------- [ File: bitcoinleveldb-util/src/leveldbutil.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/leveldbutil.cc]

pub trait Name {

    fn name(&self) -> *const u8;
}

pub trait GetName {

    /**
      | Get a name for the file, only for error
      | reporting
      |
      */
    fn get_name(&self) -> &'static str;
}

pub fn usage()  {
    
    todo!();
        /*
            fprintf(stderr,
              "Usage: leveldbutil command...\n"
              "   dump files...         -- dump contents of specified files\n");
        */
}
