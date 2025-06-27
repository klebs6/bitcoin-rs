crate::ix!();

/**
  | Return readable error string for a network
  | error code
  |
  */
#[cfg(WIN32)]
pub fn network_error_string(err: i32) -> String {
    
    todo!();
        /*
            wchar_t buf[256];
        buf[0] = 0;
        if(FormatMessageW(FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS | FORMAT_MESSAGE_MAX_WIDTH_MASK,
                nullptr, err, MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT),
                buf, ARRAYSIZE(buf), nullptr))
        {
            return strprintf("%s (%d)", std::wstring_convert<std::codecvt_utf8_utf16<wchar_t>,wchar_t>().to_bytes(buf), err);
        }
        else
        {
            return strprintf("Unknown error (%d)", err);
        }
        */
}

#[cfg(not(WIN32))]
pub fn network_error_string(err: i32) -> String {
    
    todo!();
        /*
            char buf[256];
        buf[0] = 0;
        /* Too bad there are two incompatible implementations of the
         * thread-safe strerror. */
        const char *s;
    #ifdef STRERROR_R_CHAR_P /* GNU variant can return a pointer outside the passed buffer */
        s = strerror_r(err, buf, sizeof(buf));
    #else /* POSIX variant always returns message in buffer */
        s = buf;
        if (strerror_r(err, buf, sizeof(buf)))
            buf[0] = 0;
    #endif
        return strprintf("%s (%d)", s, err);
        */
}
