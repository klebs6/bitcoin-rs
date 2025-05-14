// ---------------- [ File: bitcoin-fuzz/src/fuzz_connman.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/connman.cpp]

pub fn initialize_connman()  {
    
    todo!();
        /*
            static const auto testing_setup = MakeNoLogFileContext<>();
        */
}


#[fuzz_test(initializer = "initialize_connman")]
fn connman() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider{buffer.data(), buffer.size()};
        SetMockTime(ConsumeTime(fuzzed_data_provider));
        AddrMan addrman(/* asmap */ std::vector<bool>(), /* deterministic */ false, /* consistency_check_ratio */ 0);
        CConnman connman{fuzzed_data_provider.ConsumeIntegral<uint64_t>(), fuzzed_data_provider.ConsumeIntegral<uint64_t>(), addrman, fuzzed_data_provider.ConsumeBool()};
        CNetAddr random_netaddr;
        Node random_node = ConsumeNode(fuzzed_data_provider);
        CSubNet random_subnet;
        std::string random_string;
        while (fuzzed_data_provider.ConsumeBool()) {
            CallOneOf(
                fuzzed_data_provider,
                [&] {
                    random_netaddr = ConsumeNetAddr(fuzzed_data_provider);
                },
                [&] {
                    random_subnet = ConsumeSubNet(fuzzed_data_provider);
                },
                [&] {
                    random_string = fuzzed_data_provider.ConsumeRandomLengthString(64);
                },
                [&] {
                    connman.AddNode(random_string);
                },
                [&] {
                    connman.CheckIncomingNonce(fuzzed_data_provider.ConsumeIntegral<uint64_t>());
                },
                [&] {
                    connman.DisconnectNode(fuzzed_data_provider.ConsumeIntegral<NodeId>());
                },
                [&] {
                    connman.DisconnectNode(random_netaddr);
                },
                [&] {
                    connman.DisconnectNode(random_string);
                },
                [&] {
                    connman.DisconnectNode(random_subnet);
                },
                [&] {
                    connman.ForEachNode([](auto) {});
                },
                [&] {
                    (c_void)connman.ForNode(fuzzed_data_provider.ConsumeIntegral<NodeId>(), [&](auto) { return fuzzed_data_provider.ConsumeBool(); });
                },
                [&] {
                    (c_void)connman.GetAddresses(
                        /* max_addresses */ fuzzed_data_provider.ConsumeIntegral<size_t>(),
                        /* max_pct */ fuzzed_data_provider.ConsumeIntegral<size_t>(),
                        /* network */ std::nullopt);
                },
                [&] {
                    (c_void)connman.GetAddresses(
                        /* requestor */ random_node,
                        /* max_addresses */ fuzzed_data_provider.ConsumeIntegral<size_t>(),
                        /* max_pct */ fuzzed_data_provider.ConsumeIntegral<size_t>());
                },
                [&] {
                    (c_void)connman.GetDeterministicRandomizer(fuzzed_data_provider.ConsumeIntegral<uint64_t>());
                },
                [&] {
                    (c_void)connman.GetNodeCount(fuzzed_data_provider.PickValueInArray({ConnectionDirection::None, ConnectionDirection::In, ConnectionDirection::Out, ConnectionDirection::Both}));
                },
                [&] {
                    (c_void)connman.OutboundTargetReached(fuzzed_data_provider.ConsumeBool());
                },
                [&] {
                    // Limit now to int32_t to avoid signed integer overflow
                    (c_void)connman.PoissonNextSendInbound(
                            std::chrono::microseconds{fuzzed_data_provider.ConsumeIntegral<int32_t>()},
                            std::chrono::seconds{fuzzed_data_provider.ConsumeIntegral<int>()});
                },
                [&] {
                    CSerializedNetMsg serialized_net_msg;
                    serialized_net_msg.m_type = fuzzed_data_provider.ConsumeRandomLengthString(CMessageHeader::COMMAND_SIZE);
                    serialized_net_msg.data = ConsumeRandomLengthByteVector(fuzzed_data_provider);
                    connman.PushMessage(&random_node, std::move(serialized_net_msg));
                },
                [&] {
                    connman.RemoveAddedNode(random_string);
                },
                [&] {
                    connman.SetNetworkActive(fuzzed_data_provider.ConsumeBool());
                },
                [&] {
                    connman.SetTryNewOutboundPeer(fuzzed_data_provider.ConsumeBool());
                });
        }
        (c_void)connman.GetAddedNodeInfo();
        (c_void)connman.GetExtraFullOutboundCount();
        (c_void)connman.GetLocalServices();
        (c_void)connman.GetMaxOutboundTarget();
        (c_void)connman.GetMaxOutboundTimeframe();
        (c_void)connman.GetMaxOutboundTimeLeftInCycle();
        (c_void)connman.GetNetworkActive();
        std::vector<NodeStats> stats;
        connman.GetNodeStats(stats);
        (c_void)connman.GetOutboundTargetBytesLeft();
        (c_void)connman.GetReceiveFloodSize();
        (c_void)connman.GetTotalBytesRecv();
        (c_void)connman.GetTotalBytesSent();
        (c_void)connman.GetTryNewOutboundPeer();
        (c_void)connman.GetUseAddrmanOutgoing();

    */
}
