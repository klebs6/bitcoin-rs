crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/signet.h]

/**
  | Generate the signet tx corresponding
  | to the given block
  | 
  | The signet tx commits to everything
  | in the block except:
  | 
  | - 1. It hashes a modified merkle root
  | with the signet signature removed.
  | 
  | - 2. It skips the nonce.
  |
  */
pub struct SignetTxs {
    to_spend: Transaction,
    to_sign:  Transaction,
}

impl SignetTxs {
    
    pub fn new<T1, T2>(
        to_spend: &T1,
        to_sign:  &T2) -> Self {
    
        todo!();
        /*
            : m_to_spend{to_spend}, m_to_sign{to_sign}
        */
    }
    
    pub fn create(&mut self, 
        block:     &Block,
        challenge: &Script) -> Option<SignetTxs> {
        
        todo!();
        /*
            CMutableTransaction tx_to_spend;
        tx_to_spend.nVersion = 0;
        tx_to_spend.nLockTime = 0;
        tx_to_spend.vin.emplace_back(OutPoint(), CScript(OP_0), 0);
        tx_to_spend.vout.emplace_back(0, challenge);

        CMutableTransaction tx_spending;
        tx_spending.nVersion = 0;
        tx_spending.nLockTime = 0;
        tx_spending.vin.emplace_back(OutPoint(), CScript(), 0);
        tx_spending.vout.emplace_back(0, CScript(OP_RETURN));

        // can't fill any other fields before extracting signet
        // responses from block coinbase tx

        // find and delete signet signature
        if (block.vtx.empty()) return std::nullopt; // no coinbase tx in block; invalid
        CMutableTransaction modified_cb(*block.vtx.at(0));

        const int cidx = GetWitnessCommitmentIndex(block);
        if (cidx == NO_WITNESS_COMMITMENT) {
            return std::nullopt; // require a witness commitment
        }

        CScript& witness_commitment = modified_cb.vout.at(cidx).scriptPubKey;

        std::vector<uint8_t> signet_solution;
        if (!FetchAndClearCommitmentSection(SIGNET_HEADER, witness_commitment, signet_solution)) {
            // no signet solution -- allow this to support OP_TRUE as trivial block challenge
        } else {
            try {
                VectorReader v(SER_NETWORK, INIT_PROTO_VERSION, signet_solution, 0);
                v >> tx_spending.vin[0].scriptSig;
                v >> tx_spending.vin[0].scriptWitness.stack;
                if (!v.empty()) return std::nullopt; // extraneous data encountered
            } catch (const std::exception&) {
                return std::nullopt; // parsing error
            }
        }
        uint256 signet_merkle = ComputeModifiedMerkleRoot(modified_cb, block);

        std::vector<uint8_t> block_data;
        CVectorWriter writer(SER_NETWORK, INIT_PROTO_VERSION, block_data, 0);
        writer << block.nVersion;
        writer << block.hashPrevBlock;
        writer << signet_merkle;
        writer << block.nTime;
        tx_to_spend.vin[0].scriptSig << block_data;
        tx_spending.vin[0].prevout = OutPoint(tx_to_spend.GetHash(), 0);

        return SignetTxs{tx_to_spend, tx_spending};
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/signet.cpp]

pub const SIGNET_HEADER: [u8; 4] = [0xec, 0xc7, 0xda, 0xa2];

pub const BLOCK_SCRIPT_VERIFY_FLAGS: u32 = 
ScriptVerificationFlags::SCRIPT_VERIFY_P2SH.bits() 
| ScriptVerificationFlags::SCRIPT_VERIFY_WITNESS.bits() 
| ScriptVerificationFlags::SCRIPT_VERIFY_DERSIG.bits() 
| ScriptVerificationFlags::SCRIPT_VERIFY_NULLDUMMY.bits();

pub fn fetch_and_clear_commitment_section(
        header:             &[u8],
        witness_commitment: &mut Script,
        result:             &mut Vec<u8>) -> bool {
    
    todo!();
        /*
            CScript replacement;
        bool found_header = false;
        result.clear();

        opcodetype opcode;
        CScript::const_iterator pc = witness_commitment.begin();
        std::vector<uint8_t> pushdata;
        while (witness_commitment.GetOp(pc, opcode, pushdata)) {
            if (pushdata.size() > 0) {
                if (!found_header && pushdata.size() > (size_t) header.size() && Span<const uint8_t>(pushdata.data(), header.size()) == header) {
                    // pushdata only counts if it has the header _and_ some data
                    result.insert(result.end(), pushdata.begin() + header.size(), pushdata.end());
                    pushdata.erase(pushdata.begin() + header.size(), pushdata.end());
                    found_header = true;
                }
                replacement << pushdata;
            } else {
                replacement << opcode;
            }
        }

        if (found_header) witness_commitment = replacement;
        return found_header;
        */
}

pub fn compute_modified_merkle_root(
    cb:    &MutableTransaction,
    block: &Block
) -> u256 {
    
    todo!();
        /*
            std::vector<uint256> leaves;
        leaves.resize(block.vtx.size());
        leaves[0] = cb.GetHash();
        for (size_t s = 1; s < block.vtx.size(); ++s) {
            leaves[s] = block.vtx[s]->GetHash();
        }
        return ComputeMerkleRoot(std::move(leaves));
        */
}

/**
  | Signet block solution checker
  |
  | Extract signature and check whether
  | a block has a valid solution
  |
  */
pub fn check_signet_block_solution(
    block:            &Block,
    consensus_params: &ChainConsensusParams
) -> bool {
    
    todo!();
        /*
        if (block.GetHash() == consensusParams.hashGenesisBlock) {
            // genesis block solution is always valid
            return true;
        }

        const CScript challenge(consensusParams.signet_challenge.begin(), consensusParams.signet_challenge.end());
        const std::optional<SignetTxs> signet_txs = SignetTxs::Create(block, challenge);

        if (!signet_txs) {
            LogPrint(BCLog::VALIDATION, "CheckSignetBlockSolution: Errors in block (block solution parse failure)\n");
            return false;
        }

        const CScript& scriptSig = signet_txs->m_to_sign.vin[0].scriptSig;
        const CScriptWitness& witness = signet_txs->m_to_sign.vin[0].scriptWitness;

        PrecomputedTransactionData txdata;
        txdata.Init(signet_txs->m_to_sign, {signet_txs->m_to_spend.vout[0]});
        TransactionSignatureChecker sigcheck(&signet_txs->m_to_sign, /* nInIn= */ 0, /* amountIn= */ signet_txs->m_to_spend.vout[0].nValue, txdata, MissingDataBehavior::ASSERT_FAIL);

        if (!VerifyScript(scriptSig, signet_txs->m_to_spend.vout[0].scriptPubKey, &witness, BLOCK_SCRIPT_VERIFY_FLAGS, sigcheck)) {
            LogPrint(BCLog::VALIDATION, "CheckSignetBlockSolution: Errors in block (block solution invalid)\n");
            return false;
        }
        return true;
        */
}
