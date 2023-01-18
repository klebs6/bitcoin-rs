crate::ix!();

/**
  | Wrapper that serializes like CTransaction,
  | but with the modifications required
  | for the signature hash done in-place
  |
  */
pub struct TransactionSignatureSerializer<T> {

    /**
      | reference to the spending transaction
      | (the one being serialized)
      |
      */
    tx_to:          Rc<T>,

    /**
      | output script being consumed
      |
      */
    script_code:    Rc<Script>,

    /**
      | input index of txTo being signed
      |
      */
    n_in:           u32,

    /**
      | whether the hashtype has the
      | SIGHASH_ANYONECANPAY flag set
      |
      */
    anyone_can_pay: bool,

    /**
      | whether the hashtype is SIGHASH_SINGLE
      |
      */
    hash_single:    bool,

    /**
      | whether the hashtype is SIGHASH_NONE
      |
      */
    hash_none:      bool,
}

impl<T> TransactionSignatureSerializer<T> {
    
    pub fn new(
        tx_to_in:       &T,
        script_code_in: &Script,
        n_in_in:        u32,
        n_hash_type_in: i32) -> Self {
    
        todo!();
        /*


            :
            txTo(txToIn), scriptCode(scriptCodeIn), nIn(nInIn),
            fAnyoneCanPay(!!(nHashTypeIn & SIGHASH_ANYONECANPAY)),
            fHashSingle((nHashTypeIn & 0x1f) == SIGHASH_SINGLE),
            fHashNone((nHashTypeIn & 0x1f) == SIGHASH_NONE)
        */
    }

    /**
      | Serialize the passed scriptCode, skipping
      | OP_CODESEPARATORs
      |
      */
    pub fn serialize_script_code<S>(&self, s: &mut S)  {
    
        todo!();
        /*
            Script::const_iterator it = scriptCode.begin();
            Script::const_iterator itBegin = it;
            opcodetype opcode;
            unsigned int nCodeSeparators = 0;
            while (scriptCode.GetOp(it, opcode)) {
                if (opcode == OP_CODESEPARATOR)
                    nCodeSeparators++;
            }
            ::WriteCompactSize(s, scriptCode.size() - nCodeSeparators);
            it = itBegin;
            while (scriptCode.GetOp(it, opcode)) {
                if (opcode == OP_CODESEPARATOR) {
                    s.write((char*)&itBegin[0], it-itBegin-1);
                    itBegin = it;
                }
            }
            if (itBegin != scriptCode.end())
                s.write((char*)&itBegin[0], it-itBegin);
        */
    }

    /**
      | Serialize an input of txTo
      |
      */
    pub fn serialize_input<S>(&self, 
        s:       &mut S,
        n_input: u32)  {
    
        todo!();
        /*
            // In case of SIGHASH_ANYONECANPAY, only the input being signed is serialized
            if (fAnyoneCanPay)
                nInput = nIn;
            // Serialize the prevout
            ::Serialize(s, txTo.vin[nInput].prevout);
            // Serialize the script
            if (nInput != nIn)
                // Blank out other inputs' signatures
                ::Serialize(s, Script());
            else
                SerializeScriptCode(s);
            // Serialize the nSequence
            if (nInput != nIn && (fHashSingle || fHashNone))
                // let the others update at will
                ::Serialize(s, (int)0);
            else
                ::Serialize(s, txTo.vin[nInput].nSequence);
        */
    }

    /**
      | Serialize an output of txTo
      |
      */
    pub fn serialize_output<S>(&self, 
        s:        &mut S,
        n_output: u32)  {
    
        todo!();
        /*
            if (fHashSingle && nOutput != nIn)
                // Do not lock-in the txout payee at other indices as txin
                ::Serialize(s, CTxOut());
            else
                ::Serialize(s, txTo.vout[nOutput]);
        */
    }

    /**
      | Serialize txTo
      |
      */
    pub fn serialize<S>(&self, s: &mut S)  {
    
        todo!();
        /*
            // Serialize nVersion
            ::Serialize(s, txTo.nVersion);
            // Serialize vin
            unsigned int nInputs = fAnyoneCanPay ? 1 : txTo.vin.size();
            ::WriteCompactSize(s, nInputs);
            for (unsigned int nInput = 0; nInput < nInputs; nInput++)
                 SerializeInput(s, nInput);
            // Serialize vout
            unsigned int nOutputs = fHashNone ? 0 : (fHashSingle ? nIn+1 : txTo.vout.size());
            ::WriteCompactSize(s, nOutputs);
            for (unsigned int nOutput = 0; nOutput < nOutputs; nOutput++)
                 SerializeOutput(s, nOutput);
            // Serialize nLockTime
            ::Serialize(s, txTo.nLockTime);
        */
    }
}

