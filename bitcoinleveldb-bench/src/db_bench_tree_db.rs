// ---------------- [ File: bitcoinleveldb-bench/src/db_bench_tree_db.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/leveldb/benchmarks/db_bench_tree_db.cc]

/**
  | Comma-separated list of operations to run in
  | the specified order
  |
  |   Actual benchmarks:
  |
  |   fillseq       -- write N values in sequential key order in async mode
  |   fillrandom    -- write N values in random key order in async mode
  |   overwrite     -- overwrite N values in random key order in async mode
  |   fillseqsync   -- write N/100 values in sequential key order in sync mode
  |   fillrandsync  -- write N/100 values in random key order in sync mode
  |   fillrand100K  -- write N/1000 100K values in random order in async mode
  |   fillseq100K   -- write N/1000 100K values in seq order in async mode
  |   readseq       -- read N times sequentially
  |   readseq100K   -- read N/1000 100K values in sequential order in async mode
  |   readrand100K  -- read N/1000 100K values in sequential order in async mode
  |   readrandom    -- read N times in random order
  */
pub const FLAGS_benchmarks: &'static str = concat!{
    "fillseq,",
    "fillseqsync,",
    "fillrandsync,",
    "fillrandom,",
    "overwrite,",
    "readrandom,",
    "readseq,",
    "fillrand100K,",
    "fillseq100K,",
    "readseq100K,",
    "readrand100K,"
};

/**
   Number of key/values to place in database
  */
lazy_static!{
    /*
    static int FLAGS_num = 1000000;
    */
}

/**
   Number of read operations to do.  If negative,
   do FLAGS_num reads.
  */
lazy_static!{
    /*
    static int FLAGS_reads = -1;
    */
}

/**
   Size of each value
  */
lazy_static!{
    /*
    static int FLAGS_value_size = 100;
    */
}

/**
  | Arrange to generate values that shrink to this
  | fraction of their original size after
  | compression
  */
lazy_static!{
    /*
    static double FLAGS_compression_ratio = 0.5;
    */
}

/**
   Print histogram of operation timings
  */
lazy_static!{
    /*
    static bool FLAGS_histogram = false;
    */
}

/**
   Cache size. Default 4 MB
  */
lazy_static!{
    /*
    static int FLAGS_cache_size = 4194304;
    */
}

/**
   Page size. Default 1 KB
  */
lazy_static!{
    /*
    static int FLAGS_page_size = 1024;
    */
}

/**
  | If true, do not destroy the existing database.
  | If you set this flag and also specify
  | a benchmark that wants a fresh database, that
  | benchmark will fail.
  */
lazy_static!{
    /*
    static bool FLAGS_use_existing_db = false;
    */
}

/**
   Compression flag. If true, compression is
   on. If false, compression is off.
  */
lazy_static!{
    /*
    static bool FLAGS_compression = true;
    */
}

/**
   Use the db with the following name.
  */
lazy_static!{
    /*
    static const char* FLAGS_db = nullptr;
    */
}

#[cfg(todo_kyotocabinet)]
#[inline] pub fn db_synchronize(db: *mut kyotocabinet::TreeDB)  {
    
    todo!();
        /*
            // Synchronize will flush writes to disk
      if (!db_->synchronize()) {
        fprintf(stderr, "synchronize error: %s\n", db_->error().name());
      }
        */
}

/**
  | Helper for quickly generating random
  | data.
  |
  */
pub struct RandomGenerator {
    data: String,
    pos:  i32,
}

impl Default for RandomGenerator {
    
    fn default() -> Self {
        todo!();
        /*


            // We use a limited amount of data over and over again and ensure
        // that it is larger than the compression window (32KB), and also
        // large enough to serve all typical value sizes we want to write.
        Random rnd(301);
        std::string piece;
        while (data_.size() < 1048576) {
          // Add a short fragment that is as compressible as specified
          // by FLAGS_compression_ratio.
          test::CompressibleString(&rnd, FLAGS_compression_ratio, 100, &piece);
          data_.append(piece);
        }
        pos_ = 0;
        */
    }
}

