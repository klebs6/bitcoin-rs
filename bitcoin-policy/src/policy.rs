// ---------------- [ File: bitcoin-policy/src/policy.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/policy/policy.h]

/**
  | Used as the flags parameter to sequence
  | and nLocktime checks in non-consensus
  | code.
  |
  */
lazy_static!{
    static ref STANDARD_LOCKTIME_VERIFY_FLAGS: usize = 
        LOCKTIME_VERIFY_SEQUENCE 
        | LOCKTIME_MEDIAN_TIME_PAST;
}

/**
  | Default for -blockmaxweight, which
  | controls the range of block weights
  | the mining code will create *
  |
  */
pub const DEFAULT_BLOCK_MAX_WEIGHT: usize = MAX_BLOCK_WEIGHT - 4000;

/**
  | Default for -blockmintxfee, which
  | sets the minimum feerate for a transaction
  | in blocks created by mining code *
  |
  */
pub const DEFAULT_BLOCK_MIN_TX_FEE: Amount = 1000;

/**
  | The maximum weight for transactions
  | we're willing to relay/mine
  |
  */
pub const MAX_STANDARD_TX_WEIGHT: usize = 400000;

/**
  | The minimum non-witness size for transactions
  | we're willing to relay/mine (1 segwit
  | input + 1 P2WPKH output = 82 bytes)
  |
  */
pub const MIN_STANDARD_TX_NONWITNESS_SIZE: usize = 82;

/**
  | Maximum number of signature check operations
  | in an IsStandard() P2SH script
  |
  */
pub const MAX_P2SH_SIGOPS: usize = 15;

/**
  | The maximum number of sigops we're willing
  | to relay/mine in a single tx
  |
  */
pub const MAX_STANDARD_TX_SIGOPS_COST: usize = MAX_BLOCK_SIGOPS_COST / 5;

/**
  | Default for -maxmempool, maximum megabytes
  | of mempool memory usage
  |
  */
pub const DEFAULT_MAX_MEMPOOL_SIZE: usize = 300;

/**
  | Default for -incrementalrelayfee,
  | which sets the minimum feerate increase
  | for mempool limiting or BIP 125 replacement
  | *
  |
  */
pub const DEFAULT_INCREMENTAL_RELAY_FEE: Amount = 1000;

/**
  | Default for -bytespersigop
  |
  */
pub const DEFAULT_BYTES_PER_SIGOP: usize = 20;

/**
  | Default for -permitbaremultisig
  |
  */
pub const DEFAULT_PERMIT_BAREMULTISIG: bool = true;

/**
  | The maximum number of witness stack
  | items in a standard P2WSH script
  |
  */
pub const MAX_STANDARD_P2WSH_STACK_ITEMS: usize = 100;

/**
  | The maximum size in bytes of each witness
  | stack item in a standard P2WSH script
  |
  */
pub const MAX_STANDARD_P2WSH_STACK_ITEM_SIZE: usize = 80;

/**
  | The maximum size in bytes of each witness
  | stack item in a standard BIP 342 script
  | (Taproot, leaf version 0xc0)
  |
  */
pub const MAX_STANDARD_TAPSCRIPT_STACK_ITEM_SIZE: usize = 80;

/**
  | The maximum size in bytes of a standard
  | witnessScript
  |
  */
pub const MAX_STANDARD_P2WSH_SCRIPT_SIZE: usize = 3600;

/**
  | The maximum size of a standard ScriptSig
  |
  */
pub const MAX_STANDARD_SCRIPTSIG_SIZE: usize = 1650;

/**
  | Min feerate for defining dust. Historically
  | this has been based on the minRelayTxFee,
  | however changing the dust limit changes
  | which transactions are standard and
  | should be done with care and ideally
  | rarely. It makes sense to only increase
  | the dust limit after prior releases
  | were already not creating outputs below
  | the new threshold
  |
  */
