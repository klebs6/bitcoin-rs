crate::ix!();

#[derive(Default,Serialize,Deserialize)]
pub struct BlockFileInfo {

    /**
      | number of blocks stored in file
      |
      */
    pub n_blocks:       u32,

    /**
      | number of used bytes of block file
      |
      */
    pub n_size:         u32,

    /**
      | number of used bytes in the undo file
      |
      */
    pub n_undo_size:    u32,

    /**
      | lowest height of block in file
      |
      */
    pub n_height_first: u32,

    /**
      | highest height of block in file
      |
      */
    pub n_height_last:  u32,

    /**
      | earliest time of block in file
      |
      */
    pub n_time_first:   u64,

    /**
      | latest time of block in file
      |
      */
    pub n_time_last:    u64,
}

impl BlockFileInfo {

    pub fn set_null(&mut self)  {
        
        self.n_blocks       = 0;
        self.n_size         = 0;
        self.n_undo_size    = 0;
        self.n_height_first = 0;
        self.n_height_last  = 0;
        self.n_time_first   = 0;
        self.n_time_last    = 0;
    }
    
    /**
      | update statistics (does not update
      | nSize)
      |
      */
    pub fn add_block(&mut self, 
        n_height_in: u32,
        n_time_in:   u64)  {
        
        if self.n_blocks == 0 || self.n_height_first > n_height_in {
            self.n_height_first = n_height_in;
        }

        if self.n_blocks == 0 || self.n_time_first > n_time_in {
            self.n_time_first = n_time_in;
        }

        self.n_blocks += 1;

        if n_height_in > self.n_height_last {
            self.n_height_last = n_height_in;
        }

        if n_time_in > self.n_time_last {
            self.n_time_last = n_time_in;
        }
    }

    pub fn to_string(&self) -> String {

        format!(
            "BlockFileInfo(blocks={}, size={}, heights={}...{}, time={}...{})",
            self.n_blocks,
            self.n_size,
            self.n_height_first,
            self.n_height_last,
            format_iso8601date(self.n_time_first.try_into().unwrap()),
            format_iso8601date(self.n_time_last.try_into().unwrap())
        )
    }
}
