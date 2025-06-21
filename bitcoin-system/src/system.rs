// ---------------- [ File: bitcoin-system/src/system.rs ]
/*!
  | Server/client environment: argument
  | handling, config file parsing, thread
  | wrappers, startup time
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/types.h]

pub const ALWAYS_FALSE: bool = false;

//-------------------------------------------[.cpp/bitcoin/src/util/system.h]

pub fn error<Args>(
        fmt:  *const u8,
        args: &Args) -> bool {

    todo!();
        /*
            LogPrintf("ERROR: %s\n", tfm::format(fmt, args...));
        return false;
        */
}


/**
  | Simplification of std insertion
  |
  */
#[inline] pub fn insert<Tdst, Tsrc>(
        dst: &mut Tdst,
        src: &Tsrc)  {

    todo!();
        /*
            dst.insert(dst.begin(), src.begin(), src.end());
        */
}

#[inline] pub fn insert_into_set<TsetT, Tsrc>(
        dst: &mut HashSet<TsetT>,
        src: &Tsrc)  {

    todo!();
        /*
            dst.insert(src.begin(), src.end());
        */
}

/**
  | Helper function to access the contained
  | object of a std::any instance.
  | 
  | Returns a pointer to the object if passed
  | instance has a value and the type matches,
  | nullptr otherwise.
  |
  */
pub fn any_ptr<T>(any: &dyn Any) -> *mut T {

    todo!();
    /*
    T* const* ptr = std::any_cast<T*>(&any);
    return ptr ? *ptr : nullptr;
    */
}

//-------------------------------------------[.cpp/bitcoin/src/util/system.cpp]

/**
  | Application startup time (used for
  | uptime calculation)
  |
  */
lazy_static!{
    static ref N_STARTUP_TIME: Instant = get_time();
}

lazy_static!{
    /*

    /** Mutex to protect dir_locks. */
    static Mutex cs_dir_locks;

    /** A map that contains all the currently held directory locks. After
     * successful locking, these will be held here until the global destructor
     * cleans them up and thus automatically unlocks them, or ReleaseDirectoryLocks
     * is called.
     */
    static std::map<std::string, std::unique_ptr<fsbridge::FileLock>> dir_locks GUARDED_BY(cs_dir_locks);
    */
}

pub fn lock_directory(
        directory:     &Path,
        lockfile_name: String,
        probe_only:    Option<bool>) -> bool {

    let probe_only: bool = probe_only.unwrap_or(false);
    
    todo!();
        /*
            LOCK(cs_dir_locks);
        fs::path pathLockFile = directory / lockfile_name;

        // If a lock for this directory already exists in the map, don't try to re-lock it
        if (dir_locks.count(fs::PathToString(pathLockFile))) {
            return true;
        }

        // Create empty lock file if it doesn't exist.
        FILE* file = fsbridge::fopen(pathLockFile, "a");
        if (file) fclose(file);
        auto lock = std::make_unique<fsbridge::FileLock>(pathLockFile);
        if (!lock->TryLock()) {
            return error("Error while attempting to lock directory %s: %s", fs::PathToString(directory), lock->GetReason());
        }
        if (!probe_only) {
            // Lock successful and we're not just probing, put it into the map
            dir_locks.emplace(fs::PathToString(pathLockFile), std::move(lock));
        }
        return true;
        */
}

pub fn unlock_directory(
        directory:     &Path,
        lockfile_name: &String)  {
    
    todo!();
        /*
            LOCK(cs_dir_locks);
        dir_locks.erase(fs::PathToString(directory / lockfile_name));
        */
}

/**
  | Release all directory locks. This is
  | used for unit testing only, at runtime
  | the global destructor will take care
  | of the locks.
  |
  */
pub fn release_directory_locks()  {
    
    todo!();
        /*
            LOCK(cs_dir_locks);
        dir_locks.clear();
        */
}

pub fn dir_is_writable(directory: &Path) -> bool {
    
    todo!();
        /*
            fs::path tmpFile = GetUniquePath(directory);

        FILE* file = fsbridge::fopen(tmpFile, "a");
        if (!file) return false;

        fclose(file);
        remove(tmpFile);

        return true;
        */
}


//TODO: is usize appropriate here?
pub type StreamSize = usize;

/**
  | Get the size of a file by scanning it.
  | 
  | -----------
  | @param[in] path
  | 
  | The file path
  | ----------
  | @param[in] max
  | 
  | Stop seeking beyond this limit
  | 
  | -----------
  | @return
  | 
  | The file size or max
  |
  */
pub fn get_file_size(
    path: *const u8,
    max:  Option<StreamSize>) -> usize {

    let max = max.unwrap_or(StreamSize::MAX);
    
    todo!();
        /*
            std::ifstream file(path, std::ios::binary);
        file.ignore(max);
        return file.gcount();
        */
}


/**
  | Ignores exceptions thrown by Boost's
  | create_directories if the requested
  | directory exists.
  | 
  | Specifically handles case where path
  | p exists, but it wasn't possible for
  | the user to write to the parent directory.
  |
  */
pub fn try_create_directories(p: &Path) -> bool {
    
    todo!();
        /*
            try
        {
            return fs::create_directories(p);
        } catch (const fs::filesystem_error&) {
            if (!fs::exists(p) || !fs::is_directory(p))
                throw;
        }

        // create_directories didn't create the directory, it had to have existed already
        return false;
        */
}

/**
  | this function tries to raise the file
  | descriptor limit to the requested number.
  | 
  | It returns the actual file descriptor
  | limit (which may be more or less than
  | nMinFD)
  |
  */
pub fn raise_file_descriptor_limit(n_minfd: i32) -> i32 {
    
    todo!();
        /*
            #if defined(WIN32)
        return 2048;
    #else
        struct rlimit limitFD;
        if (getrlimit(RLIMIT_NOFILE, &limitFD) != -1) {
            if (limitFD.rlim_cur < (rlim_t)nMinFD) {
                limitFD.rlim_cur = nMinFD;
                if (limitFD.rlim_cur > limitFD.rlim_max)
                    limitFD.rlim_cur = limitFD.rlim_max;
                setrlimit(RLIMIT_NOFILE, &limitFD);
                getrlimit(RLIMIT_NOFILE, &limitFD);
            }
            return limitFD.rlim_cur;
        }
        return nMinFD; // getrlimit failed, assume it's fine
    #endif
        */
}


#[cfg(WIN32)]
pub fn get_special_folder_path(
        n_folder: i32,
        create:   bool) -> Path {

    let create: bool = create.unwrap_or(true);
    
    todo!();
        /*
            WCHAR pszPath[MAX_PATH] = L"";

        if(SHGetSpecialFolderPathW(nullptr, pszPath, nFolder, fCreate))
        {
            return fs::path(pszPath);
        }

        LogPrintf("SHGetSpecialFolderPathW() failed, could not obtain requested path.\n");
        return fs::path("");
        */
}

#[cfg(not(WIN32))]
pub fn shell_escape(arg: &String) -> String {
    
    todo!();
        /*
            std::string escaped = arg;
        boost::replace_all(escaped, "'", "'\"'\"'");
        return "'" + escaped + "'";
        */
}

#[cfg(HAVE_SYSTEM)]
pub fn run_command(str_command: &String)  {
    
    todo!();
        /*
            if (strCommand.empty()) return;
    #ifndef WIN32
        int nErr = ::system(strCommand.c_str());
    #else
        int nErr = ::_wsystem(std::wstring_convert<std::codecvt_utf8_utf16<wchar_t>,wchar_t>().from_bytes(strCommand).c_str());
    #endif
        if (nErr)
            LogPrintf("runCommand error: system(%s) returned %d\n", strCommand, nErr);
        */
}

