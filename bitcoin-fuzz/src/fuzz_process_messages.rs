crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/process_messages.cpp]

lazy_static!{
    /*
    const TestingSetup* g_setup;
    */
}

pub fn initialize_process_messages()  {
    
    todo!();
        /*
            static const auto testing_setup = MakeNoLogFileContext<const TestingSetup>();
        g_setup = testing_setup.get();
        for (int i = 0; i < 2 * COINBASE_MATURITY; i++) {
            MineBlock(g_setup->m_node, CScript() << OP_TRUE);
        }
        SyncWithValidationInterfaceQueue();
        */
}

#[fuzz_test(initializer = "initialize_process_messages")]
fn process_messages() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());

        ConnmanTestMsg& connman = *static_cast<ConnmanTestMsg*>(g_setup->m_node.connman.get());
        TestChainState& chainstate = *static_cast<TestChainState*>(&g_setup->m_node.chainman->ActiveChainstate());
        SetMockTime(1610000000); // any time to successfully reset ibd
        chainstate.ResetIbd();

        std::vector<Node*> peers;
        const auto num_peers_to_add = fuzzed_data_provider.ConsumeIntegralInRange(1, 3);
        for (int i = 0; i < num_peers_to_add; ++i) {
            peers.push_back(ConsumeNodeAsUniquePtr(fuzzed_data_provider, i).release());
            Node& p2p_node = *peers.back();

            const bool successfully_connected{fuzzed_data_provider.ConsumeBool()};
            p2p_node.fSuccessfullyConnected = successfully_connected;
            p2p_node.fPauseSend = false;
            g_setup->m_node.peerman->InitializeNode(&p2p_node);
            FillNode(fuzzed_data_provider, p2p_node, /* init_version */ successfully_connected);

            connman.AddTestNode(p2p_node);
        }

        while (fuzzed_data_provider.ConsumeBool()) {
            const std::string random_message_type{fuzzed_data_provider.ConsumeBytesAsString(CMessageHeader::COMMAND_SIZE).c_str()};

            const auto mock_time = ConsumeTime(fuzzed_data_provider);
            SetMockTime(mock_time);

            CSerializedNetMsg net_msg;
            net_msg.m_type = random_message_type;
            net_msg.data = ConsumeRandomLengthByteVector(fuzzed_data_provider);

            Node& random_node = *PickValue(fuzzed_data_provider, peers);

            (c_void)connman.ReceiveMsgFrom(random_node, net_msg);
            random_node.fPauseSend = false;

            try {
                connman.ProcessMessagesOnce(random_node);
            } catch (const std::ios_base::failure&) {
            }
            {
                LOCK(random_node.cs_sendProcessing);
                g_setup->m_node.peerman->SendMessages(&random_node);
            }
        }
        SyncWithValidationInterfaceQueue();
        g_setup->m_node.connman->StopNodes();

    */
}