pub const DUST_RELAY_TX_FEE: Amount = 3000;

/**
  | Changing the default transaction version
  | requires a two step process: first adapting
  | relay policy by bumping
  | TX_MAX_STANDARD_VERSION, and then later
  | allowing the new transaction version in the
  | wallet/RPC.
  */
lazy_static!{
    /*
    static constexpr decltype(CTransaction::nVersion) TX_MAX_STANDARD_VERSION{2};
    */
}

#[inline] pub fn get_virtual_transaction_size_from_transaction(tx: &Transaction) -> i64 {
    
    todo!();
        /*
            return GetVirtualTransactionSize(tx, 0, 0);
        */
}

#[inline] pub fn get_virtual_transaction_input_size(tx: &TxIn) -> i64 {
    
    todo!();
        /*
            return GetVirtualTransactionInputSize(tx, 0, 0);
        */
}

//-------------------------------------------[.cpp/bitcoin/src/policy/policy.cpp]

/**
  | @note
  | 
  | This file is intended to be customised
  | by the end user, and includes only local
  | node policy logic
  |
  */
pub fn get_dust_threshold(
        txout:             &TxOut,
        dust_relay_fee_in: &FeeRate) -> Amount {
    
    todo!();
        /*
            // "Dust" is defined in terms of dustRelayFee,
        // which has units satoshis-per-kilobyte.
        // If you'd pay more in fees than the value of the output
        // to spend something, then we consider it dust.
        // A typical spendable non-segwit txout is 34 bytes big, and will
        // need a CTxIn of at least 148 bytes to spend:
        // so dust is a spendable txout less than
        // 182*dustRelayFee/1000 (in satoshis).
        // 546 satoshis at the default rate of 3000 sat/kvB.
        // A typical spendable segwit P2WPKH txout is 31 bytes big, and will
        // need a CTxIn of at least 67 bytes to spend:
        // so dust is a spendable txout less than
        // 98*dustRelayFee/1000 (in satoshis).
        // 294 satoshis at the default rate of 3000 sat/kvB.
        if (txout.scriptPubKey.IsUnspendable())
            return 0;

        size_t nSize = GetSerializeSize(txout);
        int witnessversion = 0;
        std::vector<unsigned char> witnessprogram;

        // Note this computation is for spending a Segwit v0 P2WPKH output (a 33 bytes
        // public key + an ECDSA signature). For Segwit v1 Taproot outputs the minimum
        // satisfaction is lower (a single BIP340 signature) but this computation was
        // kept to not further reduce the dust level.
        // See discussion in https://github.com/bitcoin/bitcoin/pull/22779 for details.
        if (txout.scriptPubKey.IsWitnessProgram(witnessversion, witnessprogram)) {
            // sum the sizes of the parts of a transaction input
            // with 75% segwit discount applied to the script size.
            nSize += (32 + 4 + 1 + (107 / WITNESS_SCALE_FACTOR) + 4);
        } else {
            nSize += (32 + 4 + 1 + 107 + 4); // the 148 mentioned above
        }

        return dustRelayFeeIn.GetFee(nSize);
        */
}

pub fn is_dust(
        txout:             &TxOut,
        dust_relay_fee_in: &FeeRate) -> bool {
    
    todo!();
        /*
            return (txout.nValue < GetDustThreshold(txout, dustRelayFeeIn));
        */
}

pub fn is_standard(
        script_pub_key: &Script,
        which_type:     &mut TxoutType) -> bool {
    
    todo!();
        /*
            std::vector<std::vector<unsigned char> > vSolutions;
        whichType = Solver(scriptPubKey, vSolutions);

        if (whichType == TxoutType::NONSTANDARD) {
            return false;
        } else if (whichType == TxoutType::MULTISIG) {
            unsigned char m = vSolutions.front()[0];
            unsigned char n = vSolutions.back()[0];
            // Support up to x-of-3 multisig txns as standard
            if (n < 1 || n > 3)
                return false;
            if (m < 1 || m > n)
                return false;
        } else if (whichType == TxoutType::NULL_DATA &&
                   (!fAcceptDatacarrier || scriptPubKey.size() > nMaxDatacarrierBytes)) {
              return false;
        }

        return true;
        */
}

