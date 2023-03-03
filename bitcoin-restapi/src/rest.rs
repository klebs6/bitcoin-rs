crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/rest.cpp]

pub const MAX_GETUTXOS_OUTPOINTS: usize = 15; // allow a max of 15 outpoints to be queried at once

pub enum RetFormat {
    UNDEF,
    BINARY,
    HEX,
    JSON,
}

pub struct RfName {
    rf:   RetFormat,
    name: &'static str,
} 

pub const RF_NAMES: &[RfName] = &[
    RfName {rf: RetFormat::UNDEF,  name: ""},
    RfName {rf: RetFormat::BINARY, name: "bin"},
    RfName {rf: RetFormat::HEX,    name: "hex"},
    RfName {rf: RetFormat::JSON,   name: "json"},
];

pub struct Coin {
    pub n_height: u32,
    pub out:      TxOut,
}

impl Default for Coin {
    
    fn default() -> Self {
        todo!();
        /*
        : n_height(0),

        
        */
    }
}

lazy_static!{
    /*
    SERIALIZE_METHODS(CCoin, obj)
        {
            uint32_t nTxVerDummy = 0;
            READWRITE(nTxVerDummy, obj.nHeight, obj.out);
        }
    */
}

pub fn resterr(
    req:     *mut HTTPRequest,
    status:  HTTPStatusCode,
    message: String) -> bool {
    
    todo!();
        /*
        req->WriteHeader("Content-Type", "text/plain");
        req->WriteReply(status, message + "\r\n");
        return false;
        */
}

/**
  | Get the node context.
  | 
  | -----------
  | @param[in] req
  | 
  | The HTTP request, whose status code
  | will be set if node context is not found.
  | 
  | -----------
  | @return
  | 
  | Pointer to the node context or nullptr
  | if not found.
  |
  */
pub fn get_node_context(
    context: &dyn Any,
    req:     *mut HTTPRequest) -> *mut NodeContext {
    
    todo!();
        /*
        auto node_context = util::AnyPtr<NodeContext>(context);
        if (!node_context) {
            RESTERR(req, HTTP_INTERNAL_SERVER_ERROR,
                    strprintf("%s:%d (%s)\n"
                              "Internal bug detected: Node context not found!\n"
                              "You may report this issue here: %s\n",
                              __FILE__, __LINE__, __func__, PACKAGE_BUGREPORT));
            return nullptr;
        }
        return node_context;
        */
}

/**
  | Get the node context mempool.
  | 
  | -----------
  | @param[in] req
  | 
  | The HTTP request, whose status code
  | will be set if node context mempool is
  | not found.
  | 
  | -----------
  | @return
  | 
  | Pointer to the mempool or nullptr if
  | no mempool found.
  |
  */
pub fn get_mem_pool(
        context: &dyn Any,
        req:     *mut HTTPRequest) -> *mut TxMemPool {
    
    todo!();
        /*
        auto node_context = util::AnyPtr<NodeContext>(context);
        if (!node_context || !node_context->mempool) {
            RESTERR(req, HTTP_NOT_FOUND, "Mempool disabled or instance not found");
            return nullptr;
        }
        return node_context->mempool.get();
        */
}

/**
  | Get the node context chainstatemanager.
  | 
  | -----------
  | @param[in] req
  | 
  | The HTTP request, whose status code
  | will be set if node context chainstatemanager
  | is not found.
  | 
  | -----------
  | @return
  | 
  | Pointer to the chainstatemanager or
  | nullptr if none found.
  |
  */
pub fn get_chainman(
    context: &dyn Any,
    req:     *mut HTTPRequest) -> *mut ChainstateManager {
    
    todo!();
        /*
        auto node_context = util::AnyPtr<NodeContext>(context);
        if (!node_context || !node_context->chainman) {
            RESTERR(req, HTTP_INTERNAL_SERVER_ERROR,
                    strprintf("%s:%d (%s)\n"
                              "Internal bug detected: Chainman disabled or instance not found!\n"
                              "You may report this issue here: %s\n",
                              __FILE__, __LINE__, __func__, PACKAGE_BUGREPORT));
            return nullptr;
        }
        return node_context->chainman.get();
        */
}

