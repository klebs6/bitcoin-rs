crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/util/transaction_utils.h]
//-------------------------------------------[.cpp/bitcoin/src/test/util/transaction_utils.cpp]

/**
  | create crediting transaction
  |
  | [1 coinbase input => 1 output with given
  | scriptPubkey and value]
  */
pub fn build_crediting_transaction(
        script_pub_key: &Script,
        n_value:        Option<i32>) -> MutableTransaction {
    
    let n_value: i32 = n_value.unwrap_or(0);

    todo!();
        /*
            CMutableTransaction txCredit;
        txCredit.nVersion = 1;
        txCredit.nLockTime = 0;
        txCredit.vin.resize(1);
        txCredit.vout.resize(1);
        txCredit.vin[0].prevout.SetNull();
        txCredit.vin[0].scriptSig = CScript() << CScriptNum(0) << CScriptNum(0);
        txCredit.vin[0].nSequence = CTxIn::SEQUENCE_FINAL;
        txCredit.vout[0].scriptPubKey = scriptPubKey;
        txCredit.vout[0].nValue = nValue;

        return txCredit;
        */
}

/**
  | create spending transaction
  |
  | [1 input with referenced transaction outpoint,
  | scriptSig, scriptWitness =>
  |
  |  1 output with empty scriptPubKey, full value
  |  of referenced transaction]
  */
pub fn build_spending_transaction(
        script_sig:     &Script,
        script_witness: &ScriptWitness,
        tx_credit:      &Transaction) -> MutableTransaction {
    
    todo!();
        /*
            CMutableTransaction txSpend;
        txSpend.nVersion = 1;
        txSpend.nLockTime = 0;
        txSpend.vin.resize(1);
        txSpend.vout.resize(1);
        txSpend.vin[0].scriptWitness = scriptWitness;
        txSpend.vin[0].prevout.hash = txCredit.GetHash();
        txSpend.vin[0].prevout.n = 0;
        txSpend.vin[0].scriptSig = scriptSig;
        txSpend.vin[0].nSequence = CTxIn::SEQUENCE_FINAL;
        txSpend.vout[0].scriptPubKey = CScript();
        txSpend.vout[0].nValue = txCredit.vout[0].nValue;

        return txSpend;
        */
}

/**
  | Helper: create two dummy transactions, each
  | with two outputs.
  |
  | The first has nValues[0] and nValues[1] outputs
  | paid to a TxoutType::PUBKEY,
  |
  | the second nValues[2] and nValues[3] outputs
  | paid to a TxoutType::PUBKEYHASH.
  */
pub fn setup_dummy_inputs<T>(
        keystore_ret: &mut FillableSigningProvider<T>,
        coins_ret:    &mut CoinsViewCache,
        n_values:     &[Amount;4]) -> Vec<MutableTransaction> {
    
    todo!();
        /*
            std::vector<CMutableTransaction> dummyTransactions;
        dummyTransactions.resize(2);

        // Add some keys to the keystore:
        CKey key[4];
        for (int i = 0; i < 4; i++) {
            key[i].MakeNewKey(i % 2);
            keystoreRet.AddKey(key[i]);
        }

        // Create some dummy input transactions
        dummyTransactions[0].vout.resize(2);
        dummyTransactions[0].vout[0].nValue = nValues[0];
        dummyTransactions[0].vout[0].scriptPubKey << ToByteVector(key[0].GetPubKey()) << OP_CHECKSIG;
        dummyTransactions[0].vout[1].nValue = nValues[1];
        dummyTransactions[0].vout[1].scriptPubKey << ToByteVector(key[1].GetPubKey()) << OP_CHECKSIG;
        AddCoins(coinsRet, CTransaction(dummyTransactions[0]), 0);

        dummyTransactions[1].vout.resize(2);
        dummyTransactions[1].vout[0].nValue = nValues[2];
        dummyTransactions[1].vout[0].scriptPubKey = GetScriptForDestination(PKHash(key[2].GetPubKey()));
        dummyTransactions[1].vout[1].nValue = nValues[3];
        dummyTransactions[1].vout[1].scriptPubKey = GetScriptForDestination(PKHash(key[3].GetPubKey()));
        AddCoins(coinsRet, CTransaction(dummyTransactions[1]), 0);

        return dummyTransactions;
        */
}
