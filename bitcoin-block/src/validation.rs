crate::ix!();

#[derive(Default)]
pub struct BlockValidationState {
    base: ValidationState<BlockValidationResult>,
}

impl BlockValidationState {

    delegate! {
        to self.base {

            pub fn invalid(&mut self, 
                result:        BlockValidationResult,
                reject_reason: Option<&str>,
                debug_message: Option<&str>) -> bool;
            
            pub fn error(&mut self, reject_reason: &String) -> bool;
            
            pub fn is_valid(&self) -> bool;
            
            pub fn is_invalid(&self) -> bool;
            
            pub fn is_error(&self) -> bool;
            
            pub fn get_result(&self) -> BlockValidationResult;
            
            pub fn get_reject_reason(&self) -> String;
            
            pub fn get_debug_message(&self) -> String;
            
            pub fn to_string(&self) -> String;
        }
    }
}

impl Default for BlockValidationResult {
    fn default() -> Self {
        BlockValidationResult::BLOCK_RESULT_UNSET
    }
}

/**
  | A "reason" why a block was invalid, suitable
  | for determining whether the provider
  | of the block should be banned/ignored/disconnected/etc.
  | 
  | These are much more granular than the
  | rejection codes, which may be more useful
  | for some other use-cases.
  |
  */
#[derive(PartialEq,Eq,Clone,Debug)]
pub enum BlockValidationResult {

    /**
      | initial value. Block has not yet been
      | rejected
      |
      */
    BLOCK_RESULT_UNSET = 0,  

    /**
      | invalid by consensus rules (excluding
      | any below reasons)
      |
      */
    BLOCK_CONSENSUS,         

    /**
      | Invalid by a change to consensus rules
      | more recent than SegWit.
      | 
      | Currently unused as there are no such
      | consensus rule changes, and any download
      | sources realistically need to support
      | SegWit in order to provide useful data,
      | so differentiating between always-invalid
      | and invalid-by-pre-SegWit-soft-fork
      | is uninteresting.
      |
      */
    BLOCK_RECENT_CONSENSUS_CHANGE,

    /**
      | this block was cached as being invalid
      | and we didn't store the reason why
      |
      */
    BLOCK_CACHED_INVALID,    

    /**
      | invalid proof of work or time too old
      |
      */
    BLOCK_INVALID_HEADER,    

    /**
      | the block's data didn't match the data
      | committed to by the PoW
      |
      */
    BLOCK_MUTATED,           

    /**
      | We don't have the previous block the
      | checked one is built on
      |
      */
    BLOCK_MISSING_PREV,      

    /**
      | A block this one builds on is invalid
      |
      */
    BLOCK_INVALID_PREV,      

    /**
      | block timestamp was > 2 hours in the future
      | (or our clock is bad)
      |
      */
    BLOCK_TIME_FUTURE,       

    /**
      | the block failed to meet one of our checkpoints
      |
      */
    BLOCK_CHECKPOINT,        
}

