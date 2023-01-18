crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/wallet/receive.h]

pub struct OutputEntry {
    destination: TxDestination,
    amount:      Amount,
    vout:        i32,
}

pub struct Balance {

    /**
      | Trusted, at depth=GetBalance.min_depth
      | or more
      |
      */
    mine_trusted:                Amount, // default = { 0 }

    /**
      | Untrusted, but in mempool (pending)
      |
      */
    mine_untrusted_pending:      Amount, // default = { 0 }

    /**
      | Immature coinbases in the main chain
      |
      */
    mine_immature:               Amount, // default = { 0 }

    watchonly_trusted:           Amount, // default = { 0 }
    watchonly_untrusted_pending: Amount, // default = { 0 }
    watchonly_immature:          Amount, // default = { 0 }
}

//-------------------------------------------[.cpp/bitcoin/src/wallet/receive.cpp]

#[EXCLUSIVE_LOCKS_REQUIRED(wallet.cs_wallet)]
pub fn input_is_mine(
    wallet: &Wallet,
    txin:   &TxIn) -> IsMineType 
{
    todo!();
        /*
            AssertLockHeld(wallet.cs_wallet);
        std::map<uint256, CWalletTx>::const_iterator mi = wallet.mapWallet.find(txin.prevout.hash);
        if (mi != wallet.mapWallet.end())
        {
            const CWalletTx& prev = (*mi).second;
            if (txin.prevout.n < prev.tx->vout.size())
                return wallet.IsMine(prev.tx->vout[txin.prevout.n]);
        }
        return ISMINE_NO;
        */
}

/**
  | Returns whether all of the inputs match
  | the filter
  |
  */
pub fn all_inputs_mine(
    wallet: &Wallet,
    tx:     &Transaction,
    filter: &IsMineFilter) -> bool 
{
    todo!();
        /*
            LOCK(wallet.cs_wallet);

        for (const CTxIn& txin : tx.vin)
        {
            auto mi = wallet.mapWallet.find(txin.prevout.hash);
            if (mi == wallet.mapWallet.end())
                return false; // any unknown inputs can't be from us

            const CWalletTx& prev = (*mi).second;

            if (txin.prevout.n >= prev.tx->vout.size())
                return false; // invalid input!

            if (!(wallet.IsMine(prev.tx->vout[txin.prevout.n]) & filter))
                return false;
        }
        return true;
        */
}

pub fn output_get_credit(
    wallet: &Wallet,
    txout:  &TxOut,
    filter: &IsMineFilter) -> Amount 
{
    todo!();
        /*
            if (!MoneyRange(txout.nValue))
            throw std::runtime_error(std::string(__func__) + ": value out of range");
        LOCK(wallet.cs_wallet);
        return ((wallet.IsMine(txout) & filter) ? txout.nValue : 0);
        */
}

pub fn tx_get_credit(
        wallet: &Wallet,
        tx:     &Transaction,
        filter: &IsMineFilter) -> Amount {
    
    todo!();
        /*
            CAmount nCredit = 0;
        for (const CTxOut& txout : tx.vout)
        {
            nCredit += OutputGetCredit(wallet, txout, filter);
            if (!MoneyRange(nCredit))
                throw std::runtime_error(std::string(__func__) + ": value out of range");
        }
        return nCredit;
        */
}

#[EXCLUSIVE_LOCKS_REQUIRED(wallet.cs_wallet)]
pub fn script_is_change(
        wallet: &Wallet,
        script: &Script) -> bool {
    
    todo!();
        /*
            // TODO: fix handling of 'change' outputs. The assumption is that any
        // payment to a script that is ours, but is not in the address book
        // is change. That assumption is likely to break when we implement multisignature
        // wallets that return change back into a multi-signature-protected address;
        // a better way of identifying which outputs are 'the send' and which are
        // 'the change' will need to be implemented (maybe extend CWalletTx to remember
        // which output, if any, was change).
        AssertLockHeld(wallet.cs_wallet);
        if (wallet.IsMine(script))
        {
            TxDestination address;
            if (!ExtractDestination(script, address))
                return true;
            if (!wallet.FindAddressBookEntry(address)) {
                return true;
            }
        }
        return false;
        */
}

#[EXCLUSIVE_LOCKS_REQUIRED(wallet.cs_wallet)]
pub fn output_is_change(
        wallet: &Wallet,
        txout:  &TxOut) -> bool {
    
    todo!();
        /*
            return ScriptIsChange(wallet, txout.scriptPubKey);
        */
}

