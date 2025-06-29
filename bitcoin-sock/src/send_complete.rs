crate::ix!();

impl Sock {
    
    pub fn send_complete(&self, 
        data:      &String,
        timeout:   Instant /* millis */,
        interrupt: &mut ThreadInterrupt)  {
        
        todo!();
        /*
            const auto deadline = GetTime<milliseconds>() + timeout;
        size_t sent{0};

        for (;;) {
            const ssize_t ret{Send(data.data() + sent, data.size() - sent, MSG_NOSIGNAL)};

            if (ret > 0) {
                sent += static_cast<size_t>(ret);
                if (sent == data.size()) {
                    break;
                }
            } else {
                const int err{WSAGetLastError()};
                if (IOErrorIsPermanent(err)) {
                    throw std::runtime_error(strprintf("send(): %s", NetworkErrorString(err)));
                }
            }

            const auto now = GetTime<milliseconds>();

            if (now >= deadline) {
                throw std::runtime_error(strprintf(
                    "Send timeout (sent only %u of %u bytes before that)", sent, data.size()));
            }

            if (interrupt) {
                throw std::runtime_error(strprintf(
                    "Send interrupted (sent only %u of %u bytes before that)", sent, data.size()));
            }

            // Wait for a short while (or the socket to become ready for sending) before retrying
            // if nothing was sent.
            const auto wait_time = std::min(deadline - now, milliseconds{MAX_WAIT_FOR_IO});
            (c_void)Wait(wait_time, SEND);
        }
        */
    }
}
