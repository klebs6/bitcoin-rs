// ---------------- [ File: bitcoin-test/src/test_spend_tests.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/wallet/test/spend_tests.cpp]

#[cfg(test)]
#[fixture(WalletTestingSetup)]
pub mod spend_tests {

    #[test] fn subtract_fee_test_chain_100setup() {
        todo!();
        /*
        
            CreateAndProcessBlock({}, GetScriptForRawPubKey(coinbaseKey.GetPubKey()));
            auto wallet = CreateSyncedWallet(*m_node.chain, m_node.chainman->ActiveChain(), coinbaseKey);

            // Check that a subtract-from-recipient transaction slightly less than the
            // coinbase input amount does not create a change output (because it would
            // be uneconomical to add and spend the output), and make sure it pays the
            // leftover input amount which would have been change to the recipient
            // instead of the miner.
            auto check_tx = [&wallet](CAmount leftover_input_amount) {
                CRecipient recipient{GetScriptForRawPubKey({}), 50 * COIN - leftover_input_amount, true /* subtract fee */};
                CTransactionRef tx;
                CAmount fee;
                int change_pos = -1;
                bilingual_str error;
                CCoinControl coin_control;
                coin_control.m_feerate.emplace(10000);
                coin_control.fOverrideFeeRate = true;
                // We need to use a change type with high cost of change so that the leftover amount will be dropped to fee instead of added as a change output
                coin_control.m_change_type = OutputType::LEGACY;
                FeeCalculation fee_calc;
                BOOST_CHECK(CreateTransaction(*wallet, {recipient}, tx, fee, change_pos, error, coin_control, fee_calc));
                BOOST_CHECK_EQUAL(tx->vout.size(), 1);
                BOOST_CHECK_EQUAL(tx->vout[0].nValue, recipient.nAmount + leftover_input_amount - fee);
                BOOST_CHECK_GT(fee, 0);
                return fee;
            };

            // Send full input amount to recipient, check that only nonzero fee is
            // subtracted (to_reduce == fee).
            const CAmount fee{check_tx(0)};

            // Send slightly less than full input amount to recipient, check leftover
            // input amount is paid to recipient not the miner (to_reduce == fee - 123)
            BOOST_CHECK_EQUAL(fee, check_tx(123));

            // Send full input minus fee amount to recipient, check leftover input
            // amount is paid to recipient not the miner (to_reduce == 0)
            BOOST_CHECK_EQUAL(fee, check_tx(fee));

            // Send full input minus more than the fee amount to recipient, check
            // leftover input amount is paid to recipient not the miner (to_reduce ==
            // -123). This overpays the recipient instead of overpaying the miner more
            // than double the necessary fee.
            BOOST_CHECK_EQUAL(fee, check_tx(fee + 123));

        */
    }
}