/**
  | Check for standard transaction types
  | 
  | -----------
  | @return
  | 
  | True if all outputs (scriptPubKeys)
  | use only standard transaction forms
  |
  */
pub fn is_standard_tx(
        tx:                   &Transaction,
        permit_bare_multisig: bool,
        dust_relay_fee:       &FeeRate,
        reason:               &mut String) -> bool {
    
    todo!();
        /*
            if (tx.nVersion > TX_MAX_STANDARD_VERSION || tx.nVersion < 1) {
            reason = "version";
            return false;
        }

        // Extremely large transactions with lots of inputs can cost the network
        // almost as much to process as they cost the sender in fees, because
        // computing signature hashes is O(ninputs*txsize). Limiting transactions
        // to MAX_STANDARD_TX_WEIGHT mitigates CPU exhaustion attacks.
        unsigned int sz = GetTransactionWeight(tx);
        if (sz > MAX_STANDARD_TX_WEIGHT) {
            reason = "tx-size";
            return false;
        }

        for (const CTxIn& txin : tx.vin)
        {
            // Biggest 'standard' txin involving only keys is a 15-of-15 P2SH
            // multisig with compressed keys (remember the 520 byte limit on
            // redeemScript size). That works out to a (15*(33+1))+3=513 byte
            // redeemScript, 513+1+15*(73+1)+3=1627 bytes of scriptSig, which
            // we round off to 1650(MAX_STANDARD_SCRIPTSIG_SIZE) bytes for
            // some minor future-proofing. That's also enough to spend a
            // 20-of-20 CHECKMULTISIG scriptPubKey, though such a scriptPubKey
            // is not considered standard.
            if (txin.scriptSig.size() > MAX_STANDARD_SCRIPTSIG_SIZE) {
                reason = "scriptsig-size";
                return false;
            }
            if (!txin.scriptSig.IsPushOnly()) {
                reason = "scriptsig-not-pushonly";
                return false;
            }
        }

        unsigned int nDataOut = 0;
        TxoutType whichType;
        for (const CTxOut& txout : tx.vout) {
            if (!::IsStandard(txout.scriptPubKey, whichType)) {
                reason = "scriptpubkey";
                return false;
            }

            if (whichType == TxoutType::NULL_DATA)
                nDataOut++;
            else if ((whichType == TxoutType::MULTISIG) && (!permit_bare_multisig)) {
                reason = "bare-multisig";
                return false;
            } else if (IsDust(txout, dust_relay_fee)) {
                reason = "dust";
                return false;
            }
        }

        // only one OP_RETURN txout is permitted
        if (nDataOut > 1) {
            reason = "multi-op-return";
            return false;
        }

        return true;
        */
}

/**
  | Check for standard transaction types
  | 
  | -----------
  | @param[in] mapInputs
  | 
  | Map of previous transactions that have
  | outputs we're spending
  | ----------
  | @param[in] taproot_active
  | 
  | Whether or taproot consensus rules
  | are active (used to decide whether spends
  | of them are permitted)
  | 
  | -----------
  | @return
  | 
  | True if all inputs (scriptSigs) use
  | only standard transaction forms
  |
  | -----------
  | Check transaction inputs to mitigate two
  | potential denial-of-service attacks:
  |
  | 1. scriptSigs with extra data stuffed into them,
  |    not consumed by scriptPubKey (or P2SH script)
  |
  | 2. P2SH scripts with a crazy number of expensive
  |    CHECKSIG/CHECKMULTISIG operations
  |
  | Why bother? To avoid denial-of-service attacks;
  | an attacker can submit a standard
  | HASH... OP_EQUAL transaction, which will get
  | accepted into blocks. 
  |
  |
  | The redemption script can be anything; an
  | attacker could use a very
  | expensive-to-check-upon-redemption script like:
  |
  |   DUP CHECKSIG DROP ... repeated 100 times... OP_1
  |
  | Note that only the non-witness portion of the
  | transaction is checked here.
  */
