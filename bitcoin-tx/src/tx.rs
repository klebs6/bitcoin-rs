// ---------------- [ File: bitcoin-tx/src/tx.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bitcoin-tx.cpp]

lazy_static!{
    /*
    static bool fCreateBlank;
    static std::map<std::string,UniValue> registers;
    */
}

pub const CONTINUE_EXECUTION: i32 = -1;

//pub const G_TRANSLATION_FUN: Option<fn(_0: *const u8) -> String> = None;

/**
  | This function returns either one of EXIT_ codes
  | when it's expected to stop the process or
  | CONTINUE_EXECUTION when it's expected to
  | continue further.
  |
  */
pub fn app_init_raw_tx(
        argc: i32,
        argv: &[*mut u8]) -> i32 {
    
    todo!();
        /*
        SetupBitcoinTxArgs(gArgs);
        std::string error;
        if (!gArgs.ParseParameters(argc, argv, error)) {
            tfm::format(std::cerr, "Error parsing command line arguments: %s\n", error);
            return EXIT_FAILURE;
        }

        // Check for chain settings (Params() calls are only valid after this clause)
        try {
            SelectParams(gArgs.GetChainName());
        } catch (const std::exception& e) {
            tfm::format(std::cerr, "Error: %s\n", e.what());
            return EXIT_FAILURE;
        }

        fCreateBlank = gArgs.GetBoolArg("-create", false);

        if (argc < 2 || HelpRequested(gArgs) || gArgs.IsArgSet("-version")) {
            // First part of help message is specific to this utility
            std::string strUsage = PACKAGE_NAME " bitcoin-tx utility version " + FormatFullVersion() + "\n";
            if (!gArgs.IsArgSet("-version")) {
                strUsage += "\n"
                    "Usage:  bitcoin-tx [options] <hex-tx> [commands]  Update hex-encoded bitcoin transaction\n"
                    "or:     bitcoin-tx [options] -create [commands]   Create hex-encoded bitcoin transaction\n"
                    "\n";
                strUsage += gArgs.GetHelpMessage();
            }

            tfm::format(std::cout, "%s", strUsage);

            if (argc < 2) {
                tfm::format(std::cerr, "Error: too few parameters\n");
                return EXIT_FAILURE;
            }

            return EXIT_SUCCESS;
        }
        return CONTINUE_EXECUTION;
        */
}

pub fn register_set_json(
        key:      &String,
        raw_json: &String)  {
    
    todo!();
        /*
            UniValue val;
        if (!val.read(rawJson)) {
            std::string strErr = "Cannot parse JSON for key " + key;
            throw std::runtime_error(strErr);
        }

        registers[key] = val;
        */
}

pub fn register_set(str_input: &String)  {
    
    todo!();
        /*
            // separate NAME:VALUE in string
        size_t pos = strInput.find(':');
        if ((pos == std::string::npos) ||
            (pos == 0) ||
            (pos == (strInput.size() - 1)))
            throw std::runtime_error("Register input requires NAME:VALUE");

        std::string key = strInput.substr(0, pos);
        std::string valStr = strInput.substr(pos + 1, std::string::npos);

        RegisterSetJson(key, valStr);
        */
}

pub fn register_load(str_input: &String)  {
    
    todo!();
        /*
            // separate NAME:FILENAME in string
        size_t pos = strInput.find(':');
        if ((pos == std::string::npos) ||
            (pos == 0) ||
            (pos == (strInput.size() - 1)))
            throw std::runtime_error("Register load requires NAME:FILENAME");

        std::string key = strInput.substr(0, pos);
        std::string filename = strInput.substr(pos + 1, std::string::npos);

        FILE *f = fopen(filename.c_str(), "r");
        if (!f) {
            std::string strErr = "Cannot open file " + filename;
            throw std::runtime_error(strErr);
        }

        // load file chunks into one big buffer
        std::string valStr;
        while ((!feof(f)) && (!ferror(f))) {
            char buf[4096];
            int bread = fread(buf, 1, sizeof(buf), f);
            if (bread <= 0)
                break;

            valStr.insert(valStr.size(), buf, bread);
        }

        int error = ferror(f);
        fclose(f);

        if (error) {
            std::string strErr = "Error reading file " + filename;
            throw std::runtime_error(strErr);
        }

        // evaluate as JSON buffer register
        RegisterSetJson(key, valStr);
        */
}

