// ---------------- [ File: bitcoin-fuzz/src/fuzz_process_message.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/process_message.cpp]

lazy_static!{
    /*
    const TestingSetup* g_setup;
    */
}

pub fn get_num_msg_types() -> &'static mut usize {
    
    todo!();
        /*
            static size_t g_num_msg_types{0};
        return g_num_msg_types;
        */
}

macro_rules! fuzz_target_msg {
    ($msg_type:ident) => {
        /*
        
            struct msg_type##_Count_Before_Main {                                    
                msg_type##_Count_Before_Main()                                       
                {                                                                    
                    ++GetNumMsgTypes();                                              
                }                                                                    
            } const static g_##msg_type##_count_before_main;                         
            FUZZ_TARGET_INIT(process_message_##msg_type, initialize_process_message) 
            {                                                                        
                fuzz_target(buffer, #msg_type);                                      
            }
        */
    }
}

pub fn initialize_process_message()  {
    
    todo!();
        /*
            Assert(GetNumMsgTypes() == getAllNetMessageTypes().size()); // If this fails, add or remove the message type below

        static const auto testing_setup = MakeNoLogFileContext<const TestingSetup>();
        g_setup = testing_setup.get();
        for (int i = 0; i < 2 * COINBASE_MATURITY; i++) {
            MineBlock(g_setup->m_node, CScript() << OP_TRUE);
        }
        SyncWithValidationInterfaceQueue();
        */
}

pub fn fuzz_target(
        buffer:                FuzzBufferType,
        limit_to_message_type: &String)  {
    
    todo!();
        /*
            FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());

        ConnmanTestMsg& connman = *static_cast<ConnmanTestMsg*>(g_setup->m_node.connman.get());
        TestChainState& chainstate = *static_cast<TestChainState*>(&g_setup->m_node.chainman->ActiveChainstate());
        SetMockTime(1610000000); // any time to successfully reset ibd
        chainstate.ResetIbd();

        const std::string random_message_type{fuzzed_data_provider.ConsumeBytesAsString(CMessageHeader::COMMAND_SIZE).c_str()};
        if (!LIMIT_TO_MESSAGE_TYPE.empty() && random_message_type != LIMIT_TO_MESSAGE_TYPE) {
            return;
        }
        Node& p2p_node = *ConsumeNodeAsUniquePtr(fuzzed_data_provider).release();

        const bool successfully_connected{fuzzed_data_provider.ConsumeBool()};
        p2p_node.fSuccessfullyConnected = successfully_connected;
        connman.AddTestNode(p2p_node);
        g_setup->m_node.peerman->InitializeNode(&p2p_node);
        FillNode(fuzzed_data_provider, p2p_node, /* init_version */ successfully_connected);

        const auto mock_time = ConsumeTime(fuzzed_data_provider);
        SetMockTime(mock_time);

        // fuzzed_data_provider is fully consumed after this call, don't use it
        DataStream random_bytes_data_stream{fuzzed_data_provider.ConsumeRemainingBytes<unsigned char>(), SER_NETWORK, PROTOCOL_VERSION};
        try {
            g_setup->m_node.peerman->ProcessMessage(p2p_node, random_message_type, random_bytes_data_stream,
                                                    GetTime<std::chrono::microseconds>(), std::atomic<bool>{false});
        } catch (const std::ios_base::failure&) {
        }
        {
            LOCK(p2p_node.cs_sendProcessing);
            g_setup->m_node.peerman->SendMessages(&p2p_node);
        }
        SyncWithValidationInterfaceQueue();
        g_setup->m_node.connman->StopNodes();
        */
}

#[fuzz_test(initializer = "initialize_process_message")]
fn process_message() {
    todo!();
    /*
         fuzz_target(buffer, ""); 
    */
}

fuzz_target_msg!{ addr         } 
fuzz_target_msg!{ addrv2       } 
fuzz_target_msg!{ block        } 
fuzz_target_msg!{ blocktxn     } 
fuzz_target_msg!{ cfcheckpt    } 
fuzz_target_msg!{ cfheaders    } 
fuzz_target_msg!{ cfilter      } 
fuzz_target_msg!{ cmpctblock   } 
fuzz_target_msg!{ feefilter    } 
fuzz_target_msg!{ filteradd    } 
fuzz_target_msg!{ filterclear  } 
fuzz_target_msg!{ filterload   } 
fuzz_target_msg!{ getaddr      } 
fuzz_target_msg!{ getblocks    } 
fuzz_target_msg!{ getblocktxn  } 
fuzz_target_msg!{ getcfcheckpt } 
fuzz_target_msg!{ getcfheaders } 
fuzz_target_msg!{ getcfilters  } 
fuzz_target_msg!{ getdata      } 
fuzz_target_msg!{ getheaders   } 
fuzz_target_msg!{ headers      } 
fuzz_target_msg!{ inv          } 
fuzz_target_msg!{ mempool      } 
fuzz_target_msg!{ merkleblock  } 
fuzz_target_msg!{ notfound     } 
fuzz_target_msg!{ ping         } 
fuzz_target_msg!{ pong         } 
fuzz_target_msg!{ sendaddrv2   } 
fuzz_target_msg!{ sendcmpct    } 
fuzz_target_msg!{ sendheaders  } 
fuzz_target_msg!{ tx           } 
fuzz_target_msg!{ verack       } 
fuzz_target_msg!{ version      } 
fuzz_target_msg!{ wtxidrelay   } 