#[EXCLUSIVE_LOCKS_REQUIRED(wallet.cs_wallet)]
pub fn output_get_change(
        wallet: &Wallet,
        txout:  &TxOut) -> Amount {
    
    todo!();
        /*
            AssertLockHeld(wallet.cs_wallet);
        if (!MoneyRange(txout.nValue))
            throw std::runtime_error(std::string(__func__) + ": value out of range");
        return (OutputIsChange(wallet, txout) ? txout.nValue : 0);
        */
}

pub fn tx_get_change(
        wallet: &Wallet,
        tx:     &Transaction) -> Amount {
    
    todo!();
        /*
            LOCK(wallet.cs_wallet);
        CAmount nChange = 0;
        for (const CTxOut& txout : tx.vout)
        {
            nChange += OutputGetChange(wallet, txout);
            if (!MoneyRange(nChange))
                throw std::runtime_error(std::string(__func__) + ": value out of range");
        }
        return nChange;
        */
}

pub fn get_cachable_amount(
    wallet:      &Wallet,
    wtx:         &WalletTx,
    ty:          WalletTxAmountType,
    filter:      &IsMineFilter,
    recalculate: Option<bool>) -> Amount 
{
    let recalculate: bool = recalculate.unwrap_or(false);

    todo!();
        /*
            auto& amount = wtx.m_amounts[type];
        if (recalculate || !amount.m_cached[filter]) {
            amount.Set(filter, type == CWalletTx::DEBIT ? wallet.GetDebit(*wtx.tx, filter) : TxGetCredit(wallet, *wtx.tx, filter));
            wtx.m_is_cache_empty = false;
        }
        return amount.m_value[filter];
        */
}

pub fn cached_tx_get_credit(
    wallet: &Wallet,
    wtx:    &WalletTx,
    filter: &IsMineFilter) -> Amount 
{
    todo!();
        /*
            // Must wait until coinbase is safely deep enough in the chain before valuing it
        if (wallet.IsTxImmatureCoinBase(wtx))
            return 0;

        CAmount credit = 0;
        if (filter & ISMINE_SPENDABLE) {
            // GetBalance can assume transactions in mapWallet won't change
            credit += GetCachableAmount(wallet, wtx, CWalletTx::CREDIT, ISMINE_SPENDABLE);
        }
        if (filter & ISMINE_WATCH_ONLY) {
            credit += GetCachableAmount(wallet, wtx, CWalletTx::CREDIT, ISMINE_WATCH_ONLY);
        }
        return credit;
        */
}

/**
  | filter decides which addresses will
  | count towards the debit
  |
  */
pub fn cached_tx_get_debit(
    wallet: &Wallet,
    wtx:    &WalletTx,
    filter: &IsMineFilter) -> Amount 
{
    todo!();
        /*
            if (wtx.tx->vin.empty())
            return 0;

        CAmount debit = 0;
        if (filter & ISMINE_SPENDABLE) {
            debit += GetCachableAmount(wallet, wtx, CWalletTx::DEBIT, ISMINE_SPENDABLE);
        }
        if (filter & ISMINE_WATCH_ONLY) {
            debit += GetCachableAmount(wallet, wtx, CWalletTx::DEBIT, ISMINE_WATCH_ONLY);
        }
        return debit;
        */
}

pub fn cached_tx_get_change(
        wallet: &Wallet,
        wtx:    &WalletTx) -> Amount {
    
    todo!();
        /*
            if (wtx.fChangeCached)
            return wtx.nChangeCached;
        wtx.nChangeCached = TxGetChange(wallet, *wtx.tx);
        wtx.fChangeCached = true;
        return wtx.nChangeCached;
        */
}

pub fn cached_tx_get_immature_credit(
        wallet:    &Wallet,
        wtx:       &WalletTx,
        use_cache: Option<bool>) -> Amount {

    let use_cache: bool = use_cache.unwrap_or(true);
    
    todo!();
        /*
            if (wallet.IsTxImmatureCoinBase(wtx) && wallet.IsTxInMainChain(wtx)) {
            return GetCachableAmount(wallet, wtx, CWalletTx::IMMATURE_CREDIT, ISMINE_SPENDABLE, !fUseCache);
        }

        return 0;
        */
}

pub fn cached_tx_get_immature_watch_only_credit(
        wallet:    &Wallet,
        wtx:       &WalletTx,
        use_cache: Option<bool>) -> Amount {

    let use_cache: bool = use_cache.unwrap_or(true);
    
    todo!();
        /*
            if (wallet.IsTxImmatureCoinBase(wtx) && wallet.IsTxInMainChain(wtx)) {
            return GetCachableAmount(wallet, wtx, CWalletTx::IMMATURE_CREDIT, ISMINE_WATCH_ONLY, !fUseCache);
        }

        return 0;
        */
}

