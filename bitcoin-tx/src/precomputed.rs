crate::ix!();

#[derive(Default)]
pub struct PrecomputedTransactionData {

    /**
      | BIP341 precomputed data.
      | 
      | These are single-SHA256, 
      | see https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki#cite_note-15.
      |
      */
    prevouts_single_hash:      u256,

    sequences_single_hash:     u256,
    outputs_single_hash:       u256,
    spent_amounts_single_hash: u256,
    spent_scripts_single_hash: u256,

    /**
      | Whether the 5 fields above are initialized.
      |
      */
    bip341_taproot_ready:      bool, // default = false

    /**
      | BIP143 precomputed data (double-SHA256).
      |
      */
    hash_prevouts:             u256,

    /**
      | BIP143 precomputed data (double-SHA256).
      |
      */
    hash_sequence:             u256,

    /**
      | BIP143 precomputed data (double-SHA256).
      |
      */
    hash_outputs:              u256,

    /**
      | Whether the 3 fields above are initialized.
      |
      */
    bip143_segwit_ready:       bool, // default = false

    spent_outputs:             Vec<TxOut>,

    /**
      | Whether m_spent_outputs is initialized.
      |
      */
    spent_outputs_ready:       bool, // default = false
}

impl PrecomputedTransactionData {

    /**
      | Initialize this PrecomputedTransactionData
      | with transaction data.
      | 
      | -----------
      | @param[in] tx
      | 
      | The transaction for which data is being
      | precomputed.
      | ----------
      | @param[in] spent_outputs
      | 
      | The CTxOuts being spent, one for each
      | tx.vin, in order.
      | ----------
      | @param[in] force
      | 
      | Whether to precompute data for all optional
      | features, regardless of what is in the
      | inputs (used at signing time, when the
      | inputs aren't filled in yet).
      |
      */
    pub fn init<T>(&mut self, 
        tx_to:         &T,
        spent_outputs: Vec<TxOut>,
        force:         Option<bool>)  {

        let force: bool = force.unwrap_or(false);
    
        todo!();
        /*
            assert(!m_spent_outputs_ready);

        m_spent_outputs = std::move(spent_outputs);
        if (!m_spent_outputs.empty()) {
            assert(m_spent_outputs.size() == txTo.vin.size());
            m_spent_outputs_ready = true;
        }

        // Determine which precomputation-impacting features this transaction uses.
        bool uses_bip143_segwit = force;
        bool uses_bip341_taproot = force;
        for (size_t inpos = 0; inpos < txTo.vin.size() && !(uses_bip143_segwit && uses_bip341_taproot); ++inpos) {
            if (!txTo.vin[inpos].scriptWitness.IsNull()) {
                if (m_spent_outputs_ready && m_spent_outputs[inpos].scriptPubKey.size() == 2 + WITNESS_V1_TAPROOT_SIZE &&
                    m_spent_outputs[inpos].scriptPubKey[0] == OP_1) {
                    // Treat every witness-bearing spend with 34-byte scriptPubKey that starts with OP_1 as a Taproot
                    // spend. This only works if spent_outputs was provided as well, but if it wasn't, actual validation
                    // will fail anyway. Note that this branch may trigger for scriptPubKeys that aren't actually segwit
                    // but in that case validation will fail as SCRIPT_ERR_WITNESS_UNEXPECTED anyway.
                    uses_bip341_taproot = true;
                } else {
                    // Treat every spend that's not known to native witness v1 as a Witness v0 spend. This branch may
                    // also be taken for unknown witness versions, but it is harmless, and being precise would require
                    // P2SH evaluation to find the redeemScript.
                    uses_bip143_segwit = true;
                }
            }
            if (uses_bip341_taproot && uses_bip143_segwit) break; // No need to scan further if we already need all.
        }

        if (uses_bip143_segwit || uses_bip341_taproot) {
            // Computations shared between both sighash schemes.
            m_prevouts_single_hash = GetPrevoutsSHA256(txTo);
            m_sequences_single_hash = GetSequencesSHA256(txTo);
            m_outputs_single_hash = GetOutputsSHA256(txTo);
        }
        if (uses_bip143_segwit) {
            hashPrevouts = SHA256Uint256(m_prevouts_single_hash);
            hashSequence = SHA256Uint256(m_sequences_single_hash);
            hashOutputs = SHA256Uint256(m_outputs_single_hash);
            m_bip143_segwit_ready = true;
        }
        if (uses_bip341_taproot) {
            m_spent_amounts_single_hash = GetSpentAmountsSHA256(m_spent_outputs);
            m_spent_scripts_single_hash = GetSpentScriptsSHA256(m_spent_outputs);
            m_bip341_taproot_ready = true;
        }
        */
    }
    
    pub fn new<T>(tx_to: &T) -> Self {
    
        todo!();
        /*
            Init(txTo, {});
        */
    }
}
