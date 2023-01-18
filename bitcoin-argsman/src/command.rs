crate::ix!();

#[derive(Default)]
pub struct ArgsManagerCommand {

    /**
      | The command (if one has been registered
      | with AddCommand), or empty
      |
      */
    pub command: Option<String>,


    /**
      | If command is non-empty: Any args that
      | followed it
      | 
      | If command is empty: The unregistered
      | command and any args that followed it
      |
      */
    pub args:    Vec<String>,
}