pub fn extract_and_validate_value(str_value: &String) -> Amount {
    
    todo!();
        /*
            if (std::optional<CAmount> parsed = ParseMoney(strValue)) {
            return parsed.value();
        } else {
            throw std::runtime_error("invalid TX output value");
        }
        */
}

pub fn mutate_tx_version(
        tx:      &mut MutableTransaction,
        cmd_val: &String)  {
    
    todo!();
        /*
            int64_t newVersion;
        if (!ParseInt64(cmdVal, &newVersion) || newVersion < 1 || newVersion > TX_MAX_STANDARD_VERSION) {
            throw std::runtime_error("Invalid TX version requested: '" + cmdVal + "'");
        }

        tx.nVersion = (int) newVersion;
        */
}

pub fn mutate_tx_locktime(
        tx:      &mut MutableTransaction,
        cmd_val: &String)  {
    
    todo!();
        /*
            int64_t newLocktime;
        if (!ParseInt64(cmdVal, &newLocktime) || newLocktime < 0LL || newLocktime > 0xffffffffLL)
            throw std::runtime_error("Invalid TX locktime requested: '" + cmdVal + "'");

        tx.nLockTime = (unsigned int) newLocktime;
        */
}

pub fn mutate_tx_rbf_opt_in(
        tx:         &mut MutableTransaction,
        str_in_idx: &String)  {
    
    todo!();
        /*
            // parse requested index
        int64_t inIdx;
        if (!ParseInt64(strInIdx, &inIdx) || inIdx < 0 || inIdx >= static_cast<int64_t>(tx.vin.size())) {
            throw std::runtime_error("Invalid TX input index '" + strInIdx + "'");
        }

        // set the nSequence to MAX_INT - 2 (= RBF opt in flag)
        int cnt = 0;
        for (CTxIn& txin : tx.vin) {
            if (strInIdx == "" || cnt == inIdx) {
                if (txin.nSequence > MAX_BIP125_RBF_SEQUENCE) {
                    txin.nSequence = MAX_BIP125_RBF_SEQUENCE;
                }
            }
            ++cnt;
        }
        */
}

pub fn trim_and_parse<T>(
        int_str: &String,
        err:     &String) -> T {

    todo!();
        /*
            const auto parsed{ToIntegral<T>(TrimString(int_str))};
        if (!parsed.has_value()) {
            throw std::runtime_error(err + " '" + int_str + "'");
        }
        return parsed.value();
        */
}

pub fn mutate_tx_add_input(
        tx:        &mut MutableTransaction,
        str_input: &String)  {
    
    todo!();
        /*
            std::vector<std::string> vStrInputParts;
        boost::split(vStrInputParts, strInput, boost::is_any_of(":"));

        // separate TXID:VOUT in string
        if (vStrInputParts.size()<2)
            throw std::runtime_error("TX input missing separator");

        // extract and validate TXID
        uint256 txid;
        if (!ParseHashStr(vStrInputParts[0], txid)) {
            throw std::runtime_error("invalid TX input txid");
        }

        static const unsigned int minTxOutSz = 9;
        static const unsigned int maxVout = MAX_BLOCK_WEIGHT / (WITNESS_SCALE_FACTOR * minTxOutSz);

        // extract and validate vout
        const std::string& strVout = vStrInputParts[1];
        int64_t vout;
        if (!ParseInt64(strVout, &vout) || vout < 0 || vout > static_cast<int64_t>(maxVout))
            throw std::runtime_error("invalid TX input vout '" + strVout + "'");

        // extract the optional sequence number
        uint32_t nSequenceIn = CTxIn::SEQUENCE_FINAL;
        if (vStrInputParts.size() > 2) {
            nSequenceIn = TrimAndParse<uint32_t>(vStrInputParts.at(2), "invalid TX sequence id");
        }

        // append to transaction input list
        CTxIn txin(txid, vout, CScript(), nSequenceIn);
        tx.vin.push_back(txin);
        */
}

