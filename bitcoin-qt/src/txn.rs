crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/transactionrecord.h]

/**
  | UI model for transaction status. The
  | transaction status is the part of a transaction
  | that will change over time.
  |
  */
pub struct TransactionStatus {

    /**
      | Transaction counts towards available
      | balance
      |
      */
    counts_for_balance: bool,

    /**
      | Sorting key based on status
      |
      */
    sort_key:           String,

    /**
      | @name Generated (mined) transactions
      |
      */
    matures_in:         i32,

    /**
      | @name Reported status
      |
      */
    status:             TransactionStatusCode,

    depth:              i64,

    /**
      | Timestamp if status==OpenUntilDate,
      | otherwise number of additional blocks
      | that need to be mined before finalization
      |
      */
    open_for:           i64,

    /**
      | Current block hash (to know whether
      | cached status is still valid)
      |
      */
    cur_block_hash:     u256,

    needs_update:       bool,
}

pub enum TransactionStatusCode {

    /**
      | Have 6 or more confirmations (normal
      | tx) or fully mature (mined tx) *
      |
      */
    Confirmed,          

    /* ------ Normal (sent/received) transactions  ------ */

    /**
      | Transaction not yet final, waiting
      | for date
      |
      */
    OpenUntilDate,      

    /**
      | Transaction not yet final, waiting
      | for block
      |
      */
    OpenUntilBlock,     

    /**
      | Not yet mined into a block *
      |
      */
    Unconfirmed,        

    /**
      | Confirmed, but waiting for the recommended
      | number of confirmations *
      |
      */
    Confirming,         

    /**
      | Conflicts with other transaction or
      | mempool *
      |
      */
    Conflicted,         

    /**
      | Abandoned from the wallet *
      |
      */
    Abandoned,          

    /* -------- Generated (mined) transactions  -------- */

    /**
      | Mined but waiting for maturity
      |
      */
    Immature,           

    /**
      | Mined but not accepted
      |
      */
    NotAccepted,         
}

impl Default for TransactionStatus {
    
    fn default() -> Self {
        todo!();
        /*
        : counts_for_balance(false),
        : sort_key(""),
        : matures_in(0),
        : status(Unconfirmed),
        : depth(0),
        : open_for(0),

        
        */
    }
}

/**
  | UI model for a transaction. A core transaction
  | can be represented by multiple UI transactions
  | if it has multiple outputs.
  |
  */
pub struct TransactionRecord {

    /**
      | @name Immutable transaction attributes
      |
      */
    hash:                   u256,

    time:                   i64,
    ty:                     TransactionRecordType,
    address:                String,
    debit:                  Amount,
    credit:                 Amount,

    /**
      | Subtransaction index, for sort key
      |
      */
    idx:                    i32,

    /**
      | Status: can change with block chain
      | update
      |
      */
    status:                 TransactionStatus,

    /**
      | Whether the transaction was sent/received
      | with a watch-only address
      |
      */
    involves_watch_address: bool,
}

pub enum TransactionRecordType
{
    Other,
    Generated,
    SendToAddress,
    SendToOther,
    RecvWithAddress,
    RecvFromOther,
    SendToSelf
}

/**
  | Number of confirmation recommended
  | for accepting a transaction
  |
  */
pub const TRANSACTION_RECORD_RECOMMENDED_NUM_CONFIRMATIONS: i32 = 6;

impl Default for TransactionRecord {
    
    fn default() -> Self {
        todo!();
        /*
        : hash(),
        : time(0),
        : ty(Other),
        : address(""),
        : debit(0),
        : credit(0),
        : idx(0),

        
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/transactionrecord.cpp]
impl TransactionRecord {

    pub fn new_with_hash_and_time(
        hash: u256,
        time: i64) -> Self {
    
        todo!();
        /*
        : hash(_hash),
        : time(_time),
        : ty(Other),
        : address(""),
        : debit(0),
        : credit(0),
        : idx(0),

        
        */
    }
    
    pub fn new(
        hash:    u256,
        time:    i64,
        ty:      TransactionRecordType,
        address: &String,
        debit:   &Amount,
        credit:  &Amount) -> Self {
    
        todo!();
        /*
            :
            hash(_hash), time(_time), type(_type), address(_address), debit(_debit), credit(_credit),
            idx(0)
            */
    }

