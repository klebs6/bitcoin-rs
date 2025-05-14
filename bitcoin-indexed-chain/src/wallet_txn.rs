// ---------------- [ File: bitcoin-indexed-chain/src/wallet_txn.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/node/transaction.h]

/**
  | Maximum fee rate for sendrawtransaction
  | and testmempoolaccept RPC calls.
  | 
  | Also used by the GUI when broadcasting
  | a completed PSBT.
  | 
  | By default, a transaction with a fee
  | rate higher than this will be rejected
  | by these RPCs and the GUI. This can be
  | overridden with the maxfeerate argument.
  |
  */
lazy_static!{
    static ref DEFAULT_MAX_RAW_TX_FEE_RATE: FeeRate = FeeRate::new(COIN / 10);
}

//-------------------------------------------[.cpp/bitcoin/src/node/transaction.cpp]

pub fn handle_atmp_error(
    state:          &TxValidationState,
    err_string_out: &mut String) -> TransactionError 
{
    todo!();
        /*
            err_string_out = state.ToString();
        if (state.IsInvalid()) {
            if (state.GetResult() == TxValidationResult::TX_MISSING_INPUTS) {
                return TransactionError::MISSING_INPUTS;
            }
            return TransactionError::MEMPOOL_REJECTED;
        } else {
            return TransactionError::MEMPOOL_ERROR;
        }
        */
}

/**
  | Submit a transaction to the mempool
  | and (optionally) relay it to all P2P
  | peers.
  | 
  | Mempool submission can be synchronous
  | (will await mempool entry notification
  | over the CValidationInterface) or
  | asynchronous (will submit and not wait
  | for notification), depending on the
  | value of wait_callback. wait_callback
  | MUST
  | 
  | NOT be set while cs_main, cs_mempool
  | or cs_wallet are held to avoid deadlock.
  | 
  | -----------
  | @param[in] node
  | 
  | reference to node context
  | ----------
  | @param[in] tx
  | 
  | the transaction to broadcast
  | ----------
  | @param[out] err_string
  | 
  | reference to std::string to fill with
  | error string if available
  | ----------
  | @param[in] max_tx_fee
  | 
  | reject txs with fees higher than this
  | (if 0, accept any fee)
  | ----------
  | @param[in] relay
  | 
  | flag if both mempool insertion and p2p
  | relay are requested
  | ----------
  | @param[in] wait_callback
  | 
  | wait until callbacks have been processed
  | to avoid stale result due to a sequentially
  | RPC. return error
  |
  */
pub fn broadcast_transaction(
    node:          &mut NodeContext,
    tx:            TransactionRef,
    err_string:    &mut String,
    max_tx_fee:    &Amount,
    relay:         bool,
    wait_callback: bool) -> TransactionError 
{
    todo!();
        /*
            // BroadcastTransaction can be called by either sendrawtransaction RPC or the wallet.
        // chainman, mempool and peerman are initialized before the RPC server and wallet are started
        // and reset after the RPC sever and wallet are stopped.
        assert(node.chainman);
        assert(node.mempool);
        assert(node.peerman);

        std::promise<c_void> promise;
        uint256 txid = tx->GetHash();
        uint256 wtxid = tx->GetWitnessHash();
        bool callback_set = false;

        {
            LOCK(cs_main);

            // If the transaction is already confirmed in the chain, don't do anything
            // and return early.
            CCoinsViewCache &view = node.chainman->ActiveChainstate().CoinsTip();
            for (size_t o = 0; o < tx->vout.size(); o++) {
                const Coin& existingCoin = view.AccessCoin(OutPoint(txid, o));
                // IsSpent doesn't mean the coin is spent, it means the output doesn't exist.
                // So if the output does exist, then this transaction exists in the chain.
                if (!existingCoin.IsSpent()) return TransactionError::ALREADY_IN_CHAIN;
            }

            if (auto mempool_tx = node.mempool->get(txid); mempool_tx) {
                // There's already a transaction in the mempool with this txid. Don't
                // try to submit this transaction to the mempool (since it'll be
                // rejected as a TX_CONFLICT), but do attempt to reannounce the mempool
                // transaction if relay=true.
                //
                // The mempool transaction may have the same or different witness (and
                // wtxid) as this transaction. Use the mempool's wtxid for reannouncement.
                wtxid = mempool_tx->GetWitnessHash();
            } else {
                // Transaction is not already in the mempool.
                if (max_tx_fee > 0) {
                    // First, call ATMP with test_accept and check the fee. If ATMP
                    // fails here, return error immediately.
                    const MempoolAcceptResult result = AcceptToMemoryPool(node.chainman->ActiveChainstate(), *node.mempool, tx, false /* bypass_limits */,
                                                                          true /* test_accept */);
                    if (result.m_result_type != MempoolAcceptResult::ResultType::VALID) {
                        return HandleATMPError(result.m_state, err_string);
                    } else if (result.m_base_fees.value() > max_tx_fee) {
                        return TransactionError::MAX_FEE_EXCEEDED;
                    }
                }
                // Try to submit the transaction to the mempool.
                const MempoolAcceptResult result = AcceptToMemoryPool(node.chainman->ActiveChainstate(), *node.mempool, tx, false /* bypass_limits */,
                                                                      false /* test_accept */);
                if (result.m_result_type != MempoolAcceptResult::ResultType::VALID) {
                    return HandleATMPError(result.m_state, err_string);
                }

                // Transaction was accepted to the mempool.

                if (relay) {
                    // the mempool tracks locally submitted transactions to make a
                    // best-effort of initial broadcast
                    node.mempool->AddUnbroadcastTx(txid);
                }

                if (wait_callback) {
                    // For transactions broadcast from outside the wallet, make sure
                    // that the wallet has been notified of the transaction before
                    // continuing.
                    //
                    // This prevents a race where a user might call sendrawtransaction
                    // with a transaction to/from their wallet, immediately call some
                    // wallet RPC, and get a stale result because callbacks have not
                    // yet been processed.
                    CallFunctionInValidationInterfaceQueue([&promise] {
                        promise.set_value();
                    });
                    callback_set = true;
                }
            }
        } // cs_main

        if (callback_set) {
            // Wait until Validation Interface clients have been notified of the
            // transaction entering the mempool.
            promise.get_future().wait();
        }

        if (relay) {
            node.peerman->RelayTransaction(txid, wtxid);
        }

        return TransactionError::OK;
        */
}

