crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/env_posix_test.cc]

/**
  | Exit codes for the helper process spawned by
  | TestCloseOnExec* tests.
  |
  | Useful for debugging test failures.
  */
#[cfg(HAVE_O_CLOEXEC)]
const TEXT_CLOSE_ON_EXEC_HELPER_EXEC_FAILED_CODE:   i32 = 61;

#[cfg(HAVE_O_CLOEXEC)]
const TEXT_CLOSE_ON_EXEC_HELPER_DUP_2FAILED_CODE:   i32 = 62;

#[cfg(HAVE_O_CLOEXEC)]
const TEXT_CLOSE_ON_EXEC_HELPER_FOUND_OPEN_FD_CODE: i32 = 63;

/**
  | Global set by main() and read in
  | TestCloseOnExec.
  |
  | The argv[0] value is stored in a std::vector
  | instead of a std::string because std::string
  | does not return a mutable pointer to its buffer
  | until C++17.
  |
  | The vector stores the string pointed to by
  | argv[0], plus the trailing null.
  */
#[cfg(HAVE_O_CLOEXEC)]
fn get_argv_zero() -> *mut Vec<u8> {
    
    todo!();
        /*
            static std::vector<char> program_name;
      return &program_name;
        */
}

/**
   Command-line switch used to run this test as
   the CloseOnExecSwitch helper.
  */
#[cfg(HAVE_O_CLOEXEC)]
const TEST_CLOSE_ON_EXEC_SWITCH: &[u8] = "--test-close-on-exec-helper";

/*
  | Executed in a separate process by
  | TestCloseOnExec* tests.
  |
  | main() delegates to this function when the test
  | executable is launched with a special
  | command-line switch. TestCloseOnExec* tests
  | fork()+exec() the test executable and pass the
  | special command-line switch.
  |
  */

/**
  | main() delegates to this function when the test
  | executable is launched with a special
  | command-line switch. TestCloseOnExec* tests
  | fork()+exec() the test executable and pass the
  | special command-line switch.
  |
  | When main() delegates to this function, the
  | process probes whether a given file descriptor
  | is open, and communicates the result via its
  | exit code.
  */
#[cfg(HAVE_O_CLOEXEC)]
fn test_close_on_exec_helper_main(pid_arg: *mut u8) -> i32 {
    
    todo!();
        /*
            int fd = std::atoi(pid_arg);
      // When given the same file descriptor twice, dup2() returns -1 if the
      // file descriptor is closed, or the given file descriptor if it is open.
      if (::dup2(fd, fd) == fd) {
        std::fprintf(stderr, "Unexpected open fd %d\n", fd);
        return kTextCloseOnExecHelperFoundOpenFdCode;
      }
      // Double-check that dup2() is saying the file descriptor is closed.
      if (errno != EBADF) {
        std::fprintf(stderr, "Unexpected errno after calling dup2 on fd %d: %s\n",
                     fd, std::strerror(errno));
        return kTextCloseOnExecHelperDup2FailedCode;
      }
      return 0;
        */
}

/**
  | File descriptors are small non-negative
  | integers.
  |
  | Returns c_void so the implementation can use
  | ASSERT_EQ.
  */
#[cfg(HAVE_O_CLOEXEC)]
fn get_max_file_descriptor(result_fd: *mut i32)  {
    
    todo!();
        /*
            // Get the maximum file descriptor number.
      ::rlimit fd_rlimit;
      ASSERT_EQ(0, ::getrlimit(RLIMIT_NOFILE, &fd_rlimit));
      *result_fd = fd_rlimit.rlim_cur;
        */
}

/**
  | Iterates through all possible FDs and returns
  | the currently open ones.
  |
  | Returns c_void so the implementation can use
  | ASSERT_EQ.
  */
#[cfg(HAVE_O_CLOEXEC)]
fn get_open_file_descriptors(open_fds: *mut HashSet<i32>)  {
    
    todo!();
        /*
            int max_fd = 0;
      GetMaxFileDescriptor(&max_fd);

      for (int fd = 0; fd < max_fd; ++fd) {
        if (::dup2(fd, fd) != fd) {
          // When given the same file descriptor twice, dup2() returns -1 if the
          // file descriptor is closed, or the given file descriptor if it is open.
          //
          // Double-check that dup2() is saying the fd is closed.
          ASSERT_EQ(EBADF, errno)
              << "dup2() should set errno to EBADF on closed file descriptors";
          continue;
        }
        open_fds->insert(fd);
      }
        */
}

