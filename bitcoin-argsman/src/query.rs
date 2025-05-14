// ---------------- [ File: bitcoin-argsman/src/query.rs ]
crate::ix!();

impl ArgsManagerInner {

    /**
      | Return true if the given argument has
      | been manually set
      | 
      | -----------
      | @param strArg
      | 
      | Argument to get (e.g. "-foo")
      | 
      | -----------
      | @return
      | 
      | true if the argument has been set
      |
      */
    pub fn is_arg_set(&self, str_arg: &str) -> bool {
        
        !self.get_setting(str_arg).0.is_null()
    }
    
    /**
      | Return true if the argument was originally
      | passed as a negated option, i.e. -nofoo.
      | 
      | -----------
      | @param strArg
      | 
      | Argument to get (e.g. "-foo")
      | 
      | -----------
      | @return
      | 
      | true if the argument was passed negated
      |
      */
    pub fn is_arg_negated(&self, str_arg: &str) -> bool {

        self.get_setting(str_arg).0.is_false()
    }
}
