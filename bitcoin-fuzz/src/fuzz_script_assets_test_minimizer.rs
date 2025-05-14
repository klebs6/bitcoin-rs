// ---------------- [ File: bitcoin-fuzz/src/fuzz_script_assets_test_minimizer.rs ]
/*!
  | This fuzz "test" can be used to minimize test
  | cases for script_assets_test in
  | src/test/script_tests.cpp. While it written as
  | a fuzz test, and can be used as such, fuzzing
  | the inputs is unlikely to construct useful test
  | cases.
  |
  | Instead, it is primarily intended to be run on
  | a test set that was generated externally, for
  | example using
  | test/functional/feature_taproot.py's
  | --dumptests mode.  The minimized set can then
  | be concatenated together, surrounded by '[' and
  | ']', and used as the script_assets_test.json
  | input to the script_assets_test unit test:
  |
  | (normal build)
  |
  | $ mkdir dump
  |
  | $ for N in $(seq 1 10); do TEST_DUMP_DIR=dump
  | test/functional/feature_taproot.py --dumptests;
  | done
  |
  | $ ...
  |
  | (libFuzzer build)
  | $ mkdir dump-min
  |
  | $ FUZZ=script_assets_test_minimizer
  | ./src/test/fuzz/fuzz -merge=1
  | -use_value_profile=1 dump-min/ dump/
  |
  | $ (echo -en '[\n'; cat dump-min/\* | head -c -2;
  | echo -en '\n]') >script_assets_test.json
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/script_assets_test_minimizer.cpp]

pub fn checked_parse_hex(str_: &String) -> Vec<u8> {
    
    todo!();
        /*
            if (str.size() && !IsHex(str)) throw std::runtime_error("Non-hex input '" + str + "'");
        return ParseHex(str);
        */
}

pub fn script_from_hex(str_: &String) -> Script {
    
    todo!();
        /*
            std::vector<unsigned char> data = CheckedParseHex(str);
        return CScript(data.begin(), data.end());
        */
}

pub fn tx_from_hex(str_: &String) -> MutableTransaction {
    
    todo!();
        /*
            CMutableTransaction tx;
        try {
            VectorReader(SER_DISK, SERIALIZE_TRANSACTION_NO_WITNESS, CheckedParseHex(str), 0) >> tx;
        } catch (const std::ios_base::failure&) {
            throw std::runtime_error("Tx deserialization failure");
        }
        return tx;
        */
}

pub fn tx_outs_fromjson(univalue: &UniValue) -> Vec<TxOut> {
    
    todo!();
        /*
            if (!univalue.isArray()) throw std::runtime_error("Prevouts must be array");
        std::vector<CTxOut> prevouts;
        for (size_t i = 0; i < univalue.size(); ++i) {
            CTxOut txout;
            try {
                VectorReader(SER_DISK, 0, CheckedParseHex(univalue[i].get_str()), 0) >> txout;
            } catch (const std::ios_base::failure&) {
                throw std::runtime_error("Prevout invalid format");
            }
            prevouts.push_back(std::move(txout));
        }
        return prevouts;
        */
}

pub fn script_witness_fromjson(univalue: &UniValue) -> ScriptWitness {
    
    todo!();
        /*
            if (!univalue.isArray()) throw std::runtime_error("Script witness is not array");
        CScriptWitness scriptwitness;
        for (size_t i = 0; i < univalue.size(); ++i) {
            auto bytes = CheckedParseHex(univalue[i].get_str());
            scriptwitness.stack.push_back(std::move(bytes));
        }
        return scriptwitness;
        */
}

lazy_static!{
    /*
    const std::map<std::string, unsigned int> FLAG_NAMES = {
        {std::string("P2SH"), (unsigned int)SCRIPT_VERIFY_P2SH},
        {std::string("DERSIG"), (unsigned int)SCRIPT_VERIFY_DERSIG},
        {std::string("NULLDUMMY"), (unsigned int)SCRIPT_VERIFY_NULLDUMMY},
        {std::string("CHECKLOCKTIMEVERIFY"), (unsigned int)SCRIPT_VERIFY_CHECKLOCKTIMEVERIFY},
        {std::string("CHECKSEQUENCEVERIFY"), (unsigned int)SCRIPT_VERIFY_CHECKSEQUENCEVERIFY},
        {std::string("WITNESS"), (unsigned int)SCRIPT_VERIFY_WITNESS},
        {std::string("TAPROOT"), (unsigned int)SCRIPT_VERIFY_TAPROOT},
    };
    */
}