    /**
      | Return positive answer if transaction
      | should be shown in list.
      |
      | Decompose CWallet transaction to model
      | transaction records.
      |
      */
    pub fn show_transaction(&mut self) -> bool {
        
        todo!();
        /*
            // There are currently no cases where we hide transactions, but
        // we may want to use this in the future for things like RBF.
        return true;
        */
    }

    /**
      | Decompose CWallet transaction to model
      | transaction records.
      |
      */
    pub fn decompose_transaction(&mut self, wtx: &WalletTx) -> QList<TransactionRecord> {
        
        todo!();
        /*
            QList<TransactionRecord> parts;
        int64_t nTime = wtx.time;
        CAmount nCredit = wtx.credit;
        CAmount nDebit = wtx.debit;
        CAmount nNet = nCredit - nDebit;
        uint256 hash = wtx.tx->GetHash();
        std::map<std::string, std::string> mapValue = wtx.value_map;

        if (nNet > 0 || wtx.is_coinbase)
        {
            //
            // Credit
            //
            for(unsigned int i = 0; i < wtx.tx->vout.size(); i++)
            {
                const CTxOut& txout = wtx.tx->vout[i];
                isminetype mine = wtx.txout_is_mine[i];
                if(mine)
                {
                    TransactionRecord sub(hash, nTime);
                    sub.idx = i; // vout index
                    sub.credit = txout.nValue;
                    sub.involvesWatchAddress = mine & ISMINE_WATCH_ONLY;
                    if (wtx.txout_address_is_mine[i])
                    {
                        // Received by Bitcoin Address
                        sub.type = TransactionRecord::RecvWithAddress;
                        sub.address = EncodeDestination(wtx.txout_address[i]);
                    }
                    else
                    {
                        // Received by IP connection (deprecated features), or a multisignature or other non-simple transaction
                        sub.type = TransactionRecord::RecvFromOther;
                        sub.address = mapValue["from"];
                    }
                    if (wtx.is_coinbase)
                    {
                        // Generated
                        sub.type = TransactionRecord::Generated;
                    }

                    parts.append(sub);
                }
            }
        }
        else
        {
            bool involvesWatchAddress = false;
            isminetype fAllFromMe = ISMINE_SPENDABLE;
            for (const isminetype mine : wtx.txin_is_mine)
            {
                if(mine & ISMINE_WATCH_ONLY) involvesWatchAddress = true;
                if(fAllFromMe > mine) fAllFromMe = mine;
            }

            isminetype fAllToMe = ISMINE_SPENDABLE;
            for (const isminetype mine : wtx.txout_is_mine)
            {
                if(mine & ISMINE_WATCH_ONLY) involvesWatchAddress = true;
                if(fAllToMe > mine) fAllToMe = mine;
            }

            if (fAllFromMe && fAllToMe)
            {
                // Payment to self
                std::string address;
                for (auto it = wtx.txout_address.begin(); it != wtx.txout_address.end(); ++it) {
                    if (it != wtx.txout_address.begin()) address += ", ";
                    address += EncodeDestination(*it);
                }

                CAmount nChange = wtx.change;
                parts.append(TransactionRecord(hash, nTime, TransactionRecord::SendToSelf, address, -(nDebit - nChange), nCredit - nChange));
                parts.last().involvesWatchAddress = involvesWatchAddress;   // maybe pass to TransactionRecord as constructor argument
            }
            else if (fAllFromMe)
            {
                //
                // Debit
                //
                CAmount nTxFee = nDebit - wtx.tx->GetValueOut();

                for (unsigned int nOut = 0; nOut < wtx.tx->vout.size(); nOut++)
                {
                    const CTxOut& txout = wtx.tx->vout[nOut];
                    TransactionRecord sub(hash, nTime);
                    sub.idx = nOut;
                    sub.involvesWatchAddress = involvesWatchAddress;

                    if(wtx.txout_is_mine[nOut])
                    {
                        // Ignore parts sent to self, as this is usually the change
                        // from a transaction sent back to our own address.
                        continue;
                    }

                    if (!std::get_if<CNoDestination>(&wtx.txout_address[nOut]))
                    {
                        // Sent to Bitcoin Address
                        sub.type = TransactionRecord::SendToAddress;
                        sub.address = EncodeDestination(wtx.txout_address[nOut]);
                    }
                    else
                    {
                        // Sent to IP, or other non-address transaction like OP_EVAL
                        sub.type = TransactionRecord::SendToOther;
                        sub.address = mapValue["to"];
                    }

                    CAmount nValue = txout.nValue;
                    /* Add fee to first output */
                    if (nTxFee > 0)
                    {
                        nValue += nTxFee;
                        nTxFee = 0;
                    }
                    sub.debit = -nValue;

                    parts.append(sub);
                }
            }
            else
            {
                //
                // Mixed debit transaction, can't break down payees
                //
                parts.append(TransactionRecord(hash, nTime, TransactionRecord::Other, "", nNet, 0));
                parts.last().involvesWatchAddress = involvesWatchAddress;
            }
        }

        return parts;
        */
    }
    
    /**
      | Update status from core wallet tx.
      |
      */
    pub fn update_status(&mut self, 
        wtx:        &WalletTxStatus,
        block_hash: &u256,
        num_blocks: i32,
        block_time: i64)  {
        
        todo!();
        /*
            // Determine transaction status

        // Sort order, unrecorded transactions sort to the top
        status.sortKey = strprintf("%010d-%01d-%010u-%03d",
            wtx.block_height,
            wtx.is_coinbase ? 1 : 0,
            wtx.time_received,
            idx);
        status.countsForBalance = wtx.is_trusted && !(wtx.blocks_to_maturity > 0);
        status.depth = wtx.depth_in_main_chain;
        status.m_cur_block_hash = block_hash;

        const bool up_to_date = ((int64_t)QDateTime::currentMSecsSinceEpoch() / 1000 - block_time < MAX_BLOCK_TIME_GAP);
        if (up_to_date && !wtx.is_final) {
            if (wtx.lock_time < LOCKTIME_THRESHOLD) {
                status.status = TransactionStatus::OpenUntilBlock;
                status.open_for = wtx.lock_time - numBlocks;
            }
            else
            {
                status.status = TransactionStatus::OpenUntilDate;
                status.open_for = wtx.lock_time;
            }
        }
        // For generated transactions, determine maturity
        else if(type == TransactionRecord::Generated)
        {
            if (wtx.blocks_to_maturity > 0)
            {
                status.status = TransactionStatus::Immature;

                if (wtx.is_in_main_chain)
                {
                    status.matures_in = wtx.blocks_to_maturity;
                }
                else
                {
                    status.status = TransactionStatus::NotAccepted;
                }
            }
            else
            {
                status.status = TransactionStatus::Confirmed;
            }
        }
        else
        {
            if (status.depth < 0)
            {
                status.status = TransactionStatus::Conflicted;
            }
            else if (status.depth == 0)
            {
                status.status = TransactionStatus::Unconfirmed;
                if (wtx.is_abandoned)
                    status.status = TransactionStatus::Abandoned;
            }
            else if (status.depth < RecommendedNumConfirmations)
            {
                status.status = TransactionStatus::Confirming;
            }
            else
            {
                status.status = TransactionStatus::Confirmed;
            }
        }
        status.needsUpdate = false;
        */
    }
    
    /**
      | Return whether a status update is needed.
      |
      */
    pub fn status_update_needed(&self, block_hash: &u256) -> bool {
        
        todo!();
        /*
            assert(!block_hash.IsNull());
        return status.m_cur_block_hash != block_hash || status.needsUpdate;
        */
    }
    
    /**
      | Return the unique identifier for this
      | transaction (part)
      |
      */
    pub fn get_tx_hash(&self) -> String {
        
        todo!();
        /*
            return QString::fromStdString(hash.ToString());
        */
    }
    
    /**
      | Return the output index of the subtransaction
      |
      */
    pub fn get_output_index(&self) -> i32 {
        
        todo!();
        /*
            return idx;
        */
    }
}


