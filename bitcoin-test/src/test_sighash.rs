// ---------------- [ File: bitcoin-test/src/test_sighash.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/sighash_tests.cpp]

pub fn read_json(jsondata: &String) -> UniValue {
    
    todo!();
        /*
        
        */
}

/**
  | Old script.cpp SignatureHash function
  |
  */
pub fn signature_hash_old(
        script_code: Script,
        tx_to:       &Transaction,
        n_in:        u32,
        n_hash_type: i32) -> u256 {
    
    todo!();
        /*
            if (nIn >= txTo.vin.size())
        {
            return uint256::ONE;
        }
        CMutableTransaction txTmp(txTo);

        // In case concatenating two scripts ends up with two codeseparators,
        // or an extra one at the end, this prevents all those possible incompatibilities.
        FindAndDelete(scriptCode, CScript(OP_CODESEPARATOR));

        // Blank out other inputs' signatures
        for (unsigned int i = 0; i < txTmp.vin.size(); i++)
            txTmp.vin[i].scriptSig = CScript();
        txTmp.vin[nIn].scriptSig = scriptCode;

        // Blank out some of the outputs
        if ((nHashType & 0x1f) == SIGHASH_NONE)
        {
            // Wildcard payee
            txTmp.vout.clear();

            // Let the others update at will
            for (unsigned int i = 0; i < txTmp.vin.size(); i++)
                if (i != nIn)
                    txTmp.vin[i].nSequence = 0;
        }
        else if ((nHashType & 0x1f) == SIGHASH_SINGLE)
        {
            // Only lock-in the txout payee at same index as txin
            unsigned int nOut = nIn;
            if (nOut >= txTmp.vout.size())
            {
                return uint256::ONE;
            }
            txTmp.vout.resize(nOut+1);
            for (unsigned int i = 0; i < nOut; i++)
                txTmp.vout[i].SetNull();

            // Let the others update at will
            for (unsigned int i = 0; i < txTmp.vin.size(); i++)
                if (i != nIn)
                    txTmp.vin[i].nSequence = 0;
        }

        // Blank out other inputs completely, not recommended for open transactions
        if (nHashType & SIGHASH_ANYONECANPAY)
        {
            txTmp.vin[0] = txTmp.vin[nIn];
            txTmp.vin.resize(1);
        }

        // Serialize and hash
        CHashWriter ss(SER_GETHASH, SERIALIZE_TRANSACTION_NO_WITNESS);
        ss << txTmp << nHashType;
        return ss.GetHash();
        */
}

pub fn random_script(script: &mut Script)  {
    
    todo!();
        /*
            static const opcodetype oplist[] = {OP_FALSE, OP_1, OP_2, OP_3, OP_CHECKSIG, OP_IF, OP_VERIF, OP_RETURN, OP_CODESEPARATOR};
        script = CScript();
        int ops = (InsecureRandRange(10));
        for (int i=0; i<ops; i++)
            script << oplist[InsecureRandRange(std::size(oplist))];
        */
}

pub fn random_transaction(
        tx:     &mut MutableTransaction,
        single: bool)  {
    
    todo!();
        /*
            tx.nVersion = InsecureRand32();
        tx.vin.clear();
        tx.vout.clear();
        tx.nLockTime = (InsecureRandBool()) ? InsecureRand32() : 0;
        int ins = (InsecureRandBits(2)) + 1;
        int outs = fSingle ? ins : (InsecureRandBits(2)) + 1;
        for (int in = 0; in < ins; in++) {
            tx.vin.push_back(CTxIn());
            CTxIn &txin = tx.vin.back();
            txin.prevout.hash = InsecureRand256();
            txin.prevout.n = InsecureRandBits(2);
            RandomScript(txin.scriptSig);
            txin.nSequence = (InsecureRandBool()) ? InsecureRand32() : std::numeric_limits<uint32_t>::max();
        }
        for (int out = 0; out < outs; out++) {
            tx.vout.push_back(CTxOut());
            CTxOut &txout = tx.vout.back();
            txout.nValue = InsecureRandRange(100000000);
            RandomScript(txout.scriptPubKey);
        }
        */
}