pub fn are_inputs_standard(
    tx:             &Transaction,
    map_inputs:     &CoinsViewCache,
    taproot_active: bool) -> bool {
    
    todo!();
        /*
            if (tx.IsCoinBase())
            return true; // Coinbases don't use vin normally

        for (unsigned int i = 0; i < tx.vin.size(); i++)
        {
            const CTxOut& prev = mapInputs.AccessCoin(tx.vin[i].prevout).out;

            std::vector<std::vector<unsigned char> > vSolutions;
            TxoutType whichType = Solver(prev.scriptPubKey, vSolutions);
            if (whichType == TxoutType::NONSTANDARD || whichType == TxoutType::WITNESS_UNKNOWN) {
                // WITNESS_UNKNOWN failures are typically also caught with a policy
                // flag in the script interpreter, but it can be helpful to catch
                // this type of NONSTANDARD transaction earlier in transaction
                // validation.
                return false;
            } else if (whichType == TxoutType::SCRIPTHASH) {
                std::vector<std::vector<unsigned char> > stack;
                // convert the scriptSig into a stack, so we can inspect the redeemScript
                if (!EvalScript(stack, tx.vin[i].scriptSig, SCRIPT_VERIFY_NONE, BaseSignatureChecker(), SigVersion::BASE))
                    return false;
                if (stack.empty())
                    return false;
                CScript subscript(stack.back().begin(), stack.back().end());
                if (subscript.GetSigOpCount(true) > MAX_P2SH_SIGOPS) {
                    return false;
                }
            } else if (whichType == TxoutType::WITNESS_V1_TAPROOT) {
                // Don't allow Taproot spends unless Taproot is active.
                if (!taproot_active) return false;
            }
        }

        return true;
        */
}

/**
  | Check if the transaction is over standard
  | P2WSH resources limit: 3600bytes witnessScript
  | size, 80bytes per witness stack element,
  | 100 witness stack elements
  | 
  | These limits are adequate for multisignatures
  | up to n-of-100 using OP_CHECKSIG, OP_ADD,
  | and OP_EQUAL.
  | 
  | Also enforce a maximum stack item size
  | limit and no annexes for tapscript spends.
  |
  */
