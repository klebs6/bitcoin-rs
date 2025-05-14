// ---------------- [ File: bitcoin-psbt/src/input.rs ]
crate::ix!();

/**
  | Checks whether a PSBTInput is already
  | signed.
  |
  */
pub fn psbt_input_signed(input: &PSBTInput) -> bool {
    
    !input.final_script_sig.empty() 
    || !input.final_script_witness.is_null()
}

/**
  | A structure for PSBTs which contain
  | per-input information
  |
  */
#[derive(Default,Clone)]
pub struct PSBTInput {
    pub non_witness_utxo:     Arc<Transaction>,
    pub witness_utxo:         TxOut,
    pub redeem_script:        Script,
    pub witness_script:       Script,
    pub final_script_sig:     Script,
    pub final_script_witness: ScriptWitness,
    pub hd_keypaths:          HashMap<PubKey,KeyOriginInfo>,
    pub partial_sigs:         HashMap<KeyID,SigPair>,
    pub unknown:              HashMap<Vec<u8>,Vec<u8>>,
    pub sighash_type:         i32, // default = 0
}

impl PSBTInput {

    #[inline] pub fn serialize<Stream: GetType + GetVersion + StreamItems>(&self, s: &mut Stream)  {

        // Write the utxo
        if Arc::<Transaction>::strong_count(&self.non_witness_utxo) != 0 {

            serialize_to_vector(s, &PSBT_IN_NON_WITNESS_UTXO);

            let mut os: OverrideStream::<Stream> 
            = OverrideStream::<Stream>::new(
                s, 
                s.get_type(), 
                s.get_version() | SERIALIZE_TRANSACTION_NO_WITNESS
            );

            serialize_to_vector(&mut os, &self.non_witness_utxo);
        }

        if !self.witness_utxo.is_null() {
            serialize_to_vector(s, &PSBT_IN_WITNESS_UTXO);
            serialize_to_vector(s, &self.witness_utxo);
        }

        if self.final_script_sig.empty() 
        && self.final_script_witness.is_null() 
        {
            //  Write any partial signatures
            for sig_pair in self.partial_sigs.iter() {

                serialize_to_vector(s, &PSBT_IN_PARTIAL_SIG);
                serialize_to_vector(s, &sig_pair.1.0);

                s.stream(sig_pair.1.1.clone());
            }

            //  Write the sighash type
            if self.sighash_type > 0 {
                serialize_to_vector(s, &PSBT_IN_SIGHASH);
                serialize_to_vector(s, &self.sighash_type);
            }

            //  Write the redeem script
            if !self.redeem_script.empty() {
                serialize_to_vector(s, &PSBT_IN_REDEEMSCRIPT);
                s.stream(self.redeem_script.clone());
            }

            //  Write the witness script
            if !self.witness_script.empty() {
                serialize_to_vector(s, &PSBT_IN_WITNESSSCRIPT);
                s.stream(self.witness_script.clone());
            }

            //  Write any hd keypaths
            serialize_hd_keypaths(
                s,
                &self.hd_keypaths,
                PSBT_IN_BIP32_DERIVATION
            );
        }

        // Write script sig
        if !self.final_script_sig.empty() {
            serialize_to_vector(s, &PSBT_IN_SCRIPTSIG);
            s.stream(self.final_script_sig.clone());
        }

        // write script witness
        if !self.final_script_witness.is_null() {
            serialize_to_vector(s, &PSBT_IN_SCRIPTWITNESS);
            serialize_to_vector(s, &self.final_script_witness.stack);
        }

        // Write unknown things
        for entry in self.unknown.iter() {
            s.stream(entry.0);
            s.stream(entry.1);
        }

