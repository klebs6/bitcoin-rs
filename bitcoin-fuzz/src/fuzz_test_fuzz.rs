crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/fuzz.h]

/**
  | Can be used to limit a theoretically
  | unbounded loop. This caps the runtime
  | to avoid timeouts or OOMs.
  |
  */
macro_rules! limited_while {
    ($condition:ident, $limit:ident) => {
        /*
        
            for (unsigned _count{limit}; (condition) && _count; --_count)
        */
    }
}

pub type FuzzBufferType<'a> = &'a [u8];
pub type TypeTestOneInput   = fn(_0: FuzzBufferType) -> ();
pub type TypeInitialize     = fn() -> ();
pub type TypeHidden         = bool;

#[inline] pub fn fuzz_framework_empty_init_fun()  {
    
    todo!();
        /*
        
        */
}

macro_rules! fuzz_target {
    ($name:ident) => {
        /*
        
            FUZZ_TARGET_INIT(name, FuzzFrameworkEmptyInitFun)
        */
    }
}

macro_rules! fuzz_target_init {
    ($name:ident, $init_fun:ident) => {
        /*
        
            FUZZ_TARGET_INIT_HIDDEN(name, init_fun, false)
        */
    }
}

macro_rules! fuzz_target_init_hidden {
    ($name:ident, 
     $init_fun:ident, 
     $hidden:ident) => {
        /*
        
            c_void name##_fuzz_target(FuzzBufferType);                                          
            struct name##_Before_Main {                                                       
                name##_Before_Main()                                                          
                {                                                                             
                    FuzzFrameworkRegisterTarget(#name, name##_fuzz_target, init_fun, hidden); 
                }                                                                             
            } const static g_##name##_before_main;                                            
            c_void name##_fuzz_target(FuzzBufferType buffer)
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/fuzz.cpp]

lazy_static!{
    /*
    static const std::function<c_void(const std::string&)> G_TEST_LOG_FUN{};
    */
}

pub fn fuzz_targets() -> &'static mut HashMap<&'static str,(TypeTestOneInput,TypeInitialize,TypeHidden)> {
    
    todo!();
        /*
            static std::map<std::string_view, std::tuple<TypeTestOneInput, TypeInitialize, TypeHidden>> g_fuzz_targets;
        return g_fuzz_targets;
        */
}

pub fn fuzz_framework_register_target(
        name:   &str,
        target: TypeTestOneInput,
        init:   TypeInitialize,
        hidden: TypeHidden)  {
    
    todo!();
        /*
            const auto it_ins = FuzzTargets().try_emplace(name, std::move(target), std::move(init), hidden);
        Assert(it_ins.second);
        */
}

lazy_static!{
    /*
    static TypeTestOneInput* g_test_one_input{nullptr};
    */
}

pub fn initialize()  {
    
    todo!();
        /*
            // Terminate immediately if a fuzzing harness ever tries to create a TCP socket.
        CreateSock = [](const CService&) -> std::unique_ptr<Sock> { std::terminate(); };

        // Terminate immediately if a fuzzing harness ever tries to perform a DNS lookup.
        g_dns_lookup = [](const std::string& name, bool allow_lookup) {
            if (allow_lookup) {
                std::terminate();
            }
            return WrappedGetAddrInfo(name, false);
        };

        bool should_abort{false};
        if (std::getenv("PRINT_ALL_FUZZ_TARGETS_AND_ABORT")) {
            for (const auto& t : FuzzTargets()) {
                if (std::get<2>(t.second)) continue;
                std::cout << t.first << std::endl;
            }
            should_abort = true;
        }
        if (const char* out_path = std::getenv("WRITE_ALL_FUZZ_TARGETS_AND_ABORT")) {
            std::cout << "Writing all fuzz target names to '" << out_path << "'." << std::endl;
            std::ofstream out_stream(out_path, std::ios::binary);
            for (const auto& t : FuzzTargets()) {
                if (std::get<2>(t.second)) continue;
                out_stream << t.first << std::endl;
            }
            should_abort = true;
        }
        Assert(!should_abort);
        std::string_view fuzz_target{Assert(std::getenv("FUZZ"))};
        const auto it = FuzzTargets().find(fuzz_target);
        Assert(it != FuzzTargets().end());
        Assert(!g_test_one_input);
        g_test_one_input = &std::get<0>(it->second);
        std::get<1>(it->second)();
        */
}

#[cfg(PROVIDE_FUZZ_MAIN_FUNCTION)]
pub fn read_stdin(data: &mut Vec<u8>) -> bool {
    
    todo!();
        /*
            uint8_t buffer[1024];
        ssize_t length = 0;
        while ((length = read(STDIN_FILENO, buffer, 1024)) > 0) {
            data.insert(data.end(), buffer, buffer + length);
        }
        return length == 0;
        */
}

/**
  | This function is used by libFuzzer
  |
  */
pub fn llvm_fuzzer_test_one_input(
        data: *const u8,
        size: usize) -> i32 {
    
    todo!();
        /*
            static const auto& test_one_input = *Assert(g_test_one_input);
        test_one_input({data, size});
        return 0;
        */
}

/**
  | This function is used by libFuzzer
  |
  */
pub fn llvm_fuzzer_initialize(
        argc: *mut i32,
        argv: *mut *mut *mut u8) -> i32 {
    
    todo!();
        /*
            initialize();
        return 0;
        */
}

#[cfg(PROVIDE_FUZZ_MAIN_FUNCTION)]
pub fn test_fuzz_main(
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            initialize();
        static const auto& test_one_input = *Assert(g_test_one_input);
    #ifdef __AFL_INIT
        // Enable AFL deferred forkserver mode. Requires compilation using
        // afl-clang-fast++. See fuzzing.md for details.
        __AFL_INIT();
    #endif

    #ifdef __AFL_LOOP
        // Enable AFL persistent mode. Requires compilation using afl-clang-fast++.
        // See fuzzing.md for details.
        while (__AFL_LOOP(1000)) {
            std::vector<uint8_t> buffer;
            if (!read_stdin(buffer)) {
                continue;
            }
            test_one_input(buffer);
        }
    #else
        std::vector<uint8_t> buffer;
        if (!read_stdin(buffer)) {
            return 0;
        }
        test_one_input(buffer);
    #endif
        return 0;
        */
}
