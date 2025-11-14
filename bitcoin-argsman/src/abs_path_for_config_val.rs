// ---------------- [ File: bitcoin-argsman/src/abs_path_for_config_val.rs ]
crate::ix!();

/**
  | Most paths passed as configuration
  | arguments are treated as relative to
  | the datadir if they are not absolute.
  | 
  | -----------
  | @param path
  | 
  | The path to be conditionally prefixed
  | with datadir.
  | ----------
  | @param net_specific
  | 
  | Use network specific datadir variant
  | 
  | -----------
  | @return
  | 
  | The normalized path.
  |
  */
pub fn abs_path_for_config_val(path: &Path, net_specific: Option<bool>) -> PathBuf {
    let net_specific = net_specific.unwrap_or(true);

    if path.is_absolute() {
        return path.to_path_buf();
    }

    let base0 = if net_specific {
        G_ARGS.lock().cs_args.lock().get_data_dir_net()
    } else {
        G_ARGS.lock().cs_args.lock().get_data_dir_base()
    };

    let base = std::fs::canonicalize(&base0).unwrap_or(base0); // best-effort

    let mut builder = PathBuf::new();
    builder.push(base);
    builder.push(path); // do NOT canonicalize the target file (may not exist)
    builder
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::collections::HashMap;

    #[test]
    fn abs_path_for_config_val_rel_is_joined_to_net_or_base() {
        // Prepare datadir/net in G_ARGS
        let tmp = tempfile::tempdir().unwrap();
        {
            let mut am = G_ARGS.lock();
            let mut inner = am.cs_args.lock();
            inner.force_set_arg("-datadir", tmp.path().to_str().unwrap());
        }
        select_base_params(base_chain_params::REGTEST);
        let p = abs_path_for_config_val(Path::new("bitcoin.conf"), Some(true));
        assert!(p.ends_with("regtest/bitcoin.conf"));
        let p2 = abs_path_for_config_val(Path::new("bitcoin.conf"), Some(false));
        assert!(p2.ends_with("bitcoin.conf"));
    }

    /*
    #[test]
    fn get_config_options_parses_section_and_key_value() {
        let content = r#"
            # A comment
            [regtest]
            rpcuser = alice
            rpcpassword = secret
        "#;
        let mut reader = std::io::BufReader::new(Cursor::new(content.as_bytes().to_vec()));
        let mut err = String::new();
        let mut opts = Vec::new();
        let mut secs = LinkedList::new();
        assert!(get_config_options(&mut reader, "test.conf", &mut err, &mut opts, &mut secs));
        assert!(opts.iter().any(|(k, v)| k == "regtest.rpcuser" && v == "alice"));
        assert!(secs.iter().any(|s| s.name == "regtest"));
    }
    */
}