pub fn parse_data_format(
        param:   &mut String,
        str_req: &String) -> RetFormat {
    
    todo!();
        /*
        const std::string::size_type pos = strReq.rfind('.');
        if (pos == std::string::npos)
        {
            param = strReq;
            return rf_names[0].rf;
        }

        param = strReq.substr(0, pos);
        const std::string suff(strReq, pos + 1);

        for (const auto& rf_name : rf_names) {
            if (suff == rf_name.name)
                return rf_name.rf;
        }

        /* If no suffix is found, return original string.  */
        param = strReq;
        return rf_names[0].rf;
        */
}

pub fn available_data_formats_string() -> String {
    
    todo!();
        /*
        std::string formats;
        for (const auto& rf_name : rf_names) {
            if (strlen(rf_name.name) > 0) {
                formats.append(".");
                formats.append(rf_name.name);
                formats.append(", ");
            }
        }

        if (formats.length() > 0)
            return formats.substr(0, formats.length() - 2);

        return formats;
        */
}

pub fn check_warmup(req: *mut HTTPRequest) -> bool {
    
    todo!();
        /*
        std::string statusmessage;
        if (RPCIsInWarmup(&statusmessage))
             return RESTERR(req, HTTP_SERVICE_UNAVAILABLE, "Service temporarily unavailable: " + statusmessage);
        return true;
        */
}

pub fn rest_headers(
        context:      &dyn Any,
        req:          *mut HTTPRequest,
        str_uri_part: &String) -> bool {
    
    todo!();
        /*
        if (!CheckWarmup(req))
            return false;
        std::string param;
        const RetFormat rf = ParseDataFormat(param, strURIPart);
        std::vector<std::string> path;
        boost::split(path, param, boost::is_any_of("/"));

        if (path.size() != 2)
            return RESTERR(req, HTTP_BAD_REQUEST, "No header count specified. Use /rest/headers/<count>/<hash>.<ext>.");

        const auto parsed_count{ToIntegral<size_t>(path[0])};
        if (!parsed_count.has_value() || *parsed_count < 1 || *parsed_count > 2000) {
            return RESTERR(req, HTTP_BAD_REQUEST, "Header count out of range: " + path[0]);
        }

        std::string hashStr = path[1];
        uint256 hash;
        if (!ParseHashStr(hashStr, hash))
            return RESTERR(req, HTTP_BAD_REQUEST, "Invalid hash: " + hashStr);

        const CBlockIndex* tip = nullptr;
        std::vector<const CBlockIndex*> headers;
        headers.reserve(*parsed_count);
        {
            ChainstateManager* maybe_chainman = GetChainman(context, req);
            if (!maybe_chainman) return false;
            ChainstateManager& chainman = *maybe_chainman;
            LOCK(cs_main);
            CChain& active_chain = chainman.ActiveChain();
            tip = active_chain.Tip();
            const CBlockIndex* pindex = chainman.m_blockman.LookupBlockIndex(hash);
            while (pindex != nullptr && active_chain.Contains(pindex)) {
                headers.push_back(pindex);
                if (headers.size() == *parsed_count) {
                    break;
                }
                pindex = active_chain.Next(pindex);
            }
        }

        switch (rf) {
        case RetFormat::BINARY: {
            DataStream ssHeader(SER_NETWORK, PROTOCOL_VERSION);
            for (const CBlockIndex *pindex : headers) {
                ssHeader << pindex->GetBlockHeader();
            }

            std::string binaryHeader = ssHeader.str();
            req->WriteHeader("Content-Type", "application/octet-stream");
            req->WriteReply(HTTP_OK, binaryHeader);
            return true;
        }

        case RetFormat::HEX: {
            DataStream ssHeader(SER_NETWORK, PROTOCOL_VERSION);
            for (const CBlockIndex *pindex : headers) {
                ssHeader << pindex->GetBlockHeader();
            }

            std::string strHex = HexStr(ssHeader) + "\n";
            req->WriteHeader("Content-Type", "text/plain");
            req->WriteReply(HTTP_OK, strHex);
            return true;
        }
        case RetFormat::JSON: {
            UniValue jsonHeaders(UniValue::VARR);
            for (const CBlockIndex *pindex : headers) {
                jsonHeaders.push_back(blockheaderToJSON(tip, pindex));
            }
            std::string strJSON = jsonHeaders.write() + "\n";
            req->WriteHeader("Content-Type", "application/json");
            req->WriteReply(HTTP_OK, strJSON);
            return true;
        }
        default: {
            return RESTERR(req, HTTP_NOT_FOUND, "output format not found (available: .bin, .hex, .json)");
        }
        }
        */
}