impl RandomGenerator {

    pub fn generate(&mut self, len: i32) -> Slice {
        
        todo!();
        /*
            if (pos_ + len > data_.size()) {
          pos_ = 0;
          assert(len < data_.size());
        }
        pos_ += len;
        return Slice(data_.data() + pos_ - len, len);
        */
    }
}

pub fn trim_space(s: Slice) -> Slice {
    
    todo!();
        /*
            int start = 0;
      while (start < s.size() && isspace(s[start])) {
        start++;
      }
      int limit = s.size();
      while (limit > start && isspace(s[limit - 1])) {
        limit--;
      }
      return Slice(s.data() + start, limit - start);
        */
}

///---------------------
pub struct Benchmark {
    #[cfg(todo_kyotocabinet)]
    db:             *mut kyotocabinet::TreeDB,
    db_num:         i32,
    num:            i32,
    reads:          i32,
    start:          f64,
    last_op_finish: f64,
    bytes:          i64,
    message:        String,
    hist:           Histogram,
    gen:            RandomGenerator,
    rand:           Random,

    #[cfg(todo_kyotocabinet)]
    comp:           kyotocabinet::LZOCompressor<kyotocabinet::LZO::RAW>,

    /**
       State kept for progress messages
      */
    done:           i32,

    /**
       When to report next
      */
    next_report:    i32,
}

pub mod benchmark {

    pub enum Order { 
        SEQUENTIAL, 
        RANDOM 
    }

    pub enum DBState { 
        FRESH, 
        EXISTING 
    }
}

impl Default for Benchmark {
    
    fn default() -> Self {
        todo!();
        /*


            : db_(nullptr),
            num_(FLAGS_num),
            reads_(FLAGS_reads < 0 ? FLAGS_num : FLAGS_reads),
            bytes_(0),
            rand_(301) 

        std::vector<std::string> files;
        std::string test_dir;
        Env::Default()->GetTestDirectory(&test_dir);
        Env::Default()->GetChildren(test_dir.c_str(), &files);
        if (!FLAGS_use_existing_db) {
          for (int i = 0; i < files.size(); i++) {
            if (Slice(files[i]).starts_with("dbbench_polyDB")) {
              std::string file_name(test_dir);
              file_name += "/";
              file_name += files[i];
              Env::Default()->DeleteFile(file_name.c_str());
            }
          }
        }
        */
    }
}

impl Drop for Benchmark {
    fn drop(&mut self) {
        todo!();
        /*
            if (!db_->close()) {
          fprintf(stderr, "close error: %s\n", db_->error().name());
        }
        */
    }
}

impl Benchmark {

    pub fn print_header(&mut self)  {
        
        todo!();
        /*
            const int kKeySize = 16;
        PrintEnvironment();
        fprintf(stdout, "Keys:       %d bytes each\n", kKeySize);
        fprintf(stdout, "Values:     %d bytes each (%d bytes after compression)\n",
                FLAGS_value_size,
                static_cast<int>(FLAGS_value_size * FLAGS_compression_ratio + 0.5));
        fprintf(stdout, "Entries:    %d\n", num_);
        fprintf(stdout, "RawSize:    %.1f MB (estimated)\n",
                ((static_cast<int64_t>(kKeySize + FLAGS_value_size) * num_) /
                 1048576.0));
        fprintf(stdout, "FileSize:   %.1f MB (estimated)\n",
                (((kKeySize + FLAGS_value_size * FLAGS_compression_ratio) * num_) /
                 1048576.0));
        PrintWarnings();
        fprintf(stdout, "------------------------------------------------\n");
        */
    }
    
    pub fn print_warnings(&mut self)  {
        
        todo!();
        /*
            #if defined(__GNUC__) && !defined(__OPTIMIZE__)
        fprintf(
            stdout,
            "WARNING: Optimization is disabled: benchmarks unnecessarily slow\n");
    #endif
    #ifndef NDEBUG
        fprintf(stdout,
                "WARNING: Assertions are enabled; benchmarks unnecessarily slow\n");
    #endif
        */
    }
    