pub fn all_flags() -> Vec<u32> {
    
    todo!();
        /*
            std::vector<unsigned int> ret;

        for (unsigned int i = 0; i < 128; ++i) {
            unsigned int flag = 0;
            if (i & 1) flag |= SCRIPT_VERIFY_P2SH;
            if (i & 2) flag |= SCRIPT_VERIFY_DERSIG;
            if (i & 4) flag |= SCRIPT_VERIFY_NULLDUMMY;
            if (i & 8) flag |= SCRIPT_VERIFY_CHECKLOCKTIMEVERIFY;
            if (i & 16) flag |= SCRIPT_VERIFY_CHECKSEQUENCEVERIFY;
            if (i & 32) flag |= SCRIPT_VERIFY_WITNESS;
            if (i & 64) flag |= SCRIPT_VERIFY_TAPROOT;

            // SCRIPT_VERIFY_WITNESS requires SCRIPT_VERIFY_P2SH
            if (flag & SCRIPT_VERIFY_WITNESS && !(flag & SCRIPT_VERIFY_P2SH)) continue;
            // SCRIPT_VERIFY_TAPROOT requires SCRIPT_VERIFY_WITNESS
            if (flag & SCRIPT_VERIFY_TAPROOT && !(flag & SCRIPT_VERIFY_WITNESS)) continue;

            ret.push_back(flag);
        }

        return ret;
        */
}

lazy_static!{
    static ref ALL_FLAGS: Vec<u32> = all_flags();
}

pub fn parse_script_flags(str_: &String) -> u32 {
    
    todo!();
        /*
            if (str.empty()) return 0;

        unsigned int flags = 0;
        std::vector<std::string> words;
        boost::algorithm::split(words, str, boost::algorithm::is_any_of(","));

        for (const std::string& word : words) {
            auto it = FLAG_NAMES.find(word);
            if (it == FLAG_NAMES.end()) throw std::runtime_error("Unknown verification flag " + word);
            flags |= it->second;
        }

        return flags;
        */
}

pub fn test(str_: &String)  {
    
    todo!();
        /*
            UniValue test;
        if (!test.read(str) || !test.isObject()) throw std::runtime_error("Non-object test input");

        CMutableTransaction tx = TxFromHex(test["tx"].get_str());
        const std::vector<CTxOut> prevouts = TxOutsFromJSON(test["prevouts"]);
        if (prevouts.size() != tx.vin.size()) throw std::runtime_error("Incorrect number of prevouts");
        size_t idx = test["index"].get_int64();
        if (idx >= tx.vin.size()) throw std::runtime_error("Invalid index");
        unsigned int test_flags = ParseScriptFlags(test["flags"].get_str());
        bool final = test.exists("final") && test["final"].get_bool();

        if (test.exists("success")) {
            tx.vin[idx].scriptSig = ScriptFromHex(test["success"]["scriptSig"].get_str());
            tx.vin[idx].scriptWitness = ScriptWitnessFromJSON(test["success"]["witness"]);
            PrecomputedTransactionData txdata;
            txdata.Init(tx, std::vector<CTxOut>(prevouts));
            MutableTransactionSignatureChecker txcheck(&tx, idx, prevouts[idx].nValue, txdata, MissingDataBehavior::ASSERT_FAIL);
            for (const auto flags : ALL_FLAGS) {
                // "final": true tests are valid for all flags. Others are only valid with flags that are
                // a subset of test_flags.
                if (final || ((flags & test_flags) == flags)) {
                    (c_void)VerifyScript(tx.vin[idx].scriptSig, prevouts[idx].scriptPubKey, &tx.vin[idx].scriptWitness, flags, txcheck, nullptr);
                }
            }
        }

        if (test.exists("failure")) {
            tx.vin[idx].scriptSig = ScriptFromHex(test["failure"]["scriptSig"].get_str());
            tx.vin[idx].scriptWitness = ScriptWitnessFromJSON(test["failure"]["witness"]);
            PrecomputedTransactionData txdata;
            txdata.Init(tx, std::vector<CTxOut>(prevouts));
            MutableTransactionSignatureChecker txcheck(&tx, idx, prevouts[idx].nValue, txdata, MissingDataBehavior::ASSERT_FAIL);
            for (const auto flags : ALL_FLAGS) {
                // If a test is supposed to fail with test_flags, it should also fail with any superset thereof.
                if ((flags & test_flags) == test_flags) {
                    (c_void)VerifyScript(tx.vin[idx].scriptSig, prevouts[idx].scriptPubKey, &tx.vin[idx].scriptWitness, flags, txcheck, nullptr);
                }
            }
        }
        */
}

pub fn test_init()  {
    
    todo!();
        /*
            static ECCVerifyHandle handle;
        */
}


#[fuzz_test(initializer = "test_init", hidden = true)]
fn script_assets_test_minimizer() {
    todo!();
    /*
    
        if (buffer.size() < 2 || buffer.back() != '\n' || buffer[buffer.size() - 2] != ',') return;
        const std::string str((const char*)buffer.data(), buffer.size() - 2);
        try {
            Test(str);
        } catch (const std::runtime_error&) {
        }

    */
}