/**
  | Execute a command which returns JSON,
  | and parse the result.
  | 
  | -----------
  | @param str_command
  | 
  | The command to execute, including any
  | arguments
  | ----------
  | @param str_std_in
  | 
  | string to pass to stdin
  | 
  | -----------
  | @return
  | 
  | parsed JSON
  |
  */
pub fn run_command_parsejson(
        str_command: Option<&str>,
        str_std_in:  Option<&str>) -> UniValue {

    let str_std_in: &str = str_std_in.unwrap_or("");
    
    todo!();
        /*
            #ifdef ENABLE_EXTERNAL_SIGNER
        namespace bp = boost::process;

        UniValue result_json;
        bp::opstream stdin_stream;
        bp::ipstream stdout_stream;
        bp::ipstream stderr_stream;

        if (str_command.empty()) return UniValue::VNULL;

        bp::child c(
            str_command,
            bp::std_out > stdout_stream,
            bp::std_err > stderr_stream,
            bp::std_in < stdin_stream
        );
        if (!str_std_in.empty()) {
            stdin_stream << str_std_in << std::endl;
        }
        stdin_stream.pipe().close();

        std::string result;
        std::string error;
        std::getline(stdout_stream, result);
        std::getline(stderr_stream, error);

        c.wait();
        const int n_error = c.exit_code();
        if (n_error) throw std::runtime_error(strprintf("RunCommandParseJSON error: process(%s) returned %d: %s\n", str_command, n_error, error));
        if (!result_json.read(result)) throw std::runtime_error("Unable to parse JSON: " + result);

        return result_json;
    #else
        throw std::runtime_error("Compiled without external signing support (required for external signing).");
    #endif // ENABLE_EXTERNAL_SIGNER
        */
}

/**
  | Return the number of cores available
  | on the current system.
  | 
  | -----------
  | @note
  | 
  | This does count virtual cores, such
  | as those provided by HyperThreading.
  |
  */
pub fn get_num_cores() -> i32 {
    
    todo!();
        /*
            return std::thread::hardware_concurrency();
        */
}

pub fn copyright_holders(str_prefix: &String) -> String {
    
    todo!();
        /*
            const auto copyright_devs = strprintf(_(COPYRIGHT_HOLDERS).translated, COPYRIGHT_HOLDERS_SUBSTITUTION);
        std::string strCopyrightHolders = strPrefix + copyright_devs;

        // Make sure Bitcoin Core copyright is not removed by accident
        if (copyright_devs.find("Bitcoin Core") == std::string::npos) {
            strCopyrightHolders += "\n" + strPrefix + "The Bitcoin Core developers";
        }
        return strCopyrightHolders;
        */
}

/**
  | Application startup time (used for
  | uptime calculation)
  |
  */
pub fn get_startup_time() -> i64 {
    
    todo!();
        /*
            return nStartupTime;
        */
}


/**
  | On platforms that support it, tell the
  | kernel the calling thread is
  | 
  | CPU-intensive and non-interactive.
  | See SCHED_BATCH in sched(7) for details.
  |
  */
pub fn schedule_batch_priority()  {
    
    todo!();
        /*
            #ifdef SCHED_BATCH
        const static sched_param param{};
        const int rc = pthread_setschedparam(pthread_self(), SCHED_BATCH, &param);
        if (rc != 0) {
            LogPrintf("Failed to pthread_setschedparam: %s\n", strerror(rc));
        }
    #endif
        */
}


//-------------------------------------------[.cpp/bitcoin/src/util/threadnames.h]

//-------------------------------------------[.cpp/bitcoin/src/util/threadnames.cpp]

/**
  | Set the thread's name at the process
  | level. Does not affect the internal
  | name.
  |
  */
pub fn set_thread_name(name: *const u8) {
    
    todo!();
        /*
            #if defined(PR_SET_NAME)
        // Only the first 15 characters are used (16 - NUL terminator)
        ::prctl(PR_SET_NAME, name, 0, 0, 0);
    #elif (defined(__FreeBSD__) || defined(__OpenBSD__) || defined(__DragonFly__))
        pthread_set_name_np(pthread_self(), name);
    #elif defined(MAC_OSX)
        pthread_setname_np(name);
    #else
        // Prevent warnings for unused parameters...
        (c_void)name;
    #endif
        */
}

/**
  | If we have thread_local, just keep thread
  | ID and name in a thread_local global.
  |
  */
#[cfg(HAVE_THREAD_LOCAL)]
lazy_static!{
    /*
    static thread_local std::string g_thread_name;
    */
}

/**
  | Get the thread's internal (in-memory)
  | name; used e.g. for identification
  | in logging.
  |
  */
pub fn thread_get_internal_name() -> &'static str {
    
    todo!();
        /*
        #[cfg(HAVE_THREAD_LOCAL)]
        return g_thread_name;

        #[cfg(not(HAVE_THREAD_LOCAL))]
        return empty_string;
        */
}

/**
  | Set the in-memory internal name for
  | this thread. Does not affect the process
  | name.
  |
  */
#[cfg(HAVE_THREAD_LOCAL)]
pub fn set_internal_name(name: String)  {
    
    todo!();
        /*
            g_thread_name = std::move(name);
        */
}

/**
  | Without thread_local available, don't
  | handle internal name at all.
  |
  */
#[cfg(not(HAVE_THREAD_LOCAL))]
lazy_static!{
    /*
    static const std::string empty_string;
    */
}

#[cfg(not(HAVE_THREAD_LOCAL))]
pub fn set_internal_name(name: String)  {
    
    todo!();
        /*
        
        */
}

/**
  | Rename a thread both in terms of an internal
  | (in-memory) name as well as its system thread
  | name.
  |
  | @note Do not call this for the main thread, as
  | this will interfere with UNIX utilities such
  | as top and killall. Use ThreadSetInternalName
  | instead.
  */
pub fn thread_rename(name: String)  {
    
    todo!();
        /*
            SetThreadName(("b-" + name).c_str());
        SetInternalName(std::move(name));
        */
}

/**
  | Set the internal (in-memory) name of
  | the current thread only.
  |
  */
pub fn thread_set_internal_name(name: String)  {
    
    todo!();
        /*
            SetInternalName(std::move(name));
        */
}

//-------------------------------------------[.cpp/bitcoin/src/threadsafety.h]

/**
  | StdMutex provides an annotated version
  | of std::mutex for us, and should only
  | be used when sync.h Mutex/LOCK/etc
  | are not usable.
  |
  */
#[LOCKABLE]
pub struct StdMutex {
    base: RawMutex,
}

#[cfg(__clang__)]
impl Not for StdMutex {

    type Output = StdMutex;
    
    /**
      | For negative capabilities in the Clang
      | Thread Safety Analysis.
      |
      | A negative requirement uses the
      | EXCLUSIVE_LOCKS_REQUIRED attribute, in
      | conjunction with the ! operator, to
      | indicate that a mutex should not be held.
      */
    #[inline] fn not(self) -> Self::Output {
        todo!();
        /*
            return *this;
        */
    }
}

/**
  | StdLockGuard provides an annotated
  | version of std::lock_guard for us,
  | and should only be used when sync.h Mutex/LOCK/etc
  | are not usable.
  |
  */