pub fn rest_block(
        context:      &dyn Any,
        req:          *mut HTTPRequest,
        str_uri_part: &String,
        tx_verbosity: TxVerbosity) -> bool {
    
    todo!();
        /*
        if (!CheckWarmup(req))
            return false;
        std::string hashStr;
        const RetFormat rf = ParseDataFormat(hashStr, strURIPart);

        uint256 hash;
        if (!ParseHashStr(hashStr, hash))
            return RESTERR(req, HTTP_BAD_REQUEST, "Invalid hash: " + hashStr);

        CBlock block;
        CBlockIndex* pblockindex = nullptr;
        CBlockIndex* tip = nullptr;
        {
            ChainstateManager* maybe_chainman = GetChainman(context, req);
            if (!maybe_chainman) return false;
            ChainstateManager& chainman = *maybe_chainman;
            LOCK(cs_main);
            tip = chainman.ActiveChain().Tip();
            pblockindex = chainman.m_blockman.LookupBlockIndex(hash);
            if (!pblockindex) {
                return RESTERR(req, HTTP_NOT_FOUND, hashStr + " not found");
            }

            if (IsBlockPruned(pblockindex))
                return RESTERR(req, HTTP_NOT_FOUND, hashStr + " not available (pruned data)");

            if (!ReadBlockFromDisk(block, pblockindex, Params().GetConsensus()))
                return RESTERR(req, HTTP_NOT_FOUND, hashStr + " not found");
        }

        switch (rf) {
        case RetFormat::BINARY: {
            DataStream ssBlock(SER_NETWORK, PROTOCOL_VERSION | RPCSerializationFlags());
            ssBlock << block;
            std::string binaryBlock = ssBlock.str();
            req->WriteHeader("Content-Type", "application/octet-stream");
            req->WriteReply(HTTP_OK, binaryBlock);
            return true;
        }

        case RetFormat::HEX: {
            DataStream ssBlock(SER_NETWORK, PROTOCOL_VERSION | RPCSerializationFlags());
            ssBlock << block;
            std::string strHex = HexStr(ssBlock) + "\n";
            req->WriteHeader("Content-Type", "text/plain");
            req->WriteReply(HTTP_OK, strHex);
            return true;
        }

        case RetFormat::JSON: {
            UniValue objBlock = blockToJSON(block, tip, pblockindex, tx_verbosity);
            std::string strJSON = objBlock.write() + "\n";
            req->WriteHeader("Content-Type", "application/json");
            req->WriteReply(HTTP_OK, strJSON);
            return true;
        }

        default: {
            return RESTERR(req, HTTP_NOT_FOUND, "output format not found (available: " + AvailableDataFormatsString() + ")");
        }
        }
        */
}

pub fn rest_block_extended(
        context:      &dyn Any,
        req:          *mut HTTPRequest,
        str_uri_part: &String) -> bool {
    
    todo!();
        /*
            return rest_block(context, req, strURIPart, TxVerbosity::SHOW_DETAILS_AND_PREVOUT);
        */
}

pub fn rest_block_notxdetails(
        context:      &dyn Any,
        req:          *mut HTTPRequest,
        str_uri_part: &String) -> bool {
    
    todo!();
        /*
            return rest_block(context, req, strURIPart, TxVerbosity::SHOW_TXID);
        */
}

/**
  | A bit of a hack - dependency on a function
  | defined in rpc/blockchain.cpp
  |
  */
pub fn getblockchaininfo() -> RPCHelpMan {
    
    todo!();
        /*
        
        */
}

