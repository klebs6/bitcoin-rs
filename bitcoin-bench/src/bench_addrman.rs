// ---------------- [ File: bitcoin-bench/src/bench_addrman.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/addrman.cpp]

/* ------------------- Benchmarks  ------------------- */

#[bench]
fn addr_man_add(b: &mut Bencher)  {
    
    todo!();
        /*
        CreateAddresses();

        bench.run([&] {
            AddrMan addrman{/* asmap */ std::vector<bool>(), /* deterministic */ false, /* consistency_check_ratio */ 0};
            AddAddressesToAddrMan(addrman);
        });
        */
}

#[bench]
fn addr_man_select(b: &mut Bencher)  {
    
    todo!();
        /*
        AddrMan addrman(/* asmap */ std::vector<bool>(), /* deterministic */ false, /* consistency_check_ratio */ 0);

        FillAddrMan(addrman);

        bench.run([&] {
            const auto& address = addrman.Select();
            assert(address.first.GetPort() > 0);
        });
        */
}

#[bench]
fn addr_man_get_addr(b: &mut Bencher)  {
    
    todo!();
        /*
            AddrMan addrman(/* asmap */ std::vector<bool>(), /* deterministic */ false, /* consistency_check_ratio */ 0);

        FillAddrMan(addrman);

        bench.run([&] {
            const auto& addresses = addrman.GetAddr(/* max_addresses */ 2500, /* max_pct */ 23, /* network */ std::nullopt);
            assert(addresses.size() > 0);
        });
        */
}

#[bench]
fn addr_man_add_then_good(b: &mut Bencher)  {

    let mark_some_as_good = |addrman: &mut AddrMan| {
        for source_i in 0..num_sources {
            for addr_i in 0..num_addresses_per_source {
                addrman.good(g_addresses[source_i][addr_i]);
            }
        }
    };

    
    todo!();
        /*

        CreateAddresses();

        bench.run([&] {
            // To make the benchmark independent of the number of evaluations, we always prepare a new addrman.
            // This is necessary because AddrMan::Good() method modifies the object, affecting the timing of subsequent calls
            // to the same method and we want to do the same amount of work in every loop iteration.
            //
            // This has some overhead (exactly the result of AddrManAdd benchmark), but that overhead is constant so improvements in
            // AddrMan::Good() will still be noticeable.
            AddrMan addrman(/* asmap */ std::vector<bool>(), /* deterministic */ false, /* consistency_check_ratio */ 0);
            AddAddressesToAddrMan(addrman);

            markSomeAsGood(addrman);
        });
        */
}
