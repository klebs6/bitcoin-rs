// ---------------- [ File: bitcoin-test/src/test_denialofservice.rs ]
/*!
  | Unit tests for denial-of-service detection/prevention
  | code
  |
  */

crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/denialofservice_tests.cpp]

pub fn ip(i: u32) -> Service {
    
    todo!();
        /*
            struct in_addr s;
        s.s_addr = i;
        return CService(CNetAddr(s), Params().GetDefaultPort());
        */
}

lazy_static!{
    /*
    static NodeId id = 0;
    */
}

pub fn update_last_block_announce_time(
        node:            NodeId,
        time_in_seconds: i64)  {
    
    todo!();
        /*
        
        */
}

#[cfg(test)]
#[fixture(TestingSetup)]
pub mod denialofservice_tests {

    /**
      | Test eviction of an outbound peer whose chain
      | never advances
      |
      | Mock a node connection, and use mocktime to
      | simulate a peer which never sends any headers
      | messages.  PeerLogic should decide to evict
      | that outbound peer, after the appropriate
      | timeouts.
      |
      | Note that we protect 4 outbound nodes from
      | being subject to this logic; this test takes
      | advantage of that protection only being applied
      | to nodes which send headers with sufficient
      | work.
      */
    #[test] fn outbound_slow_chain_eviction() {
        todo!();
        /*
        
            const CChainParams& chainparams = Params();
            auto connman = std::make_unique<CConnman>(0x1337, 0x1337, *m_node.addrman);
            // Disable inactivity checks for this test to avoid interference
            static_cast<ConnmanTestMsg*>(connman.get())->SetPeerConnectTimeout(99999);
            auto peerLogic = PeerManager::make(chainparams, *connman, *m_node.addrman, nullptr,
                                               *m_node.chainman, *m_node.mempool, false);

            // Mock an outbound peer
            CAddress addr1(ip(0xa0b0c001), NODE_NONE);
            Node dummyNode1(id++, ServiceFlags(NODE_NETWORK | NODE_WITNESS), INVALID_SOCKET, addr1, /* nKeyedNetGroupIn */ 0, /* nLocalHostNonceIn */ 0, CAddress(), /* pszDest */ "", ConnectionType::OUTBOUND_FULL_RELAY, /* inbound_onion */ false);
            dummyNode1.SetCommonVersion(PROTOCOL_VERSION);

            peerLogic->InitializeNode(&dummyNode1);
            dummyNode1.fSuccessfullyConnected = true;

            // This test requires that we have a chain with non-zero work.
            {
                LOCK(cs_main);
                BOOST_CHECK(m_node.chainman->ActiveChain().Tip() != nullptr);
                BOOST_CHECK(m_node.chainman->ActiveChain().Tip()->nChainWork > 0);
            }

            // Test starts here
            {
                LOCK(dummyNode1.cs_sendProcessing);
                BOOST_CHECK(peerLogic->SendMessages(&dummyNode1)); // should result in getheaders
            }
            {
                LOCK(dummyNode1.cs_vSend);
                BOOST_CHECK(dummyNode1.vSendMsg.size() > 0);
                dummyNode1.vSendMsg.clear();
            }

            int64_t nStartTime = GetTime();
            // Wait 21 minutes
            SetMockTime(nStartTime+21*60);
            {
                LOCK(dummyNode1.cs_sendProcessing);
                BOOST_CHECK(peerLogic->SendMessages(&dummyNode1)); // should result in getheaders
            }
            {
                LOCK(dummyNode1.cs_vSend);
                BOOST_CHECK(dummyNode1.vSendMsg.size() > 0);
            }
            // Wait 3 more minutes
            SetMockTime(nStartTime+24*60);
            {
                LOCK(dummyNode1.cs_sendProcessing);
                BOOST_CHECK(peerLogic->SendMessages(&dummyNode1)); // should result in disconnect
            }
            BOOST_CHECK(dummyNode1.fDisconnect == true);

            peerLogic->FinalizeNode(dummyNode1);

        */
    }

    pub fn add_random_outbound_peer(
            nodes:      &mut Vec<*mut Node>,
            peer_logic: &mut PeerManager,
            connman:    &mut ConnmanTestMsg)  {
        
        todo!();
            /*
                CAddress addr(ip(g_insecure_rand_ctx.randbits(32)), NODE_NONE);
            vNodes.emplace_back(new Node(id++, ServiceFlags(NODE_NETWORK | NODE_WITNESS), INVALID_SOCKET, addr, /* nKeyedNetGroupIn */ 0, /* nLocalHostNonceIn */ 0, CAddress(), /* pszDest */ "", ConnectionType::OUTBOUND_FULL_RELAY, /* inbound_onion */ false));
            Node &node = *vNodes.back();
            node.SetCommonVersion(PROTOCOL_VERSION);

            peerLogic.InitializeNode(&node);
            node.fSuccessfullyConnected = true;

            connman.AddTestNode(node);
            */
    }

    #[test] fn stale_tip_peer_management() {
        todo!();
        /*
        
            const CChainParams& chainparams = Params();
            auto connman = std::make_unique<ConnmanTestMsg>(0x1337, 0x1337, *m_node.addrman);
            auto peerLogic = PeerManager::make(chainparams, *connman, *m_node.addrman, nullptr,
                                               *m_node.chainman, *m_node.mempool, false);

            constexpr int max_outbound_full_relay = MAX_OUTBOUND_FULL_RELAY_CONNECTIONS;
            CConnman::Options options;
            options.nMaxConnections = DEFAULT_MAX_PEER_CONNECTIONS;
            options.m_max_outbound_full_relay = max_outbound_full_relay;
            options.nMaxFeeler = MAX_FEELER_CONNECTIONS;

            connman->Init(options);
            std::vector<Node *> vNodes;

            // Mock some outbound peers
            for (int i = 0; i < max_outbound_full_relay; ++i) {
                AddRandomOutboundPeer(vNodes, *peerLogic, *connman);
            }

            peerLogic->CheckForStaleTipAndEvictPeers();

            // No nodes should be marked for disconnection while we have no extra peers
            for (const Node *node : vNodes) {
                BOOST_CHECK(node->fDisconnect == false);
            }

            SetMockTime(GetTime() + 3 * chainparams.GetConsensus().nPowTargetSpacing + 1);

            // Now tip should definitely be stale, and we should look for an extra
            // outbound peer
            peerLogic->CheckForStaleTipAndEvictPeers();
            BOOST_CHECK(connman->GetTryNewOutboundPeer());

            // Still no peers should be marked for disconnection
            for (const Node *node : vNodes) {
                BOOST_CHECK(node->fDisconnect == false);
            }

            // If we add one more peer, something should get marked for eviction
            // on the next check (since we're mocking the time to be in the future, the
            // required time connected check should be satisfied).
            AddRandomOutboundPeer(vNodes, *peerLogic, *connman);

            peerLogic->CheckForStaleTipAndEvictPeers();
            for (int i = 0; i < max_outbound_full_relay; ++i) {
                BOOST_CHECK(vNodes[i]->fDisconnect == false);
            }
            // Last added node should get marked for eviction
            BOOST_CHECK(vNodes.back()->fDisconnect == true);

            vNodes.back()->fDisconnect = false;

            // Update the last announced block time for the last
            // peer, and check that the next newest node gets evicted.
            UpdateLastBlockAnnounceTime(vNodes.back()->GetId(), GetTime());

            peerLogic->CheckForStaleTipAndEvictPeers();
            for (int i = 0; i < max_outbound_full_relay - 1; ++i) {
                BOOST_CHECK(vNodes[i]->fDisconnect == false);
            }
            BOOST_CHECK(vNodes[max_outbound_full_relay-1]->fDisconnect == true);
            BOOST_CHECK(vNodes.back()->fDisconnect == false);

            for (const Node *node : vNodes) {
                peerLogic->FinalizeNode(*node);
            }

            connman->ClearTestNodes();

        */
    }

    #[test] fn peer_discouragement() {
        todo!();
        /*
        
            const CChainParams& chainparams = Params();
            auto banman = std::make_unique<BanMan>(m_args.GetDataDirBase() / "banlist", nullptr, DEFAULT_MISBEHAVING_BANTIME);
            auto connman = std::make_unique<ConnmanTestMsg>(0x1337, 0x1337, *m_node.addrman);
            auto peerLogic = PeerManager::make(chainparams, *connman, *m_node.addrman, banman.get(),
                                               *m_node.chainman, *m_node.mempool, false);

            CNetAddr tor_netaddr;
            BOOST_REQUIRE(
                tor_netaddr.SetSpecial("pg6mmjiyjmcrsslvykfwnntlaru7p5svn6y2ymmju6nubxndf4pscryd.onion"));
            const CService tor_service{tor_netaddr, Params().GetDefaultPort()};

            const std::array<CAddress, 3> addr{CAddress{ip(0xa0b0c001), NODE_NONE},
                                               CAddress{ip(0xa0b0c002), NODE_NONE},
                                               CAddress{tor_service, NODE_NONE}};

            const CNetAddr other_addr{ip(0xa0b0ff01)}; // Not any of addr[].

            std::array<Node*, 3> nodes;

            banman->ClearBanned();
            nodes[0] = new Node{id++, NODE_NETWORK, INVALID_SOCKET, addr[0], /* nKeyedNetGroupIn */ 0,
                                 /* nLocalHostNonceIn */ 0, CAddress(), /* pszDest */ "",
                                 ConnectionType::INBOUND, /* inbound_onion */ false};
            nodes[0]->SetCommonVersion(PROTOCOL_VERSION);
            peerLogic->InitializeNode(nodes[0]);
            nodes[0]->fSuccessfullyConnected = true;
            connman->AddTestNode(*nodes[0]);
            peerLogic->Misbehaving(nodes[0]->GetId(), DISCOURAGEMENT_THRESHOLD, /* message */ ""); // Should be discouraged
            {
                LOCK(nodes[0]->cs_sendProcessing);
                BOOST_CHECK(peerLogic->SendMessages(nodes[0]));
            }
            BOOST_CHECK(banman->IsDiscouraged(addr[0]));
            BOOST_CHECK(nodes[0]->fDisconnect);
            BOOST_CHECK(!banman->IsDiscouraged(other_addr)); // Different address, not discouraged

            nodes[1] = new Node{id++, NODE_NETWORK, INVALID_SOCKET, addr[1], /* nKeyedNetGroupIn */ 1,
                                 /* nLocalHostNonceIn */ 1, CAddress(), /* pszDest */ "",
                                 ConnectionType::INBOUND, /* inbound_onion */ false};
            nodes[1]->SetCommonVersion(PROTOCOL_VERSION);
            peerLogic->InitializeNode(nodes[1]);
            nodes[1]->fSuccessfullyConnected = true;
            connman->AddTestNode(*nodes[1]);
            peerLogic->Misbehaving(nodes[1]->GetId(), DISCOURAGEMENT_THRESHOLD - 1, /* message */ "");
            {
                LOCK(nodes[1]->cs_sendProcessing);
                BOOST_CHECK(peerLogic->SendMessages(nodes[1]));
            }
            // [0] is still discouraged/disconnected.
            BOOST_CHECK(banman->IsDiscouraged(addr[0]));
            BOOST_CHECK(nodes[0]->fDisconnect);
            // [1] is not discouraged/disconnected yet.
            BOOST_CHECK(!banman->IsDiscouraged(addr[1]));
            BOOST_CHECK(!nodes[1]->fDisconnect);
            peerLogic->Misbehaving(nodes[1]->GetId(), 1, /* message */ ""); // [1] reaches discouragement threshold
            {
                LOCK(nodes[1]->cs_sendProcessing);
                BOOST_CHECK(peerLogic->SendMessages(nodes[1]));
            }
            // Expect both [0] and [1] to be discouraged/disconnected now.
            BOOST_CHECK(banman->IsDiscouraged(addr[0]));
            BOOST_CHECK(nodes[0]->fDisconnect);
            BOOST_CHECK(banman->IsDiscouraged(addr[1]));
            BOOST_CHECK(nodes[1]->fDisconnect);

            // Make sure non-IP peers are discouraged and disconnected properly.

            nodes[2] = new Node{id++, NODE_NETWORK, INVALID_SOCKET, addr[2], /* nKeyedNetGroupIn */ 1,
                                 /* nLocalHostNonceIn */ 1, CAddress(), /* pszDest */ "",
                                 ConnectionType::OUTBOUND_FULL_RELAY, /* inbound_onion */ false};
            nodes[2]->SetCommonVersion(PROTOCOL_VERSION);
            peerLogic->InitializeNode(nodes[2]);
            nodes[2]->fSuccessfullyConnected = true;
            connman->AddTestNode(*nodes[2]);
            peerLogic->Misbehaving(nodes[2]->GetId(), DISCOURAGEMENT_THRESHOLD, /* message */ "");
            {
                LOCK(nodes[2]->cs_sendProcessing);
                BOOST_CHECK(peerLogic->SendMessages(nodes[2]));
            }
            BOOST_CHECK(banman->IsDiscouraged(addr[0]));
            BOOST_CHECK(banman->IsDiscouraged(addr[1]));
            BOOST_CHECK(banman->IsDiscouraged(addr[2]));
            BOOST_CHECK(nodes[0]->fDisconnect);
            BOOST_CHECK(nodes[1]->fDisconnect);
            BOOST_CHECK(nodes[2]->fDisconnect);

            for (Node* node : nodes) {
                peerLogic->FinalizeNode(*node);
            }
            connman->ClearTestNodes();

        */
    }

    #[test] fn dos_bantime() {
        todo!();
        /*
        
            const CChainParams& chainparams = Params();
            auto banman = std::make_unique<BanMan>(m_args.GetDataDirBase() / "banlist", nullptr, DEFAULT_MISBEHAVING_BANTIME);
            auto connman = std::make_unique<CConnman>(0x1337, 0x1337, *m_node.addrman);
            auto peerLogic = PeerManager::make(chainparams, *connman, *m_node.addrman, banman.get(),
                                               *m_node.chainman, *m_node.mempool, false);

            banman->ClearBanned();
            int64_t nStartTime = GetTime();
            SetMockTime(nStartTime); // Overrides future calls to GetTime()

            CAddress addr(ip(0xa0b0c001), NODE_NONE);
            Node dummyNode(id++, NODE_NETWORK, INVALID_SOCKET, addr, /* nKeyedNetGroupIn */ 4, /* nLocalHostNonceIn */ 4, CAddress(), /* pszDest */ "", ConnectionType::INBOUND, /* inbound_onion */ false);
            dummyNode.SetCommonVersion(PROTOCOL_VERSION);
            peerLogic->InitializeNode(&dummyNode);
            dummyNode.fSuccessfullyConnected = true;

            peerLogic->Misbehaving(dummyNode.GetId(), DISCOURAGEMENT_THRESHOLD, /* message */ "");
            {
                LOCK(dummyNode.cs_sendProcessing);
                BOOST_CHECK(peerLogic->SendMessages(&dummyNode));
            }
            BOOST_CHECK(banman->IsDiscouraged(addr));

            peerLogic->FinalizeNode(dummyNode);

        */
    }

    ///-----------------
    pub struct TxOrphanageTest {
        base: TxOrphanage,
    }

    impl TxOrphanageTest {

        #[EXCLUSIVE_LOCKS_REQUIRED(G_CS_ORPHANS)]
        #[inline] pub fn count_orphans(&self) -> usize {
            
            todo!();
            /*
                return m_orphans.size();
            */
        }

        #[EXCLUSIVE_LOCKS_REQUIRED(G_CS_ORPHANS)]
        pub fn random_orphan(&mut self) -> TransactionRef {
            
            todo!();
            /*
                std::map<uint256, OrphanTx>::iterator it;
                it = m_orphans.lower_bound(InsecureRand256());
                if (it == m_orphans.end())
                    it = m_orphans.begin();
                return it->second.tx;
            */
        }
    }

    pub fn make_new_key_with_fast_random_context(key: &mut Key)  {
        
        todo!();
            /*
                std::vector<unsigned char> keydata;
            keydata = g_insecure_rand_ctx.randbytes(32);
            key.Set(keydata.data(), keydata.data() + keydata.size(), /*fCompressedIn*/ true);
            assert(key.IsValid());
            */
    }

    #[test] fn dos_map_orphans() {
        todo!();
        /*
        
            // This test had non-deterministic coverage due to
            // randomly selected seeds.
            // This seed is chosen so that all branches of the function
            // ecdsa_signature_parse_der_lax are executed during this test.
            // Specifically branches that run only when an ECDSA
            // signature's R and S values have leading zeros.
            g_insecure_rand_ctx = FastRandomContext(ArithToUint256(arith_uint256(33)));

            TxOrphanageTest orphanage;
            CKey key;
            MakeNewKeyWithFastRandomContext(key);
            FillableSigningProvider keystore;
            BOOST_CHECK(keystore.AddKey(key));

            LOCK(G_CS_ORPHANS);

            // 50 orphan transactions:
            for (int i = 0; i < 50; i++)
            {
                CMutableTransaction tx;
                tx.vin.resize(1);
                tx.vin[0].prevout.n = 0;
                tx.vin[0].prevout.hash = InsecureRand256();
                tx.vin[0].scriptSig << OP_1;
                tx.vout.resize(1);
                tx.vout[0].nValue = 1*CENT;
                tx.vout[0].scriptPubKey = GetScriptForDestination(PKHash(key.GetPubKey()));

                orphanage.AddTx(MakeTransactionRef(tx), i);
            }

            // ... and 50 that depend on other orphans:
            for (int i = 0; i < 50; i++)
            {
                CTransactionRef txPrev = orphanage.RandomOrphan();

                CMutableTransaction tx;
                tx.vin.resize(1);
                tx.vin[0].prevout.n = 0;
                tx.vin[0].prevout.hash = txPrev->GetHash();
                tx.vout.resize(1);
                tx.vout[0].nValue = 1*CENT;
                tx.vout[0].scriptPubKey = GetScriptForDestination(PKHash(key.GetPubKey()));
                BOOST_CHECK(SignSignature(keystore, *txPrev, tx, 0, SIGHASH_ALL));

                orphanage.AddTx(MakeTransactionRef(tx), i);
            }

            // This really-big orphan should be ignored:
            for (int i = 0; i < 10; i++)
            {
                CTransactionRef txPrev = orphanage.RandomOrphan();

                CMutableTransaction tx;
                tx.vout.resize(1);
                tx.vout[0].nValue = 1*CENT;
                tx.vout[0].scriptPubKey = GetScriptForDestination(PKHash(key.GetPubKey()));
                tx.vin.resize(2777);
                for (unsigned int j = 0; j < tx.vin.size(); j++)
                {
                    tx.vin[j].prevout.n = j;
                    tx.vin[j].prevout.hash = txPrev->GetHash();
                }
                BOOST_CHECK(SignSignature(keystore, *txPrev, tx, 0, SIGHASH_ALL));
                // Re-use same signature for other inputs
                // (they don't have to be valid for this test)
                for (unsigned int j = 1; j < tx.vin.size(); j++)
                    tx.vin[j].scriptSig = tx.vin[0].scriptSig;

                BOOST_CHECK(!orphanage.AddTx(MakeTransactionRef(tx), i));
            }

            // Test EraseOrphansFor:
            for (NodeId i = 0; i < 3; i++)
            {
                size_t sizeBefore = orphanage.CountOrphans();
                orphanage.EraseForPeer(i);
                BOOST_CHECK(orphanage.CountOrphans() < sizeBefore);
            }

            // Test LimitOrphanTxSize() function:
            orphanage.LimitOrphans(40);
            BOOST_CHECK(orphanage.CountOrphans() <= 40);
            orphanage.LimitOrphans(10);
            BOOST_CHECK(orphanage.CountOrphans() <= 10);
            orphanage.LimitOrphans(0);
            BOOST_CHECK(orphanage.CountOrphans() == 0);

        */
    }
}
