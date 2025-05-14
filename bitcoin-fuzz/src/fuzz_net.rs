// ---------------- [ File: bitcoin-fuzz/src/fuzz_net.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/net.cpp]

pub fn initialize_net()  {
    
    todo!();
        /*
            static const auto testing_setup = MakeNoLogFileContext<>(CBaseChainParams::MAIN);
        */
}

#[fuzz_test(initializer = "initialize_net")]
fn net() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        SetMockTime(ConsumeTime(fuzzed_data_provider));
        Node node{ConsumeNode(fuzzed_data_provider)};
        node.SetCommonVersion(fuzzed_data_provider.ConsumeIntegral<int>());
        while (fuzzed_data_provider.ConsumeBool()) {
            CallOneOf(
                fuzzed_data_provider,
                [&] {
                    node.CloseSocketDisconnect();
                },
                [&] {
                    NodeStats stats;
                    node.CopyStats(stats);
                },
                [&] {
                    const Node* add_ref_node = node.AddRef();
                    assert(add_ref_node == &node);
                },
                [&] {
                    if (node.GetRefCount() > 0) {
                        node.Release();
                    }
                },
                [&] {
                    const std::optional<CInv> inv_opt = ConsumeDeserializable<CInv>(fuzzed_data_provider);
                    if (!inv_opt) {
                        return;
                    }
                    node.AddKnownTx(inv_opt->hash);
                },
                [&] {
                    node.PushTxInventory(ConsumeUInt256(fuzzed_data_provider));
                },
                [&] {
                    const std::optional<CService> service_opt = ConsumeDeserializable<CService>(fuzzed_data_provider);
                    if (!service_opt) {
                        return;
                    }
                    node.SetAddrLocal(*service_opt);
                },
                [&] {
                    const std::vector<uint8_t> b = ConsumeRandomLengthByteVector(fuzzed_data_provider);
                    bool complete;
                    node.ReceiveMsgBytes(b, complete);
                });
        }

        (c_void)node.GetAddrLocal();
        (c_void)node.GetId();
        (c_void)node.GetLocalNonce();
        (c_void)node.GetLocalServices();
        const int ref_count = node.GetRefCount();
        assert(ref_count >= 0);
        (c_void)node.GetCommonVersion();

        const NetPermissionFlags net_permission_flags = ConsumeWeakEnum(fuzzed_data_provider, ALL_NET_PERMISSION_FLAGS);
        (c_void)node.HasPermission(net_permission_flags);
        (c_void)node.ConnectedThroughNetwork();

    */
}
