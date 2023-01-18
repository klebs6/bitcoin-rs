crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/rpc/protocol.h]

/**
  | HTTP status codes
  |
  */
#[repr(i32)]
pub enum HTTPStatusCode
{
    HTTP_OK                    = 200,
    HTTP_BAD_REQUEST           = 400,
    HTTP_UNAUTHORIZED          = 401,
    HTTP_FORBIDDEN             = 403,
    HTTP_NOT_FOUND             = 404,
    HTTP_BAD_METHOD            = 405,
    HTTP_INTERNAL_SERVER_ERROR = 500,
    HTTP_SERVICE_UNAVAILABLE   = 503,
}

/**
  | Bitcoin RPC error codes
  |
  */
bitflags! {

    pub struct RPCErrorCode: i32 {

        /* --------- Standard JSON-RPC 2.0 errors  --------- */

        /*
          | RPC_INVALID_REQUEST is internally mapped to
          | HTTP_BAD_REQUEST (400).
          |
          | It should not be used for application-layer
          | errors.
          */
        const RPC_INVALID_REQUEST  = -32600;

        /*
          | RPC_METHOD_NOT_FOUND is internally mapped
          | to HTTP_NOT_FOUND (404).
          |
          | It should not be used for application-layer
          | errors.
          */
        const RPC_METHOD_NOT_FOUND = -32601;
        const RPC_INVALID_PARAMS   = -32602;

        /*
          | RPC_INTERNAL_ERROR should only be
          | used for genuine errors in bitcoind
          | (for example datadir corruption).
          |
          */
        const RPC_INTERNAL_ERROR   = -32603;
        const RPC_PARSE_ERROR      = -32700;

        /* ------ General application defined errors  ------ */

        /*
          | std::exception thrown in command handling
          |
          */
        const RPC_MISC_ERROR                  = -1;

        /*
          | Unexpected type was passed as parameter
          |
          */
        const RPC_TYPE_ERROR                  = -3;

        /*
          | Invalid address or key
          |
          */
        const RPC_INVALID_ADDRESS_OR_KEY      = -5;

        /*
          | Ran out of memory during operation
          |
          */
        const RPC_OUT_OF_MEMORY               = -7;

        /*
          | Invalid, missing or duplicate parameter
          |
          */
        const RPC_INVALID_PARAMETER           = -8;

        /*
          | Database error
          |
          */
        const RPC_DATABASE_ERROR              = -20;

        /*
          | Error parsing or validating structure
          | in raw format
          |
          */
        const RPC_DESERIALIZATION_ERROR       = -22;

        /*
          | General error during transaction or
          | block submission
          |
          */
        const RPC_VERIFY_ERROR                = -25;

        /*
          | Transaction or block was rejected by
          | network rules
          |
          */
        const RPC_VERIFY_REJECTED             = -26;

        /*
          | Transaction already in chain
          |
          */
        const RPC_VERIFY_ALREADY_IN_CHAIN     = -27;

        /*
          | Client still warming up
          |
          */
        const RPC_IN_WARMUP                   = -28;

        /*
          | RPC method is deprecated
          |
          */
        const RPC_METHOD_DEPRECATED           = -32;


        /* ------ Aliases for backward compatibility  ------ */
        const RPC_TRANSACTION_ERROR           = Self::RPC_VERIFY_ERROR.bits;
        const RPC_TRANSACTION_REJECTED        = Self::RPC_VERIFY_REJECTED.bits;
        const RPC_TRANSACTION_ALREADY_IN_CHAIN= Self::RPC_VERIFY_ALREADY_IN_CHAIN.bits;

        /* --------------- P2P client errors  --------------- */

        /*
          | Bitcoin is not connected
          |
          */
        const RPC_CLIENT_NOT_CONNECTED        = -9;

        /*
          | Still downloading initial blocks
          |
          */
        const RPC_CLIENT_IN_INITIAL_DOWNLOAD  = -10;

        /*
          | Node is already added
          |
          */
        const RPC_CLIENT_NODE_ALREADY_ADDED   = -23;

        /*
          | Node has not been added before
          |
          */
        const RPC_CLIENT_NODE_NOT_ADDED       = -24;

        /*
          | Node to disconnect not found in connected
          | nodes
          |
          */
        const RPC_CLIENT_NODE_NOT_CONNECTED   = -29;

        /*
          | Invalid IP/Subnet
          |
          */
        const RPC_CLIENT_INVALID_IP_OR_SUBNET = -30;

        /*
          | No valid connection manager instance
          | found
          |
          */
        const RPC_CLIENT_P2P_DISABLED         = -31;

        /*
          | Max number of outbound or block-relay
          | connections already open
          |
          */
        const RPC_CLIENT_NODE_CAPACITY_REACHED= -34;

        /* ----------------- Chain errors  ----------------- */

        /*
          | No mempool instance found
          |
          */
        const RPC_CLIENT_MEMPOOL_DISABLED     = -33;

        /* ----------------- Wallet errors  ----------------- */

        /*
          | Unspecified problem with wallet (key
          | not found etc.)
          |
          */
        const RPC_WALLET_ERROR                = -4;

        /*
          | Not enough funds in wallet or account
          |
          */
        const RPC_WALLET_INSUFFICIENT_FUNDS   = -6;

        /*
          | Invalid label name
          |
          */
        const RPC_WALLET_INVALID_LABEL_NAME   = -11;

        /*
          | Keypool ran out, call keypoolrefill
          | first
          |
          */
        const RPC_WALLET_KEYPOOL_RAN_OUT      = -12;

        /*
          | Enter the wallet passphrase with walletpassphrase
          | first
          |
          */
        const RPC_WALLET_UNLOCK_NEEDED        = -13;

        /*
          | The wallet passphrase entered was incorrect
          |
          */
        const RPC_WALLET_PASSPHRASE_INCORRECT = -14;

        /*
          | Command given in wrong wallet encryption
          | state (encrypting an encrypted wallet
          | etc.)
          |
          */
        const RPC_WALLET_WRONG_ENC_STATE      = -15;

        /*
          | Failed to encrypt the wallet
          |
          */
        const RPC_WALLET_ENCRYPTION_FAILED    = -16;

        /*
          | Wallet is already unlocked
          |
          */
        const RPC_WALLET_ALREADY_UNLOCKED     = -17;

        /*
          | Invalid wallet specified
          |
          */
        const RPC_WALLET_NOT_FOUND            = -18;

        /*
          | No wallet specified (error when there
          | are multiple wallets loaded)
          |
          */
        const RPC_WALLET_NOT_SPECIFIED        = -19;

        /*
          | This same wallet is already loaded
          |
          */
        const RPC_WALLET_ALREADY_LOADED       = -35;

        /* --------- Backwards compatible aliases  --------- */
        const RPC_WALLET_INVALID_ACCOUNT_NAME = Self::RPC_WALLET_INVALID_LABEL_NAME.bits;

        /*
         | Unused reserved codes, kept around for
         | backwards compatibility. Do not reuse.  
         */

        /*
          | Server is in safe mode, and command is
          | not allowed in safe mode
          |
          */
        const RPC_FORBIDDEN_BY_SAFE_MODE      = -2;
    }
}