pub fn mutate_tx_add_out_addr(
        tx:        &mut MutableTransaction,
        str_input: &String)  {
    
    todo!();
        /*
            // Separate into VALUE:ADDRESS
        std::vector<std::string> vStrInputParts;
        boost::split(vStrInputParts, strInput, boost::is_any_of(":"));

        if (vStrInputParts.size() != 2)
            throw std::runtime_error("TX output missing or too many separators");

        // Extract and validate VALUE
        CAmount value = ExtractAndValidateValue(vStrInputParts[0]);

        // extract and validate ADDRESS
        std::string strAddr = vStrInputParts[1];
        TxDestination destination = DecodeDestination(strAddr);
        if (!IsValidDestination(destination)) {
            throw std::runtime_error("invalid TX output address");
        }
        CScript scriptPubKey = GetScriptForDestination(destination);

        // construct TxOut, append to transaction output list
        CTxOut txout(value, scriptPubKey);
        tx.vout.push_back(txout);
        */
}

pub fn mutate_tx_add_out_pub_key(
        tx:        &mut MutableTransaction,
        str_input: &String)  {
    
    todo!();
        /*
            // Separate into VALUE:PUBKEY[:FLAGS]
        std::vector<std::string> vStrInputParts;
        boost::split(vStrInputParts, strInput, boost::is_any_of(":"));

        if (vStrInputParts.size() < 2 || vStrInputParts.size() > 3)
            throw std::runtime_error("TX output missing or too many separators");

        // Extract and validate VALUE
        CAmount value = ExtractAndValidateValue(vStrInputParts[0]);

        // Extract and validate PUBKEY
        CPubKey pubkey(ParseHex(vStrInputParts[1]));
        if (!pubkey.IsFullyValid())
            throw std::runtime_error("invalid TX output pubkey");
        CScript scriptPubKey = GetScriptForRawPubKey(pubkey);

        // Extract and validate FLAGS
        bool bSegWit = false;
        bool bScriptHash = false;
        if (vStrInputParts.size() == 3) {
            std::string flags = vStrInputParts[2];
            bSegWit = (flags.find('W') != std::string::npos);
            bScriptHash = (flags.find('S') != std::string::npos);
        }

        if (bSegWit) {
            if (!pubkey.IsCompressed()) {
                throw std::runtime_error("Uncompressed pubkeys are not useable for SegWit outputs");
            }
            // Build a P2WPKH script
            scriptPubKey = GetScriptForDestination(WitnessV0KeyHash(pubkey));
        }
        if (bScriptHash) {
            // Get the ID for the script, and then construct a P2SH destination for it.
            scriptPubKey = GetScriptForDestination(ScriptHash(scriptPubKey));
        }

        // construct TxOut, append to transaction output list
        CTxOut txout(value, scriptPubKey);
        tx.vout.push_back(txout);
        */
}