pub fn rest_chaininfo(
        context:      &dyn Any,
        req:          *mut HTTPRequest,
        str_uri_part: &String) -> bool {
    
    todo!();
        /*
        if (!CheckWarmup(req))
            return false;
        std::string param;
        const RetFormat rf = ParseDataFormat(param, strURIPart);

        switch (rf) {
        case RetFormat::JSON: {
            JSONRPCRequest jsonRequest;
            jsonRequest.context = context;
            jsonRequest.params = UniValue(UniValue::VARR);
            UniValue chainInfoObject = getblockchaininfo().HandleRequest(jsonRequest);
            std::string strJSON = chainInfoObject.write() + "\n";
            req->WriteHeader("Content-Type", "application/json");
            req->WriteReply(HTTP_OK, strJSON);
            return true;
        }
        default: {
            return RESTERR(req, HTTP_NOT_FOUND, "output format not found (available: json)");
        }
        }
        */
}

pub fn rest_mempool_info(
        context:      &dyn Any,
        req:          *mut HTTPRequest,
        str_uri_part: &String) -> bool {
    
    todo!();
        /*
        if (!CheckWarmup(req))
            return false;
        const CTxMemPool* mempool = GetMemPool(context, req);
        if (!mempool) return false;
        std::string param;
        const RetFormat rf = ParseDataFormat(param, strURIPart);

        switch (rf) {
        case RetFormat::JSON: {
            UniValue mempoolInfoObject = MempoolInfoToJSON(*mempool);

            std::string strJSON = mempoolInfoObject.write() + "\n";
            req->WriteHeader("Content-Type", "application/json");
            req->WriteReply(HTTP_OK, strJSON);
            return true;
        }
        default: {
            return RESTERR(req, HTTP_NOT_FOUND, "output format not found (available: json)");
        }
        }
        */
}

pub fn rest_mempool_contents(
        context:      &dyn Any,
        req:          *mut HTTPRequest,
        str_uri_part: &String) -> bool {
    
    todo!();
        /*
        if (!CheckWarmup(req)) return false;
        const CTxMemPool* mempool = GetMemPool(context, req);
        if (!mempool) return false;
        std::string param;
        const RetFormat rf = ParseDataFormat(param, strURIPart);

        switch (rf) {
        case RetFormat::JSON: {
            UniValue mempoolObject = MempoolToJSON(*mempool, true);

            std::string strJSON = mempoolObject.write() + "\n";
            req->WriteHeader("Content-Type", "application/json");
            req->WriteReply(HTTP_OK, strJSON);
            return true;
        }
        default: {
            return RESTERR(req, HTTP_NOT_FOUND, "output format not found (available: json)");
        }
        }
        */
}

pub fn rest_tx(
        context:      &dyn Any,
        req:          *mut HTTPRequest,
        str_uri_part: &String) -> bool {
    
    todo!();
        /*
        if (!CheckWarmup(req))
            return false;
        std::string hashStr;
        const RetFormat rf = ParseDataFormat(hashStr, strURIPart);

        uint256 hash;
        if (!ParseHashStr(hashStr, hash))
            return RESTERR(req, HTTP_BAD_REQUEST, "Invalid hash: " + hashStr);

        if (g_txindex) {
            g_txindex->BlockUntilSyncedToCurrentChain();
        }

        const NodeContext* const node = GetNodeContext(context, req);
        if (!node) return false;
        uint256 hashBlock = uint256();
        const CTransactionRef tx = GetTransaction(/* block_index */ nullptr, node->mempool.get(), hash, Params().GetConsensus(), hashBlock);
        if (!tx) {
            return RESTERR(req, HTTP_NOT_FOUND, hashStr + " not found");
        }

        switch (rf) {
        case RetFormat::BINARY: {
            DataStream ssTx(SER_NETWORK, PROTOCOL_VERSION | RPCSerializationFlags());
            ssTx << tx;

            std::string binaryTx = ssTx.str();
            req->WriteHeader("Content-Type", "application/octet-stream");
            req->WriteReply(HTTP_OK, binaryTx);
            return true;
        }

        case RetFormat::HEX: {
            DataStream ssTx(SER_NETWORK, PROTOCOL_VERSION | RPCSerializationFlags());
            ssTx << tx;

            std::string strHex = HexStr(ssTx) + "\n";
            req->WriteHeader("Content-Type", "text/plain");
            req->WriteReply(HTTP_OK, strHex);
            return true;
        }

        case RetFormat::JSON: {
            UniValue objTx(UniValue::VOBJ);
            TxToUniv(*tx, hashBlock, objTx);
            std::string strJSON = objTx.write() + "\n";
            req->WriteHeader("Content-Type", "application/json");
            req->WriteReply(HTTP_OK, strJSON);
            return true;
        }

        default: {
            return RESTERR(req, HTTP_NOT_FOUND, "output format not found (available: " + AvailableDataFormatsString() + ")");
        }
        }
        */
}

