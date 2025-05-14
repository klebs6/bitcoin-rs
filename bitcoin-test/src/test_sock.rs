// ---------------- [ File: bitcoin-test/src/test_sock.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/sock_tests.cpp]

#[cfg(test)]
#[fixture(BasicTestingSetup)]
pub mod sock_tests {

    pub fn socket_is_closed(s: &Socket) -> bool {
        
        todo!();
            /*
                // Notice that if another thread is running and creates its own socket after `s` has been
            // closed, it may be assigned the same file descriptor number. In this case, our test will
            // wrongly pretend that the socket is not closed.
            int type;
            socklen_t len = sizeof(type);
            return getsockopt(s, SOL_SOCKET, SO_TYPE, (sockopt_arg_type)&type, &len) == SOCKET_ERROR;
            */
    }

    pub fn create_socket() -> Socket {
        
        todo!();
            /*
                const Socket s = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
            BOOST_REQUIRE(s != static_cast<Socket>(SOCKET_ERROR));
            return s;
            */
    }

    #[test] fn constructor_and_destructor() {
        todo!();
        /*
        
            const Socket s = CreateSocket();
            Sock* sock = new Sock(s);
            BOOST_CHECK_EQUAL(sock->Get(), s);
            BOOST_CHECK(!SocketIsClosed(s));
            delete sock;
            BOOST_CHECK(SocketIsClosed(s));

        */
    }

    #[test] fn move_constructor() {
        todo!();
        /*
        
            const Socket s = CreateSocket();
            Sock* sock1 = new Sock(s);
            Sock* sock2 = new Sock(std::move(*sock1));
            delete sock1;
            BOOST_CHECK(!SocketIsClosed(s));
            BOOST_CHECK_EQUAL(sock2->Get(), s);
            delete sock2;
            BOOST_CHECK(SocketIsClosed(s));

        */
    }

    #[test] fn move_assignment() {
        todo!();
        /*
        
            const Socket s = CreateSocket();
            Sock* sock1 = new Sock(s);
            Sock* sock2 = new Sock();
            *sock2 = std::move(*sock1);
            delete sock1;
            BOOST_CHECK(!SocketIsClosed(s));
            BOOST_CHECK_EQUAL(sock2->Get(), s);
            delete sock2;
            BOOST_CHECK(SocketIsClosed(s));

        */
    }

    #[test] fn release() {
        todo!();
        /*
        
            Socket s = CreateSocket();
            Sock* sock = new Sock(s);
            BOOST_CHECK_EQUAL(sock->Release(), s);
            delete sock;
            BOOST_CHECK(!SocketIsClosed(s));
            BOOST_REQUIRE(CloseSocket(s));

        */
    }

    #[test] fn reset() {
        todo!();
        /*
        
            const Socket s = CreateSocket();
            Sock sock(s);
            sock.Reset();
            BOOST_CHECK(SocketIsClosed(s));

        */
    }

    /* ----- Windows does not have socketpair(2).   ----- */

    #[cfg(not(WIN32))]
    pub fn create_socket_pair(s: [i32; 2])  {
        
        todo!();
            /*
                BOOST_REQUIRE_EQUAL(socketpair(AF_UNIX, SOCK_STREAM, 0, s), 0);
            */
    }

    #[cfg(not(WIN32))]
    pub fn send_and_recv_message(
            sender:   &Sock,
            receiver: &Sock)  {
        
        todo!();
            /*
                const char* msg = "abcd";
            constexpr ssize_t msg_len = 4;
            char recv_buf[10];

            BOOST_CHECK_EQUAL(sender.Send(msg, msg_len, 0), msg_len);
            BOOST_CHECK_EQUAL(receiver.Recv(recv_buf, sizeof(recv_buf), 0), msg_len);
            BOOST_CHECK_EQUAL(strncmp(msg, recv_buf, msg_len), 0);
            */
    }

    #[cfg(not(WIN32))]
    #[test] fn send_and_receive() {
        todo!();
        /*
        
            int s[2];
            CreateSocketPair(s);

            Sock* sock0 = new Sock(s[0]);
            Sock* sock1 = new Sock(s[1]);

            SendAndRecvMessage(*sock0, *sock1);

            Sock* sock0moved = new Sock(std::move(*sock0));
            Sock* sock1moved = new Sock();
            *sock1moved = std::move(*sock1);

            delete sock0;
            delete sock1;

            SendAndRecvMessage(*sock1moved, *sock0moved);

            delete sock0moved;
            delete sock1moved;

            BOOST_CHECK(SocketIsClosed(s[0]));
            BOOST_CHECK(SocketIsClosed(s[1]));

        */
    }

    #[cfg(not(WIN32))]
    #[test] fn wait() {
        todo!();
        /*
        
            int s[2];
            CreateSocketPair(s);

            Sock sock0(s[0]);
            Sock sock1(s[1]);

            std::thread waiter([&sock0]() { (c_void)sock0.Wait(24h, Sock::RECV); });

            BOOST_REQUIRE_EQUAL(sock1.Send("a", 1, 0), 1);

            waiter.join();

        */
    }

    #[cfg(not(WIN32))]
    #[test] fn recv_until_terminator_limit() {
        todo!();
        /*
        
            constexpr auto timeout = 1min; // High enough so that it is never hit.
            CThreadInterrupt interrupt;
            int s[2];
            CreateSocketPair(s);

            Sock sock_send(s[0]);
            Sock sock_recv(s[1]);

            std::thread receiver([&sock_recv, &timeout, &interrupt]() {
                constexpr size_t max_data{10};
                bool threw_as_expected{false};
                // BOOST_CHECK_EXCEPTION() writes to some variables shared with the main thread which
                // creates a data race. So mimic it manually.
                try {
                    (c_void)sock_recv.RecvUntilTerminator('\n', timeout, interrupt, max_data);
                } catch (const std::runtime_error& e) {
                    threw_as_expected = HasReason("too many bytes without a terminator")(e);
                }
                assert(threw_as_expected);
            });

            BOOST_REQUIRE_NO_THROW(sock_send.SendComplete("1234567", timeout, interrupt));
            BOOST_REQUIRE_NO_THROW(sock_send.SendComplete("89a\n", timeout, interrupt));

            receiver.join();

        */
    }
}
