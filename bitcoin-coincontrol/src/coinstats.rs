// ---------------- [ File: bitcoin-coincontrol/src/coinstats.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/wallet/coincontrol.h]

pub const DEFAULT_MIN_DEPTH: i32 = 0;
pub const DEFAULT_MAX_DEPTH: i32 = 9999999;

/**
  | Default for -avoidpartialspends
  |
  */
pub const DEFAULT_AVOIDPARTIALSPENDS: bool = false;

//-------------------------------------------[.cpp/bitcoin/src/node/coinstats.h]

//-------------------------------------------[.cpp/bitcoin/src/node/coinstats.cpp]

pub enum CoinStatsHashType {
    HASH_SERIALIZED,
    MUHASH,
    NONE,
}

pub struct CoinsStats {

    hash_type:             CoinStatsHashType,
    n_height:              i32, // default = { 0 }
    hash_block:            u256,
    n_transactions:        u64, // default = { 0 }
    n_transaction_outputs: u64, // default = { 0 }
    n_bogo_size:           u64, // default = { 0 }
    hash_serialized:       u256,
    n_disk_size:           u64, // default = { 0 }
    n_total_amount:        Amount, // default = { 0 }

    /**
      | The number of coins contained.
      |
      */
    coins_count:           u64, // default = { 0 }

    /**
      | Signals if the coinstatsindex should
      | be used (when available).
      |
      */
    index_requested:       bool, // default = { true }

    /**
      | Signals if the coinstatsindex was used
      | to retrieve the statistics.
      |
      */
    index_used:            bool, // default = { false }

    /*
      | Following values are only available
      | from coinstats index
      |
      */

    /**
      | Total cumulative amount of block subsidies
      | up to and including this block
      |
      */
    total_subsidy:                        Amount, // default = { 0 }

    /**
      | Total cumulative amount of unspendable
      | coins up to and including this block
      |
      */
    total_unspendable_amount:             Amount, // default = { 0 }

    /**
      | Total cumulative amount of prevouts
      | spent up to and including this block
      |
      */
    total_prevout_spent_amount:           Amount, // default = { 0 }

    /**
      | Total cumulative amount of outputs
      | created up to and including this block
      |
      */
    total_new_outputs_ex_coinbase_amount: Amount, // default = { 0 }

    /**
      | Total cumulative amount of coinbase
      | outputs up to and including this block
      |
      */
    total_coinbase_amount:                Amount, // default = { 0 }

    /**
      | The unspendable coinbase amount from
      | the genesis block
      |
      */
    total_unspendables_genesis_block:     Amount, // default = { 0 }

    /**
      | The two unspendable coinbase outputs
      | total amount caused by BIP30
      |
      */
    total_unspendables_bip30:             Amount, // default = { 0 }

    /**
      | Total cumulative amount of outputs
      | sent to unspendable scripts (OP_RETURN
      | for example) up to and including this
      | block
      |
      */
    total_unspendables_scripts:           Amount, // default = { 0 }

    /**
      | Total cumulative amount of coins lost
      | due to unclaimed miner rewards up to
      | and including this block
      |
      */
    total_unspendables_unclaimed_rewards: Amount, // default = { 0 }
}

impl CoinsStats {

    pub fn new(hash_type: CoinStatsHashType) -> Self {
    
        todo!();
        /*
        : hash_type(hash_type),

        
        */
    }
}

/**
  | Database-independent metric indicating
  | the UTXO set size
  |
  */
pub fn get_bogo_size(script_pub_key: &Script) -> u64 {
    
    todo!();
        /*
            return 32 /* txid */ +
               4 /* vout index */ +
               4 /* height + coinbase */ +
               8 /* amount */ +
               2 /* scriptPubKey len */ +
               script_pub_key.size() /* scriptPubKey */;
        */
}

pub fn tx_out_ser(
        outpoint: &OutPoint,
        coin:     &Coin) -> DataStream {
    
    todo!();
        /*
            DataStream ss(SER_DISK, PROTOCOL_VERSION);
        ss << outpoint;
        ss << static_cast<uint32_t>(coin.nHeight * 2 + coin.fCoinBase);
        ss << coin.out;
        return ss;
        */
}