pub fn rest_getutxos(
        context:      &dyn Any,
        req:          *mut HTTPRequest,
        str_uri_part: &String) -> bool {
    
    todo!();
        /*
        if (!CheckWarmup(req))
            return false;
        std::string param;
        const RetFormat rf = ParseDataFormat(param, strURIPart);

        std::vector<std::string> uriParts;
        if (param.length() > 1)
        {
            std::string strUriParams = param.substr(1);
            boost::split(uriParts, strUriParams, boost::is_any_of("/"));
        }

        // throw exception in case of an empty request
        std::string strRequestMutable = req->ReadBody();
        if (strRequestMutable.length() == 0 && uriParts.size() == 0)
            return RESTERR(req, HTTP_BAD_REQUEST, "Error: empty request");

        bool fInputParsed = false;
        bool fCheckMemPool = false;
        std::vector<OutPoint> vOutPoints;

        // parse/deserialize input
        // input-format = output-format, rest/getutxos/bin requires binary input, gives binary output, ...

        if (uriParts.size() > 0)
        {
            //inputs is sent over URI scheme (/rest/getutxos/checkmempool/txid1-n/txid2-n/...)
            if (uriParts[0] == "checkmempool") fCheckMemPool = true;

            for (size_t i = (fCheckMemPool) ? 1 : 0; i < uriParts.size(); i++)
            {
                uint256 txid;
                int32_t nOutput;
                std::string strTxid = uriParts[i].substr(0, uriParts[i].find('-'));
                std::string strOutput = uriParts[i].substr(uriParts[i].find('-')+1);

                if (!ParseInt32(strOutput, &nOutput) || !IsHex(strTxid))
                    return RESTERR(req, HTTP_BAD_REQUEST, "Parse error");

                txid.SetHex(strTxid);
                vOutPoints.push_back(OutPoint(txid, (uint32_t)nOutput));
            }

            if (vOutPoints.size() > 0)
                fInputParsed = true;
            else
                return RESTERR(req, HTTP_BAD_REQUEST, "Error: empty request");
        }

        switch (rf) {
        case RetFormat::HEX: {
            // convert hex to bin, continue then with bin part
            std::vector<unsigned char> strRequestV = ParseHex(strRequestMutable);
            strRequestMutable.assign(strRequestV.begin(), strRequestV.end());
            [[fallthrough]];
        }

        case RetFormat::BINARY: {
            try {
                //deserialize only if user sent a request
                if (strRequestMutable.size() > 0)
                {
                    if (fInputParsed) //don't allow sending input over URI and HTTP RAW DATA
                        return RESTERR(req, HTTP_BAD_REQUEST, "Combination of URI scheme inputs and raw post data is not allowed");

                    DataStream oss(SER_NETWORK, PROTOCOL_VERSION);
                    oss << strRequestMutable;
                    oss >> fCheckMemPool;
                    oss >> vOutPoints;
                }
            } catch (const std::ios_base::failure&) {
                // abort in case of unreadable binary data
                return RESTERR(req, HTTP_BAD_REQUEST, "Parse error");
            }
            break;
        }

        case RetFormat::JSON: {
            if (!fInputParsed)
                return RESTERR(req, HTTP_BAD_REQUEST, "Error: empty request");
            break;
        }
        default: {
            return RESTERR(req, HTTP_NOT_FOUND, "output format not found (available: " + AvailableDataFormatsString() + ")");
        }
        }

        // limit max outpoints
        if (vOutPoints.size() > MAX_GETUTXOS_OUTPOINTS)
            return RESTERR(req, HTTP_BAD_REQUEST, strprintf("Error: max outpoints exceeded (max: %d, tried: %d)", MAX_GETUTXOS_OUTPOINTS, vOutPoints.size()));

        // check spentness and form a bitmap (as well as a JSON capable human-readable string representation)
        std::vector<unsigned char> bitmap;
        std::vector<CCoin> outs;
        std::string bitmapStringRepresentation;
        std::vector<bool> hits;
        bitmap.resize((vOutPoints.size() + 7) / 8);
        ChainstateManager* maybe_chainman = GetChainman(context, req);
        if (!maybe_chainman) return false;
        ChainstateManager& chainman = *maybe_chainman;
        {
            auto process_utxos = [&vOutPoints, &outs, &hits](const CCoinsView& view, const CTxMemPool& mempool) {
                for (const OutPoint& vOutPoint : vOutPoints) {
                    Coin coin;
                    bool hit = !mempool.isSpent(vOutPoint) && view.GetCoin(vOutPoint, coin);
                    hits.push_back(hit);
                    if (hit) outs.emplace_back(std::move(coin));
                }
            };

            if (fCheckMemPool) {
                const CTxMemPool* mempool = GetMemPool(context, req);
                if (!mempool) return false;
                // use db+mempool as cache backend in case user likes to query mempool
                LOCK2(cs_main, mempool->cs);
                CCoinsViewCache& viewChain = chainman.ActiveChainstate().CoinsTip();
                CCoinsViewMemPool viewMempool(&viewChain, *mempool);
                process_utxos(viewMempool, *mempool);
            } else {
                LOCK(cs_main);  // no need to lock mempool!
                process_utxos(chainman.ActiveChainstate().CoinsTip(), CTxMemPool());
            }

            for (size_t i = 0; i < hits.size(); ++i) {
                const bool hit = hits[i];
                bitmapStringRepresentation.append(hit ? "1" : "0"); // form a binary string representation (human-readable for json output)
                bitmap[i / 8] |= ((uint8_t)hit) << (i % 8);
            }
        }

        switch (rf) {
        case RetFormat::BINARY: {
            // serialize data
            // use exact same output as mentioned in Bip64
            DataStream ssGetUTXOResponse(SER_NETWORK, PROTOCOL_VERSION);
            ssGetUTXOResponse << chainman.ActiveChain().Height() << chainman.ActiveChain().Tip()->GetBlockHash() << bitmap << outs;
            std::string ssGetUTXOResponseString = ssGetUTXOResponse.str();

            req->WriteHeader("Content-Type", "application/octet-stream");
            req->WriteReply(HTTP_OK, ssGetUTXOResponseString);
            return true;
        }

        case RetFormat::HEX: {
            DataStream ssGetUTXOResponse(SER_NETWORK, PROTOCOL_VERSION);
            ssGetUTXOResponse << chainman.ActiveChain().Height() << chainman.ActiveChain().Tip()->GetBlockHash() << bitmap << outs;
            std::string strHex = HexStr(ssGetUTXOResponse) + "\n";

            req->WriteHeader("Content-Type", "text/plain");
            req->WriteReply(HTTP_OK, strHex);
            return true;
        }

        case RetFormat::JSON: {
            UniValue objGetUTXOResponse(UniValue::VOBJ);

            // pack in some essentials
            // use more or less the same output as mentioned in Bip64
            objGetUTXOResponse.pushKV("chainHeight", chainman.ActiveChain().Height());
            objGetUTXOResponse.pushKV("chaintipHash", chainman.ActiveChain().Tip()->GetBlockHash().GetHex());
            objGetUTXOResponse.pushKV("bitmap", bitmapStringRepresentation);

            UniValue utxos(UniValue::VARR);
            for (const CCoin& coin : outs) {
                UniValue utxo(UniValue::VOBJ);
                utxo.pushKV("height", (int32_t)coin.nHeight);
                utxo.pushKV("value", ValueFromAmount(coin.out.nValue));

                // include the script in a json output
                UniValue o(UniValue::VOBJ);
                ScriptPubKeyToUniv(coin.out.scriptPubKey, o, true);
                utxo.pushKV("scriptPubKey", o);
                utxos.push_back(utxo);
            }
            objGetUTXOResponse.pushKV("utxos", utxos);

            // return json string
            std::string strJSON = objGetUTXOResponse.write() + "\n";
            req->WriteHeader("Content-Type", "application/json");
            req->WriteReply(HTTP_OK, strJSON);
            return true;
        }
        default: {
            return RESTERR(req, HTTP_NOT_FOUND, "output format not found (available: " + AvailableDataFormatsString() + ")");
        }
        }
        */
}

