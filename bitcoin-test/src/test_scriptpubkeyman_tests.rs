crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/wallet/test/scriptpubkeyman_tests.cpp]

#[cfg(test)]
#[fixture(BasicTestingSetup)]
pub mod scriptpubkeyman_tests {

    /**
      | Test LegacyScriptPubKeyMan::CanProvide
      | behavior, making sure it returns true for
      | recognized scripts even when keys may not be
      | available for signing.
      */
    #[test] fn can_provide() {
        todo!();
        /*
        
            // Set up wallet and keyman variables.
            CWallet wallet(m_node.chain.get(), "", CreateDummyWalletDatabase());
            LegacyScriptPubKeyMan& keyman = *wallet.GetOrCreateLegacyScriptPubKeyMan();

            // Make a 1 of 2 multisig script
            std::vector<CKey> keys(2);
            std::vector<CPubKey> pubkeys;
            for (CKey& key : keys) {
                key.MakeNewKey(true);
                pubkeys.emplace_back(key.GetPubKey());
            }
            CScript multisig_script = GetScriptForMultisig(1, pubkeys);
            CScript p2sh_script = GetScriptForDestination(ScriptHash(multisig_script));
            SignatureData data;

            // Verify the p2sh(multisig) script is not recognized until the multisig
            // script is added to the keystore to make it solvable
            BOOST_CHECK(!keyman.CanProvide(p2sh_script, data));
            keyman.AddCScript(multisig_script);
            BOOST_CHECK(keyman.CanProvide(p2sh_script, data));

        */
    }
}
