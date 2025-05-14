// ---------------- [ File: bitcoin-coinsview/tests/coins.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/coins_tests.cpp]

pub fn apply_tx_in_undo(
        undo: Coin,
        view: &mut CoinsViewCache,
        out:  &OutPoint) -> i32 {
    
    todo!();
        /*
        
        */
}

pub fn update_coins(
        tx:       &Transaction,
        inputs:   &mut CoinsViewCache,
        txundo:   &mut TxUndo,
        n_height: i32)  {
    
    todo!();
        /*
        
        */
}

///------------------------
pub struct CoinsViewTest {
    hash_best_block: u256,
    map:             HashMap<OutPoint,Coin>,
}

impl GetCoin for CoinsViewTest {

    fn get_coin(&self, 
        outpoint: &OutPoint,
        coin:     &mut Coin) -> bool {
        
        todo!();
        /*
            std::map<OutPoint, Coin>::const_iterator it = map_.find(outpoint);
            if (it == map_.end()) {
                return false;
            }
            coin = it->second;
            if (coin.IsSpent() && InsecureRandBool() == 0) {
                // Randomly return false in case of an empty entry.
                return false;
            }
            return true;
        */
    }
}
    
impl GetBestBlock for CoinsViewTest {
    fn get_best_block(&self) -> u256 {
        
        todo!();
        /*
            return hashBestBlock_;
        */
    }
}
    
impl BatchWrite for CoinsViewTest {
    fn batch_write(&mut self, 
        map_coins:  &mut CoinsMap,
        hash_block: &u256) -> bool {
        
        todo!();
        /*
            for (coins_map::iterator it = mapCoins.begin(); it != mapCoins.end(); ) {
                if (it->second.flags & CCoinsCacheEntry::DIRTY) {
                    // Same optimization used in CCoinsViewDB is to only write dirty entries.
                    map_[it->first] = it->second.coin;
                    if (it->second.coin.IsSpent() && InsecureRandRange(3) == 0) {
                        // Randomly delete empty entries on write.
                        map_.erase(it->first);
                    }
                }
                mapCoins.erase(it++);
            }
            if (!hashBlock.IsNull())
                hashBestBlock_ = hashBlock;
            return true;
        */
    }
}

///-------------------
pub struct CoinsViewCacheTest {
    base: CoinsViewCache,
}

impl CoinsViewCacheTest {
    
    pub fn new(base: Rc<RefCell<dyn CoinsView>>) -> Self {
    
        todo!();
        /*
        : coins_view_cache(_base),
        */
    }
    
    pub fn self_test(&self)  {
        
        todo!();
        /*
            // Manually recompute the dynamic usage of the whole data, and compare it.
            size_t ret = memusage::DynamicUsage(cacheCoins);
            size_t count = 0;
            for (const auto& entry : cacheCoins) {
                ret += entry.second.coin.DynamicMemoryUsage();
                ++count;
            }
            BOOST_CHECK_EQUAL(GetCacheSize(), count);
            BOOST_CHECK_EQUAL(DynamicMemoryUsage(), ret);
        */
    }
    
    pub fn map(&self) -> &mut CoinsMap {
        
        todo!();
        /*
            return cacheCoins;
        */
    }
    
    pub fn usage(&self) -> &mut usize {
        
        todo!();
        /*
            return cachedCoinsUsage;
        */
    }
}

///-------------------
#[cfg(test)]
#[BasicTestingSetup]
pub mod coins_tests {

    pub const NUM_SIMULATION_ITERATIONS: u32 = 40000;

