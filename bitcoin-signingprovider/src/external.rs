crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/wallet/external_signer_scriptpubkeyman.h]

pub struct ExternalSignerScriptPubKeyMan {
    base: DescriptorScriptPubKeyMan,
}

//-------------------------------------------[.cpp/bitcoin/src/wallet/external_signer_scriptpubkeyman.cpp]
impl ExternalSignerScriptPubKeyMan {
    
    /**
      | Provide a descriptor at setup time
      | 
      | Returns false if already setup or setup
      | fails, true if setup is successful
      |
      */
    pub fn setup_descriptor(&mut self, desc: Box<dyn Descriptor>) -> bool {
        
        todo!();
        /*
            LOCK(cs_desc_man);
        assert(m_storage.IsWalletFlagSet(WALLET_FLAG_DESCRIPTORS));
        assert(m_storage.IsWalletFlagSet(WALLET_FLAG_EXTERNAL_SIGNER));

        int64_t creation_time = GetTime();

        // Make the descriptor
        WalletDescriptor w_desc(std::move(desc), creation_time, 0, 0, 0);
        m_wallet_descriptor = w_desc;

        // Store the descriptor
        WalletBatch batch(m_storage.GetDatabase());
        if (!batch.WriteDescriptor(GetID(), m_wallet_descriptor)) {
            throw std::runtime_error(std::string(__func__) + ": writing descriptor failed");
        }

        // TopUp
        TopUp();

        m_storage.UnsetBlankWalletFlag(batch);
        return true;
        */
    }
    
    pub fn get_external_signer(&mut self) -> ExternalSigner {
        
        todo!();
        /*
            const std::string command = gArgs.GetArg("-signer", "");
        if (command == "") throw std::runtime_error(std::string(__func__) + ": restart bitcoind with -signer=<cmd>");
        std::vector<ExternalSigner> signers;
        ExternalSigner::Enumerate(command, signers, Params().NetworkIDString());
        if (signers.empty()) throw std::runtime_error(std::string(__func__) + ": No external signers found");
        // TODO: add fingerprint argument in case of multiple signers
        return signers[0];
        */
    }
    
    pub fn display_address(&self, 
        script_pub_key: Script,
        signer:         &ExternalSigner) -> bool {
        
        todo!();
        /*
            // TODO: avoid the need to infer a descriptor from inside a descriptor wallet
        auto provider = GetSolvingProvider(scriptPubKey);
        auto descriptor = InferDescriptor(scriptPubKey, *provider);

        signer.DisplayAddress(descriptor->ToString());
        // TODO inspect result
        return true;
        */
    }

    /**
      | If sign is true, transaction must previously
      | have been filled
      |
      */
    pub fn fillpsbt(&self, 
        psbt:         &mut PartiallySignedTransaction,
        txdata:       &PrecomputedTransactionData,
        sighash_type: Option<i32>,
        sign:         Option<bool>,
        bip_32derivs: Option<bool>,
        n_signed:     Option<*mut i32>) -> TransactionError {

        let sighash_type:   i32 = sighash_type.unwrap_or(1 ); /* SIGHASH_ALL */
        let sign:          bool = sign.unwrap_or(true);
        let bip_32derivs:  bool = bip_32derivs.unwrap_or(false);
        
        todo!();
        /*
            if (!sign) {
            return DescriptorScriptPubKeyMan::FillPSBT(psbt, txdata, sighash_type, false, bip32derivs, n_signed);
        }

        // Already complete if every input is now signed
        bool complete = true;
        for (const auto& input : psbt.inputs) {
            // TODO: for multisig wallets, we should only care if all _our_ inputs are signed
            complete &= PSBTInputSigned(input);
        }
        if (complete) return TransactionError::OK;

        std::string strFailReason;
        if(!GetExternalSigner().SignTransaction(psbt, strFailReason)) {
            tfm::format(std::cerr, "Failed to sign: %s\n", strFailReason);
            return TransactionError::EXTERNAL_SIGNER_FAILED;
        }
        FinalizePSBT(psbt); // This won't work in a multisig setup
        return TransactionError::OK;
        */
    }
}
