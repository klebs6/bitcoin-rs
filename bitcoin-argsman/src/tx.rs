crate::ix!();

pub fn setup_bitcoin_tx_args(argsman: &mut ArgsManager)  {

    // Help options first
    argsman.setup_help_options();

    // Basic options
    argsman.add_arg(&
        ArgDescriptor {
            name:     "-version",
            help:     "Print version and exit".to_string(),
            flags:    ArgsManagerFlags::ALLOW_ANY,
            category: OptionsCategory::OPTIONS,
        }
    );

    argsman.add_arg(&
        ArgDescriptor {
            name:     "-create",
            help:     "Create new, empty TX.".to_string(),
            flags:    ArgsManagerFlags::ALLOW_ANY,
            category: OptionsCategory::OPTIONS,
        }
    );

    argsman.add_arg(&
        ArgDescriptor {
            name:     "-json",
            help:     "Select JSON output".to_string(),
            flags:    ArgsManagerFlags::ALLOW_ANY,
            category: OptionsCategory::OPTIONS,
        }
    );

    argsman.add_arg(&
        ArgDescriptor {
            name:     "-txid",
            help:     "Output only the hex-encoded transaction id of the resultant transaction.".to_string(),
            flags:    ArgsManagerFlags::ALLOW_ANY,
            category: OptionsCategory::OPTIONS,
        }
    );

    // Chain params (same as C++ SetupChainParamsBaseOptions)
    argsman.setup_chain_params_base_options();

    // Command-style arguments (Category: COMMANDS). These are not subcommands
    // in the ArgsManager sense (which would use add_command), but "commands"
    // as bitcoin-tx style pseudo-ops. In Core they were registered with AddArg
    // in the COMMANDS category, not AddCommand (so we mirror that).
    argsman.add_arg(&
        ArgDescriptor {
            name:     "delin=N",
            help:     "Delete input N from TX".to_string(),
            flags:    ArgsManagerFlags::ALLOW_ANY,
            category: OptionsCategory::COMMANDS,
        }
    );

    argsman.add_arg(&
        ArgDescriptor {
            name:     "delout=N",
            help:     "Delete output N from TX".to_string(),
            flags:    ArgsManagerFlags::ALLOW_ANY,
            category: OptionsCategory::COMMANDS,
        }
    );

    argsman.add_arg(&
        ArgDescriptor {
            name:     "in=TXID:VOUT(:SEQUENCE_NUMBER)",
            help:     "Add input to TX".to_string(),
            flags:    ArgsManagerFlags::ALLOW_ANY,
            category: OptionsCategory::COMMANDS,
        }
    );

    argsman.add_arg(&
        ArgDescriptor {
            name:     "locktime=N",
            help:     "Set TX lock time to N".to_string(),
            flags:    ArgsManagerFlags::ALLOW_ANY,
            category: OptionsCategory::COMMANDS,
        }
    );

    argsman.add_arg(&
        ArgDescriptor {
            name:     "nversion=N",
            help:     "Set TX version to N".to_string(),
            flags:    ArgsManagerFlags::ALLOW_ANY,
            category: OptionsCategory::COMMANDS,
        }
    );

    argsman.add_arg(&
        ArgDescriptor {
            name:     "outaddr=VALUE:ADDRESS",
            help:     "Add address-based output to TX".to_string(),
            flags:    ArgsManagerFlags::ALLOW_ANY,
            category: OptionsCategory::COMMANDS,
        }
    );

    argsman.add_arg(&
        ArgDescriptor {
            name:     "outdata=[VALUE:]DATA",
            help:     "Add data-based output to TX".to_string(),
            flags:    ArgsManagerFlags::ALLOW_ANY,
            category: OptionsCategory::COMMANDS,
        }
    );

    argsman.add_arg(&
        ArgDescriptor {
            name:     "outmultisig=VALUE:REQUIRED:PUBKEYS:PUBKEY1:PUBKEY2:....[:FLAGS]",
            help:     concat!(
                "Add Pay To n-of-m Multi-sig output to TX. n = REQUIRED, m = PUBKEYS. ",
                "Optionally add the \"W\" flag to produce a pay-to-witness-script-hash output. ",
                "Optionally add the \"S\" flag to wrap the output in a pay-to-script-hash."
            ).to_string(),
            flags:    ArgsManagerFlags::ALLOW_ANY,
            category: OptionsCategory::COMMANDS,
        }
    );

    argsman.add_arg(&
        ArgDescriptor {
            name:     "outpubkey=VALUE:PUBKEY[:FLAGS]",
            help:     concat!(
                "Add pay-to-pubkey output to TX. ",
                "Optionally add the \"W\" flag to produce a pay-to-witness-pubkey-hash output. ",
                "Optionally add the \"S\" flag to wrap the output in a pay-to-script-hash."
            ).to_string(),
            flags:    ArgsManagerFlags::ALLOW_ANY,
            category: OptionsCategory::COMMANDS,
        }
    );

    argsman.add_arg(&
        ArgDescriptor {
            name:     "outscript=VALUE:SCRIPT[:FLAGS]",
            help:     concat!(
                "Add raw script output to TX. ",
                "Optionally add the \"W\" flag to produce a pay-to-witness-script-hash output. ",
                "Optionally add the \"S\" flag to wrap the output in a pay-to-script-hash."
            ).to_string(),
            flags:    ArgsManagerFlags::ALLOW_ANY,
            category: OptionsCategory::COMMANDS,
        }
    );

    argsman.add_arg(&
        ArgDescriptor {
            name:     "replaceable(=N)",
            help:     "Set RBF opt-in sequence number for input N (if not provided, opt-in all available inputs)".to_string(),
            flags:    ArgsManagerFlags::ALLOW_ANY,
            category: OptionsCategory::COMMANDS,
        }
    );

    argsman.add_arg(&
        ArgDescriptor {
            name:     "sign=SIGHASH-FLAGS",
            help:     concat!(
                "Add zero or more signatures to transaction. ",
                "This command requires JSON registers:",
                "prevtxs=JSON object, ",
                "privatekeys=JSON object. ",
                "See signrawtransactionwithkey docs for format of sighash flags, JSON objects."
            ).to_string(),
            flags:    ArgsManagerFlags::ALLOW_ANY,
            category: OptionsCategory::COMMANDS,
        }
    );

    // Register-type commands
    argsman.add_arg(&
        ArgDescriptor {
            name:     "load=NAME:FILENAME",
            help:     "Load JSON file FILENAME into register NAME".to_string(),
            flags:    ArgsManagerFlags::ALLOW_ANY,
            category: OptionsCategory::REGISTER_COMMANDS,
        }
    );

    argsman.add_arg(&
        ArgDescriptor {
            name:     "set=NAME:JSON-STRING",
            help:     "Set register NAME to given JSON-STRING".to_string(),
            flags:    ArgsManagerFlags::ALLOW_ANY,
            category: OptionsCategory::REGISTER_COMMANDS,
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn setup_bitcoin_tx_args_registers_core_tx_ops() {
        let mut am = ArgsManager::default();
        {
            // prime categories used by setup_bitcoin_tx_args
            let mut inner = am.cs_args.lock();
            for cat in [
                OptionsCategory::OPTIONS,
                OptionsCategory::COMMANDS,
                OptionsCategory::REGISTER_COMMANDS,
                OptionsCategory::CHAINPARAMS,
                OptionsCategory::HIDDEN,
            ] {
                inner.available_args.insert(cat, HashMap::new());
            }
        }

        setup_bitcoin_tx_args(&mut am);

        let inner = am.cs_args.lock();
        let opts = inner.available_args.get(&OptionsCategory::OPTIONS).unwrap();
        assert!(opts.contains_key("-version"));
        assert!(opts.contains_key("-create"));
        assert!(opts.contains_key("-json"));
        assert!(opts.contains_key("-txid"));

        let cmds = inner.available_args.get(&OptionsCategory::COMMANDS).unwrap();
        for k in [
            "delin", "delout", "in", "locktime", "nversion", "outaddr",
            "outdata", "outmultisig", "outpubkey", "outscript", "replaceable", "sign"
        ] {
            assert!(cmds.contains_key(k), "missing command {}", k);
        }

        let regs = inner.available_args.get(&OptionsCategory::REGISTER_COMMANDS).unwrap();
        for k in ["load", "set"] {
            assert!(regs.contains_key(k));
        }
    }
}