    /**
      | This is a large randomized insert/remove
      | simulation test on a variable-size stack of
      | caches on top of CCoinsViewTest.
      |
      | It will randomly create/update/delete Coin
      | entries to a tip of caches, with txids picked
      | from a limited list of random 256-bit
      | hashes. Occasionally, a new tip is added to the
      | stack of caches, or the tip is flushed and
      | removed.
      |
      | During the process, booleans are kept to make
      | sure that the randomized operation hits all
      | branches.
      |
      | If fake_best_block is true, assign a random
      | uint256 to mock the recording of best block on
      | flush. This is necessary when using
      | CCoinsViewDB as the base, otherwise we'll hit
      | an assertion in BatchWrite.
      |
      */
    pub fn simulation_test(
            base:            *mut CoinsView,
            fake_best_block: bool)  {
        
        todo!();
            /*
                // Various coverage trackers.
            bool removed_all_caches = false;
            bool reached_4_caches = false;
            bool added_an_entry = false;
            bool added_an_unspendable_entry = false;
            bool removed_an_entry = false;
            bool updated_an_entry = false;
            bool found_an_entry = false;
            bool missed_an_entry = false;
            bool uncached_an_entry = false;

            // A simple map to track what we expect the cache stack to represent.
            std::map<OutPoint, Coin> result;

            // The cache stack.
            std::vector<CCoinsViewCacheTest*> stack; // A stack of CCoinsViewCaches on top.
            stack.push_back(new CCoinsViewCacheTest(base)); // Start with one cache.

            // Use a limited set of random transaction ids, so we do test overwriting entries.
            std::vector<uint256> txids;
            txids.resize(NUM_SIMULATION_ITERATIONS / 8);
            for (unsigned int i = 0; i < txids.size(); i++) {
                txids[i] = InsecureRand256();
            }

            for (unsigned int i = 0; i < NUM_SIMULATION_ITERATIONS; i++) {
                // Do a random modification.
                {
                    uint256 txid = txids[InsecureRandRange(txids.size())]; // txid we're going to modify in this iteration.
                    Coin& coin = result[OutPoint(txid, 0)];

                    // Determine whether to test HaveCoin before or after Access* (or both). As these functions
                    // can influence each other's behaviour by pulling things into the cache, all combinations
                    // are tested.
                    bool test_havecoin_before = InsecureRandBits(2) == 0;
                    bool test_havecoin_after = InsecureRandBits(2) == 0;

                    bool result_havecoin = test_havecoin_before ? stack.back()->HaveCoin(OutPoint(txid, 0)) : false;
                    const Coin& entry = (InsecureRandRange(500) == 0) ? AccessByTxid(*stack.back(), txid) : stack.back()->AccessCoin(OutPoint(txid, 0));
                    BOOST_CHECK(coin == entry);
                    BOOST_CHECK(!test_havecoin_before || result_havecoin == !entry.IsSpent());

                    if (test_havecoin_after) {
                        bool ret = stack.back()->HaveCoin(OutPoint(txid, 0));
                        BOOST_CHECK(ret == !entry.IsSpent());
                    }

                    if (InsecureRandRange(5) == 0 || coin.IsSpent()) {
                        Coin newcoin;
                        newcoin.out.nValue = InsecureRand32();
                        newcoin.nHeight = 1;
                        if (InsecureRandRange(16) == 0 && coin.IsSpent()) {
                            newcoin.out.scriptPubKey.assign(1 + InsecureRandBits(6), OP_RETURN);
                            BOOST_CHECK(newcoin.out.scriptPubKey.IsUnspendable());
                            added_an_unspendable_entry = true;
                        } else {
                            newcoin.out.scriptPubKey.assign(InsecureRandBits(6), 0); // Random sizes so we can test memory usage accounting
                            (coin.IsSpent() ? added_an_entry : updated_an_entry) = true;
                            coin = newcoin;
                        }
                        stack.back()->AddCoin(OutPoint(txid, 0), std::move(newcoin), !coin.IsSpent() || InsecureRand32() & 1);
                    } else {
                        removed_an_entry = true;
                        coin.Clear();
                        BOOST_CHECK(stack.back()->SpendCoin(OutPoint(txid, 0)));
                    }
                }

                // One every 10 iterations, remove a random entry from the cache
                if (InsecureRandRange(10) == 0) {
                    OutPoint out(txids[InsecureRand32() % txids.size()], 0);
                    int cacheid = InsecureRand32() % stack.size();
                    stack[cacheid]->Uncache(out);
                    uncached_an_entry |= !stack[cacheid]->HaveCoinInCache(out);
                }

                // Once every 1000 iterations and at the end, verify the full cache.
                if (InsecureRandRange(1000) == 1 || i == NUM_SIMULATION_ITERATIONS - 1) {
                    for (const auto& entry : result) {
                        bool have = stack.back()->HaveCoin(entry.first);
                        const Coin& coin = stack.back()->AccessCoin(entry.first);
                        BOOST_CHECK(have == !coin.IsSpent());
                        BOOST_CHECK(coin == entry.second);
                        if (coin.IsSpent()) {
                            missed_an_entry = true;
                        } else {
                            BOOST_CHECK(stack.back()->HaveCoinInCache(entry.first));
                            found_an_entry = true;
                        }
                    }
                    for (const CCoinsViewCacheTest *test : stack) {
                        test->SelfTest();
                    }
                }

                if (InsecureRandRange(100) == 0) {
                    // Every 100 iterations, flush an intermediate cache
                    if (stack.size() > 1 && InsecureRandBool() == 0) {
                        unsigned int flushIndex = InsecureRandRange(stack.size() - 1);
                        if (fake_best_block) stack[flushIndex]->SetBestBlock(InsecureRand256());
                        BOOST_CHECK(stack[flushIndex]->Flush());
                    }
                }
                if (InsecureRandRange(100) == 0) {
                    // Every 100 iterations, change the cache stack.
                    if (stack.size() > 0 && InsecureRandBool() == 0) {
                        //Remove the top cache
                        if (fake_best_block) stack.back()->SetBestBlock(InsecureRand256());
                        BOOST_CHECK(stack.back()->Flush());
                        delete stack.back();
                        stack.pop_back();
                    }
                    if (stack.size() == 0 || (stack.size() < 4 && InsecureRandBool())) {
                        //Add a new cache
                        CCoinsView* tip = base;
                        if (stack.size() > 0) {
                            tip = stack.back();
                        } else {
                            removed_all_caches = true;
                        }
                        stack.push_back(new CCoinsViewCacheTest(tip));
                        if (stack.size() == 4) {
                            reached_4_caches = true;
                        }
                    }
                }
            }

            // Clean up the stack.
            while (stack.size() > 0) {
                delete stack.back();
                stack.pop_back();
            }

            // Verify coverage.
            BOOST_CHECK(removed_all_caches);
            BOOST_CHECK(reached_4_caches);
            BOOST_CHECK(added_an_entry);
            BOOST_CHECK(added_an_unspendable_entry);
            BOOST_CHECK(removed_an_entry);
            BOOST_CHECK(updated_an_entry);
            BOOST_CHECK(found_an_entry);
            BOOST_CHECK(missed_an_entry);
            BOOST_CHECK(uncached_an_entry);
            */
    }

