crate::ix!();

bitflags!{

    pub struct BlockStatus: u32 {

        /*
          | Unused.
          |
          */
        const BLOCK_VALID_UNKNOWN      =    0;

        /*
          | Reserved (was BLOCK_VALID_HEADER).
          |
          */
        const BLOCK_VALID_RESERVED     =    1;

        /*
          | All parent headers found, difficulty
          | matches, timestamp >= median previous,
          | checkpoint. Implies all parents are also at
          | least TREE.
          */
        const BLOCK_VALID_TREE         =    2;

        /*
          | Only first tx is coinbase, 2 <= coinbase
          | input script length <= 100, transactions
          | valid, no duplicate txids, sigops, size,
          | merkle root. Implies all parents are at
          | least TREE but not necessarily
          | TRANSACTIONS. When all parent blocks also
          | have TRANSACTIONS, CBlockIndex::nChainTx
          | will be set.
          */
        const BLOCK_VALID_TRANSACTIONS =    3;

        /*
          | Outputs do not overspend inputs, no double
          | spends, coinbase output ok, no immature
          | coinbase spends, BIP30.
          |
          | Implies all parents are also at least
          | CHAIN.
          */
        const BLOCK_VALID_CHAIN        =    4;

        /*
          | Scripts & signatures ok. Implies all
          | parents are also at least SCRIPTS.
          |
          */
        const BLOCK_VALID_SCRIPTS      =    5;

        /*
         | All validity bits.
         |
         */
        const BLOCK_VALID_MASK         =   
            Self::BLOCK_VALID_RESERVED.bits 
            | Self::BLOCK_VALID_TREE.bits 
            | Self::BLOCK_VALID_TRANSACTIONS.bits 
            | Self::BLOCK_VALID_CHAIN.bits 
            | Self::BLOCK_VALID_SCRIPTS.bits;

        /*
         | full block available in blk*.dat
         |
         */
        const BLOCK_HAVE_DATA          =    8;

        /*
          | undo data available in rev*.dat
          |
          */
        const BLOCK_HAVE_UNDO          =   16;

        const BLOCK_HAVE_MASK          =   
            Self::BLOCK_HAVE_DATA.bits 
            | Self::BLOCK_HAVE_UNDO.bits;

        /*
          | stage after last reached validness
          | failed
          |
          */
        const BLOCK_FAILED_VALID       =   32;

        /*
          | descends from failed block
          |
          */
        const BLOCK_FAILED_CHILD       =   64;

        const BLOCK_FAILED_MASK        =   
            Self::BLOCK_FAILED_VALID.bits 
            | Self::BLOCK_FAILED_CHILD.bits;

        /*
          | block data in blk*.dat was received
          | with a witness-enforcing client
          |
          */
        const BLOCK_OPT_WITNESS        =   128;

        /*
          | If set, this indicates that the block
          | index entry is assumed-valid.
          | 
          | Certain diagnostics will be skipped
          | in e.g. CheckBlockIndex().
          | 
          | It almost certainly means that the block's
          | full validation is pending on a background
          | chainstate. See `doc/assumeutxo.md`.
          |
          */
        const BLOCK_ASSUMED_VALID      =   256;
    }
}