#[cfg(test)]
#[fixture(BasicTestingSetup)]
pub mod sighash_tests {

    #[test] fn sighash_test() {
        todo!();
        /*
        
            #if defined(PRINT_SIGHASH_JSON)
            std::cout << "[\n";
            std::cout << "\t[\"raw_transaction, script, input_index, hashType, signature_hash (result)\"],\n";
            int nRandomTests = 500;
            #else
            int nRandomTests = 50000;
            #endif
            for (int i=0; i<nRandomTests; i++) {
                int nHashType = InsecureRand32();
                CMutableTransaction txTo;
                RandomTransaction(txTo, (nHashType & 0x1f) == SIGHASH_SINGLE);
                CScript scriptCode;
                RandomScript(scriptCode);
                int nIn = InsecureRandRange(txTo.vin.size());

                uint256 sh, sho;
                sho = SignatureHashOld(scriptCode, CTransaction(txTo), nIn, nHashType);
                sh = SignatureHash(scriptCode, txTo, nIn, nHashType, 0, SigVersion::BASE);
                #if defined(PRINT_SIGHASH_JSON)
                DataStream ss(SER_NETWORK, PROTOCOL_VERSION);
                ss << txTo;

                std::cout << "\t[\"" ;
                std::cout << HexStr(ss) << "\", \"";
                std::cout << HexStr(scriptCode) << "\", ";
                std::cout << nIn << ", ";
                std::cout << nHashType << ", \"";
                std::cout << sho.GetHex() << "\"]";
                if (i+1 != nRandomTests) {
                  std::cout << ",";
                }
                std::cout << "\n";
                #endif
                BOOST_CHECK(sh == sho);
            }
            #if defined(PRINT_SIGHASH_JSON)
            std::cout << "]\n";
            #endif

        */
    }

    /**
      | Goal: check that SignatureHash generates
      | correct hash
      |
      */
    #[test] fn sighash_from_data() {
        todo!();
        /*
        
            UniValue tests = read_json(std::string(json_tests::sighash, json_tests::sighash + sizeof(json_tests::sighash)));

            for (unsigned int idx = 0; idx < tests.size(); idx++) {
                UniValue test = tests[idx];
                std::string strTest = test.write();
                if (test.size() < 1) // Allow for extra stuff (useful for comments)
                {
                    BOOST_ERROR("Bad test: " << strTest);
                    continue;
                }
                if (test.size() == 1) continue; // comment

                std::string raw_tx, raw_script, sigHashHex;
                int nIn, nHashType;
                uint256 sh;
                CTransactionRef tx;
                CScript scriptCode = CScript();

                try {
                  // deserialize test data
                  raw_tx = test[0].get_str();
                  raw_script = test[1].get_str();
                  nIn = test[2].get_int();
                  nHashType = test[3].get_int();
                  sigHashHex = test[4].get_str();

                  DataStream stream(ParseHex(raw_tx), SER_NETWORK, PROTOCOL_VERSION);
                  stream >> tx;

                  TxValidationState state;
                  BOOST_CHECK_MESSAGE(CheckTransaction(*tx, state), strTest);
                  BOOST_CHECK(state.IsValid());

                  std::vector<unsigned char> raw = ParseHex(raw_script);
                  scriptCode.insert(scriptCode.end(), raw.begin(), raw.end());
                } catch (...) {
                  BOOST_ERROR("Bad test, couldn't deserialize data: " << strTest);
                  continue;
                }

                sh = SignatureHash(scriptCode, *tx, nIn, nHashType, 0, SigVersion::BASE);
                BOOST_CHECK_MESSAGE(sh.GetHex() == sigHashHex, strTest);
            }

        */
    }
}