/**
  | Return transaction with a given hash.
  | 
  | If mempool is provided and block_index
  | is not provided, check it first for the
  | tx.
  | 
  | If -txindex is available, check it next
  | for the tx.
  | 
  | Finally, if block_index is provided,
  | check for tx by reading entire block
  | from disk.
  | 
  | -----------
  | @param[in] block_index
  | 
  | The block to read from disk, or nullptr
  | ----------
  | @param[in] mempool
  | 
  | If provided, check mempool for tx
  | ----------
  | @param[in] hash
  | 
  | The txid
  | ----------
  | @param[in] consensusParams
  | 
  | The params
  | ----------
  | @param[out] hashBlock
  | 
  | The block hash, if the tx was found via
  | -txindex or block_index
  | 
  | -----------
  | @return
  | 
  | The tx if found, otherwise nullptr
  |
  */
pub fn get_transaction(
    block_index:      *const BlockIndex,
    mempool:          *const TxMemPool,
    hash:             &u256,
    consensus_params: &ChainConsensusParams,
    hash_block:       &mut u256) -> TransactionRef 
{
    todo!();
        /*
            if (mempool && !block_index) {
            CTransactionRef ptx = mempool->get(hash);
            if (ptx) return ptx;
        }
        if (g_txindex) {
            CTransactionRef tx;
            uint256 block_hash;
            if (g_txindex->FindTx(hash, block_hash, tx)) {
                if (!block_index || block_index->GetBlockHash() == block_hash) {
                    // Don't return the transaction if the provided block hash doesn't match.
                    // The case where a transaction appears in multiple blocks (e.g. reorgs or
                    // BIP30) is handled by the block lookup below.
                    hashBlock = block_hash;
                    return tx;
                }
            }
        }
        if (block_index) {
            CBlock block;
            if (ReadBlockFromDisk(block, block_index, consensusParams)) {
                for (const auto& tx : block.vtx) {
                    if (tx->GetHash() == hash) {
                        hashBlock = block_index->GetBlockHash();
                        return tx;
                    }
                }
            }
        }
        return nullptr;
        */
}

//-------------------------------------------[.cpp/bitcoin/src/wallet/transaction.h]
//-------------------------------------------[.cpp/bitcoin/src/wallet/transaction.cpp]

/**
  | Legacy class used for deserializing
  | vtxPrev for backwards compatibility.
  | vtxPrev was removed in commit 93a18a3650292afbb441a47d1fa1b94aeb0164e3,
  | but old wallet.dat files may still contain
  | vtxPrev vectors of CMerkleTxs.
  | 
  | These need to get deserialized for field
  | alignment when deserializing a CWalletTx,
  | but the deserialized values are discarded.*
  |
  */
pub struct MerkleTx { }

impl MerkleTx {
    
    pub fn unserialize<Stream>(&mut self, s: &mut Stream)  {
    
        todo!();
        /*
            CTransactionRef tx;
            uint256 hashBlock;
            std::vector<uint256> vMerkleBranch;
            int nIndex;

            s >> tx >> hashBlock >> vMerkleBranch >> nIndex;
        */
    }
}