    pub fn print_environment(&mut self)  {
        
        todo!();
        /*
            fprintf(stderr, "Kyoto Cabinet:    version %s, lib ver %d, lib rev %d\n",
                kyotocabinet::VERSION, kyotocabinet::LIBVER, kyotocabinet::LIBREV);

    #if defined(__linux)
        time_t now = time(nullptr);
        fprintf(stderr, "Date:           %s", ctime(&now));  // ctime() adds newline

        FILE* cpuinfo = fopen("/proc/cpuinfo", "r");
        if (cpuinfo != nullptr) {
          char line[1000];
          int num_cpus = 0;
          std::string cpu_type;
          std::string cache_size;
          while (fgets(line, sizeof(line), cpuinfo) != nullptr) {
            const char* sep = strchr(line, ':');
            if (sep == nullptr) {
              continue;
            }
            Slice key = TrimSpace(Slice(line, sep - 1 - line));
            Slice val = TrimSpace(Slice(sep + 1));
            if (key == "model name") {
              ++num_cpus;
              cpu_type = val.ToString();
            } else if (key == "cache size") {
              cache_size = val.ToString();
            }
          }
          fclose(cpuinfo);
          fprintf(stderr, "CPU:            %d * %s\n", num_cpus, cpu_type.c_str());
          fprintf(stderr, "CPUCache:       %s\n", cache_size.c_str());
        }
    #endif
        */
    }
    
    pub fn start(&mut self)  {
        
        todo!();
        /*
            start_ = Env::Default()->NowMicros() * 1e-6;
        bytes_ = 0;
        message_.clear();
        last_op_finish_ = start_;
        hist_.Clear();
        done_ = 0;
        next_report_ = 100;
        */
    }
    
    pub fn finished_single_op(&mut self)  {
        
        todo!();
        /*
            if (FLAGS_histogram) {
          double now = Env::Default()->NowMicros() * 1e-6;
          double micros = (now - last_op_finish_) * 1e6;
          hist_.Add(micros);
          if (micros > 20000) {
            fprintf(stderr, "long op: %.1f micros%30s\r", micros, "");
            fflush(stderr);
          }
          last_op_finish_ = now;
        }

        done_++;
        if (done_ >= next_report_) {
          if (next_report_ < 1000)
            next_report_ += 100;
          else if (next_report_ < 5000)
            next_report_ += 500;
          else if (next_report_ < 10000)
            next_report_ += 1000;
          else if (next_report_ < 50000)
            next_report_ += 5000;
          else if (next_report_ < 100000)
            next_report_ += 10000;
          else if (next_report_ < 500000)
            next_report_ += 50000;
          else
            next_report_ += 100000;
          fprintf(stderr, "... finished %d ops%30s\r", done_, "");
          fflush(stderr);
        }
        */
    }
    
