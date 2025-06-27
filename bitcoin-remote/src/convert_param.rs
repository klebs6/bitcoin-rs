// ---------------- [ File: bitcoin-remote/src/convert_param.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/rpc/client.h]
//-------------------------------------------[.cpp/bitcoin/src/rpc/client.cpp]

#[derive(Debug, Getters, MutGetters, Setters, Default, Builder)]
#[builder(pattern = "owned", setter(into), default)]
#[getset(get="pub")]
pub struct RPCConvertParam {

    /**
      | method whose params want conversion
      |
      */
    method_name: &'static str,

    /**
      | 0-based idx of param to convert
      |
      */
    param_idx:   i32,

    /**
      | parameter name
      |
      */
    param_name:  &'static str,
}

/**
  | Specify a (method, idx, name) here if
  | the argument is a non-string RPC argument
  | and needs to be converted from JSON.
  | 
  | -----------
  | @note
  | 
  | Parameter indexes start from 0.
  |
  */
pub const vRPCConvertParams: &[RPCConvertParam] = &[
    RPCConvertParam { method_name: "setmocktime"                  , param_idx:   0  , param_name:  "timestamp"             , } , 
    RPCConvertParam { method_name: "mockscheduler"                , param_idx:   0  , param_name:  "delta_time"            , } , 
    RPCConvertParam { method_name: "utxoupdatepsbt"               , param_idx:   1  , param_name:  "descriptors"           , } , 
    RPCConvertParam { method_name: "generatetoaddress"            , param_idx:   0  , param_name:  "nblocks"               , } , 
    RPCConvertParam { method_name: "generatetoaddress"            , param_idx:   2  , param_name:  "maxtries"              , } , 
    RPCConvertParam { method_name: "generatetodescriptor"         , param_idx:   0  , param_name:  "num_blocks"            , } , 
    RPCConvertParam { method_name: "generatetodescriptor"         , param_idx:   2  , param_name:  "maxtries"              , } , 
    RPCConvertParam { method_name: "generateblock"                , param_idx:   1  , param_name:  "transactions"          , } , 
    RPCConvertParam { method_name: "getnetworkhashps"             , param_idx:   0  , param_name:  "nblocks"               , } , 
    RPCConvertParam { method_name: "getnetworkhashps"             , param_idx:   1  , param_name:  "height"                , } , 
    RPCConvertParam { method_name: "sendtoaddress"                , param_idx:   1  , param_name:  "amount"                , } , 
    RPCConvertParam { method_name: "sendtoaddress"                , param_idx:   4  , param_name:  "subtractfeefromamount" , } , 
    RPCConvertParam { method_name: "sendtoaddress"                , param_idx:   5  , param_name:  "replaceable"           , } , 
    RPCConvertParam { method_name: "sendtoaddress"                , param_idx:   6  , param_name:  "conf_target"           , } , 
    RPCConvertParam { method_name: "sendtoaddress"                , param_idx:   8  , param_name:  "avoid_reuse"           , } , 
    RPCConvertParam { method_name: "sendtoaddress"                , param_idx:   9  , param_name:  "fee_rate"              , } , 
    RPCConvertParam { method_name: "sendtoaddress"                , param_idx:   10 , param_name:  "verbose"               , } , 
    RPCConvertParam { method_name: "settxfee"                     , param_idx:   0  , param_name:  "amount"                , } , 
    RPCConvertParam { method_name: "sethdseed"                    , param_idx:   0  , param_name:  "newkeypool"            , } , 
    RPCConvertParam { method_name: "getreceivedbyaddress"         , param_idx:   1  , param_name:  "minconf"               , } , 
    RPCConvertParam { method_name: "getreceivedbylabel"           , param_idx:   1  , param_name:  "minconf"               , } , 
    RPCConvertParam { method_name: "listreceivedbyaddress"        , param_idx:   0  , param_name:  "minconf"               , } , 
    RPCConvertParam { method_name: "listreceivedbyaddress"        , param_idx:   1  , param_name:  "include_empty"         , } , 
    RPCConvertParam { method_name: "listreceivedbyaddress"        , param_idx:   2  , param_name:  "include_watchonly"     , } , 
    RPCConvertParam { method_name: "listreceivedbylabel"          , param_idx:   0  , param_name:  "minconf"               , } , 
    RPCConvertParam { method_name: "listreceivedbylabel"          , param_idx:   1  , param_name:  "include_empty"         , } , 
    RPCConvertParam { method_name: "listreceivedbylabel"          , param_idx:   2  , param_name:  "include_watchonly"     , } , 
    RPCConvertParam { method_name: "getbalance"                   , param_idx:   1  , param_name:  "minconf"               , } , 
    RPCConvertParam { method_name: "getbalance"                   , param_idx:   2  , param_name:  "include_watchonly"     , } , 
    RPCConvertParam { method_name: "getbalance"                   , param_idx:   3  , param_name:  "avoid_reuse"           , } , 
    RPCConvertParam { method_name: "getblockhash"                 , param_idx:   0  , param_name:  "height"                , } , 
    RPCConvertParam { method_name: "waitforblockheight"           , param_idx:   0  , param_name:  "height"                , } , 
    RPCConvertParam { method_name: "waitforblockheight"           , param_idx:   1  , param_name:  "timeout"               , } , 
    RPCConvertParam { method_name: "waitforblock"                 , param_idx:   1  , param_name:  "timeout"               , } , 
    RPCConvertParam { method_name: "waitfornewblock"              , param_idx:   0  , param_name:  "timeout"               , } , 
    RPCConvertParam { method_name: "listtransactions"             , param_idx:   1  , param_name:  "count"                 , } , 
    RPCConvertParam { method_name: "listtransactions"             , param_idx:   2  , param_name:  "skip"                  , } , 
    RPCConvertParam { method_name: "listtransactions"             , param_idx:   3  , param_name:  "include_watchonly"     , } , 
    RPCConvertParam { method_name: "walletpassphrase"             , param_idx:   1  , param_name:  "timeout"               , } , 
    RPCConvertParam { method_name: "getblocktemplate"             , param_idx:   0  , param_name:  "template_request"      , } , 
    RPCConvertParam { method_name: "listsinceblock"               , param_idx:   1  , param_name:  "target_confirmations"  , } , 
    RPCConvertParam { method_name: "listsinceblock"               , param_idx:   2  , param_name:  "include_watchonly"     , } , 
    RPCConvertParam { method_name: "listsinceblock"               , param_idx:   3  , param_name:  "include_removed"       , } , 
    RPCConvertParam { method_name: "sendmany"                     , param_idx:   1  , param_name:  "amounts"               , } , 
    RPCConvertParam { method_name: "sendmany"                     , param_idx:   2  , param_name:  "minconf"               , } , 
    RPCConvertParam { method_name: "sendmany"                     , param_idx:   4  , param_name:  "subtractfeefrom"       , } , 
    RPCConvertParam { method_name: "sendmany"                     , param_idx:   5  , param_name:  "replaceable"           , } , 
    RPCConvertParam { method_name: "sendmany"                     , param_idx:   6  , param_name:  "conf_target"           , } , 
    RPCConvertParam { method_name: "sendmany"                     , param_idx:   8  , param_name:  "fee_rate"              , } , 
    RPCConvertParam { method_name: "sendmany"                     , param_idx:   9  , param_name:  "verbose"               , } , 
    RPCConvertParam { method_name: "deriveaddresses"              , param_idx:   1  , param_name:  "range"                 , } , 
    RPCConvertParam { method_name: "scantxoutset"                 , param_idx:   1  , param_name:  "scanobjects"           , } , 
    RPCConvertParam { method_name: "addmultisigaddress"           , param_idx:   0  , param_name:  "nrequired"             , } , 
    RPCConvertParam { method_name: "addmultisigaddress"           , param_idx:   1  , param_name:  "keys"                  , } , 
    RPCConvertParam { method_name: "createmultisig"               , param_idx:   0  , param_name:  "nrequired"             , } , 
    RPCConvertParam { method_name: "createmultisig"               , param_idx:   1  , param_name:  "keys"                  , } , 
    RPCConvertParam { method_name: "listunspent"                  , param_idx:   0  , param_name:  "minconf"               , } , 
    RPCConvertParam { method_name: "listunspent"                  , param_idx:   1  , param_name:  "maxconf"               , } , 
    RPCConvertParam { method_name: "listunspent"                  , param_idx:   2  , param_name:  "addresses"             , } , 
    RPCConvertParam { method_name: "listunspent"                  , param_idx:   3  , param_name:  "include_unsafe"        , } , 
    RPCConvertParam { method_name: "listunspent"                  , param_idx:   4  , param_name:  "query_options"         , } , 
    RPCConvertParam { method_name: "getblock"                     , param_idx:   1  , param_name:  "verbosity"             , } , 
    RPCConvertParam { method_name: "getblock"                     , param_idx:   1  , param_name:  "verbose"               , } , 
    RPCConvertParam { method_name: "getblockheader"               , param_idx:   1  , param_name:  "verbose"               , } , 
    RPCConvertParam { method_name: "getchaintxstats"              , param_idx:   0  , param_name:  "nblocks"               , } , 
    RPCConvertParam { method_name: "gettransaction"               , param_idx:   1  , param_name:  "include_watchonly"     , } , 
    RPCConvertParam { method_name: "gettransaction"               , param_idx:   2  , param_name:  "verbose"               , } , 
    RPCConvertParam { method_name: "getrawtransaction"            , param_idx:   1  , param_name:  "verbose"               , } , 
    RPCConvertParam { method_name: "createrawtransaction"         , param_idx:   0  , param_name:  "inputs"                , } , 
    RPCConvertParam { method_name: "createrawtransaction"         , param_idx:   1  , param_name:  "outputs"               , } , 
    RPCConvertParam { method_name: "createrawtransaction"         , param_idx:   2  , param_name:  "locktime"              , } , 
    RPCConvertParam { method_name: "createrawtransaction"         , param_idx:   3  , param_name:  "replaceable"           , } , 
    RPCConvertParam { method_name: "decoderawtransaction"         , param_idx:   1  , param_name:  "iswitness"             , } , 
    RPCConvertParam { method_name: "signrawtransactionwithkey"    , param_idx:   1  , param_name:  "privkeys"              , } , 
    RPCConvertParam { method_name: "signrawtransactionwithkey"    , param_idx:   2  , param_name:  "prevtxs"               , } , 
    RPCConvertParam { method_name: "signrawtransactionwithwallet" , param_idx:   1  , param_name:  "prevtxs"               , } , 
    RPCConvertParam { method_name: "sendrawtransaction"           , param_idx:   1  , param_name:  "maxfeerate"            , } , 
    RPCConvertParam { method_name: "testmempoolaccept"            , param_idx:   0  , param_name:  "rawtxs"                , } , 
    RPCConvertParam { method_name: "testmempoolaccept"            , param_idx:   1  , param_name:  "maxfeerate"            , } , 
    RPCConvertParam { method_name: "combinerawtransaction"        , param_idx:   0  , param_name:  "txs"                   , } , 
    RPCConvertParam { method_name: "fundrawtransaction"           , param_idx:   1  , param_name:  "options"               , } , 
    RPCConvertParam { method_name: "fundrawtransaction"           , param_idx:   2  , param_name:  "iswitness"             , } , 
    RPCConvertParam { method_name: "walletcreatefundedpsbt"       , param_idx:   0  , param_name:  "inputs"                , } , 
    RPCConvertParam { method_name: "walletcreatefundedpsbt"       , param_idx:   1  , param_name:  "outputs"               , } , 
    RPCConvertParam { method_name: "walletcreatefundedpsbt"       , param_idx:   2  , param_name:  "locktime"              , } , 
    RPCConvertParam { method_name: "walletcreatefundedpsbt"       , param_idx:   3  , param_name:  "options"               , } , 
    RPCConvertParam { method_name: "walletcreatefundedpsbt"       , param_idx:   4  , param_name:  "bip32derivs"           , } , 
    RPCConvertParam { method_name: "walletprocesspsbt"            , param_idx:   1  , param_name:  "sign"                  , } , 
    RPCConvertParam { method_name: "walletprocesspsbt"            , param_idx:   3  , param_name:  "bip32derivs"           , } , 
    RPCConvertParam { method_name: "createpsbt"                   , param_idx:   0  , param_name:  "inputs"                , } , 
    RPCConvertParam { method_name: "createpsbt"                   , param_idx:   1  , param_name:  "outputs"               , } , 
    RPCConvertParam { method_name: "createpsbt"                   , param_idx:   2  , param_name:  "locktime"              , } , 
    RPCConvertParam { method_name: "createpsbt"                   , param_idx:   3  , param_name:  "replaceable"           , } , 
    RPCConvertParam { method_name: "combinepsbt"                  , param_idx:   0  , param_name:  "txs"                   , } , 
    RPCConvertParam { method_name: "joinpsbts"                    , param_idx:   0  , param_name:  "txs"                   , } , 
    RPCConvertParam { method_name: "finalizepsbt"                 , param_idx:   1  , param_name:  "extract"               , } , 
    RPCConvertParam { method_name: "converttopsbt"                , param_idx:   1  , param_name:  "permitsigdata"         , } , 
    RPCConvertParam { method_name: "converttopsbt"                , param_idx:   2  , param_name:  "iswitness"             , } , 
    RPCConvertParam { method_name: "gettxout"                     , param_idx:   1  , param_name:  "n"                     , } , 
    RPCConvertParam { method_name: "gettxout"                     , param_idx:   2  , param_name:  "include_mempool"       , } , 
    RPCConvertParam { method_name: "gettxoutproof"                , param_idx:   0  , param_name:  "txids"                 , } , 
    RPCConvertParam { method_name: "gettxoutsetinfo"              , param_idx:   1  , param_name:  "hash_or_height"        , } , 
    RPCConvertParam { method_name: "gettxoutsetinfo"              , param_idx:   2  , param_name:  "use_index"             , } , 
    RPCConvertParam { method_name: "lockunspent"                  , param_idx:   0  , param_name:  "unlock"                , } , 
    RPCConvertParam { method_name: "lockunspent"                  , param_idx:   1  , param_name:  "transactions"          , } , 
    RPCConvertParam { method_name: "lockunspent"                  , param_idx:   2  , param_name:  "persistent"            , } , 
    RPCConvertParam { method_name: "send"                         , param_idx:   0  , param_name:  "outputs"               , } , 
    RPCConvertParam { method_name: "send"                         , param_idx:   1  , param_name:  "conf_target"           , } , 
    RPCConvertParam { method_name: "send"                         , param_idx:   3  , param_name:  "fee_rate"              , } , 
    RPCConvertParam { method_name: "send"                         , param_idx:   4  , param_name:  "options"               , } , 
    RPCConvertParam { method_name: "importprivkey"                , param_idx:   2  , param_name:  "rescan"                , } , 
    RPCConvertParam { method_name: "importaddress"                , param_idx:   2  , param_name:  "rescan"                , } , 
    RPCConvertParam { method_name: "importaddress"                , param_idx:   3  , param_name:  "p2sh"                  , } , 
    RPCConvertParam { method_name: "importpubkey"                 , param_idx:   2  , param_name:  "rescan"                , } , 
    RPCConvertParam { method_name: "importmulti"                  , param_idx:   0  , param_name:  "requests"              , } , 
    RPCConvertParam { method_name: "importmulti"                  , param_idx:   1  , param_name:  "options"               , } , 
    RPCConvertParam { method_name: "importdescriptors"            , param_idx:   0  , param_name:  "requests"              , } , 
    RPCConvertParam { method_name: "listdescriptors"              , param_idx:   0  , param_name:  "private"               , } , 
    RPCConvertParam { method_name: "verifychain"                  , param_idx:   0  , param_name:  "checklevel"            , } , 
    RPCConvertParam { method_name: "verifychain"                  , param_idx:   1  , param_name:  "nblocks"               , } , 
    RPCConvertParam { method_name: "getblockstats"                , param_idx:   0  , param_name:  "hash_or_height"        , } , 
    RPCConvertParam { method_name: "getblockstats"                , param_idx:   1  , param_name:  "stats"                 , } , 
    RPCConvertParam { method_name: "pruneblockchain"              , param_idx:   0  , param_name:  "height"                , } , 
    RPCConvertParam { method_name: "keypoolrefill"                , param_idx:   0  , param_name:  "newsize"               , } , 
    RPCConvertParam { method_name: "getrawmempool"                , param_idx:   0  , param_name:  "verbose"               , } , 
    RPCConvertParam { method_name: "getrawmempool"                , param_idx:   1  , param_name:  "mempool_sequence"      , } , 
    RPCConvertParam { method_name: "estimatesmartfee"             , param_idx:   0  , param_name:  "conf_target"           , } , 
    RPCConvertParam { method_name: "estimaterawfee"               , param_idx:   0  , param_name:  "conf_target"           , } , 
    RPCConvertParam { method_name: "estimaterawfee"               , param_idx:   1  , param_name:  "threshold"             , } , 
    RPCConvertParam { method_name: "prioritisetransaction"        , param_idx:   1  , param_name:  "dummy"                 , } , 
    RPCConvertParam { method_name: "prioritisetransaction"        , param_idx:   2  , param_name:  "fee_delta"             , } , 
    RPCConvertParam { method_name: "setban"                       , param_idx:   2  , param_name:  "bantime"               , } , 
    RPCConvertParam { method_name: "setban"                       , param_idx:   3  , param_name:  "absolute"              , } , 
    RPCConvertParam { method_name: "setnetworkactive"             , param_idx:   0  , param_name:  "state"                 , } , 
    RPCConvertParam { method_name: "setwalletflag"                , param_idx:   1  , param_name:  "value"                 , } , 
    RPCConvertParam { method_name: "getmempoolancestors"          , param_idx:   1  , param_name:  "verbose"               , } , 
    RPCConvertParam { method_name: "getmempooldescendants"        , param_idx:   1  , param_name:  "verbose"               , } , 
    RPCConvertParam { method_name: "bumpfee"                      , param_idx:   1  , param_name:  "options"               , } , 
    RPCConvertParam { method_name: "psbtbumpfee"                  , param_idx:   1  , param_name:  "options"               , } , 
    RPCConvertParam { method_name: "logging"                      , param_idx:   0  , param_name:  "include"               , } , 
    RPCConvertParam { method_name: "logging"                      , param_idx:   1  , param_name:  "exclude"               , } , 
    RPCConvertParam { method_name: "disconnectnode"               , param_idx:   1  , param_name:  "nodeid"                , } , 
    RPCConvertParam { method_name: "upgradewallet"                , param_idx:   0  , param_name:  "version"               , } , 

    // Echo with conversion (For testing only)
    RPCConvertParam { method_name: "echojson"                     , param_idx:   0  , param_name:  "arg0"                  , } , 
    RPCConvertParam { method_name: "echojson"                     , param_idx:   1  , param_name:  "arg1"                  , } , 
    RPCConvertParam { method_name: "echojson"                     , param_idx:   2  , param_name:  "arg2"                  , } , 
    RPCConvertParam { method_name: "echojson"                     , param_idx:   3  , param_name:  "arg3"                  , } , 
    RPCConvertParam { method_name: "echojson"                     , param_idx:   4  , param_name:  "arg4"                  , } , 
    RPCConvertParam { method_name: "echojson"                     , param_idx:   5  , param_name:  "arg5"                  , } , 
    RPCConvertParam { method_name: "echojson"                     , param_idx:   6  , param_name:  "arg6"                  , } , 
    RPCConvertParam { method_name: "echojson"                     , param_idx:   7  , param_name:  "arg7"                  , } , 
    RPCConvertParam { method_name: "echojson"                     , param_idx:   8  , param_name:  "arg8"                  , } , 
    RPCConvertParam { method_name: "echojson"                     , param_idx:   9  , param_name:  "arg9"                  , } , 
    RPCConvertParam { method_name: "rescanblockchain"             , param_idx:   0  , param_name:  "start_height"          , } , 
    RPCConvertParam { method_name: "rescanblockchain"             , param_idx:   1  , param_name:  "stop_height"           , } , 
    RPCConvertParam { method_name: "createwallet"                 , param_idx:   1  , param_name:  "disable_private_keys"  , } , 
    RPCConvertParam { method_name: "createwallet"                 , param_idx:   2  , param_name:  "blank"                 , } , 
    RPCConvertParam { method_name: "createwallet"                 , param_idx:   4  , param_name:  "avoid_reuse"           , } , 
    RPCConvertParam { method_name: "createwallet"                 , param_idx:   5  , param_name:  "descriptors"           , } , 
    RPCConvertParam { method_name: "createwallet"                 , param_idx:   6  , param_name:  "load_on_startup"       , } , 
    RPCConvertParam { method_name: "createwallet"                 , param_idx:   7  , param_name:  "external_signer"       , } , 
    RPCConvertParam { method_name: "restorewallet"                , param_idx:   2  , param_name:  "load_on_startup"       , } , 
    RPCConvertParam { method_name: "loadwallet"                   , param_idx:   1  , param_name:  "load_on_startup"       , } , 
    RPCConvertParam { method_name: "unloadwallet"                 , param_idx:   1  , param_name:  "load_on_startup"       , } , 
    RPCConvertParam { method_name: "getnodeaddresses"             , param_idx:   0  , param_name:  "count"                 , } , 
    RPCConvertParam { method_name: "addpeeraddress"               , param_idx:   1  , param_name:  "port"                  , } , 
    RPCConvertParam { method_name: "addpeeraddress"               , param_idx:   2  , param_name:  "tried"                 , } , 
    RPCConvertParam { method_name: "stop"                         , param_idx:   0  , param_name:  "wait"                  , }
];
