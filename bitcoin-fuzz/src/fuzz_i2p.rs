crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/i2p.cpp]

pub fn initialize_i2p()  {
    
    todo!();
        /*
            static const auto testing_setup = MakeNoLogFileContext<>();
        */
}

#[fuzz_test(initializer = "initialize_i2p")]
fn i2p() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider{buffer.data(), buffer.size()};

        // Mock CreateSock() to create FuzzedSock.
        auto CreateSockOrig = CreateSock;
        CreateSock = [&fuzzed_data_provider](const CService&) {
            return std::make_unique<FuzzedSock>(fuzzed_data_provider);
        };

        const CService sam_proxy;
        CThreadInterrupt interrupt;

        i2p::sam::Session sess{gArgs.GetDataDirNet() / "fuzzed_i2p_private_key", sam_proxy, &interrupt};

        i2p::Connection conn;

        if (sess.Listen(conn)) {
            if (sess.Accept(conn)) {
                try {
                    (c_void)conn.sock->RecvUntilTerminator('\n', 10ms, interrupt, i2p::sam::MAX_MSG_SIZE);
                } catch (const std::runtime_error&) {
                }
            }
        }

        const CService to;
        bool proxy_error;

        if (sess.Connect(to, conn, proxy_error)) {
            try {
                conn.sock->SendComplete("verack\n", 10ms, interrupt);
            } catch (const std::runtime_error&) {
            }
        }

        CreateSock = CreateSockOrig;

    */
}