    pub fn stop(&mut self, name: &Slice)  {
        
        todo!();
        /*
            double finish = Env::Default()->NowMicros() * 1e-6;

        // Pretend at least one op was done in case we are running a benchmark
        // that does not call FinishedSingleOp().
        if (done_ < 1) done_ = 1;

        if (bytes_ > 0) {
          char rate[100];
          snprintf(rate, sizeof(rate), "%6.1f MB/s",
                   (bytes_ / 1048576.0) / (finish - start_));
          if (!message_.empty()) {
            message_ = std::string(rate) + " " + message_;
          } else {
            message_ = rate;
          }
        }

        fprintf(stdout, "%-12s : %11.3f micros/op;%s%s\n", name.ToString().c_str(),
                (finish - start_) * 1e6 / done_, (message_.empty() ? "" : " "),
                message_.c_str());
        if (FLAGS_histogram) {
          fprintf(stdout, "Microseconds per op:\n%s\n", hist_.ToString().c_str());
        }
        fflush(stdout);
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            PrintHeader();
        Open(false);

        const char* benchmarks = FLAGS_benchmarks;
        while (benchmarks != nullptr) {
          const char* sep = strchr(benchmarks, ',');
          Slice name;
          if (sep == nullptr) {
            name = benchmarks;
            benchmarks = nullptr;
          } else {
            name = Slice(benchmarks, sep - benchmarks);
            benchmarks = sep + 1;
          }

          Start();

          bool known = true;
          bool write_sync = false;
          if (name == Slice("fillseq")) {
            Write(write_sync, SEQUENTIAL, FRESH, num_, FLAGS_value_size, 1);
            DBSynchronize(db_);
          } else if (name == Slice("fillrandom")) {
            Write(write_sync, RANDOM, FRESH, num_, FLAGS_value_size, 1);
            DBSynchronize(db_);
          } else if (name == Slice("overwrite")) {
            Write(write_sync, RANDOM, EXISTING, num_, FLAGS_value_size, 1);
            DBSynchronize(db_);
          } else if (name == Slice("fillrandsync")) {
            write_sync = true;
            Write(write_sync, RANDOM, FRESH, num_ / 100, FLAGS_value_size, 1);
            DBSynchronize(db_);
          } else if (name == Slice("fillseqsync")) {
            write_sync = true;
            Write(write_sync, SEQUENTIAL, FRESH, num_ / 100, FLAGS_value_size, 1);
            DBSynchronize(db_);
          } else if (name == Slice("fillrand100K")) {
            Write(write_sync, RANDOM, FRESH, num_ / 1000, 100 * 1000, 1);
            DBSynchronize(db_);
          } else if (name == Slice("fillseq100K")) {
            Write(write_sync, SEQUENTIAL, FRESH, num_ / 1000, 100 * 1000, 1);
            DBSynchronize(db_);
          } else if (name == Slice("readseq")) {
            ReadSequential();
          } else if (name == Slice("readrandom")) {
            ReadRandom();
          } else if (name == Slice("readrand100K")) {
            int n = reads_;
            reads_ /= 1000;
            ReadRandom();
            reads_ = n;
          } else if (name == Slice("readseq100K")) {
            int n = reads_;
            reads_ /= 1000;
            ReadSequential();
            reads_ = n;
          } else {
            known = false;
            if (name != Slice()) {  // No error message for empty name
              fprintf(stderr, "unknown benchmark '%s'\n", name.ToString().c_str());
            }
          }
          if (known) {
            Stop(name);
          }
        }
        */
    }
    
    pub fn open(&mut self, sync: bool)  {
        
        todo!();
        /*
            assert(db_ == nullptr);

        // Initialize db_
        db_ = new kyotocabinet::TreeDB();
        char file_name[100];
        db_num_++;
        std::string test_dir;
        Env::Default()->GetTestDirectory(&test_dir);
        snprintf(file_name, sizeof(file_name), "%s/dbbench_polyDB-%d.kct",
                 test_dir.c_str(), db_num_);

        // Create tuning options and open the database
        int open_options =
            kyotocabinet::PolyDB::OWRITER | kyotocabinet::PolyDB::OCREATE;
        int tune_options =
            kyotocabinet::TreeDB::TSMALL | kyotocabinet::TreeDB::TLINEAR;
        if (FLAGS_compression) {
          tune_options |= kyotocabinet::TreeDB::TCOMPRESS;
          db_->tune_compressor(&comp_);
        }
        db_->tune_options(tune_options);
        db_->tune_page_cache(FLAGS_cache_size);
        db_->tune_page(FLAGS_page_size);
        db_->tune_map(256LL << 20);
        if (sync) {
          open_options |= kyotocabinet::PolyDB::OAUTOSYNC;
        }
        if (!db_->open(file_name, open_options)) {
          fprintf(stderr, "open error: %s\n", db_->error().name());
        }
        */
    }
    
