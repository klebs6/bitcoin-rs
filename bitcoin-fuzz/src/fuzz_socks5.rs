crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/socks5.cpp]

lazy_static!{
    /*
    int default_socks5_recv_timeout;
    */
}

lazy_static!{
    /*
    extern int g_socks5_recv_timeout;
    */
}

pub fn initialize_socks5()  {
    
    todo!();
        /*
            static const auto testing_setup = MakeNoLogFileContext<const BasicTestingSetup>();
        default_socks5_recv_timeout = g_socks5_recv_timeout;
        */
}

#[fuzz_test(initializer = "initialize_socks5")]
fn socks5() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider{buffer.data(), buffer.size()};
        ProxyCredentials proxy_credentials;
        proxy_credentials.username = fuzzed_data_provider.ConsumeRandomLengthString(512);
        proxy_credentials.password = fuzzed_data_provider.ConsumeRandomLengthString(512);
        InterruptSocks5(fuzzed_data_provider.ConsumeBool());
        // Set FUZZED_SOCKET_FAKE_LATENCY=1 to exercise recv timeout code paths. This
        // will slow down fuzzing.
        g_socks5_recv_timeout = (fuzzed_data_provider.ConsumeBool() && std::getenv("FUZZED_SOCKET_FAKE_LATENCY") != nullptr) ? 1 : default_socks5_recv_timeout;
        FuzzedSock fuzzed_sock = ConsumeSock(fuzzed_data_provider);
        // This Socks5(...) fuzzing harness would have caught CVE-2017-18350 within
        // a few seconds of fuzzing.
        (c_void)Socks5(fuzzed_data_provider.ConsumeRandomLengthString(512),
                     fuzzed_data_provider.ConsumeIntegral<uint16_t>(),
                     fuzzed_data_provider.ConsumeBool() ? &proxy_credentials : nullptr,
                     fuzzed_sock);

    */
}
