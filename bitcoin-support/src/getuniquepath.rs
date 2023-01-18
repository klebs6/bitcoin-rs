crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/getuniquepath.h]

/**
  | Helper function for getting a unique
  | path
  | 
  | -----------
  | @param[in] base
  | 
  | Base path
  | 
  | -----------
  | @return
  | 
  | base joined with a random 8-character
  | long string. @post Returned path is
  | unique with high probability.
  |
  */
pub fn get_unique_path(_base: &Box<Path>) -> Box<Path> {
    
    todo!();
        /*
            FastRandomContext rnd;
        fs::path tmpFile = base / HexStr(rnd.randbytes(8));
        return tmpFile;
        */
}
