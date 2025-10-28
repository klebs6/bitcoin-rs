// ---------------- [ File: bitcoin-argsman/src/get_help_message.rs ]
crate::ix!();

pub const SCREEN_WIDTH: usize = 79;
pub const OPT_INDENT:   usize = 2;
pub const MSG_INDENT:   usize = 7;

/**
  | Format a string to be used as group of
  | options in help messages
  | 
  | -----------
  | @param message
  | 
  | Group name (e.g. "RPC server options:")
  | 
  | -----------
  | @return
  | 
  | the formatted string
  |
  */
pub fn help_message_group(message: &str) -> String {
    
    format!{"{}\n\n",message}
}

#[derive(Error, Debug)]
pub enum FormatParagraphError {

    #[error("received io::Error during write")]
    IoError(#[from] std::io::Error),

    #[error("bad utf8, got error: `{0:?}`")]
    FromUtf8Error(#[from] FromUtf8Error),

    #[error("could not convert BufWriter to bytes! got error: `{0:?}`")]
    IntoInnerError(#[from] std::io::IntoInnerError<BufWriter<Vec<u8>>>),
}

/**
  | Format a paragraph of text to a fixed
  | width, adding spaces for indentation
  | to any added line.
  |
  */
pub fn format_paragraph(
        in_:    &str,
        width:  Option<usize>,
        indent: Option<usize>) -> Result<String,FormatParagraphError> {

    let width:  usize = width.unwrap_or(79);
    let indent: usize = indent.unwrap_or(0);
    
    let mut out = BufWriter::new(Vec::new());
    let mut ptr:      usize = 0;
    let mut indented: usize = 0;

    while ptr < in_.len() {

        let mut lineend: Option<usize> = in_[ptr..].find('\n');

        if lineend == None {
            lineend = Some(in_.len());
        }

        let linelen:   usize = lineend.unwrap() - ptr;
        let rem_width: usize = width - indented;

        if linelen <= rem_width {

            out.write(in_[ptr..linelen + 1].as_bytes())?;

            ptr = lineend.unwrap() + 1;

            indented = 0;

        } else {

            let mut finalspace: Option<usize> = in_[..=ptr + rem_width].rfind(" \n");

            if finalspace == None || finalspace.unwrap() < ptr {

                // No place to break; just include
                // the entire word and move on
                finalspace = in_[ptr..].find("\n ");

                if finalspace == None {

                    let c: char = in_
                        .chars()
                        .nth(ptr)
                        .unwrap();

                    let s = format!{"{}",c};

                    // End of the string, just add
                    // it and break
                    out.write(s.as_bytes())?;

                    break;
                }
            }

            out.write(
                format!{
                    "{}\n", 
                    in_[ptr..finalspace.unwrap() - ptr].to_string()
                }.as_bytes()
            )?;

            if in_.chars().nth(finalspace.unwrap()).unwrap() == '\n' {

                indented = 0;

            } else {

                if indent != 0 {
                    out.write(" ".repeat(indent).as_bytes())?;
                    indented = indent;
                }
            }

            ptr = finalspace.unwrap() + 1;
        }
    }

    let bytes = out.into_inner()?;
    let result = String::from_utf8(bytes)?;

    Ok(result)
}

/**
  | Format a string to be used as option description
  | in help messages
  | 
  | -----------
  | @param option
  | 
  | Option message (e.g. "-rpcuser=<user>")
  | ----------
  | @param message
  | 
  | Option description (e.g. "Username
  | for JSON-RPC connections")
  | 
  | -----------
  | @return
  | 
  | the formatted string
  |
  */
pub fn help_message_opt(
        option:  &str,
        message: &str) -> String {

    let paragraph = format_paragraph(
        message,
        Some(SCREEN_WIDTH - MSG_INDENT),
        Some(MSG_INDENT)
    );

    match paragraph {
        Ok(paragraph) => {
            format!{
                "{}{}\n{}{}\n\n",
                " ".repeat(OPT_INDENT),
                option,
                " ".repeat(MSG_INDENT),
                paragraph
            }
        },
        Err(e) => {
            panic!{"format_paragraph failed with error: {:?}", e}
        }
    }
}

impl ArgsManagerInner {

    /**
      | Get the help string
      |
      */
    pub fn get_help_message(&self) -> String {

        let show_debug: bool = self.get_bool_arg("-help-debug",false);

        let mut usage: String = "".to_string();

        for arg_map in self.available_args.iter() {

            match arg_map.0 {

                OptionsCategory::OPTIONS  => {
                    usage += &help_message_group("Options:");
                },

                OptionsCategory::CONNECTION  => {
                    usage += &help_message_group("Connection options:");
                },

                OptionsCategory::ZMQ  => {
                    usage += &help_message_group("ZeroMQ notification options:");
                },

                OptionsCategory::DEBUG_TEST  => {
                    usage += &help_message_group("Debugging/Testing options:");
                },

                OptionsCategory::NODE_RELAY  => {
                    usage += &help_message_group("Node relay options:");
                },

                OptionsCategory::BLOCK_CREATION  => {
                    usage += &help_message_group("Block creation options:");
                },

                OptionsCategory::RPC  => {
                    usage += &help_message_group("RPC server options:");
                },

                OptionsCategory::WALLET  => {
                    usage += &help_message_group("Wallet options:");
                },

                OptionsCategory::WALLET_DEBUG_TEST  => {
                    if show_debug {
                        usage += &help_message_group("Wallet debugging/testing options:");
                    }
                },

                OptionsCategory::CHAINPARAMS  => {
                    usage += &help_message_group("Chain selection options:");
                },

                OptionsCategory::GUI  => {
                    usage += &help_message_group("UI Options:");
                },

                OptionsCategory::COMMANDS  => {
                    usage += &help_message_group("Commands:");
                },

                OptionsCategory::REGISTER_COMMANDS  => {
                    usage += &help_message_group("Register Commands:");
                },

                _  => { },
            }

            // When we get to the hidden options, stop
            if arg_map.0 == &OptionsCategory::HIDDEN {
                break;
            }

            for arg in arg_map.1.iter() {

                if show_debug || (arg.1.flags & ArgsManagerFlags::DEBUG_ONLY.bits()) == 0 {

                    let mut name = String::default();

                    if arg.1.help_param.is_empty() {
                        name = arg.0.to_string();
                    } else {
                        name = format!{"{}{}", arg.0, arg.1.help_param};
                    }

                    usage += &help_message_opt(
                        &name,
                        &arg.1.help_text
                    );
                }
            }
        }

        usage
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn inner_with_two_options(show_debug_only: bool) -> ArgsManagerInner {
        let mut inner = ArgsManagerInner::default();
        inner.available_args.insert(OptionsCategory::OPTIONS, HashMap::new());

        // Option visible always
        let a = ArgDescriptor {
            name:     "-alpha=<n>",
            help:     "Alpha opt".into(),
            flags:    ArgsManagerFlags::ALLOW_INT,
            category: OptionsCategory::OPTIONS,
        };
        inner.add_arg(&a);

        // Debug-only
        let b = ArgDescriptor {
            name:     "-bravo",
            help:     "Bravo opt".into(),
            flags:    ArgsManagerFlags::ALLOW_ANY | ArgsManagerFlags::DEBUG_ONLY,
            category: OptionsCategory::OPTIONS,
        };
        inner.add_arg(&b);

        if show_debug_only {
            inner.force_set_arg("-help-debug", "1");
        }
        inner
    }

    #[test]
    fn help_message_hides_debug_by_default() {
        let inner = inner_with_two_options(false);
        let s = inner.get_help_message();
        assert!(s.contains("-alpha=<n>"));
        assert!(!s.contains("-bravo"), "debug-only should be hidden");
    }

    #[test]
    fn help_message_shows_debug_when_requested() {
        let inner = inner_with_two_options(true);
        let s = inner.get_help_message();
        assert!(s.contains("-alpha=<n>"));
        assert!(s.contains("-bravo"));
    }

    #[test]
    fn formatting_helpers_behave() {
        let group = help_message_group("Group:");
        assert!(group.starts_with("Group:"));

        let opt = help_message_opt("-x", "Some message goes here");
        assert!(opt.contains("-x"));
        assert!(opt.contains("Some message goes here"));
    }
}
