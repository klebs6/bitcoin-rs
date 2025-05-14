// ---------------- [ File: bitcoin-psbt/src/output.rs ]
crate::ix!();

/**
  | A structure for PSBTs which contains
  | per output information
  |
  */
#[derive(Default,Clone)]
pub struct PSBTOutput {
    redeem_script:  Script,
    witness_script: Script,
    hd_keypaths:    HashMap<PubKey,KeyOriginInfo>,
    unknown:        HashMap<Vec<u8>,Vec<u8>>,
}

impl PSBTOutput {

    #[inline] pub fn serialize<Stream: StreamItems>(&self, s: &mut Stream)  {

        // Write the redeem script
        if !self.redeem_script.empty() {

            serialize_to_vector(s, &PSBT_OUT_REDEEMSCRIPT);

            s.stream(self.redeem_script.clone());
        }

        // Write the witness script
        if !self.witness_script.empty() {

            serialize_to_vector(s, &PSBT_OUT_WITNESSSCRIPT);

            s.stream(self.witness_script.clone());
        }
    
        // Write any hd keypaths
        serialize_hd_keypaths(
            s, 
            &self.hd_keypaths, 
            PSBT_OUT_BIP32_DERIVATION
        );

        // Write unknown things
        for entry in self.unknown.iter() {

            s.stream(entry.0);
            s.stream(entry.1);
        }

        s.stream(PSBT_SEPARATOR);
    }
    
    #[inline] pub fn unserialize<Stream: StreamInto + ExactSizeIterator>(&mut self, 
        s: &mut Stream) -> Result<(), StdException>  {
    
        //  Used for duplicate key detection
        let mut key_lookup = HashSet::<Vec::<u8>>::default();;

        //  Read loop
        let mut found_sep: bool = false;;

        while s.len() == 0 {

            //  Read
            let mut key = Vec::<u8>::default();;

            s.stream_into(&mut key);

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

                PSBT_OUT_REDEEMSCRIPT  => {

                    if !key_lookup.insert(key.clone()) {

                        return Err(ios_base_failure("Duplicate Key, output redeemScript already provided"));

                    } else if key.len() != 1 {

                        return Err(ios_base_failure("Output redeemScript key is more than one byte type"));
                    }

                    s.stream_into(&mut self.redeem_script);

                    break;
                },

                PSBT_OUT_WITNESSSCRIPT  => {

                    if !key_lookup.insert(key.clone()) {

                        return Err(ios_base_failure("Duplicate Key, output witnessScript already provided"));

                    } else if key.len() != 1 {

                        return Err(ios_base_failure("Output witnessScript key is more than one byte type"));
                    }

                    s.stream_into(&mut self.witness_script);

                    break;
                },

                PSBT_OUT_BIP32_DERIVATION  => {
                    deserialize_hd_keypaths(s, &key, &mut self.hd_keypaths);
                    break;
                },

                _  => {

                    if self.unknown.contains_key(&key) {
                        return Err(ios_base_failure("Duplicate Key, key for unknown value already provided"));
                    }

                    //  Read in the value
                    let mut val_bytes = Vec::<u8>::default();;

                    s.stream_into(&mut val_bytes);

                    self.unknown.insert(key, val_bytes);

                    break;
                },

            }
        }

        if !found_sep {
            return Err(ios_base_failure("Separator is missing at the end of an output map"));
        }

        Ok(())
    }
    
    pub fn new<Stream: ExactSizeIterator + StreamInto>(
        _0: DeserializeType,
        s:  &mut Stream) -> Self {
    
        let mut x = Self::default();

        x.unserialize(s);

        x
    }

    pub fn fill_signature_data(&self, sigdata: &mut SignatureData)  {

        if !self.redeem_script.empty() {
            sigdata.redeem_script = self.redeem_script.clone();
        }

        if !self.witness_script.empty() {
            sigdata.witness_script = self.witness_script.clone();
        }

        for key_pair in self.hd_keypaths.iter() {

            sigdata.misc_pubkeys.insert(
                key_pair.0.getid(), 
                (key_pair.0.clone(),key_pair.1.clone())
            );
        }
    }
    
    pub fn from_signature_data(&mut self, sigdata: &SignatureData)  {
        
        if self.redeem_script.empty() && !sigdata.redeem_script.empty() {
            self.redeem_script = sigdata.redeem_script.clone();
        }

        if self.witness_script.empty() && !sigdata.witness_script.empty() {
            self.witness_script = sigdata.witness_script.clone();
        }

        for entry in sigdata.misc_pubkeys.iter() {

            self.hd_keypaths.insert(
                entry.1.0.clone(), 
                Default::default()
            );
        }
    }
    
    pub fn is_null(&self) -> bool {
        
        self.redeem_script.empty() 
        && self.witness_script.empty() 
        && self.hd_keypaths.is_empty() 
        && self.unknown.is_empty()
    }
    
    pub fn merge(&mut self, output: &PSBTOutput)  {

        for (k,v) in output.hd_keypaths.iter() {
            self.hd_keypaths.insert(k.clone(), v.clone());
        }

        for (k,v) in output.unknown.iter() {
            self.unknown.insert(k.to_vec(),v.to_vec());
        }

        if self.redeem_script.empty() && !output.redeem_script.empty() {
            self.redeem_script = output.redeem_script.clone();
        }

        if self.witness_script.empty() && !output.witness_script.empty() {
            self.witness_script = output.witness_script.clone();
        }
    }
}