/**
  | Finds an FD open since a previous call to
  | GetOpenFileDescriptors().
  |
  | |baseline_open_fds| is the result of a previous
  | GetOpenFileDescriptors() call. Assumes that
  | exactly one FD was opened since that call.
  |
  | Returns c_void so the implementation can use
  | ASSERT_EQ.
  */
#[cfg(HAVE_O_CLOEXEC)]
fn get_newly_opened_file_descriptor(
        baseline_open_fds: &HashSet<i32>,
        result_fd:         *mut i32)  {
    
    todo!();
        /*
            std::unordered_set<int> open_fds;
      GetOpenFileDescriptors(&open_fds);
      for (int fd : baseline_open_fds) {
        ASSERT_EQ(1, open_fds.count(fd))
            << "Previously opened file descriptor was closed during test setup";
        open_fds.erase(fd);
      }
      ASSERT_EQ(1, open_fds.size())
          << "Expected exactly one newly opened file descriptor during test setup";
      *result_fd = *open_fds.begin();
        */
}

/**
  | Check that a fork()+exec()-ed child
  | process does not have an extra open FD.
  |
  */
#[cfg(HAVE_O_CLOEXEC)]
fn check_close_on_exec_does_not_leak_fds(baseline_open_fds: &HashSet<i32>)  {
    
    todo!();
        /*
            // Prepare the argument list for the child process.
      // execv() wants mutable buffers.
      char switch_buffer[sizeof(kTestCloseOnExecSwitch)];
      std::memcpy(switch_buffer, kTestCloseOnExecSwitch,
                  sizeof(kTestCloseOnExecSwitch));

      int probed_fd;
      GetNewlyOpenedFileDescriptor(baseline_open_fds, &probed_fd);
      std::string fd_string = std::to_string(probed_fd);
      std::vector<char> fd_buffer(fd_string.begin(), fd_string.end());
      fd_buffer.emplace_back('\0');

      // The helper process is launched with the command below.
      //      env_posix_tests --test-close-on-exec-helper 3
      char* child_argv[] = {GetArgvZero()->data(), switch_buffer, fd_buffer.data(),
                            nullptr};

      constexpr int kForkInChildProcessReturnValue = 0;
      int child_pid = fork();
      if (child_pid == kForkInChildProcessReturnValue) {
        ::execv(child_argv[0], child_argv);
        std::fprintf(stderr, "Error spawning child process: %s\n", strerror(errno));
        std::exit(kTextCloseOnExecHelperExecFailedCode);
      }

      int child_status = 0;
      ASSERT_EQ(child_pid, ::waitpid(child_pid, &child_status, 0));
      ASSERT_TRUE(WIFEXITED(child_status))
          << "The helper process did not exit with an exit code";
      ASSERT_EQ(0, WEXITSTATUS(child_status))
          << "The helper process encountered an error";
        */
}

const READ_ONLY_FILE_LIMIT: i32 = 4;
const MMAP_LIMIT:           i32 = 4;

struct EnvPosixTest {
    env: Rc<RefCell<dyn Env>>,
}

impl Default for EnvPosixTest {
    
    fn default() -> Self {
        todo!();
        /*


            : env_(Env::Default())
        */
    }
}

impl EnvPosixTest {
    
    fn set_file_limits(
        read_only_file_limit: i32,
        mmap_limit:           i32)  {
        
        todo!();
        /*
            EnvPosixTestHelper::SetReadOnlyFDLimit(read_only_file_limit);
        EnvPosixTestHelper::SetReadOnlyMMapLimit(mmap_limit);
        */
    }
}