#[SCOPED_LOCKABLE]
pub struct StdLockGuard<'a> {
    base: LockGuard<'a,StdMutex>,
}

impl<'a> StdLockGuard<'a> {

    #[EXCLUSIVE_LOCK_FUNCTION(cs)]
    pub fn new(cs: &mut StdMutex) -> Self {
    
        todo!();
        /*


            : lock_guard<StdMutex>(cs)
        */
    }
}

impl<'a> Drop for StdLockGuard<'a> {

    #[UNLOCK_FUNCTION()]
    fn drop(&mut self) {
        todo!();
        /*
        
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/util/spanparsing.h]
//-------------------------------------------[.cpp/bitcoin/src/util/spanparsing.cpp]

/**
  | Parse a constant.
  | 
  | If sp's initial part matches str, sp
  | is updated to skip that part, and true
  | is returned.
  | 
  | Otherwise sp is unmodified and false
  | is returned.
  |
  */
pub fn spanparsing_const(
        str_: &String,
        sp:   &mut [u8]) -> bool {
    
    todo!();
        /*
            if ((size_t)sp.size() >= str.size() && std::equal(str.begin(), str.end(), sp.begin())) {
            sp = sp.subspan(str.size());
            return true;
        }
        return false;
        */
}

/**
  | Parse a function call.
  | 
  | If sp's initial part matches str + "(",
  | and sp ends with ")", sp is updated to
  | be the section between the braces, and
  | true is returned. Otherwise sp is unmodified
  | and false is returned.
  |
  */
pub fn spanparsing_func(
        str_: &String,
        sp:   &mut [u8]) -> bool {
    
    todo!();
        /*
            if ((size_t)sp.size() >= str.size() + 2 && sp[str.size()] == '(' && sp[sp.size() - 1] == ')' && std::equal(str.begin(), str.end(), sp.begin())) {
            sp = sp.subspan(str.size() + 1, sp.size() - str.size() - 2);
            return true;
        }
        return false;
        */
}

/**
  | Extract the expression that sp begins
  | with.
  | 
  | This function will return the initial
  | part of sp, up to (but not including)
  | the first comma or closing brace, skipping
  | ones that are surrounded by braces.
  | So for example, for "foo(bar(1),2),3"
  | the initial part "foo(bar(1),2)" will
  | be returned. sp will be updated to skip
  | the initial part that is returned.
  |
  */
pub fn spanparsing_expr(sp: &mut [u8]) -> &[u8] {
    
    todo!();
        /*
            int level = 0;
        auto it = sp.begin();
        while (it != sp.end()) {
            if (*it == '(' || *it == '{') {
                ++level;
            } else if (level && (*it == ')' || *it == '}')) {
                --level;
            } else if (level == 0 && (*it == ')' || *it == '}' || *it == ',')) {
                break;
            }
            ++it;
        }
        Span<const char> ret = sp.first(it - sp.begin());
        sp = sp.subspan(it - sp.begin());
        return ret;
        */
}

/**
  | Split a string on every instance of sep,
  | returning a vector.
  | 
  | If sep does not occur in sp, a singleton
  | with the entirety of sp is returned.
  | 
  | -----------
  | @note
  | 
  | this function does not care about braces,
  | so splitting "foo(bar(1),2),3) on
  | ',' will return {"foo(bar(1)", "2)",
  | "3)"}.
  |
  */
pub fn spanparsing_split(
        sp:  &[u8],
        sep: u8) -> Vec<&[u8]> {
    
    todo!();
        /*
            std::vector<Span<const char>> ret;
        auto it = sp.begin();
        auto start = it;
        while (it != sp.end()) {
            if (*it == sep) {
                ret.emplace_back(start, it);
                start = it + 1;
            }
            ++it;
        }
        ret.emplace_back(start, it);
        return ret;
        */
}

//-------------------------------------------[.cpp/bitcoin/src/bitcoin-util.cpp]

pub const CONTINUE_EXECUTION: i32 = -1;

//pub const G_TRANSLATION_FUN: Option<fn(_0: *const u8) -> String> = None;

pub fn setup_bitcoin_util_args(argsman: &mut ArgsManager)  {
    
    todo!();
        /*
            SetupHelpOptions(argsman);

        argsman.AddArg("-version", "Print version and exit", ArgsManager::ALLOW_ANY, OptionsCategory::OPTIONS);

        argsman.AddCommand("grind", "Perform proof of work on hex header string");

        SetupChainParamsBaseOptions(argsman);
        */
}

/**
  | This function returns either one of EXIT_ codes
  | when it's expected to stop the process or
  | CONTINUE_EXECUTION when it's expected to
  | continue further.
  */
pub fn app_init_util(
        args: &mut ArgsManager,
        argc: i32,
        argv: &[*mut u8]) -> i32 {
    
    todo!();
        /*
            SetupBitcoinUtilArgs(args);
        std::string error;
        if (!args.ParseParameters(argc, argv, error)) {
            tfm::format(std::cerr, "Error parsing command line arguments: %s\n", error);
            return EXIT_FAILURE;
        }

        if (HelpRequested(args) || args.IsArgSet("-version")) {
            // First part of help message is specific to this utility
            std::string strUsage = PACKAGE_NAME " bitcoin-util utility version " + FormatFullVersion() + "\n";
            if (!args.IsArgSet("-version")) {
                strUsage += "\n"
                    "Usage:  bitcoin-util [options] [commands]  Do stuff\n";
                strUsage += "\n" + args.GetHelpMessage();
            }

            tfm::format(std::cout, "%s", strUsage);

            if (argc < 2) {
                tfm::format(std::cerr, "Error: too few parameters\n");
                return EXIT_FAILURE;
            }
            return EXIT_SUCCESS;
        }

        // Check for chain settings (Params() calls are only valid after this clause)
        try {
            SelectParams(args.GetChainName());
        } catch (const std::exception& e) {
            tfm::format(std::cerr, "Error: %s\n", e.what());
            return EXIT_FAILURE;
        }

        return CONTINUE_EXECUTION;
        */
}

pub fn grind_task(
        n_bits:      u32,
        header_orig: &mut BlockHeader,
        offset:      u32,
        step:        u32,
        found:       &mut AtomicBool)  {
    
    todo!();
        /*
            arith_uint256 target;
        bool neg, over;
        target.SetCompact(nBits, &neg, &over);
        if (target == 0 || neg || over) return;
        CBlockHeader header = header_orig; // working copy
        header.nNonce = offset;

        uint32_t finish = std::numeric_limits<uint32_t>::max() - step;
        finish = finish - (finish % step) + offset;

        while (!found && header.nNonce < finish) {
            const uint32_t next = (finish - header.nNonce < 5000*step) ? finish : header.nNonce + 5000*step;
            do {
                if (UintToArith256(header.GetHash()) <= target) {
                    if (!found.exchange(true)) {
                        header_orig.nNonce = header.nNonce;
                    }
                    return;
                }
                header.nNonce += step;
            } while(header.nNonce != next);
        }
        */
}

pub fn grind(
        args:      &Vec<String>,
        str_print: &mut String) -> i32 {
    
    todo!();
        /*
            if (args.size() != 1) {
            strPrint = "Must specify block header to grind";
            return EXIT_FAILURE;
        }

        CBlockHeader header;
        if (!DecodeHexBlockHeader(header, args[0])) {
            strPrint = "Could not decode block header";
            return EXIT_FAILURE;
        }

        uint32_t nBits = header.nBits;
        std::atomic<bool> found{false};

        std::vector<std::thread> threads;
        int n_tasks = std::max(1u, std::thread::hardware_concurrency());
        for (int i = 0; i < n_tasks; ++i) {
            threads.emplace_back( grind_task, nBits, std::ref(header), i, n_tasks, std::ref(found) );
        }
        for (auto& t : threads) {
            t.join();
        }
        if (!found) {
            strPrint = "Could not satisfy difficulty target";
            return EXIT_FAILURE;
        }

        DataStream ss(SER_NETWORK, PROTOCOL_VERSION);
        ss << header;
        strPrint = HexStr(ss);
        return EXIT_SUCCESS;
        */
}

pub fn util_main(
        argc: i32,
        argv: &[*mut u8]) -> i32 {
    
    todo!();
        /*
            ArgsManager& args = gArgs;
        SetupEnvironment();

        try {
            int ret = AppInitUtil(args, argc, argv);
            if (ret != CONTINUE_EXECUTION) {
                return ret;
            }
        } catch (const std::exception& e) {
            PrintExceptionContinue(&e, "AppInitUtil()");
            return EXIT_FAILURE;
        } catch (...) {
            PrintExceptionContinue(nullptr, "AppInitUtil()");
            return EXIT_FAILURE;
        }

        const auto cmd = args.GetCommand();
        if (!cmd) {
            tfm::format(std::cerr, "Error: must specify a command\n");
            return EXIT_FAILURE;
        }

        int ret = EXIT_FAILURE;
        std::string strPrint;
        try {
            if (cmd->command == "grind") {
                ret = Grind(cmd->args, strPrint);
            } else {
                assert(false); // unknown command should be caught earlier
            }
        } catch (const std::exception& e) {
            strPrint = std::string("error: ") + e.what();
        } catch (...) {
            strPrint = "unknown error";
        }

        if (strPrint != "") {
            tfm::format(ret == 0 ? std::cout : std::cerr, "%s\n", strPrint);
        }

        return ret;
        */
}

//-------------------------------------------[.cpp/bitcoin/src/util/vector.h]

/**
  | Construct a vector with the specified
  | elements.
  | 
  | This is preferable over the list initializing
  | constructor of std::vector:
  | 
  | - It automatically infers the element
  | type from its arguments.
  | 
  | - If any arguments are rvalue references,
  | they will be moved into the vector (list
  | initialization always copies).
  |
  */
#[inline] pub fn vector<Args>(args: Args) -> Vec<CommonType<Args>> {

    todo!();
        /*
            std::vector<typename std::common_type<Args...>::type> ret;
        ret.reserve(sizeof...(args));
        // The line below uses the trick from https://www.experts-exchange.com/articles/32502/None-recursive-variadic-templates-with-std-initializer-list.html
        (c_void)std::initializer_list<int>{(ret.emplace_back(std::forward<Args>(args)), 0)...};
        return ret;
        */
}

//-------------------------------------------[.cpp/bitcoin/src/warnings.h]
//-------------------------------------------[.cpp/bitcoin/src/warnings.cpp]

lazy_static!{
    /*
    static Mutex g_warnings_mutex;
    static bilingual_str g_misc_warnings GUARDED_BY(g_warnings_mutex);
    static bool fLargeWorkInvalidChainFound GUARDED_BY(g_warnings_mutex) = false;
    */
}

pub fn set_misc_warning(warning: &BilingualStr)  {
    
    todo!();
        /*
            LOCK(g_warnings_mutex);
        g_misc_warnings = warning;
        */
}

pub fn setf_large_work_invalid_chain_found(flag: bool)  {
    
    todo!();
        /*
            LOCK(g_warnings_mutex);
        fLargeWorkInvalidChainFound = flag;
        */
}

/**
  | Format a string that describes several
  | potential problems detected by the
  | core.
  | 
  | -----------
  | @param[in] verbose
  | 
  | bool
  | 
  | - if true, get all warnings separated
  | by <hr />
  | 
  | - if false, get the most important warning
  | 
  | -----------
  | @return
  | 
  | the warning string
  |
  */
pub fn get_warnings(verbose: bool) -> BilingualStr {
    
    todo!();
        /*
            bilingual_str warnings_concise;
        std::vector<bilingual_str> warnings_verbose;

        LOCK(g_warnings_mutex);

        // Pre-release build warning
        if (!CLIENT_VERSION_IS_RELEASE) {
            warnings_concise = _("This is a pre-release test build - use at your own risk - do not use for mining or merchant applications");
            warnings_verbose.emplace_back(warnings_concise);
        }

        // Misc warnings like out of disk space and clock is wrong
        if (!g_misc_warnings.empty()) {
            warnings_concise = g_misc_warnings;
            warnings_verbose.emplace_back(warnings_concise);
        }

        if (fLargeWorkInvalidChainFound) {
            warnings_concise = _("Warning: We do not appear to fully agree with our peers! You may need to upgrade, or other nodes may need to upgrade.");
            warnings_verbose.emplace_back(warnings_concise);
        }

        if (verbose) {
            return Join(warnings_verbose, Untranslated("<hr />"));
        }

        return warnings_concise;
        */
}

//-------------------------------------------[.cpp/bitcoin/src/shutdown.h]
//-------------------------------------------[.cpp/bitcoin/src/shutdown.cpp]

/**
  | Abort with a message
  |
  */
pub fn abort_node(
        str_message:  &String,
        user_message: Option<BilingualStr>) -> bool {

    let user_message: BilingualStr = user_message.unwrap_or(BilingualStr::default());
    
    todo!();
        /*
        SetMiscWarning(Untranslated(strMessage));
        LogPrintf("*** %s\n", strMessage);
        if (user_message.empty()) {
            user_message = _("A fatal internal error occurred, see debug.log for details");
        }
        AbortError(user_message);
        StartShutdown();
        return false;
        */
}

lazy_static!{
    /*
    static std::atomic<bool> fRequestShutdown = false;
    */
}

/**
  | On windows it is possible to simply use
  | a condition variable.
  |
  */
#[cfg(WIN32)]
lazy_static!{
    /*
    std::mutex g_shutdown_mutex;
    std::condition_variable g_shutdown_cv;
    */
}

/**
  | On UNIX-like operating systems use
  | the self-pipe trick.
  |
  */
#[cfg(not(WIN32))]
lazy_static!{
    /*
    static TokenPipeEnd g_shutdown_r;
    static TokenPipeEnd g_shutdown_w;
    */
}

/**
  | Initialize shutdown state. This must
  | be called before using either StartShutdown(),
  | 
  | AbortShutdown() or WaitForShutdown().
  | Calling ShutdownRequested() is always
  | safe.
  |
  */
pub fn init_shutdown_state() -> bool {
    
    todo!();
        /*
    #ifndef WIN32
        std::optional<TokenPipe> pipe = TokenPipe::Make();
        if (!pipe) return false;
        g_shutdown_r = pipe->TakeReadEnd();
        g_shutdown_w = pipe->TakeWriteEnd();
    #endif
        return true;
        */
}

/**
  | Request shutdown of the application.
  |
  */
pub fn start_shutdown()  {
    
    todo!();
        /*
            #ifdef WIN32
        std::unique_lock<std::mutex> lk(g_shutdown_mutex);
        fRequestShutdown = true;
        g_shutdown_cv.notify_one();
    #else
        // This must be reentrant and safe for calling in a signal handler, so using a condition variable is not safe.
        // Make sure that the token is only written once even if multiple threads call this concurrently or in
        // case of a reentrant signal.
        if (!fRequestShutdown.exchange(true)) {
            // Write an arbitrary byte to the write end of the shutdown pipe.
            int res = g_shutdown_w.TokenWrite('x');
            if (res != 0) {
                LogPrintf("Sending shutdown token failed\n");
                assert(0);
            }
        }
    #endif
        */
}

/**
  | Clear shutdown flag. Only use this during
  | init (before calling WaitForShutdown
  | in any thread), or in the unit tests.
  | Calling it in other circumstances will
  | cause a race condition.
  |
  */
pub fn abort_shutdown()  {
    
    todo!();
        /*
            if (fRequestShutdown) {
            // Cancel existing shutdown by waiting for it, this will reset condition flags and remove
            // the shutdown token from the pipe.
            WaitForShutdown();
        }
        fRequestShutdown = false;
        */
}

/**
  | Returns true if a shutdown is requested,
  | false otherwise.
  |
  */
pub fn shutdown_requested() -> bool {
    
    todo!();
        /*
            return fRequestShutdown;
        */
}

/**
  | Wait for StartShutdown to be called
  | in any thread. This can only be used from
  | a single thread.
  |
  */
pub fn wait_for_shutdown()  {
    
    todo!();
        /*
            #ifdef WIN32
        std::unique_lock<std::mutex> lk(g_shutdown_mutex);
        g_shutdown_cv.wait(lk, [] { return fRequestShutdown.load(); });
    #else
        int res = g_shutdown_r.TokenRead();
        if (res != 'x') {
            LogPrintf("Reading shutdown token failed\n");
            assert(0);
        }
    #endif
        */
}

//-------------------------------------------[.cpp/bitcoin/src/util/serfloat.h]
//-------------------------------------------[.cpp/bitcoin/src/util/serfloat.cpp]

/**
  | Reverse operation of DecodeDouble.
  | DecodeDouble(EncodeDouble(f))==f
  | unless isnan(f).
  |
  */
pub fn decode_double(v: u64) -> f64 {
    
    todo!();
        /*
            static constexpr double NANVAL = std::numeric_limits<double>::quiet_NaN();
        static constexpr double INFVAL = std::numeric_limits<double>::infinity();
        double sign = 1.0;
        if (v & 0x8000000000000000) {
            sign = -1.0;
            v ^= 0x8000000000000000;
        }
        // Zero
        if (v == 0) return copysign(0.0, sign);
        // Infinity
        if (v == 0x7ff0000000000000) return copysign(INFVAL, sign);
        // Other numbers
        int exp = (v & 0x7FF0000000000000) >> 52;
        uint64_t man = v & 0xFFFFFFFFFFFFF;
        if (exp == 2047) {
            // NaN
            return NANVAL;
        } else if (exp == 0) {
            // Subnormal
            return copysign(ldexp((double)man, -1074), sign);
        } else {
            // Normal
            return copysign(ldexp((double)(man + 0x10000000000000), -1075 + exp), sign);
        }
        */
}

/**
  | Encode a double using the IEEE 754 binary64
  | format. All NaNs are encoded as x86/ARM's
  | positive quiet NaN with payload 0.
  |
  */
pub fn encode_double(f: f64) -> u64 {
    
    todo!();
        /*
            int cls = std::fpclassify(f);
        uint64_t sign = 0;
        if (copysign(1.0, f) == -1.0) {
            f = -f;
            sign = 0x8000000000000000;
        }
        // Zero
        if (cls == FP_ZERO) return sign;
        // Infinity
        if (cls == FP_INFINITE) return sign | 0x7ff0000000000000;
        // NaN
        if (cls == FP_NAN) return 0x7ff8000000000000;
        // Other numbers
        int exp;
        uint64_t man = std::round(std::frexp(f, &exp) * 9007199254740992.0);
        if (exp < -1021) {
            // Too small to represent, encode 0
            if (exp < -1084) return sign;
            // Subnormal numbers
            return sign | (man >> (-1021 - exp));
        } else {
            // Too big to represent, encode infinity
            if (exp > 1024) return sign | 0x7ff0000000000000;
            // Normal numbers
            return sign | (((uint64_t)(1022 + exp)) << 52) | (man & 0xFFFFFFFFFFFFF);
        }
        */
}

/// Taken from
/// https://gist.github.com/arvidsson/7231973
///
/// 
//-------------------------------------------[.cpp/bitcoin/src/util/readwritefile.h]
//-------------------------------------------[.cpp/bitcoin/src/util/readwritefile.cpp]

/**
  | Read full contents of a file and return
  | them in a std::string.
  | 
  | Returns a pair <status, string>.
  | 
  | If an error occurred, status will be
  | false, otherwise status will be true
  | and the data will be returned in string.
  | 
  | -----------
  | @param maxsize
  | 
  | Puts a maximum size limit on the file
  | that is read. If the file is larger than
  | this, truncated data (with len > maxsize)
  | will be returned.
  |
  */
pub fn read_binary_file(
        filename: &Path,
        maxsize:  Option<usize>) -> (bool,String) {

    let maxsize = maxsize.unwrap_or(usize::MAX);
    
    todo!();
        /*
            FILE *f = fsbridge::fopen(filename, "rb");
        if (f == nullptr)
            return std::make_pair(false,"");
        std::string retval;
        char buffer[128];
        do {
            const size_t n = fread(buffer, 1, sizeof(buffer), f);
            // Check for reading errors so we don't return any data if we couldn't
            // read the entire file (or up to maxsize)
            if (ferror(f)) {
                fclose(f);
                return std::make_pair(false,"");
            }
            retval.append(buffer, buffer+n);
        } while (!feof(f) && retval.size() <= maxsize);
        fclose(f);
        return std::make_pair(true,retval);
        */
}

/**
  | Write contents of std::string to a file.
  | 
  | -----------
  | @return
  | 
  | true on success.
  |
  */
pub fn write_binary_file(
        filename: &Path,
        data:     &String) -> bool {
    
    todo!();
        /*
            FILE *f = fsbridge::fopen(filename, "wb");
        if (f == nullptr)
            return false;
        if (fwrite(data.data(), 1, data.size(), f) != data.size()) {
            fclose(f);
            return false;
        }
        if (fclose(f) != 0) {
            return false;
        }
        return true;
        */
}

pub fn parse_op_code(s: &String) -> OpcodeType {
    
    todo!();
        /*
            static std::map<std::string, opcodetype> mapOpNames;

        if (mapOpNames.empty()) {
            for (unsigned int op = 0; op <= MAX_OPCODE; op++) {
                // Allow OP_RESERVED to get into mapOpNames
                if (op < OP_NOP && op != OP_RESERVED) {
                    continue;
                }

                std::string strName = GetOpName(static_cast<opcodetype>(op));
                if (strName == "OP_UNKNOWN") {
                    continue;
                }
                mapOpNames[strName] = static_cast<opcodetype>(op);
                // Convenience: OP_ADD and just ADD are both recognized:
                if (strName.compare(0, 3, "OP_") == 0) { // strName starts with "OP_"
                    mapOpNames[strName.substr(3)] = static_cast<opcodetype>(op);
                }
            }
        }

        auto it = mapOpNames.find(s);
        if (it == mapOpNames.end()) throw std::runtime_error("script parse error: unknown opcode");
        return it->second;
        */
}

pub fn parse_script(s: &String) -> Script {
    
    todo!();
        /*
            CScript result;

        std::vector<std::string> words;
        boost::algorithm::split(words, s, boost::algorithm::is_any_of(" \t\n"), boost::algorithm::token_compress_on);

        for (const std::string& w : words) {
            if (w.empty()) {
                // Empty string, ignore. (boost::split given '' will return one word)
            } else if (std::all_of(w.begin(), w.end(), ::IsDigit) ||
                       (w.front() == '-' && w.size() > 1 && std::all_of(w.begin() + 1, w.end(), ::IsDigit)))
            {
                // Number
                const auto num{ToIntegral<int64_t>(w)};

                // limit the range of numbers ParseScript accepts in decimal
                // since numbers outside -0xFFFFFFFF...0xFFFFFFFF are illegal in scripts
                if (!num.has_value() || num > int64_t{0xffffffff} || num < -1 * int64_t{0xffffffff}) {
                    throw std::runtime_error("script parse error: decimal numeric value only allowed in the "
                                             "range -0xFFFFFFFF...0xFFFFFFFF");
                }

                result << num.value();
            } else if (w.substr(0, 2) == "0x" && w.size() > 2 && IsHex(std::string(w.begin() + 2, w.end()))) {
                // Raw hex data, inserted NOT pushed onto stack:
                std::vector<unsigned char> raw = ParseHex(std::string(w.begin() + 2, w.end()));
                result.insert(result.end(), raw.begin(), raw.end());
            } else if (w.size() >= 2 && w.front() == '\'' && w.back() == '\'') {
                // Single-quoted string, pushed as data. NOTE: this is poor-man's
                // parsing, spaces/tabs/newlines in single-quoted strings won't work.
                std::vector<unsigned char> value(w.begin() + 1, w.end() - 1);
                result << value;
            } else {
                // opcode, e.g. OP_ADD or ADD:
                result << ParseOpCode(w);
            }
        }

        return result;
        */
}

/**
   Check that all of the input and output scripts
   of a transaction contains valid opcodes
  */
pub fn check_tx_scripts_sanity(tx: &MutableTransaction) -> bool {
    
    todo!();
        /*
            // Check input scripts for non-coinbase txs
        if (!CTransaction(tx).IsCoinBase()) {
            for (unsigned int i = 0; i < tx.vin.size(); i++) {
                if (!tx.vin[i].scriptSig.HasValidOps() || tx.vin[i].scriptSig.size() > MAX_SCRIPT_SIZE) {
                    return false;
                }
            }
        }
        // Check output scripts
        for (unsigned int i = 0; i < tx.vout.size(); i++) {
            if (!tx.vout[i].scriptPubKey.HasValidOps() || tx.vout[i].scriptPubKey.size() > MAX_SCRIPT_SIZE) {
                return false;
            }
        }

        return true;
        */
}

pub fn decode_tx(
        tx:             &mut MutableTransaction,
        tx_data:        &Vec<u8>,
        try_no_witness: bool,
        try_witness:    bool) -> bool {
    
    todo!();
        /*
            // General strategy:
        // - Decode both with extended serialization (which interprets the 0x0001 tag as a marker for
        //   the presence of witnesses) and with legacy serialization (which interprets the tag as a
        //   0-input 1-output incomplete transaction).
        //   - Restricted by try_no_witness (which disables legacy if false) and try_witness (which
        //     disables extended if false).
        //   - Ignore serializations that do not fully consume the hex string.
        // - If neither succeeds, fail.
        // - If only one succeeds, return that one.
        // - If both decode attempts succeed:
        //   - If only one passes the CheckTxScriptsSanity check, return that one.
        //   - If neither or both pass CheckTxScriptsSanity, return the extended one.

        CMutableTransaction tx_extended, tx_legacy;
        bool ok_extended = false, ok_legacy = false;

        // Try decoding with extended serialization support, and remember if the result successfully
        // consumes the entire input.
        if (try_witness) {
            DataStream ssData(tx_data, SER_NETWORK, PROTOCOL_VERSION);
            try {
                ssData >> tx_extended;
                if (ssData.empty()) ok_extended = true;
            } catch (const std::exception&) {
                // Fall through.
            }
        }

        // Optimization: if extended decoding succeeded and the result passes CheckTxScriptsSanity,
        // don't bother decoding the other way.
        if (ok_extended && CheckTxScriptsSanity(tx_extended)) {
            tx = std::move(tx_extended);
            return true;
        }

        // Try decoding with legacy serialization, and remember if the result successfully consumes the entire input.
        if (try_no_witness) {
            DataStream ssData(tx_data, SER_NETWORK, PROTOCOL_VERSION | SERIALIZE_TRANSACTION_NO_WITNESS);
            try {
                ssData >> tx_legacy;
                if (ssData.empty()) ok_legacy = true;
            } catch (const std::exception&) {
                // Fall through.
            }
        }

        // If legacy decoding succeeded and passes CheckTxScriptsSanity, that's our answer, as we know
        // at this point that extended decoding either failed or doesn't pass the sanity check.
        if (ok_legacy && CheckTxScriptsSanity(tx_legacy)) {
            tx = std::move(tx_legacy);
            return true;
        }

        // If extended decoding succeeded, and neither decoding passes sanity, return the extended one.
        if (ok_extended) {
            tx = std::move(tx_extended);
            return true;
        }

        // If legacy decoding succeeded and extended didn't, return the legacy one.
        if (ok_legacy) {
            tx = std::move(tx_legacy);
            return true;
        }

        // If none succeeded, we failed.
        return false;
        */
}

pub fn decode_hex_tx(
        tx:             &mut MutableTransaction,
        hex_tx:         &String,
        try_no_witness: bool,
        try_witness:    bool) -> bool {
    
    todo!();
        /*
            if (!IsHex(hex_tx)) {
            return false;
        }

        std::vector<unsigned char> txData(ParseHex(hex_tx));
        return DecodeTx(tx, txData, try_no_witness, try_witness);
        */
}

pub fn decode_hex_block_header(
        header:     &mut BlockHeader,
        hex_header: &String) -> bool {
    
    todo!();
        /*
            if (!IsHex(hex_header)) return false;

        const std::vector<unsigned char> header_data{ParseHex(hex_header)};
        DataStream ser_header(header_data, SER_NETWORK, PROTOCOL_VERSION);
        try {
            ser_header >> header;
        } catch (const std::exception&) {
            return false;
        }
        return true;
        */
}

pub fn decode_hex_blk(
        block:       &mut Block,
        str_hex_blk: &String) -> bool {
    
    todo!();
        /*
            if (!IsHex(strHexBlk))
            return false;

        std::vector<unsigned char> blockData(ParseHex(strHexBlk));
        DataStream ssBlock(blockData, SER_NETWORK, PROTOCOL_VERSION);
        try {
            ssBlock >> block;
        }
        catch (const std::exception&) {
            return false;
        }

        return true;
        */
}

pub fn parse_hash_str(
        str_hex: &String,
        result:  &mut u256) -> bool {
    
    todo!();
        /*
            if ((strHex.size() != 64) || !IsHex(strHex))
            return false;

        result.SetHex(strHex);
        return true;
        */
}

pub fn parse_hexuv(
        v:        &UniValue,
        str_name: &String) -> Vec<u8> {
    
    todo!();
        /*
            std::string strHex;
        if (v.isStr())
            strHex = v.getValStr();
        if (!IsHex(strHex))
            throw std::runtime_error(strName + " must be hexadecimal string (not '" + strHex + "')");
        return ParseHex(strHex);
        */
}

pub fn parse_sighash_string(sighash: &UniValue) -> i32 {
    
    todo!();
        /*
            int hash_type = SIGHASH_ALL;
        if (!sighash.isNull()) {
            static std::map<std::string, int> map_sighash_values = {
                {std::string("DEFAULT"), int(SIGHASH_DEFAULT)},
                {std::string("ALL"), int(SIGHASH_ALL)},
                {std::string("ALL|ANYONECANPAY"), int(SIGHASH_ALL|SIGHASH_ANYONECANPAY)},
                {std::string("NONE"), int(SIGHASH_NONE)},
                {std::string("NONE|ANYONECANPAY"), int(SIGHASH_NONE|SIGHASH_ANYONECANPAY)},
                {std::string("SINGLE"), int(SIGHASH_SINGLE)},
                {std::string("SINGLE|ANYONECANPAY"), int(SIGHASH_SINGLE|SIGHASH_ANYONECANPAY)},
            };
            std::string strHashType = sighash.get_str();
            const auto& it = map_sighash_values.find(strHashType);
            if (it != map_sighash_values.end()) {
                hash_type = it->second;
            } else {
                throw std::runtime_error(strHashType + " is not a valid sighash parameter.");
            }
        }
        return hash_type;
        */
}

pub fn value_from_amount(amount: Amount) -> UniValue {
    
    todo!();
        /*
            const_assert(COIN > 1);
        int64_t quotient = amount / COIN;
        int64_t remainder = amount % COIN;
        if (amount < 0) {
            quotient = -quotient;
            remainder = -remainder;
        }
        return UniValue(UniValue::VNUM,
                strprintf("%s%d.%08d", amount < 0 ? "-" : "", quotient, remainder));
        */
}

pub fn format_script(script: &Script) -> String {
    
    todo!();
        /*
            std::string ret;
        CScript::const_iterator it = script.begin();
        opcodetype op;
        while (it != script.end()) {
            CScript::const_iterator it2 = it;
            std::vector<unsigned char> vch;
            if (script.GetOp(it, op, vch)) {
                if (op == OP_0) {
                    ret += "0 ";
                    continue;
                } else if ((op >= OP_1 && op <= OP_16) || op == OP_1NEGATE) {
                    ret += strprintf("%i ", op - OP_1NEGATE - 1);
                    continue;
                } else if (op >= OP_NOP && op <= OP_NOP10) {
                    std::string str(GetOpName(op));
                    if (str.substr(0, 3) == std::string("OP_")) {
                        ret += str.substr(3, std::string::npos) + " ";
                        continue;
                    }
                }
                if (vch.size() > 0) {
                    ret += strprintf("0x%x 0x%x ", HexStr(std::vector<uint8_t>(it2, it - vch.size())),
                                                   HexStr(std::vector<uint8_t>(it - vch.size(), it)));
                } else {
                    ret += strprintf("0x%x ", HexStr(std::vector<uint8_t>(it2, it)));
                }
                continue;
            }
            ret += strprintf("0x%x ", HexStr(std::vector<uint8_t>(it2, script.end())));
            break;
        }
        return ret.substr(0, ret.size() - 1);
        */
}

lazy_static!{
    /*
    const std::map<unsigned char, std::string> mapSigHashTypes = {
        {static_cast<unsigned char>(SIGHASH_ALL), std::string("ALL")},
        {static_cast<unsigned char>(SIGHASH_ALL|SIGHASH_ANYONECANPAY), std::string("ALL|ANYONECANPAY")},
        {static_cast<unsigned char>(SIGHASH_NONE), std::string("NONE")},
        {static_cast<unsigned char>(SIGHASH_NONE|SIGHASH_ANYONECANPAY), std::string("NONE|ANYONECANPAY")},
        {static_cast<unsigned char>(SIGHASH_SINGLE), std::string("SINGLE")},
        {static_cast<unsigned char>(SIGHASH_SINGLE|SIGHASH_ANYONECANPAY), std::string("SINGLE|ANYONECANPAY")},
    };
    */
}

pub fn sighash_to_str(sighash_type: u8) -> String {
    
    todo!();
        /*
            const auto& it = mapSigHashTypes.find(sighash_type);
        if (it == mapSigHashTypes.end()) return "";
        return it->second;
        */
}

/**
  | Create the assembly string representation
  | of a CScript object.
  | 
  | -----------
  | @param[in] script
  | 
  | CScript object to convert into the asm
  | string representation.
  | ----------
  | @param[in] fAttemptSighashDecode
  | 
  | Whether to attempt to decode sighash
  | types on data within the script that
  | matches the format of a signature. Only
  | pass true for scripts you believe could
  | contain signatures. For example, pass
  | false, or omit the this argument (defaults
  | to false), for scriptPubKeys.
  |
  */
pub fn script_to_asm_str(
        script:                 &Script,
        attempt_sighash_decode: bool) -> String {
    
    todo!();
        /*
            std::string str;
        opcodetype opcode;
        std::vector<unsigned char> vch;
        CScript::const_iterator pc = script.begin();
        while (pc < script.end()) {
            if (!str.empty()) {
                str += " ";
            }
            if (!script.GetOp(pc, opcode, vch)) {
                str += "[error]";
                return str;
            }
            if (0 <= opcode && opcode <= OP_PUSHDATA4) {
                if (vch.size() <= static_cast<std::vector<unsigned char>::size_type>(4)) {
                    str += strprintf("%d", CScriptNum(vch, false).getint());
                } else {
                    // the IsUnspendable check makes sure not to try to decode OP_RETURN data that may match the format of a signature
                    if (fAttemptSighashDecode && !script.IsUnspendable()) {
                        std::string strSigHashDecode;
                        // goal: only attempt to decode a defined sighash type from data that looks like a signature within a scriptSig.
                        // this won't decode correctly formatted public keys in Pubkey or Multisig scripts due to
                        // the restrictions on the pubkey formats (see IsCompressedOrUncompressedPubKey) being incongruous with the
                        // checks in CheckSignatureEncoding.
                        if (CheckSignatureEncoding(vch, SCRIPT_VERIFY_STRICTENC, nullptr)) {
                            const unsigned char chSigHashType = vch.back();
                            const auto it = mapSigHashTypes.find(chSigHashType);
                            if (it != mapSigHashTypes.end()) {
                                strSigHashDecode = "[" + it->second + "]";
                                vch.pop_back(); // remove the sighash type byte. it will be replaced by the decode.
                            }
                        }
                        str += HexStr(vch) + strSigHashDecode;
                    } else {
                        str += HexStr(vch);
                    }
                }
            } else {
                str += GetOpName(opcode);
            }
        }
        return str;
        */
}

pub fn encode_hex_tx(
        tx:              &Transaction,
        serialize_flags: i32) -> String {
    
    todo!();
        /*
            DataStream ssTx(SER_NETWORK, PROTOCOL_VERSION | serializeFlags);
        ssTx << tx;
        return HexStr(ssTx);
        */
}

pub fn script_to_univ(
        script: &Script,
        out:    &mut UniValue)  {
    
    todo!();
        /*
            ScriptPubKeyToUniv(script, out, /* include_hex */ true, /* include_address */ false);
        */
}