pub fn mutate_tx_add_out_multi_sig(
        tx:        &mut MutableTransaction,
        str_input: &String)  {
    
    todo!();
        /*
            // Separate into VALUE:REQUIRED:NUMKEYS:PUBKEY1:PUBKEY2:....[:FLAGS]
        std::vector<std::string> vStrInputParts;
        boost::split(vStrInputParts, strInput, boost::is_any_of(":"));

        // Check that there are enough parameters
        if (vStrInputParts.size()<3)
            throw std::runtime_error("Not enough multisig parameters");

        // Extract and validate VALUE
        CAmount value = ExtractAndValidateValue(vStrInputParts[0]);

        // Extract REQUIRED
        const uint32_t required{TrimAndParse<uint32_t>(vStrInputParts.at(1), "invalid multisig required number")};

        // Extract NUMKEYS
        const uint32_t numkeys{TrimAndParse<uint32_t>(vStrInputParts.at(2), "invalid multisig total number")};

        // Validate there are the correct number of pubkeys
        if (vStrInputParts.size() < numkeys + 3)
            throw std::runtime_error("incorrect number of multisig pubkeys");

        if (required < 1 || required > MAX_PUBKEYS_PER_MULTISIG || numkeys < 1 || numkeys > MAX_PUBKEYS_PER_MULTISIG || numkeys < required)
            throw std::runtime_error("multisig parameter mismatch. Required " \
                                + ToString(required) + " of " + ToString(numkeys) + "signatures.");

        // extract and validate PUBKEYs
        std::vector<CPubKey> pubkeys;
        for(int pos = 1; pos <= int(numkeys); pos++) {
            CPubKey pubkey(ParseHex(vStrInputParts[pos + 2]));
            if (!pubkey.IsFullyValid())
                throw std::runtime_error("invalid TX output pubkey");
            pubkeys.push_back(pubkey);
        }

        // Extract FLAGS
        bool bSegWit = false;
        bool bScriptHash = false;
        if (vStrInputParts.size() == numkeys + 4) {
            std::string flags = vStrInputParts.back();
            bSegWit = (flags.find('W') != std::string::npos);
            bScriptHash = (flags.find('S') != std::string::npos);
        }
        else if (vStrInputParts.size() > numkeys + 4) {
            // Validate that there were no more parameters passed
            throw std::runtime_error("Too many parameters");
        }

        CScript scriptPubKey = GetScriptForMultisig(required, pubkeys);

        if (bSegWit) {
            for (const CPubKey& pubkey : pubkeys) {
                if (!pubkey.IsCompressed()) {
                    throw std::runtime_error("Uncompressed pubkeys are not useable for SegWit outputs");
                }
            }
            // Build a P2WSH with the multisig script
            scriptPubKey = GetScriptForDestination(WitnessV0ScriptHash(scriptPubKey));
        }
        if (bScriptHash) {
            if (scriptPubKey.size() > MAX_SCRIPT_ELEMENT_SIZE) {
                throw std::runtime_error(strprintf(
                            "redeemScript exceeds size limit: %d > %d", scriptPubKey.size(), MAX_SCRIPT_ELEMENT_SIZE));
            }
            // Get the ID for the script, and then construct a P2SH destination for it.
            scriptPubKey = GetScriptForDestination(ScriptHash(scriptPubKey));
        }

        // construct TxOut, append to transaction output list
        CTxOut txout(value, scriptPubKey);
        tx.vout.push_back(txout);
        */
}

pub fn mutate_tx_add_out_data(
        tx:        &mut MutableTransaction,
        str_input: &String)  {
    
    todo!();
        /*
            CAmount value = 0;

        // separate [VALUE:]DATA in string
        size_t pos = strInput.find(':');

        if (pos==0)
            throw std::runtime_error("TX output value not specified");

        if (pos != std::string::npos) {
            // Extract and validate VALUE
            value = ExtractAndValidateValue(strInput.substr(0, pos));
        }

        // extract and validate DATA
        std::string strData = strInput.substr(pos + 1, std::string::npos);

        if (!IsHex(strData))
            throw std::runtime_error("invalid TX output data");

        std::vector<unsigned char> data = ParseHex(strData);

        CTxOut txout(value, CScript() << OP_RETURN << data);
        tx.vout.push_back(txout);
        */
}

