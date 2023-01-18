crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/psbt.h]

/**
  | A version of CTransaction with the PSBT
  | format
  |
  */
#[derive(Default)]
pub struct PartiallySignedTransaction {
    pub tx:      Option<MutableTransaction>,
    pub inputs:  Vec<PSBTInput>,
    pub outputs: Vec<PSBTOutput>,
    pub unknown: HashMap<Vec<u8>,Vec<u8>>,
}

//-------------------------------------------[.cpp/bitcoin/src/psbt.cpp]
impl From<MutableTransaction> for PartiallySignedTransaction {

    fn from(tx: MutableTransaction) -> Self {
    
        let mut x = Self::default();

        x.inputs.reserve(tx.vin.len());
        x.outputs.reserve(tx.vout.len());

        x.tx = Some(tx);

        x
    }
}
    
impl PartiallySignedTransaction {
    
    #[inline] pub fn serialize<Stream: 
    GetType 
    + GetVersion 
    + StreamItems
    >(&self, s: &mut Stream)  {
    
        // magic bytes
        s.stream(&PSBT_MAGIC_BYTES);

        // unsigned tx flag
        serialize_to_vector(s, &PSBT_GLOBAL_UNSIGNED_TX);

        // Write serialized tx to a stream

        let mut os: OverrideStream::<Stream> 
        = OverrideStream::<Stream>::new(
            s, 
            s.get_type(), 
            s.get_version() | SERIALIZE_TRANSACTION_NO_WITNESS
        );

        serialize_to_vector(&mut os, self.tx.as_ref().unwrap());

        // Write the unknown things
        for entry in self.unknown.iter() {
            s.stream(entry.0);
            s.stream(entry.1);
        }

        // Separator
        s.stream(PSBT_SEPARATOR);

        // Write inputs
        for input in self.inputs.iter() {
            s.stream(input);
        }

        // Write outputs
        for output in self.outputs.iter() {
            s.stream(output);
        }
    }
    