pub fn rest_blockhash_by_height(
        context:      &dyn Any,
        req:          *mut HTTPRequest,
        str_uri_part: &String) -> bool {
    
    todo!();
        /*
        if (!CheckWarmup(req)) return false;
        std::string height_str;
        const RetFormat rf = ParseDataFormat(height_str, str_uri_part);

        int32_t blockheight = -1; // Initialization done only to prevent valgrind false positive, see https://github.com/bitcoin/bitcoin/pull/18785
        if (!ParseInt32(height_str, &blockheight) || blockheight < 0) {
            return RESTERR(req, HTTP_BAD_REQUEST, "Invalid height: " + SanitizeString(height_str));
        }

        CBlockIndex* pblockindex = nullptr;
        {
            ChainstateManager* maybe_chainman = GetChainman(context, req);
            if (!maybe_chainman) return false;
            ChainstateManager& chainman = *maybe_chainman;
            LOCK(cs_main);
            const CChain& active_chain = chainman.ActiveChain();
            if (blockheight > active_chain.Height()) {
                return RESTERR(req, HTTP_NOT_FOUND, "Block height out of range");
            }
            pblockindex = active_chain[blockheight];
        }
        switch (rf) {
        case RetFormat::BINARY: {
            DataStream ss_blockhash(SER_NETWORK, PROTOCOL_VERSION);
            ss_blockhash << pblockindex->GetBlockHash();
            req->WriteHeader("Content-Type", "application/octet-stream");
            req->WriteReply(HTTP_OK, ss_blockhash.str());
            return true;
        }
        case RetFormat::HEX: {
            req->WriteHeader("Content-Type", "text/plain");
            req->WriteReply(HTTP_OK, pblockindex->GetBlockHash().GetHex() + "\n");
            return true;
        }
        case RetFormat::JSON: {
            req->WriteHeader("Content-Type", "application/json");
            UniValue resp = UniValue(UniValue::VOBJ);
            resp.pushKV("blockhash", pblockindex->GetBlockHash().GetHex());
            req->WriteReply(HTTP_OK, resp.write() + "\n");
            return true;
        }
        default: {
            return RESTERR(req, HTTP_NOT_FOUND, "output format not found (available: " + AvailableDataFormatsString() + ")");
        }
        }
        */
}