pub fn mutate_tx_add_out_script(
        tx:        &mut MutableTransaction,
        str_input: &String)  {
    
    todo!();
        /*
            // separate VALUE:SCRIPT[:FLAGS]
        std::vector<std::string> vStrInputParts;
        boost::split(vStrInputParts, strInput, boost::is_any_of(":"));
        if (vStrInputParts.size() < 2)
            throw std::runtime_error("TX output missing separator");

        // Extract and validate VALUE
        CAmount value = ExtractAndValidateValue(vStrInputParts[0]);

        // extract and validate script
        std::string strScript = vStrInputParts[1];
        CScript scriptPubKey = ParseScript(strScript);

        // Extract FLAGS
        bool bSegWit = false;
        bool bScriptHash = false;
        if (vStrInputParts.size() == 3) {
            std::string flags = vStrInputParts.back();
            bSegWit = (flags.find('W') != std::string::npos);
            bScriptHash = (flags.find('S') != std::string::npos);
        }

        if (scriptPubKey.size() > MAX_SCRIPT_SIZE) {
            throw std::runtime_error(strprintf(
                        "script exceeds size limit: %d > %d", scriptPubKey.size(), MAX_SCRIPT_SIZE));
        }

        if (bSegWit) {
            scriptPubKey = GetScriptForDestination(WitnessV0ScriptHash(scriptPubKey));
        }
        if (bScriptHash) {
            if (scriptPubKey.size() > MAX_SCRIPT_ELEMENT_SIZE) {
                throw std::runtime_error(strprintf(
                            "redeemScript exceeds size limit: %d > %d", scriptPubKey.size(), MAX_SCRIPT_ELEMENT_SIZE));
            }
            scriptPubKey = GetScriptForDestination(ScriptHash(scriptPubKey));
        }

        // construct TxOut, append to transaction output list
        CTxOut txout(value, scriptPubKey);
        tx.vout.push_back(txout);
        */
}

pub fn mutate_tx_del_input(
        tx:         &mut MutableTransaction,
        str_in_idx: &String)  {
    
    todo!();
        /*
            // parse requested deletion index
        int64_t inIdx;
        if (!ParseInt64(strInIdx, &inIdx) || inIdx < 0 || inIdx >= static_cast<int64_t>(tx.vin.size())) {
            throw std::runtime_error("Invalid TX input index '" + strInIdx + "'");
        }

        // delete input from transaction
        tx.vin.erase(tx.vin.begin() + inIdx);
        */
}

pub fn mutate_tx_del_output(
        tx:          &mut MutableTransaction,
        str_out_idx: &String)  {
    
    todo!();
        /*
            // parse requested deletion index
        int64_t outIdx;
        if (!ParseInt64(strOutIdx, &outIdx) || outIdx < 0 || outIdx >= static_cast<int64_t>(tx.vout.size())) {
            throw std::runtime_error("Invalid TX output index '" + strOutIdx + "'");
        }

        // delete output from transaction
        tx.vout.erase(tx.vout.begin() + outIdx);
        */
}

pub const N_SIGHASH_OPTS: usize = 7;

pub struct SigHashOption {
    flag_str: &'static str,
    flags:    usize,
} 

pub const sighash_options: [SigHashOption; N_SIGHASH_OPTS] = [
    SigHashOption {flag_str: "DEFAULT",             flags: SIGHASH_DEFAULT},
    SigHashOption {flag_str: "ALL",                 flags: SIGHASH_ALL},
    SigHashOption {flag_str: "NONE",                flags: SIGHASH_NONE},
    SigHashOption {flag_str: "SINGLE",              flags: SIGHASH_SINGLE},
    SigHashOption {flag_str: "ALL|ANYONECANPAY",    flags: SIGHASH_ALL|SIGHASH_ANYONECANPAY},
    SigHashOption {flag_str: "NONE|ANYONECANPAY",   flags: SIGHASH_NONE|SIGHASH_ANYONECANPAY},
    SigHashOption {flag_str: "SINGLE|ANYONECANPAY", flags: SIGHASH_SINGLE|SIGHASH_ANYONECANPAY},
];