/**
  | TODO: Remove "NO_THREAD_SAFETY_ANALYSIS" and
  | replace it with the correct annotation
  | "EXCLUSIVE_LOCKS_REQUIRED(pwallet->cs_wallet)".
  |
  | The annotation "NO_THREAD_SAFETY_ANALYSIS" was
  | temporarily added to avoid having to resolve
  | the issue of member access into incomplete type
  | CWallet.
  */
#[NO_THREAD_SAFETY_ANALYSIS]
pub fn cached_tx_get_available_credit(
        wallet:    &Wallet,
        wtx:       &WalletTx,
        use_cache: Option<bool>,
        filter:    Option<IsMineFilter>) -> Amount {

    let use_cache: bool = use_cache.unwrap_or(true);

    let filter: IsMineFilter = filter.unwrap_or(
        IsMineType::ISMINE_SPENDABLE.bits().try_into().unwrap()
    );
    
    todo!();
        /*
            // Avoid caching ismine for NO or ALL cases (could remove this check and simplify in the future).
        bool allow_cache = (filter & ISMINE_ALL) && (filter & ISMINE_ALL) != ISMINE_ALL;

        // Must wait until coinbase is safely deep enough in the chain before valuing it
        if (wallet.IsTxImmatureCoinBase(wtx))
            return 0;

        if (fUseCache && allow_cache && wtx.m_amounts[CWalletTx::AVAILABLE_CREDIT].m_cached[filter]) {
            return wtx.m_amounts[CWalletTx::AVAILABLE_CREDIT].m_value[filter];
        }

        bool allow_used_addresses = (filter & ISMINE_USED) || !wallet.IsWalletFlagSet(WALLET_FLAG_AVOID_REUSE);
        CAmount nCredit = 0;
        uint256 hashTx = wtx.GetHash();
        for (unsigned int i = 0; i < wtx.tx->vout.size(); i++)
        {
            if (!wallet.IsSpent(hashTx, i) && (allow_used_addresses || !wallet.IsSpentKey(hashTx, i))) {
                const CTxOut &txout = wtx.tx->vout[i];
                nCredit += OutputGetCredit(wallet, txout, filter);
                if (!MoneyRange(nCredit))
                    throw std::runtime_error(std::string(__func__) + " : value out of range");
            }
        }

        if (allow_cache) {
            wtx.m_amounts[CWalletTx::AVAILABLE_CREDIT].Set(filter, nCredit);
            wtx.m_is_cache_empty = false;
        }

        return nCredit;
        */
}

pub fn cached_tx_get_amounts(
        wallet:        &Wallet,
        wtx:           &WalletTx,
        list_received: &mut LinkedList<OutputEntry>,
        list_sent:     &mut LinkedList<OutputEntry>,
        n_fee:         &mut Amount,
        filter:        &IsMineFilter)  {
    
    todo!();
        /*
            nFee = 0;
        listReceived.clear();
        listSent.clear();

        // Compute fee:
        CAmount nDebit = CachedTxGetDebit(wallet, wtx, filter);
        if (nDebit > 0) // debit>0 means we signed/sent this transaction
        {
            CAmount nValueOut = wtx.tx->GetValueOut();
            nFee = nDebit - nValueOut;
        }

        LOCK(wallet.cs_wallet);
        // Sent/received.
        for (unsigned int i = 0; i < wtx.tx->vout.size(); ++i)
        {
            const CTxOut& txout = wtx.tx->vout[i];
            isminetype fIsMine = wallet.IsMine(txout);
            // Only need to handle txouts if AT LEAST one of these is true:
            //   1) they debit from us (sent)
            //   2) the output is to us (received)
            if (nDebit > 0)
            {
                // Don't report 'change' txouts
                if (OutputIsChange(wallet, txout))
                    continue;
            }
            else if (!(fIsMine & filter))
                continue;

            // In either case, we need to get the destination address
            TxDestination address;

            if (!ExtractDestination(txout.scriptPubKey, address) && !txout.scriptPubKey.IsUnspendable())
            {
                wallet.WalletLogPrintf("CWalletTx::GetAmounts: Unknown transaction type found, txid %s\n",
                                        wtx.GetHash().ToString());
                address = CNoDestination();
            }

            COutputEntry output = {address, txout.nValue, (int)i};

            // If we are debited by the transaction, add the output as a "sent" entry
            if (nDebit > 0)
                listSent.push_back(output);

            // If we are receiving the output, add it as a "received" entry
            if (fIsMine & filter)
                listReceived.push_back(output);
        }
        */
}