pub fn script_pub_key_to_univ(
        script_pub_key:  &Script,
        out:             &mut UniValue,
        include_hex:     bool,
        include_address: bool)  {
    
    todo!();
        /*
            TxDestination address;

        out.pushKV("asm", ScriptToAsmStr(scriptPubKey));
        if (include_hex) out.pushKV("hex", HexStr(scriptPubKey));

        std::vector<std::vector<unsigned char>> solns;
        const TxoutType type{Solver(scriptPubKey, solns)};

        if (include_address && ExtractDestination(scriptPubKey, address) && type != TxoutType::PUBKEY) {
            out.pushKV("address", EncodeDestination(address));
        }
        out.pushKV("type", GetTxnOutputType(type));
        */
}

pub fn tx_to_univ(
        tx:              &Transaction,
        hash_block:      &u256,
        entry:           &mut UniValue,
        include_hex:     bool,
        serialize_flags: i32,
        txundo:          *const TxUndo,
        verbosity:       TxVerbosity)  {
    
    todo!();
        /*
            entry.pushKV("txid", tx.GetHash().GetHex());
        entry.pushKV("hash", tx.GetWitnessHash().GetHex());
        // Transaction version is actually unsigned in consensus checks, just signed in memory,
        // so cast to unsigned before giving it to the user.
        entry.pushKV("version", static_cast<int64_t>(static_cast<uint32_t>(tx.nVersion)));
        entry.pushKV("size", (int)::GetSerializeSize(tx, PROTOCOL_VERSION));
        entry.pushKV("vsize", (GetTransactionWeight(tx) + WITNESS_SCALE_FACTOR - 1) / WITNESS_SCALE_FACTOR);
        entry.pushKV("weight", GetTransactionWeight(tx));
        entry.pushKV("locktime", (int64_t)tx.nLockTime);

        UniValue vin{UniValue::VARR};

        // If available, use Undo data to calculate the fee. Note that txundo == nullptr
        // for coinbase transactions and for transactions where undo data is unavailable.
        const bool have_undo = txundo != nullptr;
        CAmount amt_total_in = 0;
        CAmount amt_total_out = 0;

        for (unsigned int i = 0; i < tx.vin.size(); i++) {
            const CTxIn& txin = tx.vin[i];
            UniValue in(UniValue::VOBJ);
            if (tx.IsCoinBase()) {
                in.pushKV("coinbase", HexStr(txin.scriptSig));
            } else {
                in.pushKV("txid", txin.prevout.hash.GetHex());
                in.pushKV("vout", (int64_t)txin.prevout.n);
                UniValue o(UniValue::VOBJ);
                o.pushKV("asm", ScriptToAsmStr(txin.scriptSig, true));
                o.pushKV("hex", HexStr(txin.scriptSig));
                in.pushKV("scriptSig", o);
            }
            if (!tx.vin[i].scriptWitness.IsNull()) {
                UniValue txinwitness(UniValue::VARR);
                for (const auto& item : tx.vin[i].scriptWitness.stack) {
                    txinwitness.push_back(HexStr(item));
                }
                in.pushKV("txinwitness", txinwitness);
            }
            if (have_undo) {
                const Coin& prev_coin = txundo->vprevout[i];
                const CTxOut& prev_txout = prev_coin.out;

                amt_total_in += prev_txout.nValue;
                switch (verbosity) {
                    case TxVerbosity::SHOW_TXID:
                    case TxVerbosity::SHOW_DETAILS:
                        break;

                    case TxVerbosity::SHOW_DETAILS_AND_PREVOUT:
                        UniValue o_script_pub_key(UniValue::VOBJ);
                        ScriptPubKeyToUniv(prev_txout.scriptPubKey, o_script_pub_key, /* includeHex */ true);

                        UniValue p(UniValue::VOBJ);
                        p.pushKV("generated", bool(prev_coin.fCoinBase));
                        p.pushKV("height", uint64_t(prev_coin.nHeight));
                        p.pushKV("value", ValueFromAmount(prev_txout.nValue));
                        p.pushKV("scriptPubKey", o_script_pub_key);
                        in.pushKV("prevout", p);
                        break;
                }
            }
            in.pushKV("sequence", (int64_t)txin.nSequence);
            vin.push_back(in);
        }
        entry.pushKV("vin", vin);

        UniValue vout(UniValue::VARR);
        for (unsigned int i = 0; i < tx.vout.size(); i++) {
            const CTxOut& txout = tx.vout[i];

            UniValue out(UniValue::VOBJ);

            out.pushKV("value", ValueFromAmount(txout.nValue));
            out.pushKV("n", (int64_t)i);

            UniValue o(UniValue::VOBJ);
            ScriptPubKeyToUniv(txout.scriptPubKey, o, true);
            out.pushKV("scriptPubKey", o);
            vout.push_back(out);

            if (have_undo) {
                amt_total_out += txout.nValue;
            }
        }
        entry.pushKV("vout", vout);

        if (have_undo) {
            const CAmount fee = amt_total_in - amt_total_out;
            CHECK_NONFATAL(MoneyRange(fee));
            entry.pushKV("fee", ValueFromAmount(fee));
        }

        if (!hashBlock.IsNull())
            entry.pushKV("blockhash", hashBlock.GetHex());

        if (include_hex) {
            entry.pushKV("hex", EncodeHexTx(tx, serialize_flags)); // The hex-encoded transaction. Used the name "hex" to be consistent with the verbose output of "getrawtransaction".
        }
        */
}

