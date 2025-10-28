// ---------------- [ File: bitcoin-argsman/src/help.rs ]
crate::ix!();

pub fn setting_name(arg: &str) -> String {
    
    match arg.len() > 0 && arg.chars().nth(0).unwrap() == '-' {
        true   => arg[1..].to_string(),
        false  => arg.to_string()
    }
}

impl ArgsManagerInner {

    /**
      | @return
      | 
      | true if help has been requested via a
      | command-line arg
      |
      */
    pub fn help_requested(&self) -> bool {

        self.is_arg_set("-?") 
            || self.is_arg_set("-h") 
            || self.is_arg_set("-help") 
            || self.is_arg_set("-help-debug")
    }

    /**
      | Add help options to the args manager
      |
      */
    pub fn setup_help_options(&mut self)  {
        
        self.add_arg(&ARG_HELP);

        self.add_hidden_args(&vec!{"-h", "-help"});
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setting_name_strips_single_leading_dash() {
        assert_eq!(setting_name("-foo"), "foo");
        assert_eq!(setting_name("bar"), "bar");
        assert_eq!(setting_name(""), "");
    }
}