    pub fn write(&mut self, 
        sync:              bool,
        order:             benchmark::Order,
        state:             benchmark::DBState,
        num_entries:       i32,
        value_size:        i32,
        entries_per_batch: i32)  {
        
        todo!();
        /*
            // Create new database if state == FRESH
        if (state == FRESH) {
          if (FLAGS_use_existing_db) {
            message_ = "skipping (--use_existing_db is true)";
            return;
          }
          delete db_;
          db_ = nullptr;
          Open(sync);
          Start();  // Do not count time taken to destroy/open
        }

        if (num_entries != num_) {
          char msg[100];
          snprintf(msg, sizeof(msg), "(%d ops)", num_entries);
          message_ = msg;
        }

        // Write to database
        for (int i = 0; i < num_entries; i++) {
          const int k = (order == SEQUENTIAL) ? i : (rand_.Next() % num_entries);
          char key[100];
          snprintf(key, sizeof(key), "%016d", k);
          bytes_ += value_size + strlen(key);
          std::string cpp_key = key;
          if (!db_->set(cpp_key, gen_.Generate(value_size).ToString())) {
            fprintf(stderr, "set error: %s\n", db_->error().name());
          }
          FinishedSingleOp();
        }
        */
    }
    
    pub fn read_sequential(&mut self)  {
        
        todo!();
        /*
            kyotocabinet::DB::Cursor* cur = db_->cursor();
        cur->jump();
        std::string ckey, cvalue;
        while (cur->get(&ckey, &cvalue, true)) {
          bytes_ += ckey.size() + cvalue.size();
          FinishedSingleOp();
        }
        delete cur;
        */
    }
    
    pub fn read_random(&mut self)  {
        
        todo!();
        /*
            std::string value;
        for (int i = 0; i < reads_; i++) {
          char key[100];
          const int k = rand_.Next() % reads_;
          snprintf(key, sizeof(key), "%016d", k);
          db_->get(key, &value);
          FinishedSingleOp();
        }
        */
    }
}

pub fn benchdb_bench_tree_db_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            std::string default_db_path;
      for (int i = 1; i < argc; i++) {
        double d;
        int n;
        char junk;
        if (leveldb::Slice(argv[i]).starts_with("--benchmarks=")) {
          FLAGS_benchmarks = argv[i] + strlen("--benchmarks=");
        } else if (sscanf(argv[i], "--compression_ratio=%lf%c", &d, &junk) == 1) {
          FLAGS_compression_ratio = d;
        } else if (sscanf(argv[i], "--histogram=%d%c", &n, &junk) == 1 &&
                   (n == 0 || n == 1)) {
          FLAGS_histogram = n;
        } else if (sscanf(argv[i], "--num=%d%c", &n, &junk) == 1) {
          FLAGS_num = n;
        } else if (sscanf(argv[i], "--reads=%d%c", &n, &junk) == 1) {
          FLAGS_reads = n;
        } else if (sscanf(argv[i], "--value_size=%d%c", &n, &junk) == 1) {
          FLAGS_value_size = n;
        } else if (sscanf(argv[i], "--cache_size=%d%c", &n, &junk) == 1) {
          FLAGS_cache_size = n;
        } else if (sscanf(argv[i], "--page_size=%d%c", &n, &junk) == 1) {
          FLAGS_page_size = n;
        } else if (sscanf(argv[i], "--compression=%d%c", &n, &junk) == 1 &&
                   (n == 0 || n == 1)) {
          FLAGS_compression = (n == 1) ? true : false;
        } else if (strncmp(argv[i], "--db=", 5) == 0) {
          FLAGS_db = argv[i] + 5;
        } else {
          fprintf(stderr, "Invalid flag '%s'\n", argv[i]);
          exit(1);
        }
      }

      // Choose a location for the test database if none given with --db=<path>
      if (FLAGS_db == nullptr) {
        leveldb::Env::Default()->GetTestDirectory(&default_db_path);
        default_db_path += "/dbbench";
        FLAGS_db = default_db_path.c_str();
      }

      leveldb::Benchmark benchmark;
      benchmark.Run();
      return 0;
        */
}
