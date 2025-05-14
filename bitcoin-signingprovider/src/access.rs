// ---------------- [ File: bitcoin-signingprovider/src/access.rs ]
crate::ix!();

pub fn get_cscript(
        provider: &SigningProvider,
        sigdata:  &SignatureData,
        scriptid: &ScriptID,
        script:   &mut Script) -> bool {
    
    todo!();
        /*
            if (provider.GetCScript(scriptid, script)) {
            return true;
        }
        // Look for scripts in SignatureData
        if (CScriptID(sigdata.redeem_script) == scriptid) {
            script = sigdata.redeem_script;
            return true;
        } else if (CScriptID(sigdata.witness_script) == scriptid) {
            script = sigdata.witness_script;
            return true;
        }
        return false;
        */
}

pub fn get_pub_key(
        provider: &SigningProvider,
        sigdata:  &SignatureData,
        address:  &KeyID,
        pubkey:   &mut PubKey) -> bool {
    
    todo!();
        /*
            // Look for pubkey in all partial sigs
        const auto it = sigdata.signatures.find(address);
        if (it != sigdata.signatures.end()) {
            pubkey = it->second.first;
            return true;
        }
        // Look for pubkey in pubkey list
        const auto& pk_it = sigdata.misc_pubkeys.find(address);
        if (pk_it != sigdata.misc_pubkeys.end()) {
            pubkey = pk_it->second.first;
            return true;
        }
        // Query the underlying provider
        return provider.GetPubKey(address, pubkey);
        */
}

/**
  | Check whether we know how to sign for
  | an output like this, assuming we have
  | all private keys. While this function
  | does not need private keys, the passed
  | provider is used to look up public keys
  | and redeemscripts by hash.
  | 
  | Solvability is unrelated to whether
  | we consider this output to be ours.
  |
  */
pub fn is_solvable(
        provider: &SigningProvider,
        script:   &Script) -> bool {

    /*
      | This check is to make sure that the script
      | we created can actually be solved for and
      | signed by us if we were to have the private
      | keys. This is just to make sure that the
      | script is valid and that, if found in
      | a transaction, we would still accept and
      | relay that transaction. In particular, it
      | will reject witness outputs that require
      | signing with an uncompressed public key.
      */
    let mut sigs: SignatureData = Default::default();

    /*
      | Make sure that STANDARD_SCRIPT_VERIFY_FLAGS
      | includes SCRIPT_VERIFY_WITNESS_PUBKEYTYPE,
      | the most important property this function
      | is designed to test for.
      */
    const_assert!(
        //"IsSolvable requires standard script
        //flags to include WITNESS_PUBKEYTYPE"
        ScriptVerificationFlags::STANDARD_SCRIPT_VERIFY_FLAGS
        .contains(ScriptVerificationFlags::SCRIPT_VERIFY_WITNESS_PUBKEYTYPE)
    ); 

    if produce_signature(
        provider, 
        &**DUMMY_SIGNATURE_CREATOR.lock().unwrap(), 
        script, 
        &mut sigs) 
    {
        /*
          | VerifyScript check is just defensive,
          | and should never fail.
          |
          */
        let verified: bool = verify_script_with_checker(
            &sigs.script_sig, 
            script, 
            &sigs.script_witness, 
            ScriptVerificationFlags::STANDARD_SCRIPT_VERIFY_FLAGS.bits(), 
            &DUMMY_CHECKER,
            None
        );

        assert!(verified);

        return true;
    }

    false
}

/**
  | Check whether a scriptPubKey is known
  | to be segwit.
  |
  */
pub fn is_seg_wit_output(
        provider: &SigningProvider,
        script:   &Script) -> bool {
    
    todo!();
        /*
            int version;
        valtype program;
        if (script.IsWitnessProgram(version, program)) return true;
        if (script.IsPayToScriptHash()) {
            std::vector<valtype> solutions;
            auto whichtype = Solver(script, solutions);
            if (whichtype == TxoutType::SCRIPTHASH) {
                auto h160 = u160(solutions[0]);
                CScript subscript;
                if (provider.GetCScript(CScriptID{h160}, subscript)) {
                    if (subscript.IsWitnessProgram(version, program)) return true;
                }
            }
        }
        return false;
        */
}
