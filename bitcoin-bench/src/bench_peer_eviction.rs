// ---------------- [ File: bitcoin-bench/src/bench_peer_eviction.rs ]
/*!
  | Candidate numbers used for the benchmarks:
  |
  | -  50 candidates simulates a possible use of
  | -maxconnections
  |
  | - 100 candidates approximates an average node
  | with default settings
  |
  | - 250 candidates is the number of peers
  | reported by operators of busy nodes
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/peer_eviction.cpp]

pub fn eviction_protection_common(
    bench:              &mut Bencher,
    num_candidates:     i32,
    candidate_setup_fn: fn(_0: &mut NodeEvictionCandidate) -> ())  {
    
    todo!();
        /*
            using Candidates = std::vector<NodeEvictionCandidate>;
        FastRandomContext random_context{true};

        Candidates candidates{GetRandomNodeEvictionCandidates(num_candidates, random_context)};
        for (auto& c : candidates) {
            candidate_setup_fn(c);
        }

        bench.run([&] {
            // creating a copy has an overhead of about 3%, so it does not influence the benchmark results much.
            auto copy = candidates;
            ProtectEvictionCandidatesByRatio(copy);
        });
        */
}

/* ------------------- Benchmarks  ------------------- */

/**
  | No disadvantaged networks, with 250
  | eviction candidates.
  |
  */
#[bench] fn eviction_protection_0networks_250candidates(b: &mut Bencher)  {
    
    todo!();
        /*
            EvictionProtectionCommon(
            bench,
            250 /* num_candidates */,
            [](NodeEvictionCandidate& c) {
                c.nTimeConnected = c.id;
                c.m_network = NET_IPV4;
            });
        */
}

/**
  | 1 disadvantaged network (Tor) with
  | 250 eviction candidates.
  |
  */
#[bench] fn eviction_protection_1networks_250candidates(b: &mut Bencher)  {
    
    todo!();
        /*
            EvictionProtectionCommon(
            bench,
            250 /* num_candidates */,
            [](NodeEvictionCandidate& c) {
                c.nTimeConnected = c.id;
                c.m_is_local = false;
                if (c.id >= 130 && c.id < 240) { // 110 Tor
                    c.m_network = NET_ONION;
                } else {
                    c.m_network = NET_IPV4;
                }
            });
        */
}

/**
  | 2 disadvantaged networks (I2P, Tor)
  | with 250 eviction candidates.
  |
  */
#[bench] fn eviction_protection_2networks_250candidates(b: &mut Bencher)  {
    
    todo!();
        /*
            EvictionProtectionCommon(
            bench,
            250 /* num_candidates */,
            [](NodeEvictionCandidate& c) {
                c.nTimeConnected = c.id;
                c.m_is_local = false;
                if (c.id >= 90 && c.id < 160) { // 70 Tor
                    c.m_network = NET_ONION;
                } else if (c.id >= 170 && c.id < 250) { // 80 I2P
                    c.m_network = NET_I2P;
                } else {
                    c.m_network = NET_IPV4;
                }
            });
        */
}

/**
  | 3 disadvantaged networks (I2P/localhost/Tor)
  | with 50/100/250 eviction candidates.
  |
  */
#[bench] fn eviction_protection_3networks_050candidates(b: &mut Bencher)  {
    
    todo!();
        /*
            EvictionProtectionCommon(
            bench,
            50 /* num_candidates */,
            [](NodeEvictionCandidate& c) {
                c.nTimeConnected = c.id;
                c.m_is_local = (c.id == 28 || c.id == 47); //  2 localhost
                if (c.id >= 30 && c.id < 47) {             // 17 I2P
                    c.m_network = NET_I2P;
                } else if (c.id >= 24 && c.id < 28) { //  4 Tor
                    c.m_network = NET_ONION;
                } else {
                    c.m_network = NET_IPV4;
                }
            });
        */
}

/**
  | 3 disadvantaged networks (I2P/localhost/Tor)
  | with 50/100/250 eviction candidates.
  |
  */
#[bench] fn eviction_protection_3networks_100candidates(b: &mut Bencher)  {
    
    todo!();
        /*
            EvictionProtectionCommon(
            bench,
            100 /* num_candidates */,
            [](NodeEvictionCandidate& c) {
                c.nTimeConnected = c.id;
                c.m_is_local = (c.id >= 55 && c.id < 60); //  5 localhost
                if (c.id >= 70 && c.id < 80) {            // 10 I2P
                    c.m_network = NET_I2P;
                } else if (c.id >= 80 && c.id < 96) { // 16 Tor
                    c.m_network = NET_ONION;
                } else {
                    c.m_network = NET_IPV4;
                }
            });
        */
}

/**
  | 3 disadvantaged networks (I2P/localhost/Tor)
  | with 50/100/250 eviction candidates.
  |
  */
#[bench] fn eviction_protection_3networks_250candidates(b: &mut Bencher)  {
    
    todo!();
        /*
            EvictionProtectionCommon(
            bench,
            250 /* num_candidates */,
            [](NodeEvictionCandidate& c) {
                c.nTimeConnected = c.id;
                c.m_is_local = (c.id >= 140 && c.id < 160); // 20 localhost
                if (c.id >= 170 && c.id < 180) {            // 10 I2P
                    c.m_network = NET_I2P;
                } else if (c.id >= 190 && c.id < 240) { // 50 Tor
                    c.m_network = NET_ONION;
                } else {
                    c.m_network = NET_IPV4;
                }
            });
        */
}
