crate::ix!();

impl Sock {
    
    pub fn recv_until_terminator(&self, 
        terminator: u8,
        timeout:    Instant /* millis */,
        interrupt:  &mut ThreadInterrupt,
        max_data:   usize) -> String {
        
        todo!();
        /*
            const auto deadline = GetTime<milliseconds>() + timeout;
        std::string data;
        bool terminator_found{false};

        // We must not consume any bytes past the terminator from the socket.
        // One option is to read one byte at a time and check if we have read a terminator.
        // However that is very slow. Instead, we peek at what is in the socket and only read
        // as many bytes as possible without crossing the terminator.
        // Reading 64 MiB of random data with 262526 terminator chars takes 37 seconds to read
        // one byte at a time VS 0.71 seconds with the "peek" solution below. Reading one byte
        // at a time is about 50 times slower.

        for (;;) {
            if (data.size() >= max_data) {
                throw std::runtime_error(
                    strprintf("Received too many bytes without a terminator (%u)", data.size()));
            }

            char buf[512];

            const ssize_t peek_ret{Recv(buf, std::min(sizeof(buf), max_data - data.size()), MSG_PEEK)};

            switch (peek_ret) {
            case -1: {
                const int err{WSAGetLastError()};
                if (IOErrorIsPermanent(err)) {
                    throw std::runtime_error(strprintf("recv(): %s", NetworkErrorString(err)));
                }
                break;
            }
            case 0:
                throw std::runtime_error("Connection unexpectedly closed by peer");
            default:
                auto end = buf + peek_ret;
                auto terminator_pos = std::find(buf, end, terminator);
                terminator_found = terminator_pos != end;

                const size_t try_len{terminator_found ? terminator_pos - buf + 1 :
                                                        static_cast<size_t>(peek_ret)};

                const ssize_t read_ret{Recv(buf, try_len, 0)};

                if (read_ret < 0 || static_cast<size_t>(read_ret) != try_len) {
                    throw std::runtime_error(
                        strprintf("recv() returned %u bytes on attempt to read %u bytes but previous "
                                  "peek claimed %u bytes are available",
                                  read_ret, try_len, peek_ret));
                }

                // Don't include the terminator in the output.
                const size_t append_len{terminator_found ? try_len - 1 : try_len};

                data.append(buf, buf + append_len);

                if (terminator_found) {
                    return data;
                }
            }

            const auto now = GetTime<milliseconds>();

            if (now >= deadline) {
                throw std::runtime_error(strprintf(
                    "Receive timeout (received %u bytes without terminator before that)", data.size()));
            }

            if (interrupt) {
                throw std::runtime_error(strprintf(
                    "Receive interrupted (received %u bytes without terminator before that)",
                    data.size()));
            }

            // Wait for a short while (or the socket to become ready for reading) before retrying.
            const auto wait_time = std::min(deadline - now, milliseconds{MAX_WAIT_FOR_IO});
            (c_void)Wait(wait_time, RECV);
        }
        */
    }
}