#[test] fn env_posix_test_open_on_read() {
    todo!();
    /*
    
      // Write some test data to a single file that will be opened |n| times.
      std::string test_dir;
      ASSERT_OK(env_->GetTestDirectory(&test_dir));
      std::string test_file = test_dir + "/open_on_read.txt";

      FILE* f = fopen(test_file.c_str(), "we");
      ASSERT_TRUE(f != nullptr);
      const char kFileData[] = "abcdefghijklmnopqrstuvwxyz";
      fputs(kFileData, f);
      fclose(f);

      // Open test file some number above the sum of the two limits to force
      // open-on-read behavior of POSIX Env leveldb::RandomAccessFile.
      const int kNumFiles = kReadOnlyFileLimit + kMMapLimit + 5;
      leveldb::RandomAccessFile* files[kNumFiles] = {0};
      for (int i = 0; i < kNumFiles; i++) {
        ASSERT_OK(env_->NewRandomAccessFile(test_file, &files[i]));
      }
      char scratch;
      Slice read_result;
      for (int i = 0; i < kNumFiles; i++) {
        ASSERT_OK(files[i]->Read(i, 1, &read_result, &scratch));
        ASSERT_EQ(kFileData[i], read_result[0]);
      }
      for (int i = 0; i < kNumFiles; i++) {
        delete files[i];
      }
      ASSERT_OK(env_->DeleteFile(test_file));

    */
}

#[cfg(HAVE_O_CLOEXEC)]
#[test] fn env_posix_test_close_on_exec_sequential_file() {
    todo!();
    /*
    
      std::unordered_set<int> open_fds;
      GetOpenFileDescriptors(&open_fds);

      std::string test_dir;
      ASSERT_OK(env_->GetTestDirectory(&test_dir));
      std::string file_path = test_dir + "/close_on_exec_sequential.txt";
      ASSERT_OK(WriteStringToFile(env_, "0123456789", file_path));

      leveldb::SequentialFile* file = nullptr;
      ASSERT_OK(env_->NewSequentialFile(file_path, &file));
      CheckCloseOnExecDoesNotLeakFDs(open_fds);
      delete file;

      ASSERT_OK(env_->DeleteFile(file_path));

    */
}

#[cfg(HAVE_O_CLOEXEC)]
#[test] fn env_posix_test_close_on_exec_random_access_file() {
    todo!();
    /*
    
      std::unordered_set<int> open_fds;
      GetOpenFileDescriptors(&open_fds);

      std::string test_dir;
      ASSERT_OK(env_->GetTestDirectory(&test_dir));
      std::string file_path = test_dir + "/close_on_exec_random_access.txt";
      ASSERT_OK(WriteStringToFile(env_, "0123456789", file_path));

      // Exhaust the RandomAccessFile mmap limit. This way, the test
      // RandomAccessFile instance below is backed by a file descriptor, not by an
      // mmap region.
      leveldb::RandomAccessFile* mmapped_files[kReadOnlyFileLimit] = {nullptr};
      for (int i = 0; i < kReadOnlyFileLimit; i++) {
        ASSERT_OK(env_->NewRandomAccessFile(file_path, &mmapped_files[i]));
      }

      leveldb::RandomAccessFile* file = nullptr;
      ASSERT_OK(env_->NewRandomAccessFile(file_path, &file));
      CheckCloseOnExecDoesNotLeakFDs(open_fds);
      delete file;

      for (int i = 0; i < kReadOnlyFileLimit; i++) {
        delete mmapped_files[i];
      }
      ASSERT_OK(env_->DeleteFile(file_path));

    */
}

#[cfg(HAVE_O_CLOEXEC)]
#[test] fn env_posix_test_close_on_exec_writable_file() {
    todo!();
    /*
    
      std::unordered_set<int> open_fds;
      GetOpenFileDescriptors(&open_fds);

      std::string test_dir;
      ASSERT_OK(env_->GetTestDirectory(&test_dir));
      std::string file_path = test_dir + "/close_on_exec_writable.txt";
      ASSERT_OK(WriteStringToFile(env_, "0123456789", file_path));

      leveldb::WritableFile* file = nullptr;
      ASSERT_OK(env_->NewWritableFile(file_path, &file));
      CheckCloseOnExecDoesNotLeakFDs(open_fds);
      delete file;

      ASSERT_OK(env_->DeleteFile(file_path));

    */
}

