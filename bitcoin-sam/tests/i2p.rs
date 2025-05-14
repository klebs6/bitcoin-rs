// ---------------- [ File: bitcoin-sam/tests/i2p.rs ]
use bitcoin_sam::*;

//-------------------------------------------[.cpp/bitcoin/src/test/i2p_tests.cpp]
#[test] fn unlimited_recv() {
    todo!();
    /*
    
        auto CreateSockOrig = CreateSock;

        // Mock CreateSock() to create MockSock.
        CreateSock = [](const CService&) {
            return std::make_unique<StaticContentsSock>(std::string(i2p::sam::MAX_MSG_SIZE + 1, 'a'));
        };

        CThreadInterrupt interrupt;
        i2p::sam::Session session(gArgs.GetDataDirNet() / "test_i2p_private_key", CService{}, &interrupt);

        {
            ASSERT_DEBUG_LOG("Creating SAM session");
            ASSERT_DEBUG_LOG("too many bytes without a terminator");

            i2p::Connection conn;
            bool proxy_error;
            BOOST_REQUIRE(!session.Connect(CService{}, conn, proxy_error));
        }

        CreateSock = CreateSockOrig;

    */
}
