// ---------------- [ File: bitcoin-fuzz/src/fuzz_utxo_snapshot.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/utxo_snapshot.cpp]

lazy_static!{
    /*
    static const std::vector<std::shared_ptr<CBlock>>* g_chain;
    */
}

pub fn initialize_chain()  {
    
    todo!();
        /*
            const auto params{CreateChainParams(ArgsManager{}, CBaseChainParams::REGTEST)};
        static const auto chain{CreateBlockChain(2 * COINBASE_MATURITY, *params)};
        g_chain = &chain;
        */
}

#[fuzz_test(initializer = "initialize_chain")]
fn utxo_snapshot() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        std::unique_ptr<const TestingSetup> setup{MakeNoLogFileContext<const TestingSetup>()};
        const auto& node = setup->m_node;
        auto& chainman{*node.chainman};

        const auto snapshot_path = gArgs.GetDataDirNet() / "fuzzed_snapshot.dat";

        Assert(!chainman.SnapshotBlockhash());

        {
            CAutoFile outfile{fsbridge::fopen(snapshot_path, "wb"), SER_DISK, CLIENT_VERSION};
            const auto file_data{ConsumeRandomLengthByteVector(fuzzed_data_provider)};
            outfile << Span<const uint8_t>{file_data};
        }

        const auto ActivateFuzzedSnapshot{[&] {
            CAutoFile infile{fsbridge::fopen(snapshot_path, "rb"), SER_DISK, CLIENT_VERSION};
            SnapshotMetadata metadata;
            try {
                infile >> metadata;
            } catch (const std::ios_base::failure&) {
                return false;
            }
            return chainman.ActivateSnapshot(infile, metadata, /* in_memory */ true);
        }};

        if (fuzzed_data_provider.ConsumeBool()) {
            for (const auto& block : *g_chain) {
                BlockValidationState dummy;
                bool processed{chainman.ProcessNewBlockHeaders({*block}, dummy, ::Params())};
                Assert(processed);
                const auto* index{
    [&]() { LOCK(::cs_main);  return chainman.m_blockman.LookupBlockIndex(block->GetHash()) }()
    };
                Assert(index);
            }
        }

        if (ActivateFuzzedSnapshot()) {
            LOCK(::cs_main);
            Assert(!chainman.ActiveChainstate().m_from_snapshot_blockhash->IsNull());
            Assert(*chainman.ActiveChainstate().m_from_snapshot_blockhash ==
                   *chainman.SnapshotBlockhash());
            const auto& coinscache{chainman.ActiveChainstate().CoinsTip()};
            int64_t chain_tx{};
            for (const auto& block : *g_chain) {
                Assert(coinscache.HaveCoin(OutPoint{block->vtx.at(0)->GetHash(), 0}));
                const auto* index{chainman.m_blockman.LookupBlockIndex(block->GetHash())};
                const auto num_tx{Assert(index)->nTx};
                Assert(num_tx == 1);
                chain_tx += num_tx;
            }
            Assert(g_chain->size() == coinscache.GetCacheSize());
            Assert(chain_tx == chainman.ActiveTip()->nChainTx);
        } else {
            Assert(!chainman.SnapshotBlockhash());
            Assert(!chainman.ActiveChainstate().m_from_snapshot_blockhash);
        }
        // Snapshot should refuse to load a second time regardless of validity
        Assert(!ActivateFuzzedSnapshot());

    */
}