    #[inline] pub fn unserialize<Stream: GetType + GetVersion + StreamInto + ExactSizeIterator>(&mut self, 
        s: &mut Stream) -> Result<(), StdException>  
    {
        // Read the magic bytes
        let mut magic: [u8; 5] = [u8::default(); 5];

        s.stream_into(&mut magic);

        if magic[0..5] != PSBT_MAGIC_BYTES[0..5] {
            return Err(ios_base_failure("Invalid PSBT magic bytes"));
        }

        //  Used for duplicate key detection
        let mut key_lookup = HashSet::<Vec::<u8>>::default();;

        //  Read global data
        let mut found_sep: bool = false;;

        while s.len() == 0 {

            //  Read
            let mut key = Vec::<u8>::default();;

            s.stream_into(&mut key);

            //  the key is empty if that was
            //  actually a separator byte
            //
            //  This is a special case for key
            //  lengths 0 as those are not allowed
            //  (except for separator)
            if key.is_empty() {
                found_sep = true;
                break;
            }

            //  First byte of key is the type
            let ty: u8 = key[0];;

            //  Do stuff based on type
            match ty {

                PSBT_GLOBAL_UNSIGNED_TX  => {

                    if key_lookup.insert(key.clone()) {

                        return Err(ios_base_failure("Duplicate Key, unsigned tx already provided"));

                    } else if key.len() != 1 {

                        return Err(ios_base_failure("Global unsigned tx key is more than one byte type"));
                    }

                    let mut mtx = MutableTransaction::default();

                    //  Set the stream to
                    //  serialize with non-witness
                    //  since this should always
                    //  be non-witness
                    let mut os: OverrideStream::<Stream> 
                    = OverrideStream::<Stream>::new(
                        s, 
                        s.get_type(), 
                        s.get_version() | SERIALIZE_TRANSACTION_NO_WITNESS
                    );

                    unserialize_from_vector(&mut os, &mut mtx);

                    self.tx = Some(mtx);

                    //  Make sure that all
                    //  scriptSigs and
                    //  scriptWitnesses are empty
                    for txin in self.tx.as_ref().unwrap().vin.iter() {

                        if !txin.script_sig.empty() 
                        || !txin.script_witness.is_null() 
                        {
                            return Err(ios_base_failure("Unsigned tx does not have empty scriptSigs and scriptWitnesses."));
                        }
                    }

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
                },
            }
        }

        if !found_sep {
            return Err(ios_base_failure("Separator is missing at the end of the global map"));
        }

        //  Make sure that we got an unsigned tx
        if self.tx.is_none() {
            return Err(ios_base_failure("No unsigned transcation was provided"));
        }

        //  Read input data
        let mut i: usize = 0;;

        while s.len() == 0 && i < self.tx.as_ref().unwrap().vin.len() {

            let mut input = PSBTInput::default();

            s.stream_into(&mut input);

            self.inputs.push(input.clone());

            //  Make sure the non-witness utxo
            //  matches the outpoint
            if Arc::<Transaction>::strong_count(&input.non_witness_utxo) != 0
            && (*input.non_witness_utxo).get_hash() != &self.tx.as_ref().unwrap().vin[i].prevout.hash 
            {
                return Err(ios_base_failure("Non-witness UTXO does not match outpoint hash"));
            }

            i += 1;;
        }

        //  Make sure that the number of inputs
        //  matches the number of inputs in the
        //  transaction
        if self.inputs.len() != self.tx.as_ref().unwrap().vin.len() {
            return Err(ios_base_failure("Inputs provided does not match the number of inputs in transaction."));
        }

        //  Read output data
        i = 0;

        while s.len() == 0 && i < self.tx.as_ref().unwrap().vout.len(){

            let mut output = PSBTOutput::default();

            s.stream_into(&mut output);

            self.outputs.push(output);

            i += 1;;
        }

        //  Make sure that the number of outputs
        //  matches the number of outputs in the
        //  transaction
        if self.outputs.len() != self.tx.as_ref().unwrap().vout.len() {

            return Err(ios_base_failure("Outputs provided does not match the number of outputs in transaction."));
        }

        Ok(())
    }
    
    pub fn new<Stream: GetType + GetVersion + ExactSizeIterator + StreamInto>(
        _0: DeserializeType,
        s:  &mut Stream) -> Self {
    
        let mut x = Self::default();

        x.unserialize(s);

        x
    }

    pub fn is_null(&self) -> bool {
        
        self.tx.is_none() 
        && self.inputs.is_empty()
        && self.outputs.is_empty()
        && self.unknown.is_empty()
    }
    
    /**
      | Merge psbt into this. The two psbts must
      | have the same underlying CTransaction
      | (i.e. the same actual Bitcoin transaction.)
      | Returns true if the merge succeeded,
      | false otherwise.
      |
      */
    pub fn merge(&mut self, psbt: &PartiallySignedTransaction) -> bool {
        
        // Prohibited to merge two PSBTs over
        // different transactions
        if self.tx.as_ref().unwrap().get_hash() 
        != psbt.tx.as_ref().unwrap().get_hash() 
        {
            return false;
        }

        for i in 0..self.inputs.len() {
            self.inputs[i].merge(&psbt.inputs[i]);
        }

        for i in 0..self.outputs.len() {
            self.outputs[i].merge(&psbt.outputs[i]);
        }

        for (k,v) in psbt.unknown.iter() {
            self.unknown.insert(k.to_vec(),v.to_vec());
        }

        true
    }
    
    pub fn add_input(&mut self, 
        txin:   &TxIn,
        psbtin: &mut PSBTInput) -> bool {
        
        if let Some(ref mut tx) = self.tx {

            if tx.vin.iter().find(|&x| x == txin) != None
            {
                return false;
            }

            tx.vin.push((*txin).clone());
            psbtin.partial_sigs.clear();
            psbtin.final_script_sig.clear();
            psbtin.final_script_witness.set_null();
            self.inputs.push((*psbtin).clone());

            true

        } else {

            false
        }
    }
    
    pub fn add_output(&mut self, 
        txout:   &TxOut,
        psbtout: &PSBTOutput) -> bool {
        
        if let Some(ref mut tx) = self.tx {

            tx.vout.push(txout.clone());

            self.outputs.push((*psbtout).clone());

            true

        } else {

            false
        }
    }
    
    /**
      | Finds the UTXO for a given input index
      | 
      | -----------
      | @param[out] utxo
      | 
      | The UTXO of the input if found
      | ----------
      | @param[in] input_index
      | 
      | Index of the input to retrieve the UTXO
      | of
      | 
      | -----------
      | @return
      | 
      | Whether the UTXO for the specified input
      | was found
      |
      */
    pub fn get_inpututxo(&self, 
        utxo:        &mut TxOut,
        input_index: i32) -> bool {
        
        let input: &PSBTInput = &self.inputs[input_index as usize];

        if let Some(ref tx) = self.tx {

            let prevout_index: usize = tx.vin[input_index as usize].prevout.n.try_into().unwrap();

            if Arc::<Transaction>::strong_count(&input.non_witness_utxo) != 0 {

                if prevout_index >= (*input.non_witness_utxo).vout.len() {
                    return false;
                }

                if (*input.non_witness_utxo).get_hash() != &tx.vin[input_index as usize].prevout.hash {
                    return false;
                }

                *utxo = (*input.non_witness_utxo).vout[prevout_index].clone();

            } else if !input.witness_utxo.is_null() {

                *utxo = input.witness_utxo.clone();

            } else {

                return false;
            }

            true

        } else {

            false
        }
    }
}
