// ---------------- [ File: bitcoin-argsman/src/command.rs ]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_command_with_registered_command() {
        let mut inner = ArgsManagerInner::default();
        inner.accept_any_command = false;
        inner.command = vec!["in".into(), "TXID:0".into(), "42".into()];
        let cmd = inner.get_command().unwrap();
        assert_eq!(cmd.command.as_deref(), Some("in"));
        assert_eq!(cmd.args, vec!["TXID:0", "42"]);
    }

    #[test]
    fn get_command_accept_any_returns_all_in_args() {
        let mut inner = ArgsManagerInner::default();
        inner.accept_any_command = true;
        inner.command = vec!["unregistered".into(), "arg1".into()];
        let cmd = inner.get_command().unwrap();
        assert!(cmd.command.is_none());
        assert_eq!(cmd.args, vec!["unregistered", "arg1"]);
    }

    #[test]
    fn get_command_none_when_no_command() {
        let inner = ArgsManagerInner::default();
        assert!(inner.get_command().is_none());
    }
}
