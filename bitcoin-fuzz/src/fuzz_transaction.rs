// ---------------- [ File: bitcoin-fuzz/src/fuzz_transaction.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/transaction.cpp]

pub fn initialize_transaction()  {
    
    todo!();
        /*
            SelectParams(CBaseChainParams::REGTEST);
        */
}

#[fuzz_test(initializer = "initialize_transaction")]
fn transaction() {
    todo!();
    /*
    
        DataStream ds(buffer, SER_NETWORK, INIT_PROTO_VERSION);
        try {
            int nVersion;
            ds >> nVersion;
            ds.SetVersion(nVersion);
        } catch (const std::ios_base::failure&) {
            return;
        }
        bool valid_tx = true;
        const CTransaction tx = [&] {
            try {
                return CTransaction(deserialize, ds);
            } catch (const std::ios_base::failure&) {
                valid_tx = false;
                return CTransaction{CMutableTransaction{}};
            }
        }();
        bool valid_mutable_tx = true;
        DataStream ds_mtx(buffer, SER_NETWORK, INIT_PROTO_VERSION);
        CMutableTransaction mutable_tx;
        try {
            int nVersion;
            ds_mtx >> nVersion;
            ds_mtx.SetVersion(nVersion);
            ds_mtx >> mutable_tx;
        } catch (const std::ios_base::failure&) {
            valid_mutable_tx = false;
        }
        assert(valid_tx == valid_mutable_tx);
        if (!valid_tx) {
            return;
        }

        {
            TxValidationState state_with_dupe_check;
            const bool res{CheckTransaction(tx, state_with_dupe_check)};
            Assert(res == state_with_dupe_check.IsValid());
        }

        const CFeeRate dust_relay_fee{DUST_RELAY_TX_FEE};
        std::string reason;
        const bool is_standard_with_permit_bare_multisig = IsStandardTx(tx, /* permit_bare_multisig= */ true, dust_relay_fee, reason);
        const bool is_standard_without_permit_bare_multisig = IsStandardTx(tx, /* permit_bare_multisig= */ false, dust_relay_fee, reason);
        if (is_standard_without_permit_bare_multisig) {
            assert(is_standard_with_permit_bare_multisig);
        }

        (c_void)tx.GetHash();
        (c_void)tx.GetTotalSize();
        try {
            (c_void)tx.GetValueOut();
        } catch (const std::runtime_error&) {
        }
        (c_void)tx.GetWitnessHash();
        (c_void)tx.HasWitness();
        (c_void)tx.IsCoinBase();
        (c_void)tx.IsNull();
        (c_void)tx.ToString();

        (c_void)EncodeHexTx(tx);
        (c_void)GetLegacySigOpCount(tx);
        (c_void)GetTransactionWeight(tx);
        (c_void)GetVirtualTransactionSize(tx);
        (c_void)IsFinalTx(tx, /* nBlockHeight= */ 1024, /* nBlockTime= */ 1024);
        (c_void)IsStandardTx(tx, reason);
        (c_void)RecursiveDynamicUsage(tx);
        (c_void)SignalsOptInRBF(tx);

        CCoinsView coins_view;
        const CCoinsViewCache coins_view_cache(&coins_view);
        (c_void)AreInputsStandard(tx, coins_view_cache, false);
        (c_void)AreInputsStandard(tx, coins_view_cache, true);
        (c_void)IsWitnessStandard(tx, coins_view_cache);

        UniValue u(UniValue::VOBJ);
        TxToUniv(tx, /* hashBlock */ uint256::ZERO, u);
        TxToUniv(tx, /* hashBlock */ uint256::ONE, u);

    */
}