//-------------------------------------------[.cpp/bitcoin/src/util/macros.h]

macro_rules! paste {
    ($x:ident, $y:ident) => {
        /*
                x ## y
        */
    }
}

macro_rules! paste2 {
    ($x:ident, $y:ident) => {
        /*
                PASTE(x, y)
        */
    }
}

/**
  | Converts the parameter X to a string
  | after macro replacement on X has been
  | performed.
  | 
  | Don't merge these into one macro!
  |
  */
macro_rules! stringize {
    ($X:ident) => {
        /*
                DO_STRINGIZE(X)
        */
    }
}

macro_rules! do_stringize {
    ($X:ident) => {
        /*
                #X
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/util/check.h]

pub struct NonFatalCheckError { }

impl RuntimeErrorInterface for NonFatalCheckError {}

/**
  | Throw a NonFatalCheckError when the
  | condition evaluates to false
  | 
  | This should only be used
  | 
  | - where the condition is assumed to be
  | true, not for error handling or validating
  | user input
  | 
  | - where a failure to fulfill the condition
  | is recoverable and does not abort the
  | program
  | 
  | For example in RPC code, where it is undesirable
  | to crash the whole program, this can
  | be generally used to replace asserts
  | or recoverable logic errors. A NonFatalCheckError
  | in RPC code is caught and passed as a string
  | to the RPC caller, which can then report
  | the issue to the developers.
  |
  */
macro_rules! check_nonfatal {
    ($condition:ident) => {
        /*
        
            do {                                                          
                if (!(condition)) {                                       
                    throw NonFatalCheckError(                             
                        strprintf("%s:%d (%s)\n"                          
                                  "Internal bug detected: '%s'\n"         
                                  "You may report this issue here: %s\n", 
                            __FILE__, __LINE__, __func__,                 
                            (#condition),                                 
                            PACKAGE_BUGREPORT));                          
                }                                                         
            } while (false)
        */
    }
}

lazy_static!{
    /*
    #if defined(NDEBUG)
    #error "Cannot compile without assertions!"
    #endif
    */
}

/**
  | Helper for Assert()
  |
  */
pub fn get_pure_r_value<T>(val: T) -> T {

    todo!();
        /*
            return std::forward<T>(val);
        */
}