pub struct UriPrefix {
    prefix:  &'static str,

    handler: fn(
            context: &dyn Any,
            req:     *mut HTTPRequest,
            str_req: &String
    ) -> bool,
} 

pub const uri_prefixes: &[UriPrefix] = &[
      UriPrefix {prefix: "/rest/tx/",                handler: rest_tx},
      UriPrefix {prefix: "/rest/block/notxdetails/", handler: rest_block_notxdetails},
      UriPrefix {prefix: "/rest/block/",             handler: rest_block_extended},
      UriPrefix {prefix: "/rest/chaininfo",          handler: rest_chaininfo},
      UriPrefix {prefix: "/rest/mempool/info",       handler: rest_mempool_info},
      UriPrefix {prefix: "/rest/mempool/contents",   handler: rest_mempool_contents},
      UriPrefix {prefix: "/rest/headers/",           handler: rest_headers},
      UriPrefix {prefix: "/rest/getutxos",           handler: rest_getutxos},
      UriPrefix {prefix: "/rest/blockhashbyheight/", handler: rest_blockhash_by_height},
];

pub fn startrest(context: &dyn Any)  {
    
    todo!();
        /*
        for (const auto& up : uri_prefixes) {
            auto handler = [context, up](HTTPRequest* req, const std::string& prefix) { return up.handler(context, req, prefix); };
            RegisterHTTPHandler(up.prefix, false, handler);
        }
        */
}

pub fn interruptrest()  {
    
    todo!();
        /*
        
        */
}

pub fn stoprest()  {
    
    todo!();
        /*
        for (const auto& up : uri_prefixes) {
            UnregisterHTTPHandler(up.prefix, false);
        }
        */
}

