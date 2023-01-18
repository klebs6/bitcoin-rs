crate::ix!();

pub fn setup_networking() -> bool {
    
    todo!();
        /*
            #ifdef WIN32
        // Initialize Windows Sockets
        WSADATA wsadata;
        int ret = WSAStartup(MAKEWORD(2,2), &wsadata);
        if (ret != NO_ERROR || LOBYTE(wsadata.wVersion ) != 2 || HIBYTE(wsadata.wVersion) != 2)
            return false;
    #endif
        return true;
        */
}

pub fn setup_environment()  {
    
    todo!();
        /*
            #ifdef HAVE_MALLOPT_ARENA_MAX
        // glibc-specific: On 32-bit systems set the number of arenas to 1.
        // By default, since glibc 2.10, the C library will create up to two heap
        // arenas per core. This is known to cause excessive virtual address space
        // usage in our usage. Work around it by setting the maximum number of
        // arenas to 1.
        if (sizeof(c_void*) == 4) {
            mallopt(M_ARENA_MAX, 1);
        }
    #endif
        // On most POSIX systems (e.g. Linux, but not BSD) the environment's locale
        // may be invalid, in which case the "C.UTF-8" locale is used as fallback.
    #if !defined(WIN32) && !defined(MAC_OSX) && !defined(__FreeBSD__) && !defined(__OpenBSD__) && !defined(__NetBSD__)
        try {
            std::locale(""); // Raises a runtime error if current locale is invalid
        } catch (const std::runtime_error&) {
            setenv("LC_ALL", "C.UTF-8", 1);
        }
    #elif defined(WIN32)
        // Set the default input/output charset is utf-8
        SetConsoleCP(CP_UTF8);
        SetConsoleOutputCP(CP_UTF8);
    #endif
        // The path locale is lazy initialized and to avoid deinitialization errors
        // in multithreading environments, it is set explicitly by the main thread.
        // A dummy locale is used to extract the internal default locale, used by
        // fs::path, which is then used to explicitly imbue the path.
        std::locale loc = fs::path::imbue(std::locale::classic());
    #ifndef WIN32
        fs::path::imbue(loc);
    #else
        fs::path::imbue(std::locale(loc, new std::codecvt_utf8_utf16<wchar_t>()));
    #endif
        */
}