pub fn is_witness_standard(
        tx:         &Transaction,
        map_inputs: &CoinsViewCache) -> bool {
    
    todo!();
        /*
            if (tx.IsCoinBase())
            return true; // Coinbases are skipped

        for (unsigned int i = 0; i < tx.vin.size(); i++)
        {
            // We don't care if witness for this input is empty, since it must not be bloated.
            // If the script is invalid without witness, it would be caught sooner or later during validation.
            if (tx.vin[i].scriptWitness.IsNull())
                continue;

            const CTxOut &prev = mapInputs.AccessCoin(tx.vin[i].prevout).out;

            // get the scriptPubKey corresponding to this input:
            CScript prevScript = prev.scriptPubKey;

            bool p2sh = false;
            if (prevScript.IsPayToScriptHash()) {
                std::vector <std::vector<unsigned char> > stack;
                // If the scriptPubKey is P2SH, we try to extract the redeemScript casually by converting the scriptSig
                // into a stack. We do not check IsPushOnly nor compare the hash as these will be done later anyway.
                // If the check fails at this stage, we know that this txid must be a bad one.
                if (!EvalScript(stack, tx.vin[i].scriptSig, SCRIPT_VERIFY_NONE, BaseSignatureChecker(), SigVersion::BASE))
                    return false;
                if (stack.empty())
                    return false;
                prevScript = CScript(stack.back().begin(), stack.back().end());
                p2sh = true;
            }

            int witnessversion = 0;
            std::vector<unsigned char> witnessprogram;

            // Non-witness program must not be associated with any witness
            if (!prevScript.IsWitnessProgram(witnessversion, witnessprogram))
                return false;

            // Check P2WSH standard limits
            if (witnessversion == 0 && witnessprogram.size() == WITNESS_V0_SCRIPTHASH_SIZE) {
                if (tx.vin[i].scriptWitness.stack.back().size() > MAX_STANDARD_P2WSH_SCRIPT_SIZE)
                    return false;
                size_t sizeWitnessStack = tx.vin[i].scriptWitness.stack.size() - 1;
                if (sizeWitnessStack > MAX_STANDARD_P2WSH_STACK_ITEMS)
                    return false;
                for (unsigned int j = 0; j < sizeWitnessStack; j++) {
                    if (tx.vin[i].scriptWitness.stack[j].size() > MAX_STANDARD_P2WSH_STACK_ITEM_SIZE)
                        return false;
                }
            }

            // Check policy limits for Taproot spends:
            // - MAX_STANDARD_TAPSCRIPT_STACK_ITEM_SIZE limit for stack item size
            // - No annexes
            if (witnessversion == 1 && witnessprogram.size() == WITNESS_V1_TAPROOT_SIZE && !p2sh) {
                // Taproot spend (non-P2SH-wrapped, version 1, witness program size 32; see BIP 341)
                auto stack = MakeSpan(tx.vin[i].scriptWitness.stack);
                if (stack.size() >= 2 && !stack.back().empty() && stack.back()[0] == ANNEX_TAG) {
                    // Annexes are nonstandard as long as no semantics are defined for them.
                    return false;
                }
                if (stack.size() >= 2) {
                    // Script path spend (2 or more stack elements after removing optional annex)
                    const auto& control_block = SpanPopBack(stack);
                    SpanPopBack(stack); // Ignore script
                    if (control_block.empty()) return false; // Empty control block is invalid
                    if ((control_block[0] & TAPROOT_LEAF_MASK) == TAPROOT_LEAF_TAPSCRIPT) {
                        // Leaf version 0xc0 (aka Tapscript, see BIP 342)
                        for (const auto& item : stack) {
                            if (item.size() > MAX_STANDARD_TAPSCRIPT_STACK_ITEM_SIZE) return false;
                        }
                    }
                } else if (stack.size() == 1) {
                    // Key path spend (1 stack element after removing optional annex)
                    // (no policy rules apply)
                } else {
                    // 0 stack elements; this is already invalid by consensus rules
                    return false;
                }
            }
        }
        return true;
        */
}

/**
  | Compute the virtual transaction size
  | (weight reinterpreted as bytes).
  |
  */
pub fn get_virtual_transaction_size_for_weight(
        n_weight:        i64,
        n_sig_op_cost:   i64,
        bytes_per_sigop: u32) -> i64 {
    
    todo!();
        /*
            return (std::max(nWeight, nSigOpCost * bytes_per_sigop) + WITNESS_SCALE_FACTOR - 1) / WITNESS_SCALE_FACTOR;
        */
}

pub fn get_virtual_transaction_size_for_tx(
        tx:              &Transaction,
        n_sig_op_cost:   i64,
        bytes_per_sigop: u32) -> i64 {
    
    todo!();
        /*
            return GetVirtualTransactionSize(GetTransactionWeight(tx), nSigOpCost, bytes_per_sigop);
        */
}

pub fn get_virtual_transaction_input_size_for_txin(
        txin:            &TxIn,
        n_sig_op_cost:   i64,
        bytes_per_sigop: u32) -> i64 {
    
    todo!();
        /*
            return GetVirtualTransactionSize(GetTransactionInputWeight(txin), nSigOpCost, bytes_per_sigop);
        */
}