pub fn find_sighash_flags(
        flags:    &mut i32,
        flag_str: &String) -> bool {
    
    todo!();
        /*
            flags = 0;

        for (unsigned int i = 0; i < N_SIGHASH_OPTS; i++) {
            if (flagStr == sighashOptions[i].flagStr) {
                flags = sighashOptions[i].flags;
                return true;
            }
        }

        return false;
        */
}

pub fn amount_from_value(value: &UniValue) -> Amount {
    
    todo!();
        /*
            if (!value.isNum() && !value.isStr())
            throw std::runtime_error("Amount is not a number or string");
        CAmount amount;
        if (!ParseFixedPoint(value.getValStr(), 8, &amount))
            throw std::runtime_error("Invalid amount");
        if (!MoneyRange(amount))
            throw std::runtime_error("Amount out of range");
        return amount;
        */
}

pub fn mutate_tx_sign(
        tx:       &mut MutableTransaction,
        flag_str: &String)  {
    
    todo!();
        /*
            int nHashType = SIGHASH_ALL;

        if (flagStr.size() > 0)
            if (!findSighashFlags(nHashType, flagStr))
                throw std::runtime_error("unknown sighash flag/sign option");

        // mergedTx will end up with all the signatures; it
        // starts as a clone of the raw tx:
        CMutableTransaction mergedTx{tx};
        const CMutableTransaction txv{tx};
        CCoinsView viewDummy;
        CCoinsViewCache view(&viewDummy);

        if (!registers.count("privatekeys"))
            throw std::runtime_error("privatekeys register variable must be set.");
        FillableSigningProvider tempKeystore;
        UniValue keysObj = registers["privatekeys"];

        for (unsigned int kidx = 0; kidx < keysObj.size(); kidx++) {
            if (!keysObj[kidx].isStr())
                throw std::runtime_error("privatekey not a std::string");
            CKey key = DecodeSecret(keysObj[kidx].getValStr());
            if (!key.IsValid()) {
                throw std::runtime_error("privatekey not valid");
            }
            tempKeystore.AddKey(key);
        }

        // Add previous txouts given in the RPC call:
        if (!registers.count("prevtxs"))
            throw std::runtime_error("prevtxs register variable must be set.");
        UniValue prevtxsObj = registers["prevtxs"];
        {
            for (unsigned int previdx = 0; previdx < prevtxsObj.size(); previdx++) {
                UniValue prevOut = prevtxsObj[previdx];
                if (!prevOut.isObject())
                    throw std::runtime_error("expected prevtxs internal object");

                std::map<std::string, UniValue::VType> types = {
                    {"txid", UniValue::VSTR},
                    {"vout", UniValue::VNUM},
                    {"scriptPubKey", UniValue::VSTR},
                };
                if (!prevOut.checkObject(types))
                    throw std::runtime_error("prevtxs internal object typecheck fail");

                uint256 txid;
                if (!ParseHashStr(prevOut["txid"].get_str(), txid)) {
                    throw std::runtime_error("txid must be hexadecimal string (not '" + prevOut["txid"].get_str() + "')");
                }

                const int nOut = prevOut["vout"].get_int();
                if (nOut < 0)
                    throw std::runtime_error("vout cannot be negative");

                OutPoint out(txid, nOut);
                std::vector<unsigned char> pkData(ParseHexUV(prevOut["scriptPubKey"], "scriptPubKey"));
                CScript scriptPubKey(pkData.begin(), pkData.end());

                {
                    const Coin& coin = view.AccessCoin(out);
                    if (!coin.IsSpent() && coin.out.scriptPubKey != scriptPubKey) {
                        std::string err("Previous output scriptPubKey mismatch:\n");
                        err = err + ScriptToAsmStr(coin.out.scriptPubKey) + "\nvs:\n"+
                            ScriptToAsmStr(scriptPubKey);
                        throw std::runtime_error(err);
                    }
                    Coin newcoin;
                    newcoin.out.scriptPubKey = scriptPubKey;
                    newcoin.out.nValue = 0;
                    if (prevOut.exists("amount")) {
                        newcoin.out.nValue = AmountFromValue(prevOut["amount"]);
                    }
                    newcoin.nHeight = 1;
                    view.AddCoin(out, std::move(newcoin), true);
                }

                // if redeemScript given and private keys given,
                // add redeemScript to the tempKeystore so it can be signed:
                if ((scriptPubKey.IsPayToScriptHash() || scriptPubKey.IsPayToWitnessScriptHash()) &&
                    prevOut.exists("redeemScript")) {
                    UniValue v = prevOut["redeemScript"];
                    std::vector<unsigned char> rsData(ParseHexUV(v, "redeemScript"));
                    CScript redeemScript(rsData.begin(), rsData.end());
                    tempKeystore.AddCScript(redeemScript);
                }
            }
        }

        const FillableSigningProvider& keystore = tempKeystore;

        bool fHashSingle = ((nHashType & ~SIGHASH_ANYONECANPAY) == SIGHASH_SINGLE);

        // Sign what we can:
        for (unsigned int i = 0; i < mergedTx.vin.size(); i++) {
            CTxIn& txin = mergedTx.vin[i];
            const Coin& coin = view.AccessCoin(txin.prevout);
            if (coin.IsSpent()) {
                continue;
            }
            const CScript& prevPubKey = coin.out.scriptPubKey;
            const CAmount& amount = coin.out.nValue;

            SignatureData sigdata = DataFromTransaction(mergedTx, i, coin.out);
            // Only sign SIGHASH_SINGLE if there's a corresponding output:
            if (!fHashSingle || (i < mergedTx.vout.size()))
                ProduceSignature(keystore, MutableTransactionSignatureCreator(&mergedTx, i, amount, nHashType), prevPubKey, sigdata);

            UpdateInput(txin, sigdata);
        }

        tx = mergedTx;
        */
}

