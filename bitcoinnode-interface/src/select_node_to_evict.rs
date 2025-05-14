// ---------------- [ File: bitcoinnode-interface/src/select_node_to_evict.rs ]
crate::ix!();

/**
  | Select an inbound peer to evict after
  | filtering out (protecting) peers having
  | distinct, difficult-to-forge characteristics.
  | The protection logic picks out fixed
  | numbers of desirable peers per various
  | criteria, followed by (mostly) ratios
  | of desirable or disadvantaged peers.
  | If any eviction candidates remain,
  | the selection logic chooses a peer to
  | evict.
  |
  */
pub fn select_node_to_evict(eviction_candidates: Vec<NodeEvictionCandidate>) -> Option<NodeId> {
    
    todo!();
        /*
            // Protect connections with certain characteristics

        // Deterministically select 4 peers to protect by netgroup.
        // An attacker cannot predict which netgroups will be protected
        EraseLastKElements(vEvictionCandidates, CompareNetGroupKeyed, 4);
        // Protect the 8 nodes with the lowest minimum ping time.
        // An attacker cannot manipulate this metric without physically moving nodes closer to the target.
        EraseLastKElements(vEvictionCandidates, ReverseCompareNodeMinPingTime, 8);
        // Protect 4 nodes that most recently sent us novel transactions accepted into our mempool.
        // An attacker cannot manipulate this metric without performing useful work.
        EraseLastKElements(vEvictionCandidates, CompareNodeTXTime, 4);
        // Protect up to 8 non-tx-relay peers that have sent us novel blocks.
        EraseLastKElements(vEvictionCandidates, CompareNodeBlockRelayOnlyTime, 8,
                           [](const NodeEvictionCandidate& n) { return !n.fRelayTxes && n.fRelevantServices; });

        // Protect 4 nodes that most recently sent us novel blocks.
        // An attacker cannot manipulate this metric without performing useful work.
        EraseLastKElements(vEvictionCandidates, CompareNodeBlockTime, 4);

        // Protect some of the remaining eviction candidates by ratios of desirable
        // or disadvantaged characteristics.
        ProtectEvictionCandidatesByRatio(vEvictionCandidates);

        if (vEvictionCandidates.empty()) return std::nullopt;

        // If any remaining peers are preferred for eviction consider only them.
        // This happens after the other preferences since if a peer is really the best by other criteria (esp relaying blocks)
        //  then we probably don't want to evict it no matter what.
        if (std::any_of(vEvictionCandidates.begin(),vEvictionCandidates.end(),[](NodeEvictionCandidate const &n){return n.prefer_evict;})) {
            vEvictionCandidates.erase(std::remove_if(vEvictionCandidates.begin(),vEvictionCandidates.end(),
                                      [](NodeEvictionCandidate const &n){return !n.prefer_evict;}),vEvictionCandidates.end());
        }

        // Identify the network group with the most connections and youngest member.
        // (vEvictionCandidates is already sorted by reverse connect time)
        uint64_t naMostConnections;
        unsigned int nMostConnections = 0;
        int64_t nMostConnectionsTime = 0;
        std::map<uint64_t, std::vector<NodeEvictionCandidate> > mapNetGroupNodes;
        for (const NodeEvictionCandidate &node : vEvictionCandidates) {
            std::vector<NodeEvictionCandidate> &group = mapNetGroupNodes[node.nKeyedNetGroup];
            group.push_back(node);
            const int64_t grouptime = group[0].nTimeConnected;

            if (group.size() > nMostConnections || (group.size() == nMostConnections && grouptime > nMostConnectionsTime)) {
                nMostConnections = group.size();
                nMostConnectionsTime = grouptime;
                naMostConnections = node.nKeyedNetGroup;
            }
        }

        // Reduce to the network group with the most connections
        vEvictionCandidates = std::move(mapNetGroupNodes[naMostConnections]);

        // Disconnect from the network group with the most connections
        return vEvictionCandidates.front().id;
        */
}