    /**
      | Run the above simulation for multiple
      | base types.
      |
      */
    #[test] fn coins_cache_simulation_test() {
        todo!();
        /*
        
            CCoinsViewTest base;
            SimulationTest(&base, false);

            CCoinsViewDB db_base{"test", /*nCacheSize*/ 1 << 23, /*fMemory*/ true, /*fWipe*/ false};
            SimulationTest(&db_base, true);

        */
    }

    /**
      | Store of all necessary tx and undo data
      | for next test
      |
      */
    pub type UtxoData = HashMap<OutPoint,(Transaction,TxUndo,Coin)>;

    lazy_static!{
        /*
        UtxoData utxoData;
        */
    }

    pub fn find_random_from(utxo_set: &HashSet<OutPoint>) -> UtxoData::iterator {
        
        todo!();
            /*
                assert(utxoSet.size());
            auto utxoSetIt = utxoSet.lower_bound(OutPoint(InsecureRand256(), 0));
            if (utxoSetIt == utxoSet.end()) {
                utxoSetIt = utxoSet.begin();
            }
            auto utxoDataIt = utxoData.find(*utxoSetIt);
            assert(utxoDataIt != utxoData.end());
            return utxoDataIt;
            */
    }

    /**
      | This test is similar to the previous test
      | except the emphasis is on testing the
      | functionality of UpdateCoins random txs are
      | created and UpdateCoins is used to update the
      | cache stack
      |
      | In particular it is tested that spending
      | a duplicate coinbase tx has the expected effect
      | (the other duplicate is overwritten at all
      | cache levels)
      */
    #[test] fn updatecoins_simulation_test() {
        todo!();
        /*
        
            SeedInsecureRand(SeedRand::ZEROS);
            g_mock_deterministic_tests = true;

            bool spent_a_duplicate_coinbase = false;
            // A simple map to track what we expect the cache stack to represent.
            std::map<OutPoint, Coin> result;

            // The cache stack.
            CCoinsViewTest base; // A CCoinsViewTest at the bottom.
            std::vector<CCoinsViewCacheTest*> stack; // A stack of CCoinsViewCaches on top.
            stack.push_back(new CCoinsViewCacheTest(&base)); // Start with one cache.

            // Track the txids we've used in various sets
            std::set<OutPoint> coinbase_coins;
            std::set<OutPoint> disconnected_coins;
            std::set<OutPoint> duplicate_coins;
            std::set<OutPoint> utxoset;

            for (unsigned int i = 0; i < NUM_SIMULATION_ITERATIONS; i++) {
                uint32_t randiter = InsecureRand32();

                // 19/20 txs add a new transaction
                if (randiter % 20 < 19) {
                    CMutableTransaction tx;
                    tx.vin.resize(1);
                    tx.vout.resize(1);
                    tx.vout[0].nValue = i; //Keep txs unique unless intended to duplicate
                    tx.vout[0].scriptPubKey.assign(InsecureRand32() & 0x3F, 0); // Random sizes so we can test memory usage accounting
                    unsigned int height = InsecureRand32();
                    Coin old_coin;

                    // 2/20 times create a new coinbase
                    if (randiter % 20 < 2 || coinbase_coins.size() < 10) {
                        // 1/10 of those times create a duplicate coinbase
                        if (InsecureRandRange(10) == 0 && coinbase_coins.size()) {
                            auto utxod = FindRandomFrom(coinbase_coins);
                            // Reuse the exact same coinbase
                            tx = CMutableTransaction{std::get<0>(utxod->second)};
                            // shouldn't be available for reconnection if it's been duplicated
                            disconnected_coins.erase(utxod->first);

                            duplicate_coins.insert(utxod->first);
                        }
                        else {
                            coinbase_coins.insert(OutPoint(tx.GetHash(), 0));
                        }
                        assert(CTransaction(tx).IsCoinBase());
                    }

                    // 17/20 times reconnect previous or add a regular tx
                    else {

                        OutPoint prevout;
                        // 1/20 times reconnect a previously disconnected tx
                        if (randiter % 20 == 2 && disconnected_coins.size()) {
                            auto utxod = FindRandomFrom(disconnected_coins);
                            tx = CMutableTransaction{std::get<0>(utxod->second)};
                            prevout = tx.vin[0].prevout;
                            if (!CTransaction(tx).IsCoinBase() && !utxoset.count(prevout)) {
                                disconnected_coins.erase(utxod->first);
                                continue;
                            }

                            // If this tx is already IN the UTXO, then it must be a coinbase, and it must be a duplicate
                            if (utxoset.count(utxod->first)) {
                                assert(CTransaction(tx).IsCoinBase());
                                assert(duplicate_coins.count(utxod->first));
                            }
                            disconnected_coins.erase(utxod->first);
                        }

                        // 16/20 times create a regular tx
                        else {
                            auto utxod = FindRandomFrom(utxoset);
                            prevout = utxod->first;

                            // Construct the tx to spend the coins of prevouthash
                            tx.vin[0].prevout = prevout;
                            assert(!CTransaction(tx).IsCoinBase());
                        }
                        // In this simple test coins only have two states, spent or unspent, save the unspent state to restore
                        old_coin = result[prevout];
                        // Update the expected result of prevouthash to know these coins are spent
                        result[prevout].Clear();

                        utxoset.erase(prevout);

                        // The test is designed to ensure spending a duplicate coinbase will work properly
                        // if that ever happens and not resurrect the previously overwritten coinbase
                        if (duplicate_coins.count(prevout)) {
                            spent_a_duplicate_coinbase = true;
                        }

                    }
                    // Update the expected result to know about the new output coins
                    assert(tx.vout.size() == 1);
                    const OutPoint outpoint(tx.GetHash(), 0);
                    result[outpoint] = Coin(tx.vout[0], height, CTransaction(tx).IsCoinBase());

                    // Call UpdateCoins on the top cache
                    CTxUndo undo;
                    UpdateCoins(CTransaction(tx), *(stack.back()), undo, height);

                    // Update the utxo set for future spends
                    utxoset.insert(outpoint);

                    // Track this tx and undo info to use later
                    utxoData.emplace(outpoint, std::make_tuple(tx,undo,old_coin));
                } else if (utxoset.size()) {
                    //1/20 times undo a previous transaction
                    auto utxod = FindRandomFrom(utxoset);

                    CTransaction &tx = std::get<0>(utxod->second);
                    CTxUndo &undo = std::get<1>(utxod->second);
                    Coin &orig_coin = std::get<2>(utxod->second);

                    // Update the expected result
                    // Remove new outputs
                    result[utxod->first].Clear();
                    // If not coinbase restore prevout
                    if (!tx.IsCoinBase()) {
                        result[tx.vin[0].prevout] = orig_coin;
                    }

                    // Disconnect the tx from the current UTXO
                    // See code in DisconnectBlock
                    // remove outputs
                    BOOST_CHECK(stack.back()->SpendCoin(utxod->first));
                    // restore inputs
                    if (!tx.IsCoinBase()) {
                        const OutPoint &out = tx.vin[0].prevout;
                        Coin coin = undo.vprevout[0];
                        ApplyTxInUndo(std::move(coin), *(stack.back()), out);
                    }
                    // Store as a candidate for reconnection
                    disconnected_coins.insert(utxod->first);

                    // Update the utxoset
                    utxoset.erase(utxod->first);
                    if (!tx.IsCoinBase())
                        utxoset.insert(tx.vin[0].prevout);
                }

                // Once every 1000 iterations and at the end, verify the full cache.
                if (InsecureRandRange(1000) == 1 || i == NUM_SIMULATION_ITERATIONS - 1) {
                    for (const auto& entry : result) {
                        bool have = stack.back()->HaveCoin(entry.first);
                        const Coin& coin = stack.back()->AccessCoin(entry.first);
                        BOOST_CHECK(have == !coin.IsSpent());
                        BOOST_CHECK(coin == entry.second);
                    }
                }

                // One every 10 iterations, remove a random entry from the cache
                if (utxoset.size() > 1 && InsecureRandRange(30) == 0) {
                    stack[InsecureRand32() % stack.size()]->Uncache(FindRandomFrom(utxoset)->first);
                }
                if (disconnected_coins.size() > 1 && InsecureRandRange(30) == 0) {
                    stack[InsecureRand32() % stack.size()]->Uncache(FindRandomFrom(disconnected_coins)->first);
                }
                if (duplicate_coins.size() > 1 && InsecureRandRange(30) == 0) {
                    stack[InsecureRand32() % stack.size()]->Uncache(FindRandomFrom(duplicate_coins)->first);
                }

                if (InsecureRandRange(100) == 0) {
                    // Every 100 iterations, flush an intermediate cache
                    if (stack.size() > 1 && InsecureRandBool() == 0) {
                        unsigned int flushIndex = InsecureRandRange(stack.size() - 1);
                        BOOST_CHECK(stack[flushIndex]->Flush());
                    }
                }
                if (InsecureRandRange(100) == 0) {
                    // Every 100 iterations, change the cache stack.
                    if (stack.size() > 0 && InsecureRandBool() == 0) {
                        BOOST_CHECK(stack.back()->Flush());
                        delete stack.back();
                        stack.pop_back();
                    }
                    if (stack.size() == 0 || (stack.size() < 4 && InsecureRandBool())) {
                        CCoinsView* tip = &base;
                        if (stack.size() > 0) {
                            tip = stack.back();
                        }
                        stack.push_back(new CCoinsViewCacheTest(tip));
                    }
                }
            }

            // Clean up the stack.
            while (stack.size() > 0) {
                delete stack.back();
                stack.pop_back();
            }

            // Verify coverage.
            BOOST_CHECK(spent_a_duplicate_coinbase);

            g_mock_deterministic_tests = false;

        */
    }

    #[test] fn ccoins_serialization() {
        todo!();
        /*
        
            // Good example
            DataStream ss1(ParseHex("97f23c835800816115944e077fe7c803cfa57f29b36bf87c1d35"), SER_DISK, CLIENT_VERSION);
            Coin cc1;
            ss1 >> cc1;
            BOOST_CHECK_EQUAL(cc1.fCoinBase, false);
            BOOST_CHECK_EQUAL(cc1.nHeight, 203998U);
            BOOST_CHECK_EQUAL(cc1.out.nValue, CAmount{60000000000});
            BOOST_CHECK_EQUAL(HexStr(cc1.out.scriptPubKey), HexStr(GetScriptForDestination(PKHash(u160(ParseHex("816115944e077fe7c803cfa57f29b36bf87c1d35"))))));

            // Good example
            DataStream ss2(ParseHex("8ddf77bbd123008c988f1a4a4de2161e0f50aac7f17e7f9555caa4"), SER_DISK, CLIENT_VERSION);
            Coin cc2;
            ss2 >> cc2;
            BOOST_CHECK_EQUAL(cc2.fCoinBase, true);
            BOOST_CHECK_EQUAL(cc2.nHeight, 120891U);
            BOOST_CHECK_EQUAL(cc2.out.nValue, 110397);
            BOOST_CHECK_EQUAL(HexStr(cc2.out.scriptPubKey), HexStr(GetScriptForDestination(PKHash(u160(ParseHex("8c988f1a4a4de2161e0f50aac7f17e7f9555caa4"))))));

            // Smallest possible example
            DataStream ss3(ParseHex("000006"), SER_DISK, CLIENT_VERSION);
            Coin cc3;
            ss3 >> cc3;
            BOOST_CHECK_EQUAL(cc3.fCoinBase, false);
            BOOST_CHECK_EQUAL(cc3.nHeight, 0U);
            BOOST_CHECK_EQUAL(cc3.out.nValue, 0);
            BOOST_CHECK_EQUAL(cc3.out.scriptPubKey.size(), 0U);

            // scriptPubKey that ends beyond the end of the stream
            DataStream ss4(ParseHex("000007"), SER_DISK, CLIENT_VERSION);
            try {
                Coin cc4;
                ss4 >> cc4;
                BOOST_CHECK_MESSAGE(false, "We should have thrown");
            } catch (const std::ios_base::failure&) {
            }

            // Very large scriptPubKey (3*10^9 bytes) past the end of the stream
            DataStream tmp(SER_DISK, CLIENT_VERSION);
            uint64_t x = 3000000000ULL;
            tmp << VARINT(x);
            BOOST_CHECK_EQUAL(HexStr(tmp), "8a95c0bb00");
            DataStream ss5(ParseHex("00008a95c0bb00"), SER_DISK, CLIENT_VERSION);
            try {
                Coin cc5;
                ss5 >> cc5;
                BOOST_CHECK_MESSAGE(false, "We should have thrown");
            } catch (const std::ios_base::failure&) {
            }

        */
    }

    pub const OUTPOINT: OutPoint = todo!();
    pub const SPENT:    Amount = -1;
    pub const ABSENT:   Amount = -2;
    pub const FAIL:     Amount = -3;
    pub const VALUE1:   Amount = 100;
    pub const VALUE2:   Amount = 200;
    pub const VALUE3:   Amount = 300;
    pub const DIRTY:    u8 = CoinsCacheEntry::DIRTY;
    pub const FRESH:    u8 = CoinsCacheEntry::FRESH;
    pub const NO_ENTRY: u8 = -1;

    pub const FLAGS:        &[u8] = &[0, FRESH, DIRTY, DIRTY | FRESH];
    pub const CLEAN_FLAGS:  &[u8] = &[0, FRESH];
    pub const ABSENT_FLAGS: &[u8] = &[NO_ENTRY];

    pub fn set_coins_value(
            value: Amount,
            coin:  &mut Coin)  {
        
        todo!();
            /*
                assert(value != ABSENT);
            coin.Clear();
            assert(coin.IsSpent());
            if (value != SPENT) {
                coin.out.nValue = value;
                coin.nHeight = 1;
                assert(!coin.IsSpent());
            }
            */
    }

    pub fn insert_coins_map_entry(
            map:   &mut CoinsMap,
            value: Amount,
            flags: u8) -> usize {
        
        todo!();
            /*
                if (value == ABSENT) {
                assert(flags == NO_ENTRY);
                return 0;
            }
            assert(flags != NO_ENTRY);
            CCoinsCacheEntry entry;
            entry.flags = flags;
            SetCoinsValue(value, entry.coin);
            auto inserted = map.emplace(OUTPOINT, std::move(entry));
            assert(inserted.second);
            return inserted.first->second.coin.DynamicMemoryUsage();
            */
    }

    pub fn get_coins_map_entry(
            map:   &CoinsMap,
            value: &mut Amount,
            flags: &mut u8)  {
        
        todo!();
            /*
                auto it = map.find(OUTPOINT);
            if (it == map.end()) {
                value = ABSENT;
                flags = NO_ENTRY;
            } else {
                if (it->second.coin.IsSpent()) {
                    value = SPENT;
                } else {
                    value = it->second.coin.out.nValue;
                }
                flags = it->second.flags;
                assert(flags != NO_ENTRY);
            }
            */
    }

    pub fn write_coins_view_entry(
            view:  &mut CoinsView,
            value: Amount,
            flags: u8)  {
        
        todo!();
            /*
                CCoinsMap map;
            InsertCoinsMapEntry(map, value, flags);
            BOOST_CHECK(view.BatchWrite(map, {}));
            */
    }

    ///--------------------------
    pub struct SingleEntryCacheTest {
        root:  CoinsView,
        base:  CoinsViewCacheTest, //{&root};
        cache: CoinsViewCacheTest, //{&base};
    }

    impl SingleEntryCacheTest {
        
        pub fn new(
            base_value:  Amount,
            cache_value: Amount,
            cache_flags: u8) -> Self {
        
            todo!();
            /*


                WriteCoinsViewEntry(base, base_value, base_value == ABSENT ? NO_ENTRY : DIRTY);
                cache.usage() += InsertCoinsMapEntry(cache.map(), cache_value, cache_flags);
            */
        }
    }

    pub fn check_access_coin(
            base_value:     Amount,
            cache_value:    Amount,
            expected_value: Amount,
            cache_flags:    u8,
            expected_flags: u8)  {
        
        todo!();
            /*
                SingleEntryCacheTest test(base_value, cache_value, cache_flags);
            test.cache.AccessCoin(OUTPOINT);
            test.cache.SelfTest();

            CAmount result_value;
            char result_flags;
            GetCoinsMapEntry(test.cache.map(), result_value, result_flags);
            BOOST_CHECK_EQUAL(result_value, expected_value);
            BOOST_CHECK_EQUAL(result_flags, expected_flags);
            */
    }

    #[test] fn ccoins_access() {
        todo!();
        /*
        
            /* Check AccessCoin behavior, requesting a coin from a cache view layered on
             * top of a base view, and checking the resulting entry in the cache after
             * the access.
             *
             *               Base    Cache   Result  Cache        Result
             *               Value   Value   Value   Flags        Flags
             */
            CheckAccessCoin(ABSENT, ABSENT, ABSENT, NO_ENTRY   , NO_ENTRY   );
            CheckAccessCoin(ABSENT, SPENT , SPENT , 0          , 0          );
            CheckAccessCoin(ABSENT, SPENT , SPENT , FRESH      , FRESH      );
            CheckAccessCoin(ABSENT, SPENT , SPENT , DIRTY      , DIRTY      );
            CheckAccessCoin(ABSENT, SPENT , SPENT , DIRTY|FRESH, DIRTY|FRESH);
            CheckAccessCoin(ABSENT, VALUE2, VALUE2, 0          , 0          );
            CheckAccessCoin(ABSENT, VALUE2, VALUE2, FRESH      , FRESH      );
            CheckAccessCoin(ABSENT, VALUE2, VALUE2, DIRTY      , DIRTY      );
            CheckAccessCoin(ABSENT, VALUE2, VALUE2, DIRTY|FRESH, DIRTY|FRESH);
            CheckAccessCoin(SPENT , ABSENT, ABSENT, NO_ENTRY   , NO_ENTRY   );
            CheckAccessCoin(SPENT , SPENT , SPENT , 0          , 0          );
            CheckAccessCoin(SPENT , SPENT , SPENT , FRESH      , FRESH      );
            CheckAccessCoin(SPENT , SPENT , SPENT , DIRTY      , DIRTY      );
            CheckAccessCoin(SPENT , SPENT , SPENT , DIRTY|FRESH, DIRTY|FRESH);
            CheckAccessCoin(SPENT , VALUE2, VALUE2, 0          , 0          );
            CheckAccessCoin(SPENT , VALUE2, VALUE2, FRESH      , FRESH      );
            CheckAccessCoin(SPENT , VALUE2, VALUE2, DIRTY      , DIRTY      );
            CheckAccessCoin(SPENT , VALUE2, VALUE2, DIRTY|FRESH, DIRTY|FRESH);
            CheckAccessCoin(VALUE1, ABSENT, VALUE1, NO_ENTRY   , 0          );
            CheckAccessCoin(VALUE1, SPENT , SPENT , 0          , 0          );
            CheckAccessCoin(VALUE1, SPENT , SPENT , FRESH      , FRESH      );
            CheckAccessCoin(VALUE1, SPENT , SPENT , DIRTY      , DIRTY      );
            CheckAccessCoin(VALUE1, SPENT , SPENT , DIRTY|FRESH, DIRTY|FRESH);
            CheckAccessCoin(VALUE1, VALUE2, VALUE2, 0          , 0          );
            CheckAccessCoin(VALUE1, VALUE2, VALUE2, FRESH      , FRESH      );
            CheckAccessCoin(VALUE1, VALUE2, VALUE2, DIRTY      , DIRTY      );
            CheckAccessCoin(VALUE1, VALUE2, VALUE2, DIRTY|FRESH, DIRTY|FRESH);

        */
    }

    pub fn check_spend_coins(
            base_value:     Amount,
            cache_value:    Amount,
            expected_value: Amount,
            cache_flags:    u8,
            expected_flags: u8)  {
        
        todo!();
            /*
                SingleEntryCacheTest test(base_value, cache_value, cache_flags);
            test.cache.SpendCoin(OUTPOINT);
            test.cache.SelfTest();

            CAmount result_value;
            char result_flags;
            GetCoinsMapEntry(test.cache.map(), result_value, result_flags);
            BOOST_CHECK_EQUAL(result_value, expected_value);
            BOOST_CHECK_EQUAL(result_flags, expected_flags);
            */
    }

    #[test] fn ccoins_spend() {
        todo!();
        /*
        
            /* Check SpendCoin behavior, requesting a coin from a cache view layered on
             * top of a base view, spending, and then checking
             * the resulting entry in the cache after the modification.
             *
             *              Base    Cache   Result  Cache        Result
             *              Value   Value   Value   Flags        Flags
             */
            CheckSpendCoins(ABSENT, ABSENT, ABSENT, NO_ENTRY   , NO_ENTRY   );
            CheckSpendCoins(ABSENT, SPENT , SPENT , 0          , DIRTY      );
            CheckSpendCoins(ABSENT, SPENT , ABSENT, FRESH      , NO_ENTRY   );
            CheckSpendCoins(ABSENT, SPENT , SPENT , DIRTY      , DIRTY      );
            CheckSpendCoins(ABSENT, SPENT , ABSENT, DIRTY|FRESH, NO_ENTRY   );
            CheckSpendCoins(ABSENT, VALUE2, SPENT , 0          , DIRTY      );
            CheckSpendCoins(ABSENT, VALUE2, ABSENT, FRESH      , NO_ENTRY   );
            CheckSpendCoins(ABSENT, VALUE2, SPENT , DIRTY      , DIRTY      );
            CheckSpendCoins(ABSENT, VALUE2, ABSENT, DIRTY|FRESH, NO_ENTRY   );
            CheckSpendCoins(SPENT , ABSENT, ABSENT, NO_ENTRY   , NO_ENTRY   );
            CheckSpendCoins(SPENT , SPENT , SPENT , 0          , DIRTY      );
            CheckSpendCoins(SPENT , SPENT , ABSENT, FRESH      , NO_ENTRY   );
            CheckSpendCoins(SPENT , SPENT , SPENT , DIRTY      , DIRTY      );
            CheckSpendCoins(SPENT , SPENT , ABSENT, DIRTY|FRESH, NO_ENTRY   );
            CheckSpendCoins(SPENT , VALUE2, SPENT , 0          , DIRTY      );
            CheckSpendCoins(SPENT , VALUE2, ABSENT, FRESH      , NO_ENTRY   );
            CheckSpendCoins(SPENT , VALUE2, SPENT , DIRTY      , DIRTY      );
            CheckSpendCoins(SPENT , VALUE2, ABSENT, DIRTY|FRESH, NO_ENTRY   );
            CheckSpendCoins(VALUE1, ABSENT, SPENT , NO_ENTRY   , DIRTY      );
            CheckSpendCoins(VALUE1, SPENT , SPENT , 0          , DIRTY      );
            CheckSpendCoins(VALUE1, SPENT , ABSENT, FRESH      , NO_ENTRY   );
            CheckSpendCoins(VALUE1, SPENT , SPENT , DIRTY      , DIRTY      );
            CheckSpendCoins(VALUE1, SPENT , ABSENT, DIRTY|FRESH, NO_ENTRY   );
            CheckSpendCoins(VALUE1, VALUE2, SPENT , 0          , DIRTY      );
            CheckSpendCoins(VALUE1, VALUE2, ABSENT, FRESH      , NO_ENTRY   );
            CheckSpendCoins(VALUE1, VALUE2, SPENT , DIRTY      , DIRTY      );
            CheckSpendCoins(VALUE1, VALUE2, ABSENT, DIRTY|FRESH, NO_ENTRY   );

        */
    }

    pub fn check_add_coin_base(
            base_value:     Amount,
            cache_value:    Amount,
            modify_value:   Amount,
            expected_value: Amount,
            cache_flags:    u8,
            expected_flags: u8,
            coinbase:       bool)  {
        
        todo!();
            /*
                SingleEntryCacheTest test(base_value, cache_value, cache_flags);

            CAmount result_value;
            char result_flags;
            try {
                CTxOut output;
                output.nValue = modify_value;
                test.cache.AddCoin(OUTPOINT, Coin(std::move(output), 1, coinbase), coinbase);
                test.cache.SelfTest();
                GetCoinsMapEntry(test.cache.map(), result_value, result_flags);
            } catch (std::logic_error&) {
                result_value = FAIL;
                result_flags = NO_ENTRY;
            }

            BOOST_CHECK_EQUAL(result_value, expected_value);
            BOOST_CHECK_EQUAL(result_flags, expected_flags);
            */
    }

    /**
      | Simple wrapper for CheckAddCoinBase function
      | above that loops through different possible
      | base_values, making sure each one gives the
      | same results.
      |
      | This wrapper lets the coins_add test below be
      | shorter and less repetitive, while still
      | verifying that the CoinsViewCache::AddCoin
      | implementation ignores base values.
      */
    pub fn check_add_coin<Args>(args: Args)  {

        todo!();
            /*
                for (const CAmount base_value : {ABSENT, SPENT, VALUE1})
                CheckAddCoinBase(base_value, std::forward<Args>(args)...);
            */
    }

    #[test] fn ccoins_add() {
        todo!();
        /*
        
            /* Check AddCoin behavior, requesting a new coin from a cache view,
             * writing a modification to the coin, and then checking the resulting
             * entry in the cache after the modification. Verify behavior with the
             * AddCoin possible_overwrite argument set to false, and to true.
             *
             *           Cache   Write   Result  Cache        Result       possible_overwrite
             *           Value   Value   Value   Flags        Flags
             */
            CheckAddCoin(ABSENT, VALUE3, VALUE3, NO_ENTRY   , DIRTY|FRESH, false);
            CheckAddCoin(ABSENT, VALUE3, VALUE3, NO_ENTRY   , DIRTY      , true );
            CheckAddCoin(SPENT , VALUE3, VALUE3, 0          , DIRTY|FRESH, false);
            CheckAddCoin(SPENT , VALUE3, VALUE3, 0          , DIRTY      , true );
            CheckAddCoin(SPENT , VALUE3, VALUE3, FRESH      , DIRTY|FRESH, false);
            CheckAddCoin(SPENT , VALUE3, VALUE3, FRESH      , DIRTY|FRESH, true );
            CheckAddCoin(SPENT , VALUE3, VALUE3, DIRTY      , DIRTY      , false);
            CheckAddCoin(SPENT , VALUE3, VALUE3, DIRTY      , DIRTY      , true );
            CheckAddCoin(SPENT , VALUE3, VALUE3, DIRTY|FRESH, DIRTY|FRESH, false);
            CheckAddCoin(SPENT , VALUE3, VALUE3, DIRTY|FRESH, DIRTY|FRESH, true );
            CheckAddCoin(VALUE2, VALUE3, FAIL  , 0          , NO_ENTRY   , false);
            CheckAddCoin(VALUE2, VALUE3, VALUE3, 0          , DIRTY      , true );
            CheckAddCoin(VALUE2, VALUE3, FAIL  , FRESH      , NO_ENTRY   , false);
            CheckAddCoin(VALUE2, VALUE3, VALUE3, FRESH      , DIRTY|FRESH, true );
            CheckAddCoin(VALUE2, VALUE3, FAIL  , DIRTY      , NO_ENTRY   , false);
            CheckAddCoin(VALUE2, VALUE3, VALUE3, DIRTY      , DIRTY      , true );
            CheckAddCoin(VALUE2, VALUE3, FAIL  , DIRTY|FRESH, NO_ENTRY   , false);
            CheckAddCoin(VALUE2, VALUE3, VALUE3, DIRTY|FRESH, DIRTY|FRESH, true );

        */
    }

    pub fn check_write_coins(
            parent_value:   Amount,
            child_value:    Amount,
            expected_value: Amount,
            parent_flags:   u8,
            child_flags:    u8,
            expected_flags: u8)  {
        
        todo!();
            /*
                SingleEntryCacheTest test(ABSENT, parent_value, parent_flags);

            CAmount result_value;
            char result_flags;
            try {
                WriteCoinsViewEntry(test.cache, child_value, child_flags);
                test.cache.SelfTest();
                GetCoinsMapEntry(test.cache.map(), result_value, result_flags);
            } catch (std::logic_error&) {
                result_value = FAIL;
                result_flags = NO_ENTRY;
            }

            BOOST_CHECK_EQUAL(result_value, expected_value);
            BOOST_CHECK_EQUAL(result_flags, expected_flags);
            */
    }

    #[test] fn ccoins_write() {
        todo!();
        /*
        
            /* Check BatchWrite behavior, flushing one entry from a child cache to a
             * parent cache, and checking the resulting entry in the parent cache
             * after the write.
             *
             *              Parent  Child   Result  Parent       Child        Result
             *              Value   Value   Value   Flags        Flags        Flags
             */
            CheckWriteCoins(ABSENT, ABSENT, ABSENT, NO_ENTRY   , NO_ENTRY   , NO_ENTRY   );
            CheckWriteCoins(ABSENT, SPENT , SPENT , NO_ENTRY   , DIRTY      , DIRTY      );
            CheckWriteCoins(ABSENT, SPENT , ABSENT, NO_ENTRY   , DIRTY|FRESH, NO_ENTRY   );
            CheckWriteCoins(ABSENT, VALUE2, VALUE2, NO_ENTRY   , DIRTY      , DIRTY      );
            CheckWriteCoins(ABSENT, VALUE2, VALUE2, NO_ENTRY   , DIRTY|FRESH, DIRTY|FRESH);
            CheckWriteCoins(SPENT , ABSENT, SPENT , 0          , NO_ENTRY   , 0          );
            CheckWriteCoins(SPENT , ABSENT, SPENT , FRESH      , NO_ENTRY   , FRESH      );
            CheckWriteCoins(SPENT , ABSENT, SPENT , DIRTY      , NO_ENTRY   , DIRTY      );
            CheckWriteCoins(SPENT , ABSENT, SPENT , DIRTY|FRESH, NO_ENTRY   , DIRTY|FRESH);
            CheckWriteCoins(SPENT , SPENT , SPENT , 0          , DIRTY      , DIRTY      );
            CheckWriteCoins(SPENT , SPENT , SPENT , 0          , DIRTY|FRESH, DIRTY      );
            CheckWriteCoins(SPENT , SPENT , ABSENT, FRESH      , DIRTY      , NO_ENTRY   );
            CheckWriteCoins(SPENT , SPENT , ABSENT, FRESH      , DIRTY|FRESH, NO_ENTRY   );
            CheckWriteCoins(SPENT , SPENT , SPENT , DIRTY      , DIRTY      , DIRTY      );
            CheckWriteCoins(SPENT , SPENT , SPENT , DIRTY      , DIRTY|FRESH, DIRTY      );
            CheckWriteCoins(SPENT , SPENT , ABSENT, DIRTY|FRESH, DIRTY      , NO_ENTRY   );
            CheckWriteCoins(SPENT , SPENT , ABSENT, DIRTY|FRESH, DIRTY|FRESH, NO_ENTRY   );
            CheckWriteCoins(SPENT , VALUE2, VALUE2, 0          , DIRTY      , DIRTY      );
            CheckWriteCoins(SPENT , VALUE2, VALUE2, 0          , DIRTY|FRESH, DIRTY      );
            CheckWriteCoins(SPENT , VALUE2, VALUE2, FRESH      , DIRTY      , DIRTY|FRESH);
            CheckWriteCoins(SPENT , VALUE2, VALUE2, FRESH      , DIRTY|FRESH, DIRTY|FRESH);
            CheckWriteCoins(SPENT , VALUE2, VALUE2, DIRTY      , DIRTY      , DIRTY      );
            CheckWriteCoins(SPENT , VALUE2, VALUE2, DIRTY      , DIRTY|FRESH, DIRTY      );
            CheckWriteCoins(SPENT , VALUE2, VALUE2, DIRTY|FRESH, DIRTY      , DIRTY|FRESH);
            CheckWriteCoins(SPENT , VALUE2, VALUE2, DIRTY|FRESH, DIRTY|FRESH, DIRTY|FRESH);
            CheckWriteCoins(VALUE1, ABSENT, VALUE1, 0          , NO_ENTRY   , 0          );
            CheckWriteCoins(VALUE1, ABSENT, VALUE1, FRESH      , NO_ENTRY   , FRESH      );
            CheckWriteCoins(VALUE1, ABSENT, VALUE1, DIRTY      , NO_ENTRY   , DIRTY      );
            CheckWriteCoins(VALUE1, ABSENT, VALUE1, DIRTY|FRESH, NO_ENTRY   , DIRTY|FRESH);
            CheckWriteCoins(VALUE1, SPENT , SPENT , 0          , DIRTY      , DIRTY      );
            CheckWriteCoins(VALUE1, SPENT , FAIL  , 0          , DIRTY|FRESH, NO_ENTRY   );
            CheckWriteCoins(VALUE1, SPENT , ABSENT, FRESH      , DIRTY      , NO_ENTRY   );
            CheckWriteCoins(VALUE1, SPENT , FAIL  , FRESH      , DIRTY|FRESH, NO_ENTRY   );
            CheckWriteCoins(VALUE1, SPENT , SPENT , DIRTY      , DIRTY      , DIRTY      );
            CheckWriteCoins(VALUE1, SPENT , FAIL  , DIRTY      , DIRTY|FRESH, NO_ENTRY   );
            CheckWriteCoins(VALUE1, SPENT , ABSENT, DIRTY|FRESH, DIRTY      , NO_ENTRY   );
            CheckWriteCoins(VALUE1, SPENT , FAIL  , DIRTY|FRESH, DIRTY|FRESH, NO_ENTRY   );
            CheckWriteCoins(VALUE1, VALUE2, VALUE2, 0          , DIRTY      , DIRTY      );
            CheckWriteCoins(VALUE1, VALUE2, FAIL  , 0          , DIRTY|FRESH, NO_ENTRY   );
            CheckWriteCoins(VALUE1, VALUE2, VALUE2, FRESH      , DIRTY      , DIRTY|FRESH);
            CheckWriteCoins(VALUE1, VALUE2, FAIL  , FRESH      , DIRTY|FRESH, NO_ENTRY   );
            CheckWriteCoins(VALUE1, VALUE2, VALUE2, DIRTY      , DIRTY      , DIRTY      );
            CheckWriteCoins(VALUE1, VALUE2, FAIL  , DIRTY      , DIRTY|FRESH, NO_ENTRY   );
            CheckWriteCoins(VALUE1, VALUE2, VALUE2, DIRTY|FRESH, DIRTY      , DIRTY|FRESH);
            CheckWriteCoins(VALUE1, VALUE2, FAIL  , DIRTY|FRESH, DIRTY|FRESH, NO_ENTRY   );

            // The checks above omit cases where the child flags are not DIRTY, since
            // they would be too repetitive (the parent cache is never updated in these
            // cases). The loop below covers these cases and makes sure the parent cache
            // is always left unchanged.
            for (const CAmount parent_value : {ABSENT, SPENT, VALUE1})
                for (const CAmount child_value : {ABSENT, SPENT, VALUE2})
                    for (const char parent_flags : parent_value == ABSENT ? ABSENT_FLAGS : FLAGS)
                        for (const char child_flags : child_value == ABSENT ? ABSENT_FLAGS : CLEAN_FLAGS)
                            CheckWriteCoins(parent_value, child_value, parent_value, parent_flags, child_flags, parent_flags);

        */
    }
}