pub struct Init {
    global_verify_handle: ECCVerifyHandle,
}

impl Default for Init {
    
    fn default() -> Self {
        todo!();
        /*


            ECC_Start();
        */
    }
}

impl Drop for Init {
    fn drop(&mut self) {
        todo!();
        /*
            ECC_Stop();
        */
    }
}

pub fn mutate_tx(
        tx:          &mut MutableTransaction,
        command:     &String,
        command_val: &String)  {
    
    todo!();
        /*
            std::unique_ptr<Init> ecc;

        if (command == "nversion")
            MutateTxVersion(tx, commandVal);
        else if (command == "locktime")
            MutateTxLocktime(tx, commandVal);
        else if (command == "replaceable") {
            MutateTxRBFOptIn(tx, commandVal);
        }

        else if (command == "delin")
            MutateTxDelInput(tx, commandVal);
        else if (command == "in")
            MutateTxAddInput(tx, commandVal);

        else if (command == "delout")
            MutateTxDelOutput(tx, commandVal);
        else if (command == "outaddr")
            MutateTxAddOutAddr(tx, commandVal);
        else if (command == "outpubkey") {
            ecc.reset(new Init());
            MutateTxAddOutPubKey(tx, commandVal);
        } else if (command == "outmultisig") {
            ecc.reset(new Init());
            MutateTxAddOutMultiSig(tx, commandVal);
        } else if (command == "outscript")
            MutateTxAddOutScript(tx, commandVal);
        else if (command == "outdata")
            MutateTxAddOutData(tx, commandVal);

        else if (command == "sign") {
            ecc.reset(new Init());
            MutateTxSign(tx, commandVal);
        }

        else if (command == "load")
            RegisterLoad(commandVal);

        else if (command == "set")
            RegisterSet(commandVal);

        else
            throw std::runtime_error("unknown command");
        */
}

pub fn output_txjson(tx: &Transaction)  {
    
    todo!();
        /*
            UniValue entry(UniValue::VOBJ);
        TxToUniv(tx, uint256(), entry);

        std::string jsonOutput = entry.write(4);
        tfm::format(std::cout, "%s\n", jsonOutput);
        */
}

