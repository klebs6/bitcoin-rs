crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/external_signer.h]

/**
  | Enables interaction with an external signing
  | device or service, such as a hardware
  | wallet. See doc/external-signer.md
  */
pub struct ExternalSigner {

    /**
      | The command which handles interaction
      | with the external signer.
      |
      */
    command:     String,

    /**
      | Bitcoin mainnet, testnet, etc
      |
      */
    chain:       String,

    /**
      | Master key fingerprint of the signer
      |
      */
    fingerprint: String,

    /**
      | Name of signer
      |
      */
    name:        String,
}

//-------------------------------------------[.cpp/bitcoin/src/external_signer.cpp]
impl ExternalSigner {

    /**
      | @param[in] command
      | 
      | the command which handles interaction
      | with the external signer
      | ----------
      | @param[in] fingerprint
      | 
      | master key fingerprint of the signer
      | ----------
      | @param[in] chain
      | 
      | "main", "test", "regtest" or "signet"
      | ----------
      | @param[in] name
      | 
      | device name
      |
      */
    pub fn new(
        command:     &String,
        chain:       String,
        fingerprint: &String,
        name:        String) -> Self {
    
        todo!();
        /*
        : command(command),
        : chain(chain),
        : fingerprint(fingerprint),
        : name(name),

        
        */
    }
    
    pub fn network_arg(&self) -> String {
        
        todo!();
        /*
            return " --chain " + m_chain;
        */
    }
    
    /**
      | Obtain a list of signers. Calls `<command>
      | enumerate`.
      | 
      | -----------
      | @param[in] command
      | 
      | the command which handles interaction
      | with the external signer
      | ----------
      | @param[in,out] signers
      | 
      | vector to which new signers (with a unique
      | master key fingerprint) are added
      | ----------
      | @param chain
      | 
      | "main", "test", "regtest" or "signet"
      | 
      | -----------
      | @return
      | 
      | success
      |
      */
    pub fn enumerate(&mut self, 
        command: &String,
        signers: &mut Vec<ExternalSigner>,
        chain:   String) -> bool {
        
        todo!();
        /*
            // Call <command> enumerate
        const UniValue result = RunCommandParseJSON(command + " enumerate");
        if (!result.isArray()) {
            throw std::runtime_error(strprintf("'%s' received invalid response, expected array of signers", command));
        }
        for (UniValue signer : result.getValues()) {
            // Check for error
            const UniValue& error = find_value(signer, "error");
            if (!error.isNull()) {
                if (!error.isStr()) {
                    throw std::runtime_error(strprintf("'%s' error", command));
                }
                throw std::runtime_error(strprintf("'%s' error: %s", command, error.getValStr()));
            }
            // Check if fingerprint is present
            const UniValue& fingerprint = find_value(signer, "fingerprint");
            if (fingerprint.isNull()) {
                throw std::runtime_error(strprintf("'%s' received invalid response, missing signer fingerprint", command));
            }
            const std::string fingerprintStr = fingerprint.get_str();
            // Skip duplicate signer
            bool duplicate = false;
            for (const ExternalSigner& signer : signers) {
                if (signer.m_fingerprint.compare(fingerprintStr) == 0) duplicate = true;
            }
            if (duplicate) break;
            std::string name = "";
            const UniValue& model_field = find_value(signer, "model");
            if (model_field.isStr() && model_field.getValStr() != "") {
                name += model_field.getValStr();
            }
            signers.push_back(ExternalSigner(command, chain, fingerprintStr, name));
        }
        return true;
        */
    }
    
    /**
      | Display address on the device. Calls
      | `<command> displayaddress --desc
      | <descriptor>`.
      | 
      | -----------
      | @param[in] descriptor
      | 
      | Descriptor specifying which address
      | to display.
      | 
      | Must include a public key or xpub, as
      | well as key origin.
      |
      */
    pub fn display_address(&self, descriptor: &String) -> UniValue {
        
        todo!();
        /*
            return RunCommandParseJSON(m_command + " --fingerprint \"" + m_fingerprint + "\"" + NetworkArg() + " displayaddress --desc \"" + descriptor + "\"");
        */
    }
    
    /**
      | Get receive and change Descriptor(s)
      | from device for a given account.
      | 
      | Calls `<command> getdescriptors --account
      | <account>`
      | 
      | -----------
      | @param[in] account
      | 
      | which BIP32 account to use (e.g. `m/44'/0'/account'`)
      | 
      | -----------
      | @return
      | 
      | see doc/external-signer.md
      |
      */
    pub fn get_descriptors(&mut self, account: i32) -> UniValue {
        
        todo!();
        /*
            return RunCommandParseJSON(m_command + " --fingerprint \"" + m_fingerprint + "\"" + NetworkArg() + " getdescriptors --account " + strprintf("%d", account));
        */
    }
    
    /**
      | Sign PartiallySignedTransaction
      | on the device.
      | 
      | Calls `<command> signtransaction`
      | and passes the PSBT via stdin.
      | 
      | -----------
      | @param[in,out] psbt
      | 
      | PartiallySignedTransaction to be
      | signed
      |
      */
    pub fn sign_transaction(&mut self, 
        psbtx: &mut PartiallySignedTransaction,
        error: &mut String) -> bool {
        
        todo!();
        /*
            // Serialize the PSBT
        DataStream ssTx(SER_NETWORK, PROTOCOL_VERSION);
        ssTx << psbtx;

        // Check if signer fingerprint matches any input master key fingerprint
        auto matches_signer_fingerprint = [&](const PSBTInput& input) {
            for (const auto& entry : input.hd_keypaths) {
                if (m_fingerprint == strprintf("%08x", ReadBE32(entry.second.fingerprint))) return true;
            }
            return false;
        };

        if (!std::any_of(psbtx.inputs.begin(), psbtx.inputs.end(), matches_signer_fingerprint)) {
            error = "Signer fingerprint " + m_fingerprint + " does not match any of the inputs:\n" + EncodeBase64(ssTx.str());
            return false;
        }

        const std::string command = m_command + " --stdin --fingerprint \"" + m_fingerprint + "\"" + NetworkArg();
        const std::string stdinStr = "signtx \"" + EncodeBase64(ssTx.str()) + "\"";

        const UniValue signer_result = RunCommandParseJSON(command, stdinStr);

        if (find_value(signer_result, "error").isStr()) {
            error = find_value(signer_result, "error").get_str();
            return false;
        }

        if (!find_value(signer_result, "psbt").isStr()) {
            error = "Unexpected result from signer";
            return false;
        }

        PartiallySignedTransaction signer_psbtx;
        std::string signer_psbt_error;
        if (!DecodeBase64PSBT(signer_psbtx, find_value(signer_result, "psbt").get_str(), signer_psbt_error)) {
            error = strprintf("TX decode failed %s", signer_psbt_error);
            return false;
        }

        psbtx = signer_psbtx;

        return true;
        */
    }
}