#[cfg(HAVE_O_CLOEXEC)]
#[test] fn env_posix_test_close_on_exec_appendable_file() {
    todo!();
    /*
    
      std::unordered_set<int> open_fds;
      GetOpenFileDescriptors(&open_fds);

      std::string test_dir;
      ASSERT_OK(env_->GetTestDirectory(&test_dir));
      std::string file_path = test_dir + "/close_on_exec_appendable.txt";
      ASSERT_OK(WriteStringToFile(env_, "0123456789", file_path));

      leveldb::WritableFile* file = nullptr;
      ASSERT_OK(env_->NewAppendableFile(file_path, &file));
      CheckCloseOnExecDoesNotLeakFDs(open_fds);
      delete file;

      ASSERT_OK(env_->DeleteFile(file_path));

    */
}

#[cfg(HAVE_O_CLOEXEC)]
#[test] fn env_posix_test_close_on_exec_lock_file() {
    todo!();
    /*
    
      std::unordered_set<int> open_fds;
      GetOpenFileDescriptors(&open_fds);

      std::string test_dir;
      ASSERT_OK(env_->GetTestDirectory(&test_dir));
      std::string file_path = test_dir + "/close_on_exec_lock.txt";
      ASSERT_OK(WriteStringToFile(env_, "0123456789", file_path));

      leveldb::FileLock* lock = nullptr;
      ASSERT_OK(env_->LockFile(file_path, &lock));
      CheckCloseOnExecDoesNotLeakFDs(open_fds);
      ASSERT_OK(env_->UnlockFile(lock));

      ASSERT_OK(env_->DeleteFile(file_path));

    */
}

#[cfg(HAVE_O_CLOEXEC)]
#[test] fn env_posix_test_close_on_exec_logger() {
    todo!();
    /*
    
      std::unordered_set<int> open_fds;
      GetOpenFileDescriptors(&open_fds);

      std::string test_dir;
      ASSERT_OK(env_->GetTestDirectory(&test_dir));
      std::string file_path = test_dir + "/close_on_exec_logger.txt";
      ASSERT_OK(WriteStringToFile(env_, "0123456789", file_path));

      leveldb::Logger* file = nullptr;
      ASSERT_OK(env_->NewLogger(file_path, &file));
      CheckCloseOnExecDoesNotLeakFDs(open_fds);
      delete file;

      ASSERT_OK(env_->DeleteFile(file_path));

    */
}

fn testenv_posix_test_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            #if HAVE_O_CLOEXEC
      // Check if we're invoked as a helper program, or as the test suite.
      for (int i = 1; i < argc; ++i) {
        if (!std::strcmp(argv[i], kTestCloseOnExecSwitch)) {
          return TestCloseOnExecHelperMain(argv[i + 1]);
        }
      }

      // Save argv[0] early, because googletest may modify argv.
      GetArgvZero()->assign(argv[0], argv[0] + std::strlen(argv[0]) + 1);
    #endif  // HAVE_O_CLOEXEC

      // All tests currently run with the same read-only file limits.
      leveldb::EnvPosixTest::SetFileLimits(leveldb::kReadOnlyFileLimit,
                                           leveldb::kMMapLimit);
      return leveldb::test::RunAllTests();
        */
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/env_posix_test_helper.h]

/**
   A helper for the POSIX Env to facilitate
   testing.
  */
struct EnvPosixTestHelper {

}

impl EnvPosixTestHelper {

    /**
      | Set the maximum number of read-only files
      | that will be opened.
      |
      | Must be called before creating an Env.
      */
    pub fn set_read_only_fd_limit(&mut self, limit: i32)  {
        
        todo!();
        /*
            PosixDefaultEnv::AssertEnvNotInitialized();
      g_open_read_only_file_limit = limit;
        */
    }
    
    /**
      | Set the maximum number of read-only files
      | that will be mapped via mmap.
      |
      | Must be called before creating an Env.
      */
    pub fn set_read_only_mmap_limit(&mut self, limit: i32)  {
        
        todo!();
        /*
            PosixDefaultEnv::AssertEnvNotInitialized();
      g_mmap_limit = limit;
        */
    }
}