pub fn output_tx_hash(tx: &Transaction)  {
    
    todo!();
        /*
            std::string strHexHash = tx.GetHash().GetHex(); // the hex-encoded transaction hash (aka the transaction id)

        tfm::format(std::cout, "%s\n", strHexHash);
        */
}

pub fn output_tx_hex(tx: &Transaction)  {
    
    todo!();
        /*
            std::string strHex = EncodeHexTx(tx);

        tfm::format(std::cout, "%s\n", strHex);
        */
}

pub fn output_tx(tx: &Transaction)  {
    
    todo!();
        /*
            if (gArgs.GetBoolArg("-json", false))
            OutputTxJSON(tx);
        else if (gArgs.GetBoolArg("-txid", false))
            OutputTxHash(tx);
        else
            OutputTxHex(tx);
        */
}

pub fn read_stdin() -> String {
    
    todo!();
        /*
            char buf[4096];
        std::string ret;

        while (!feof(stdin)) {
            size_t bread = fread(buf, 1, sizeof(buf), stdin);
            ret.append(buf, bread);
            if (bread < sizeof(buf))
                break;
        }

        if (ferror(stdin))
            throw std::runtime_error("error reading stdin");

        return TrimString(ret);
        */
}

pub fn command_line_raw_tx(
        argc: i32,
        argv: &[*mut u8]) -> i32 {
    
    todo!();
        /*
            std::string strPrint;
        int nRet = 0;
        try {
            // Skip switches; Permit common stdin convention "-"
            while (argc > 1 && IsSwitchChar(argv[1][0]) &&
                   (argv[1][1] != 0)) {
                argc--;
                argv++;
            }

            CMutableTransaction tx;
            int startArg;

            if (!fCreateBlank) {
                // require at least one param
                if (argc < 2)
                    throw std::runtime_error("too few parameters");

                // param: hex-encoded bitcoin transaction
                std::string strHexTx(argv[1]);
                if (strHexTx == "-")                 // "-" implies standard input
                    strHexTx = readStdin();

                if (!DecodeHexTx(tx, strHexTx, true))
                    throw std::runtime_error("invalid transaction encoding");

                startArg = 2;
            } else
                startArg = 1;

            for (int i = startArg; i < argc; i++) {
                std::string arg = argv[i];
                std::string key, value;
                size_t eqpos = arg.find('=');
                if (eqpos == std::string::npos)
                    key = arg;
                else {
                    key = arg.substr(0, eqpos);
                    value = arg.substr(eqpos + 1);
                }

                MutateTx(tx, key, value);
            }

            OutputTx(CTransaction(tx));
        }
        catch (const std::exception& e) {
            strPrint = std::string("error: ") + e.what();
            nRet = EXIT_FAILURE;
        }
        catch (...) {
            PrintExceptionContinue(nullptr, "CommandLineRawTx()");
            throw;
        }

        if (strPrint != "") {
            tfm::format(nRet == 0 ? std::cout : std::cerr, "%s\n", strPrint);
        }
        return nRet;
        */
}

pub fn tx_main(
        argc: i32,
        argv: &[*mut u8]) -> i32 {
    
    todo!();
        /*
            SetupEnvironment();

        try {
            int ret = AppInitRawTx(argc, argv);
            if (ret != CONTINUE_EXECUTION)
                return ret;
        }
        catch (const std::exception& e) {
            PrintExceptionContinue(&e, "AppInitRawTx()");
            return EXIT_FAILURE;
        } catch (...) {
            PrintExceptionContinue(nullptr, "AppInitRawTx()");
            return EXIT_FAILURE;
        }

        int ret = EXIT_FAILURE;
        try {
            ret = CommandLineRawTx(argc, argv);
        }
        catch (const std::exception& e) {
            PrintExceptionContinue(&e, "CommandLineRawTx()");
        } catch (...) {
            PrintExceptionContinue(nullptr, "CommandLineRawTx()");
        }
        return ret;
        */
}
