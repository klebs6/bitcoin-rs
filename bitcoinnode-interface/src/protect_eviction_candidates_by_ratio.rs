crate::ix!();

/**
  | Protect desirable or disadvantaged
  | inbound peers from eviction by ratio.
  | 
  | This function protects half of the peers
  | which have been connected the longest,
  | to replicate the non-eviction implicit
  | behavior and preclude attacks that
  | start later.
  | 
  | Half of these protected spots (1/4 of
  | the total) are reserved for the following
  | categories of peers, sorted by longest
  | uptime, even if they're not longest
  | uptime overall:
  | 
  | - onion peers connected via our tor control
  | service
  | 
  | - localhost peers, as manually configured
  | hidden services not using `-bind=addr[:port]=onion`
  | will not be detected as inbound onion
  | connections
  | 
  | - I2P peers
  | 
  | This helps protect these privacy network
  | peers, which tend to be otherwise disadvantaged
  | under our eviction criteria for their
  | higher min ping times relative to IPv4/IPv6
  | peers, and favorise the diversity of
  | peer connections.
  |
  */
pub fn protect_eviction_candidates_by_ratio(eviction_candidates: &mut Vec<NodeEvictionCandidate>)  {
    
    todo!();
        /*
            // Protect the half of the remaining nodes which have been connected the longest.
        // This replicates the non-eviction implicit behavior, and precludes attacks that start later.
        // To favorise the diversity of our peer connections, reserve up to half of these protected
        // spots for Tor/onion, localhost and I2P peers, even if they're not longest uptime overall.
        // This helps protect these higher-latency peers that tend to be otherwise
        // disadvantaged under our eviction criteria.
        const size_t initial_size = eviction_candidates.size();
        const size_t total_protect_size{initial_size / 2};

        // Disadvantaged networks to protect: I2P, localhost, Tor/onion. In case of equal counts, earlier
        // array members have first opportunity to recover unused slots from the previous iteration.
        struct Net { bool is_local; Network id; size_t count; };
        std::array<Net, 3> networks{
            {{false, NET_I2P, 0}, {/* localhost */ true, NET_MAX, 0}, {false, NET_ONION, 0}}};

        // Count and store the number of eviction candidates per network.
        for (Net& n : networks) {
            n.count = std::count_if(eviction_candidates.cbegin(), eviction_candidates.cend(),
                                    [&n](const NodeEvictionCandidate& c) {
                                        return n.is_local ? c.m_is_local : c.m_network == n.id;
                                    });
        }
        // Sort `networks` by ascending candidate count, to give networks having fewer candidates
        // the first opportunity to recover unused protected slots from the previous iteration.
        std::stable_sort(networks.begin(), networks.end(), [](Net a, Net b) { return a.count < b.count; });

        // Protect up to 25% of the eviction candidates by disadvantaged network.
        const size_t max_protect_by_network{total_protect_size / 2};
        size_t num_protected{0};

        while (num_protected < max_protect_by_network) {
            // Count the number of disadvantaged networks from which we have peers to protect.
            auto num_networks = std::count_if(networks.begin(), networks.end(), [](const Net& n) { return n.count; });
            if (num_networks == 0) {
                break;
            }
            const size_t disadvantaged_to_protect{max_protect_by_network - num_protected};
            const size_t protect_per_network{std::max(disadvantaged_to_protect / num_networks, static_cast<size_t>(1))};
            // Early exit flag if there are no remaining candidates by disadvantaged network.
            bool protected_at_least_one{false};

            for (Net& n : networks) {
                if (n.count == 0) continue;
                const size_t before = eviction_candidates.size();
                EraseLastKElements(eviction_candidates, CompareNodeNetworkTime(n.is_local, n.id),
                                   protect_per_network, [&n](const NodeEvictionCandidate& c) {
                                       return n.is_local ? c.m_is_local : c.m_network == n.id;
                                   });
                const size_t after = eviction_candidates.size();
                if (before > after) {
                    protected_at_least_one = true;
                    const size_t delta{before - after};
                    num_protected += delta;
                    if (num_protected >= max_protect_by_network) {
                        break;
                    }
                    n.count -= delta;
                }
            }
            if (!protected_at_least_one) {
                break;
            }
        }

        // Calculate how many we removed, and update our total number of peers that
        // we want to protect based on uptime accordingly.
        assert(num_protected == initial_size - eviction_candidates.size());
        const size_t remaining_to_protect{total_protect_size - num_protected};
        EraseLastKElements(eviction_candidates, ReverseCompareNodeTimeConnected, remaining_to_protect);
        */
}