pub fn cached_tx_is_from_me(
        wallet: &Wallet,
        wtx:    &WalletTx,
        filter: &IsMineFilter) -> bool {
    
    todo!();
        /*
            return (CachedTxGetDebit(wallet, wtx, filter) > 0);
        */
}

#[EXCLUSIVE_LOCKS_REQUIRED(wallet.cs_wallet)]
pub fn cached_tx_is_trusted_with_trusted_parents(
        wallet:          &Wallet,
        wtx:             &WalletTx,
        trusted_parents: &mut HashSet<u256>) -> bool {
    
    todo!();
        /*
            AssertLockHeld(wallet.cs_wallet);
        // Quick answer in most cases
        if (!wallet.chain().checkFinalTx(*wtx.tx)) return false;
        int nDepth = wallet.GetTxDepthInMainChain(wtx);
        if (nDepth >= 1) return true;
        if (nDepth < 0) return false;
        // using wtx's cached debit
        if (!wallet.m_spend_zero_conf_change || !CachedTxIsFromMe(wallet, wtx, ISMINE_ALL)) return false;

        // Don't trust unconfirmed transactions from us unless they are in the mempool.
        if (!wtx.InMempool()) return false;

        // Trusted if all inputs are from us and are in the mempool:
        for (const CTxIn& txin : wtx.tx->vin)
        {
            // Transactions not sent by us: not trusted
            const CWalletTx* parent = wallet.GetWalletTx(txin.prevout.hash);
            if (parent == nullptr) return false;
            const CTxOut& parentOut = parent->tx->vout[txin.prevout.n];
            // Check that this specific input being spent is trusted
            if (wallet.IsMine(parentOut) != ISMINE_SPENDABLE) return false;
            // If we've already trusted this parent, continue
            if (trusted_parents.count(parent->GetHash())) continue;
            // Recurse to check that the parent is also trusted
            if (!CachedTxIsTrusted(wallet, *parent, trusted_parents)) return false;
            trusted_parents.insert(parent->GetHash());
        }
        return true;
        */
}

pub fn cached_tx_is_trusted(
        wallet: &Wallet,
        wtx:    &WalletTx) -> bool {
    
    todo!();
        /*
            std::set<uint256> trusted_parents;
        LOCK(wallet.cs_wallet);
        return CachedTxIsTrusted(wallet, wtx, trusted_parents);
        */
}

pub fn get_balance(
        wallet:      &Wallet,
        min_depth:   Option<i32>,
        avoid_reuse: Option<bool>) -> Balance {

    let min_depth:    i32 = min_depth.unwrap_or(0);
    let avoid_reuse: bool = avoid_reuse.unwrap_or(true);
    
    todo!();
        /*
            Balance ret;
        isminefilter reuse_filter = avoid_reuse ? ISMINE_NO : ISMINE_USED;
        {
            LOCK(wallet.cs_wallet);
            std::set<uint256> trusted_parents;
            for (const auto& entry : wallet.mapWallet)
            {
                const CWalletTx& wtx = entry.second;
                const bool is_trusted{CachedTxIsTrusted(wallet, wtx, trusted_parents)};
                const int tx_depth{wallet.GetTxDepthInMainChain(wtx)};
                const CAmount tx_credit_mine{CachedTxGetAvailableCredit(wallet, wtx, /* fUseCache */ true, ISMINE_SPENDABLE | reuse_filter)};
                const CAmount tx_credit_watchonly{CachedTxGetAvailableCredit(wallet, wtx, /* fUseCache */ true, ISMINE_WATCH_ONLY | reuse_filter)};
                if (is_trusted && tx_depth >= min_depth) {
                    ret.m_mine_trusted += tx_credit_mine;
                    ret.m_watchonly_trusted += tx_credit_watchonly;
                }
                if (!is_trusted && tx_depth == 0 && wtx.InMempool()) {
                    ret.m_mine_untrusted_pending += tx_credit_mine;
                    ret.m_watchonly_untrusted_pending += tx_credit_watchonly;
                }
                ret.m_mine_immature += CachedTxGetImmatureCredit(wallet, wtx);
                ret.m_watchonly_immature += CachedTxGetImmatureWatchOnlyCredit(wallet, wtx);
            }
        }
        return ret;
        */
}

