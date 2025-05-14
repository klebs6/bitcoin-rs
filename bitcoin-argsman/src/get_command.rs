// ---------------- [ File: bitcoin-argsman/src/get_command.rs ]
crate::ix!();

impl ArgsManagerInner {

    /**
      | Get the command and command args (returns
      | std::nullopt if no command provided)
      |
      */
    pub fn get_command(&self) -> Option<ArgsManagerCommand> {
        
        let mut ret = ArgsManagerCommand::default();

        if self.command.is_empty() {
            // No command was passed
            return None;
        }

        let mut it = self.command.iter();

        if !self.accept_any_command {

            // The registered command
            ret.command = Some(it.as_slice()[0].clone());

            it.next();
        }

        while let Some(item) = it.next() {

            // The unregistered command and args
            // (if any)
            ret.args.push(item.to_string());
        }

        Some(ret)
    }
}
