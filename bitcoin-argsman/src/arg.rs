// ---------------- [ File: bitcoin-argsman/src/arg.rs ]
crate::ix!();

pub struct ArgDescriptor {
    pub name:      &'static str,
    pub help:      String,
    pub flags:     ArgsManagerFlags,
    pub category:  OptionsCategory,
}

#[derive(Default)]
pub struct ArgsManagerArg
{
    pub help_param: String,
    pub help_text:  String,
    pub flags:      u32,
}

impl ArgsManagerArg {
    pub fn new(
        help_param: &str, 
        help_text:  &str, 
        flags:      ArgsManagerFlags) -> Self 
    {
        Self {
            help_param: help_param.to_string(),
            help_text:  help_text.to_string(),
            flags:      flags.bits()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_manager_arg_new_sets_fields() {
        let a = ArgsManagerArg::new("=<x>", "help here", ArgsManagerFlags::ALLOW_INT | ArgsManagerFlags::SENSITIVE);
        assert_eq!(a.help_param, "=<x>");
        assert_eq!(a.help_text, "help here");
        assert_eq!(a.flags, (ArgsManagerFlags::ALLOW_INT | ArgsManagerFlags::SENSITIVE).bits());
    }
}