pub fn get_address_balances(wallet: &Wallet) -> HashMap<TxDestination,Amount> {
    
    todo!();
        /*
            std::map<TxDestination, CAmount> balances;

        {
            LOCK(wallet.cs_wallet);
            std::set<uint256> trusted_parents;
            for (const auto& walletEntry : wallet.mapWallet)
            {
                const CWalletTx& wtx = walletEntry.second;

                if (!CachedTxIsTrusted(wallet, wtx, trusted_parents))
                    continue;

                if (wallet.IsTxImmatureCoinBase(wtx))
                    continue;

                int nDepth = wallet.GetTxDepthInMainChain(wtx);
                if (nDepth < (CachedTxIsFromMe(wallet, wtx, ISMINE_ALL) ? 0 : 1))
                    continue;

                for (unsigned int i = 0; i < wtx.tx->vout.size(); i++)
                {
                    TxDestination addr;
                    if (!wallet.IsMine(wtx.tx->vout[i]))
                        continue;
                    if(!ExtractDestination(wtx.tx->vout[i].scriptPubKey, addr))
                        continue;

                    CAmount n = wallet.IsSpent(walletEntry.first, i) ? 0 : wtx.tx->vout[i].nValue;
                    balances[addr] += n;
                }
            }
        }

        return balances;
        */
}

#[EXCLUSIVE_LOCKS_REQUIRED(wallet.cs_wallet)]
pub fn get_address_groupings(wallet: &Wallet) -> HashSet<HashSet<TxDestination>> {
    
    todo!();
        /*
            AssertLockHeld(wallet.cs_wallet);
        std::set< std::set<TxDestination> > groupings;
        std::set<TxDestination> grouping;

        for (const auto& walletEntry : wallet.mapWallet)
        {
            const CWalletTx& wtx = walletEntry.second;

            if (wtx.tx->vin.size() > 0)
            {
                bool any_mine = false;
                // group all input addresses with each other
                for (const CTxIn& txin : wtx.tx->vin)
                {
                    TxDestination address;
                    if(!InputIsMine(wallet, txin)) /* If this input isn't mine, ignore it */
                        continue;
                    if(!ExtractDestination(wallet.mapWallet.at(txin.prevout.hash).tx->vout[txin.prevout.n].scriptPubKey, address))
                        continue;
                    grouping.insert(address);
                    any_mine = true;
                }

                // group change with input addresses
                if (any_mine)
                {
                   for (const CTxOut& txout : wtx.tx->vout)
                       if (OutputIsChange(wallet, txout))
                       {
                           TxDestination txoutAddr;
                           if(!ExtractDestination(txout.scriptPubKey, txoutAddr))
                               continue;
                           grouping.insert(txoutAddr);
                       }
                }
                if (grouping.size() > 0)
                {
                    groupings.insert(grouping);
                    grouping.clear();
                }
            }

            // group lone addrs by themselves
            for (const auto& txout : wtx.tx->vout)
                if (wallet.IsMine(txout))
                {
                    TxDestination address;
                    if(!ExtractDestination(txout.scriptPubKey, address))
                        continue;
                    grouping.insert(address);
                    groupings.insert(grouping);
                    grouping.clear();
                }
        }

        std::set< std::set<TxDestination>* > uniqueGroupings; // a set of pointers to groups of addresses
        std::map< TxDestination, std::set<TxDestination>* > setmap;  // map addresses to the unique group containing it
        for (std::set<TxDestination> _grouping : groupings)
        {
            // make a set of all the groups hit by this new group
            std::set< std::set<TxDestination>* > hits;
            std::map< TxDestination, std::set<TxDestination>* >::iterator it;
            for (const TxDestination& address : _grouping)
                if ((it = setmap.find(address)) != setmap.end())
                    hits.insert((*it).second);

            // merge all hit groups into a new single group and delete old groups
            std::set<TxDestination>* merged = new std::set<TxDestination>(_grouping);
            for (std::set<TxDestination>* hit : hits)
            {
                merged->insert(hit->begin(), hit->end());
                uniqueGroupings.erase(hit);
                delete hit;
            }
            uniqueGroupings.insert(merged);

            // update setmap
            for (const TxDestination& element : *merged)
                setmap[element] = merged;
        }

        std::set< std::set<TxDestination> > ret;
        for (const std::set<TxDestination>* uniqueGrouping : uniqueGroupings)
        {
            ret.insert(*uniqueGrouping);
            delete uniqueGrouping;
        }

        return ret;
        */
}
