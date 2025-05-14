// ---------------- [ File: bitcoin-block/src/flatfile_pos.rs ]
crate::ix!();

pub struct FlatFilePos {
    pub n_file: i32,
    pub n_pos:  u32,
}

lazy_static!{
    /*
    SERIALIZE_METHODS(FlatFilePos, obj) { 
        READWRITE(VARINT_MODE(obj.nFile, VarIntMode::NONNEGATIVE_SIGNED), VARINT(obj.nPos)); 
    }
    */
}

impl Default for FlatFilePos {
    
    fn default() -> Self {
        Self {
            n_file: -1,
            n_pos:   0,
        }
    }
}

impl PartialEq<FlatFilePos> for FlatFilePos {
    
    #[inline] fn eq(&self, other: &FlatFilePos) -> bool {
        self.n_file == other.n_file 
        && self.n_pos == other.n_pos
    }
}

impl Eq for FlatFilePos {}

impl FlatFilePos {
    
    pub fn new(
        n_file_in: i32,
        n_pos_in:  u32) -> Self {
    
        Self {
            n_file: n_file_in,
            n_pos:  n_pos_in,
        }
    }
    
    pub fn set_null(&mut self)  {
        
        self.n_file = -1;
        self.n_pos  = 0;
    }
    
    pub fn is_null(&self) -> bool {
        
        self.n_file == -1
    }

    pub fn to_string(&self) -> String {
        
        format!{
            "FlatFilePos(nFile={}, nPos={})",
            self.n_file,
            self.n_pos
        }
    }
}
