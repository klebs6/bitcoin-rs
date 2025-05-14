// ---------------- [ File: bitcoin-argsman/src/tx.rs ]
crate::ix!();

pub fn setup_bitcoin_tx_args(argsman: &mut ArgsManager)  {
    
    todo!();
        /*
            SetupHelpOptions(argsman);

        argsman.AddArg("-version", "Print version and exit", ArgsManager::ALLOW_ANY, OptionsCategory::OPTIONS);
        argsman.AddArg("-create", "Create new, empty TX.", ArgsManager::ALLOW_ANY, OptionsCategory::OPTIONS);
        argsman.AddArg("-json", "Select JSON output", ArgsManager::ALLOW_ANY, OptionsCategory::OPTIONS);
        argsman.AddArg("-txid", "Output only the hex-encoded transaction id of the resultant transaction.", ArgsManager::ALLOW_ANY, OptionsCategory::OPTIONS);
        SetupChainParamsBaseOptions(argsman);

        argsman.AddArg("delin=N", "Delete input N from TX", ArgsManager::ALLOW_ANY, OptionsCategory::COMMANDS);
        argsman.AddArg("delout=N", "Delete output N from TX", ArgsManager::ALLOW_ANY, OptionsCategory::COMMANDS);
        argsman.AddArg("in=TXID:VOUT(:SEQUENCE_NUMBER)", "Add input to TX", ArgsManager::ALLOW_ANY, OptionsCategory::COMMANDS);
        argsman.AddArg("locktime=N", "Set TX lock time to N", ArgsManager::ALLOW_ANY, OptionsCategory::COMMANDS);
        argsman.AddArg("nversion=N", "Set TX version to N", ArgsManager::ALLOW_ANY, OptionsCategory::COMMANDS);
        argsman.AddArg("outaddr=VALUE:ADDRESS", "Add address-based output to TX", ArgsManager::ALLOW_ANY, OptionsCategory::COMMANDS);
        argsman.AddArg("outdata=[VALUE:]DATA", "Add data-based output to TX", ArgsManager::ALLOW_ANY, OptionsCategory::COMMANDS);
        argsman.AddArg("outmultisig=VALUE:REQUIRED:PUBKEYS:PUBKEY1:PUBKEY2:....[:FLAGS]", "Add Pay To n-of-m Multi-sig output to TX. n = REQUIRED, m = PUBKEYS. "
            "Optionally add the \"W\" flag to produce a pay-to-witness-script-hash output. "
            "Optionally add the \"S\" flag to wrap the output in a pay-to-script-hash.", ArgsManager::ALLOW_ANY, OptionsCategory::COMMANDS);
        argsman.AddArg("outpubkey=VALUE:PUBKEY[:FLAGS]", "Add pay-to-pubkey output to TX. "
            "Optionally add the \"W\" flag to produce a pay-to-witness-pubkey-hash output. "
            "Optionally add the \"S\" flag to wrap the output in a pay-to-script-hash.", ArgsManager::ALLOW_ANY, OptionsCategory::COMMANDS);
        argsman.AddArg("outscript=VALUE:SCRIPT[:FLAGS]", "Add raw script output to TX. "
            "Optionally add the \"W\" flag to produce a pay-to-witness-script-hash output. "
            "Optionally add the \"S\" flag to wrap the output in a pay-to-script-hash.", ArgsManager::ALLOW_ANY, OptionsCategory::COMMANDS);
        argsman.AddArg("replaceable(=N)", "Set RBF opt-in sequence number for input N (if not provided, opt-in all available inputs)", ArgsManager::ALLOW_ANY, OptionsCategory::COMMANDS);
        argsman.AddArg("sign=SIGHASH-FLAGS", "Add zero or more signatures to transaction. "
            "This command requires JSON registers:"
            "prevtxs=JSON object, "
            "privatekeys=JSON object. "
            "See signrawtransactionwithkey docs for format of sighash flags, JSON objects.", ArgsManager::ALLOW_ANY, OptionsCategory::COMMANDS);

        argsman.AddArg("load=NAME:FILENAME", "Load JSON file FILENAME into register NAME", ArgsManager::ALLOW_ANY, OptionsCategory::REGISTER_COMMANDS);
        argsman.AddArg("set=NAME:JSON-STRING", "Set register NAME to given JSON-STRING", ArgsManager::ALLOW_ANY, OptionsCategory::REGISTER_COMMANDS);
        */
}

