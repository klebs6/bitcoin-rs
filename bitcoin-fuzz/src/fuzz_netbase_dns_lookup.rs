// ---------------- [ File: bitcoin-fuzz/src/fuzz_netbase_dns_lookup.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/netbase_dns_lookup.cpp]

#[fuzz_test] fn netbase_dns_lookup() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider{buffer.data(), buffer.size()};
        const std::string name = fuzzed_data_provider.ConsumeRandomLengthString(512);
        const unsigned int max_results = fuzzed_data_provider.ConsumeIntegral<unsigned int>();
        const bool allow_lookup = fuzzed_data_provider.ConsumeBool();
        const uint16_t default_port = fuzzed_data_provider.ConsumeIntegral<uint16_t>();

        auto fuzzed_dns_lookup_function = [&](const std::string&, bool) {
            std::vector<CNetAddr> resolved_addresses;
            while (fuzzed_data_provider.ConsumeBool()) {
                resolved_addresses.push_back(ConsumeNetAddr(fuzzed_data_provider));
            }
            return resolved_addresses;
        };

        {
            std::vector<CNetAddr> resolved_addresses;
            if (LookupHost(name, resolved_addresses, max_results, allow_lookup, fuzzed_dns_lookup_function)) {
                for (const CNetAddr& resolved_address : resolved_addresses) {
                    assert(!resolved_address.IsInternal());
                }
            }
            assert(resolved_addresses.size() <= max_results || max_results == 0);
        }
        {
            CNetAddr resolved_address;
            if (LookupHost(name, resolved_address, allow_lookup, fuzzed_dns_lookup_function)) {
                assert(!resolved_address.IsInternal());
            }
        }
        {
            std::vector<CService> resolved_services;
            if (Lookup(name, resolved_services, default_port, allow_lookup, max_results, fuzzed_dns_lookup_function)) {
                for (const CNetAddr& resolved_service : resolved_services) {
                    assert(!resolved_service.IsInternal());
                }
            }
            assert(resolved_services.size() <= max_results || max_results == 0);
        }
        {
            CService resolved_service;
            if (Lookup(name, resolved_service, default_port, allow_lookup, fuzzed_dns_lookup_function)) {
                assert(!resolved_service.IsInternal());
            }
        }
        {
            CService resolved_service = LookupNumeric(name, default_port, fuzzed_dns_lookup_function);
            assert(!resolved_service.IsInternal());
        }
        {
            CSubNet resolved_subnet;
            if (LookupSubNet(name, resolved_subnet, fuzzed_dns_lookup_function)) {
                assert(resolved_subnet.IsValid());
            }
        }

    */
}