        s.stream(PSBT_SEPARATOR);
    }
    
    #[inline] pub fn unserialize<Stream: ExactSizeIterator + GetType + StreamItems + GetVersion>(&mut self, 
        s: &mut Stream) -> Result<(), StdException>  {

        // Used for duplicate key detection
        let mut key_lookup = HashSet::<Vec::<u8>>::default();
    
        // Read loop
        let mut found_sep: bool = false;

        while !s.len() == 0 {

            //  Read
            let mut key = Vec::<u8>::default();;

            s.stream(&key);

            /*
              |  the key is empty if that was
              |  actually a separator byte
              |
              |  This is a special case for key
              |  lengths 0 as those are not allowed
              |  (except for separator)
              */
            if key.is_empty() {
                found_sep = true;
                break;
            }

            //  First byte of key is the type
            let ty: u8 = key[0];;

            //  Do stuff based on type
            match ty {

                PSBT_IN_NON_WITNESS_UTXO  => {

                    if !key_lookup.insert(key.clone()) {

                        return Err(ios_base_failure("Duplicate Key, input non-witness utxo already provided"));

                    } else if key.len() != 1 {

                        return Err(ios_base_failure("Non-witness utxo key is more than one byte type"));
                    }

                    //  Set the stream to
                    //  unserialize with witness
                    //  since this is always
                    //  a valid network
                    //  transaction
                    let mut os: OverrideStream::<Stream> 
                        = OverrideStream::<Stream>::new(
                            s, 
                            s.get_type(), 
                            s.get_version() & !SERIALIZE_TRANSACTION_NO_WITNESS
                        );

                    unserialize_from_vector(&mut os, &mut self.non_witness_utxo);

                    break;
                },

                PSBT_IN_WITNESS_UTXO  => {

                    if !key_lookup.insert(key.clone()) {

                        return Err(ios_base_failure("Duplicate Key, input witness utxo already provided"));

                    } else if key.len() != 1 {

                        return Err(ios_base_failure("Witness utxo key is more than one byte type"));
                    }

                    unserialize_from_vector(s, &mut self.witness_utxo);

                    break;
                },

                PSBT_IN_PARTIAL_SIG  => {

                    //  Make sure that the key is the size of pubkey + 1
                    if key.len() != PUB_KEY_SIZE + 1 
                        && key.len() != PUB_KEY_COMPRESSED_SIZE + 1 
                    {
                        return Err(ios_base_failure("Size of key was not the expected size for the type partial signature pubkey"));
                    }

                    //  Read in the pubkey from key
                    let pubkey: PubKey = PubKey::new(&key[1..]);

                    if !pubkey.is_fully_valid() {
                        return Err(ios_base_failure("Invalid pubkey"));
                    }

                    if self.partial_sigs.contains_key(&pubkey.getid()) {
                        return Err(ios_base_failure("Duplicate Key, input partial signature for pubkey already provided"));
                    }

                    //  Read in the signature from value
                    let mut sig = Vec::<u8>::default();;

                    s.stream(&sig);

                    //  Add to list
                    self.partial_sigs.insert(pubkey.getid(), (pubkey,sig));

                    break;
                },

                PSBT_IN_SIGHASH  => {

                    if !key_lookup.insert(key.clone()) {

                        return Err(ios_base_failure("Duplicate Key, input sighash type already provided"));

                    } else if key.len() != 1 {

                        return Err(ios_base_failure("Sighash type key is more than one byte type"));
                    }

                    unserialize_from_vector(s, &mut self.sighash_type);

                    break;
                },

                PSBT_IN_REDEEMSCRIPT  => {

                    if !key_lookup.insert(key.clone()) {

                        return Err(ios_base_failure("Duplicate Key, input redeemScript already provided"));

                    } else if key.len() != 1 {

                        return Err(ios_base_failure("Input redeemScript key is more than one byte type"));
                    }

                    s.stream(&self.redeem_script);

                    break;

                },

                PSBT_IN_WITNESSSCRIPT  => {

                    if !key_lookup.insert(key.clone()) {

                        return Err(ios_base_failure("Duplicate Key, input witnessScript already provided"));

                    } else if key.len() != 1 {

                        return Err(ios_base_failure("Input witnessScript key is more than one byte type"));
                    }

                    s.stream(&self.witness_script);

                    break;
                },

                PSBT_IN_BIP32_DERIVATION  => {

                    deserialize_hd_keypaths(s, &key, &mut self.hd_keypaths);

                    break;
                },

                PSBT_IN_SCRIPTSIG  => {

                    if !key_lookup.insert(key.clone()) {

                        return Err(ios_base_failure("Duplicate Key, input final scriptSig already provided"));

                    } else if key.len() != 1 {

                        return Err(ios_base_failure("Final scriptSig key is more than one byte type"));
                    }

                    s.stream(&self.final_script_sig);

                    break;
                },

                PSBT_IN_SCRIPTWITNESS  => {

                    if !key_lookup.insert(key.clone()) {

                        return Err(ios_base_failure("Duplicate Key, input final scriptWitness already provided"));

                    } else if key.len() != 1 {

                        return Err(ios_base_failure("Final scriptWitness key is more than one byte type"));
                    }

                    unserialize_from_vector(s, &mut self.final_script_witness.stack);

                    break;
                },

                _  => {

                    if self.unknown.contains_key(&key) {

                        return Err(ios_base_failure("Duplicate Key, key for unknown value already provided"));
                    }

                    //  Read in the value
                    let mut val_bytes = Vec::<u8>::default();;

                    s.stream(&val_bytes);

                    self.unknown.insert(key, val_bytes);

                    break;
                },
            }
        }

        if !found_sep {
            return Err(ios_base_failure("Separator is missing at the end of an input map"));
        }

        Ok(())
    }
    
    pub fn new<Stream: ExactSizeIterator + GetVersion + GetType + StreamItems>(
        _0: DeserializeType,
        s:  &mut Stream) -> Self {
    
        let mut x = Self::default();

        x.unserialize(s);

        x
    }

    pub fn is_null(&self) -> bool {

        Arc::<Transaction>::strong_count(&self.non_witness_utxo) == 0

        && self.witness_utxo.is_null()

        && self.partial_sigs.is_empty()

        && self.unknown.is_empty()

        && self.hd_keypaths.is_empty()

        && self.redeem_script.empty()

        && self.witness_script.empty()
    }

    pub fn fill_signature_data(&self, sigdata: &mut SignatureData)  {

        if !self.final_script_sig.empty() {
            sigdata.script_sig = self.final_script_sig.clone();
            sigdata.complete = true;
        }

        if !self.final_script_witness.is_null() {
            sigdata.script_witness = self.final_script_witness.clone();
            sigdata.complete = true;
        }

        if sigdata.complete {
            return;
        }

        for (kid,sigpair) in self.partial_sigs.iter() {
            sigdata.signatures.insert(kid.clone(),sigpair.clone());
        }

        if !self.redeem_script.empty() {
            sigdata.redeem_script = self.redeem_script.clone();
        }

        if !self.witness_script.empty() {
            sigdata.witness_script = self.witness_script.clone();
        }

        for key_pair in self.hd_keypaths.iter() {
            sigdata.misc_pubkeys.insert(
                key_pair.0.getid(), 
                (key_pair.0.clone(), key_pair.1.clone())
            );
        }
    }
    
    pub fn from_signature_data(&mut self, sigdata: &SignatureData)  {
        
        if sigdata.complete {

            self.partial_sigs.clear();
            self.hd_keypaths.clear();
            self.redeem_script.clear();
            self.witness_script.clear();

            if !sigdata.script_sig.empty() {
                self.final_script_sig = sigdata.script_sig.clone();
            }

            if !sigdata.script_witness.is_null() {
                self.final_script_witness = sigdata.script_witness.clone();
            }

            return;
        }

        for (kid,sigpair) in sigdata.signatures.iter() {
            self.partial_sigs.insert(kid.clone(),sigpair.clone());
        }

        if self.redeem_script.empty() && !sigdata.redeem_script.empty() {
            self.redeem_script = sigdata.redeem_script.clone();
        }

        if self.witness_script.empty() && !sigdata.witness_script.empty() {
            self.witness_script = sigdata.witness_script.clone();
        }

        for entry in sigdata.misc_pubkeys.iter() {

            self.hd_keypaths.insert(
                entry.1.0.clone(), 
                KeyOriginInfo::default()
            );
        }
    }
    
    pub fn merge(&mut self, input: &PSBTInput)  {
        
        if Arc::<Transaction>::strong_count(&self.non_witness_utxo) == 0 
        && Arc::<Transaction>::strong_count(&input.non_witness_utxo) != 0 
        {
            self.non_witness_utxo = input.non_witness_utxo.clone();
        }

        if self.witness_utxo.is_null() && !input.witness_utxo.is_null() {

            //  TODO: For segwit v1, we will want
            //  to clear out the non-witness utxo
            //  when setting a witness one. For v0
            //  and non-segwit, this is not safe
            self.witness_utxo = input.witness_utxo.clone();
        }

        for (k,v) in input.partial_sigs.iter() {
            self.partial_sigs.insert(k.clone(), v.clone());
        }

        for (k,v) in input.hd_keypaths.iter() {
            self.hd_keypaths.insert(k.clone(), (*v).clone());
        }

        for (k,v) in input.unknown.iter() {
            self.unknown.insert(k.clone(),v.clone());
        }

        if self.redeem_script.empty() && !input.redeem_script.empty() {
            self.redeem_script = input.redeem_script.clone();
        }

        if self.witness_script.empty() && !input.witness_script.empty() {
            self.witness_script = input.witness_script.clone();
        }

        if self.final_script_sig.empty() && !input.final_script_sig.empty() {
            self.final_script_sig = input.final_script_sig.clone();
        }

        if self.final_script_witness.is_null() && !input.final_script_witness.is_null() {
            self.final_script_witness = input.final_script_witness.clone();
        }
    }
}
