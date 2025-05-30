// ---------------- [ File: bitcoin-bench/src/bench_verify_script.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/verify_script.cpp]

/**
  | Microbenchmark for verification of
  | a basic P2WPKH script. Can be easily 
  | modified to measure performance of 
  | other types of scripts.
  |
  */
#[bench] fn verify_script_bench(b: &mut Bencher)  {
    
    todo!();
        /*
            const ECCVerifyHandle verify_handle;
        ECC_Start();

        const uint32_t flags{SCRIPT_VERIFY_WITNESS | SCRIPT_VERIFY_P2SH};
        const int witnessversion = 0;

        // Key pair.
        CKey key;
        static const std::array<unsigned char, 32> vchKey = {
            {
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
            }
        };
        key.Set(vchKey.begin(), vchKey.end(), false);
        CPubKey pubkey = key.GetPubKey();
        u160 pubkeyHash;
        CHash160().Write(pubkey).Finalize(pubkeyHash);

        // Script.
        CScript scriptPubKey = CScript() << witnessversion << ToByteVector(pubkeyHash);
        CScript scriptSig;
        CScript witScriptPubkey = CScript() << OP_DUP << OP_HASH160 << ToByteVector(pubkeyHash) << OP_EQUALVERIFY << OP_CHECKSIG;
        const CMutableTransaction& txCredit = BuildCreditingTransaction(scriptPubKey, 1);
        CMutableTransaction txSpend = BuildSpendingTransaction(scriptSig, CScriptWitness(), CTransaction(txCredit));
        CScriptWitness& witness = txSpend.vin[0].scriptWitness;
        witness.stack.emplace_back();
        key.Sign(SignatureHash(witScriptPubkey, txSpend, 0, SIGHASH_ALL, txCredit.vout[0].nValue, SigVersion::WITNESS_V0), witness.stack.back());
        witness.stack.back().push_back(static_cast<unsigned char>(SIGHASH_ALL));
        witness.stack.push_back(ToByteVector(pubkey));

        // Benchmark.
        bench.run([&] {
            ScriptError err;
            bool success = VerifyScript(
                txSpend.vin[0].scriptSig,
                txCredit.vout[0].scriptPubKey,
                &txSpend.vin[0].scriptWitness,
                flags,
                MutableTransactionSignatureChecker(&txSpend, 0, txCredit.vout[0].nValue, MissingDataBehavior::ASSERT_FAIL),
                &err);
            assert(err == SCRIPT_ERR_OK);
            assert(success);

    #if defined(HAVE_CONSENSUS_LIB)
            DataStream stream(SER_NETWORK, PROTOCOL_VERSION);
            stream << txSpend;
            int csuccess = bitcoinconsensus_verify_script_with_amount(
                txCredit.vout[0].scriptPubKey.data(),
                txCredit.vout[0].scriptPubKey.size(),
                txCredit.vout[0].nValue,
                (const unsigned char*)stream.data(), stream.size(), 0, flags, nullptr);
            assert(csuccess == 1);
    #endif
        });
        ECC_Stop();
        */
}

#[bench] fn verify_nested_if_script(b: &mut Bencher)  {
    
    todo!();
        /*
            std::vector<std::vector<unsigned char>> stack;
        CScript script;
        for (int i = 0; i < 100; ++i) {
            script << OP_1 << OP_IF;
        }
        for (int i = 0; i < 1000; ++i) {
            script << OP_1;
        }
        for (int i = 0; i < 100; ++i) {
            script << OP_ENDIF;
        }
        bench.run([&] {
            auto stack_copy = stack;
            ScriptError error;
            bool ret = EvalScript(stack_copy, script, 0, BaseSignatureChecker(), SigVersion::BASE, &error);
            assert(ret);
        });
        */
}
