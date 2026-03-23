// ---------------- [ File: bitcoinleveldbt-bench/src/db_bench.rs ]
crate::ix!();

use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::ptr;
//-------------------------------------------[.cpp/bitcoin/src/leveldb/benchmarks/db_bench.cc]

/**
  | Comma-separated list of operations to run in
  | the specified order
  |
  |   Actual benchmarks:
  |      fillseq       -- write N values in sequential key order in async mode
  |      fillrandom    -- write N values in random key order in async mode
  |      overwrite     -- overwrite N values in random key order in async mode
  |      fillsync      -- write N/100 values in random key order in sync mode
  |      fill100K      -- write N/1000 100K values in random order in async mode
  |      deleteseq     -- delete N keys in sequential order
  |      deleterandom  -- delete N keys in random order
  |      readseq       -- read N times sequentially
  |      readreverse   -- read N times in reverse order
  |      readrandom    -- read N times in random order
  |      readmissing   -- read N missing keys in random order
  |      readhot       -- read N times in random order from 1% section of DB
  |      seekrandom    -- N random seeks
  |      open          -- cost of opening a DB
  |      crc32c        -- repeated crc32c of 4K of data
  |   Meta operations:
  |      compact     -- Compact the entire DB
  |      stats       -- Print DB stats
  |      sstables    -- Print sstable info
  |      heapprofile -- Dump a heap profile (if supported by this port)
  */
pub const FLAGS_benchmarks: &'static str = concat!{
    "fillseq,",
    "fillsync,",
    "fillrandom,",
    "overwrite,",
    "readrandom,",
    "readrandom,",  // Extra run to allow previous compactions to quiesce
    "readseq,",
    "readreverse,",
    "compact,",
    "readrandom,",
    "readseq,",
    "readreverse,",
    "fill100K,",
    "crc32c,",
    "snappycomp,",
    "snappyuncomp,"
};

/// Number of key/values to place in database
pub static BITCOINLEVELDB_BENCH_FLAGS_NUM: AtomicI32 = AtomicI32::new(1_000_000);

/// Number of read operations to do. If negative, do FLAGS_num reads.
pub static BITCOINLEVELDB_BENCH_FLAGS_READS: AtomicI32 = AtomicI32::new(-1);

/// Number of concurrent threads to run.
pub static BITCOINLEVELDB_BENCH_FLAGS_THREADS: AtomicI32 = AtomicI32::new(1);

/// Size of each value
pub static BITCOINLEVELDB_BENCH_FLAGS_VALUE_SIZE: AtomicI32 = AtomicI32::new(100);

/// Print histogram of operation timings
pub static BITCOINLEVELDB_BENCH_FLAGS_HISTOGRAM: AtomicBool = AtomicBool::new(false);

/// Number of bytes to buffer in memtable before compacting (initialized to default value by "main")
pub static BITCOINLEVELDB_BENCH_FLAGS_WRITE_BUFFER_SIZE: AtomicI32 = AtomicI32::new(0);

/// Number of bytes written to each file. (initialized to default value by "main")
pub static BITCOINLEVELDB_BENCH_FLAGS_MAX_FILE_SIZE: AtomicI32 = AtomicI32::new(0);

/// Approximate size of user data packed per block (before compression.  (initialized to default value by "main")
pub static BITCOINLEVELDB_BENCH_FLAGS_BLOCK_SIZE: AtomicI32 = AtomicI32::new(0);

/// Number of bytes to use as a cache of uncompressed data.  Negative means use default settings.
pub static BITCOINLEVELDB_BENCH_FLAGS_CACHE_SIZE: AtomicI32 = AtomicI32::new(-1);

/// Maximum number of files to keep open at the same time (use default if == 0)
pub static BITCOINLEVELDB_BENCH_FLAGS_OPEN_FILES: AtomicI32 = AtomicI32::new(0);

/// Bloom filter bits per key. Negative means use default settings.
pub static BITCOINLEVELDB_BENCH_FLAGS_BLOOM_BITS: AtomicI32 = AtomicI32::new(-1);

/// If true, do not destroy the existing database.
/// If you set this flag and also specify a benchmark that wants a fresh database, that benchmark will fail.
pub static BITCOINLEVELDB_BENCH_FLAGS_USE_EXISTING_DB: AtomicBool = AtomicBool::new(false);

/// If true, reuse existing log/MANIFEST files when re-opening a database.
pub static BITCOINLEVELDB_BENCH_FLAGS_REUSE_LOGS: AtomicBool = AtomicBool::new(false);

lazy_static!{
    pub static ref BITCOINLEVELDB_BENCH_FLAGS_BENCHMARKS_TEXT: Mutex<String> =
        Mutex::new(String::from(FLAGS_benchmarks));
}

lazy_static!{
    /// Arrange to generate values that shrink to this fraction of their original size after compression
    pub static ref BITCOINLEVELDB_BENCH_FLAGS_COMPRESSION_RATIO: Mutex<f64> =
        Mutex::new(0.5_f64);
}

lazy_static!{
    /// Use the db with the following name.
    pub static ref BITCOINLEVELDB_BENCH_FLAGS_DB_PATH: Mutex<Option<String>> =
        Mutex::new(None);
}

pub fn bitcoinleveldb_bench_flag_benchmarks_get() -> String {
    BITCOINLEVELDB_BENCH_FLAGS_BENCHMARKS_TEXT.lock().clone()
}

pub fn bitcoinleveldb_bench_flag_benchmarks_set(value: String) {
    *BITCOINLEVELDB_BENCH_FLAGS_BENCHMARKS_TEXT.lock() = value;
}

pub fn bitcoinleveldb_bench_flag_compression_ratio_get() -> f64 {
    *BITCOINLEVELDB_BENCH_FLAGS_COMPRESSION_RATIO.lock()
}

pub fn bitcoinleveldb_bench_flag_compression_ratio_set(value: f64) {
    *BITCOINLEVELDB_BENCH_FLAGS_COMPRESSION_RATIO.lock() = value;
}

pub fn bitcoinleveldb_bench_flag_db_path_get() -> Option<String> {
    BITCOINLEVELDB_BENCH_FLAGS_DB_PATH.lock().clone()
}

pub fn bitcoinleveldb_bench_flag_db_path_set(value: Option<String>) {
    *BITCOINLEVELDB_BENCH_FLAGS_DB_PATH.lock() = value;
}

pub fn bitcoinleveldb_bench_parse_i32_flag(
    argument: &str,
    prefix:   &str) -> Option<i32> {

    match argument.strip_prefix(prefix) {
        Some(rest) => match rest.parse::<i32>() {
            Ok(value) => Some(value),
            Err(_) => None,
        },
        None => None,
    }
}

pub fn bitcoinleveldb_bench_parse_f64_flag(
    argument: &str,
    prefix:   &str) -> Option<f64> {

    match argument.strip_prefix(prefix) {
        Some(rest) => match rest.parse::<f64>() {
            Ok(value) => Some(value),
            Err(_) => None,
        },
        None => None,
    }
}

pub fn bitcoinleveldb_bench_parse_bool01_flag(
    argument: &str,
    prefix:   &str) -> Option<bool> {

    match bitcoinleveldb_bench_parse_i32_flag(argument, prefix) {
        Some(0) => Some(false),
        Some(1) => Some(true),
        Some(_) => None,
        None => None,
    }
}

pub fn bitcoinleveldb_bench_now_seconds() -> f64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs_f64(),
        Err(_) => 0.0,
    }
}

pub fn bitcoinleveldb_bench_get_test_directory() -> Option<String> {
    unsafe {
        let env = leveldb_create_default_env();
        if env.is_null() {
            return None;
        }

        let dir_ptr = leveldb_env_get_test_directory(env);
        let result = if dir_ptr.is_null() {
            None
        } else {
            Some(Slice::from(dir_ptr as *const u8).to_string())
        };

        if !dir_ptr.is_null() {
            leveldb_free(dir_ptr as *mut c_void);
        }
        leveldb_env_destroy(env);
        result
    }
}

pub fn bitcoinleveldb_bench_cstring_or_exit(input: &str) -> CString {
    match CString::new(input) {
        Ok(value) => value,
        Err(_) => {
            eprintln!("leveldb string contains interior NUL");
            unsafe {
                exit(1);
            }
        }
    }
}

pub fn bitcoinleveldb_bench_leveldb_error_to_string(errptr: *mut u8) -> String {
    if errptr.is_null() {
        String::new()
    } else {
        Slice::from(errptr as *const u8).to_string()
    }
}

pub fn bitcoinleveldb_bench_leveldb_error_clear(errptr: *mut u8) {
    unsafe {
        if !errptr.is_null() {
            leveldb_free(errptr as *mut c_void);
        }
    }
}

pub fn bitcoinleveldb_bench_leveldb_error_check_or_exit(
    context: &str,
    errptr:  *mut u8) {

    if !errptr.is_null() {
        let rendered_message = bitcoinleveldb_bench_leveldb_error_to_string(errptr);
        eprintln!("{} error: {}", context, rendered_message);
        bitcoinleveldb_bench_leveldb_error_clear(errptr);
        unsafe {
            exit(1);
        }
    }
}

pub fn bitcoinleveldb_bench_destroy_db_ignore_error(dbname: &str) {
    let options = LevelDBOptions::default();
    let c_dbname = bitcoinleveldb_bench_cstring_or_exit(dbname);
    let mut errptr: *mut u8 = ptr::null_mut();

    unsafe {
        leveldb_destroy_db(
            &options as *const LevelDBOptions,
            c_dbname.as_ptr() as *const u8,
            &mut errptr,
        );
    }

    bitcoinleveldb_bench_leveldb_error_clear(errptr);
}

pub fn bitcoinleveldb_bench_make_write_options_from_rep(
    rep: &WriteOptions) -> LevelDBWriteOptions {

    let mut result = LevelDBWriteOptions::default();
    *result.rep_mut() = rep.clone();
    result
}

pub fn bitcoinleveldb_bench_crc32c_value(data: &[u8]) -> u32 {
    let mut crc: u32 = !0u32;

    for byte in data {
        crc ^= u32::from(*byte);
        let mut bit_index = 0usize;
        while bit_index < 8 {
            let mask = 0u32.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (0x82F63B78u32 & mask);
            bit_index += 1;
        }
    }

    !crc
}

pub fn bitcoinleveldb_bench_dispatch_open_bench(
    bm:     *mut Benchmark,
    thread: *mut ThreadState) {

    unsafe {
        if let Some(benchmark_ref) = bm.as_mut() {
            benchmark_ref.open_bench(thread);
        }
    }
}

pub fn bitcoinleveldb_bench_dispatch_write_seq(
    bm:     *mut Benchmark,
    thread: *mut ThreadState) {

    unsafe {
        if let Some(benchmark_ref) = bm.as_mut() {
            benchmark_ref.write_seq(thread);
        }
    }
}

pub fn bitcoinleveldb_bench_dispatch_write_random(
    bm:     *mut Benchmark,
    thread: *mut ThreadState) {

    unsafe {
        if let Some(benchmark_ref) = bm.as_mut() {
            benchmark_ref.write_random(thread);
        }
    }
}

pub fn bitcoinleveldb_bench_dispatch_read_sequential(
    bm:     *mut Benchmark,
    thread: *mut ThreadState) {

    unsafe {
        if let Some(benchmark_ref) = bm.as_mut() {
            benchmark_ref.read_sequential(thread);
        }
    }
}

pub fn bitcoinleveldb_bench_dispatch_read_reverse(
    bm:     *mut Benchmark,
    thread: *mut ThreadState) {

    unsafe {
        if let Some(benchmark_ref) = bm.as_mut() {
            benchmark_ref.read_reverse(thread);
        }
    }
}

pub fn bitcoinleveldb_bench_dispatch_read_random(
    bm:     *mut Benchmark,
    thread: *mut ThreadState) {

    unsafe {
        if let Some(benchmark_ref) = bm.as_mut() {
            benchmark_ref.read_random(thread);
        }
    }
}

pub fn bitcoinleveldb_bench_dispatch_read_missing(
    bm:     *mut Benchmark,
    thread: *mut ThreadState) {

    unsafe {
        if let Some(benchmark_ref) = bm.as_mut() {
            benchmark_ref.read_missing(thread);
        }
    }
}

pub fn bitcoinleveldb_bench_dispatch_seek_random(
    bm:     *mut Benchmark,
    thread: *mut ThreadState) {

    unsafe {
        if let Some(benchmark_ref) = bm.as_mut() {
            benchmark_ref.seek_random(thread);
        }
    }
}

pub fn bitcoinleveldb_bench_dispatch_read_hot(
    bm:     *mut Benchmark,
    thread: *mut ThreadState) {

    unsafe {
        if let Some(benchmark_ref) = bm.as_mut() {
            benchmark_ref.read_hot(thread);
        }
    }
}

pub fn bitcoinleveldb_bench_dispatch_delete_seq(
    bm:     *mut Benchmark,
    thread: *mut ThreadState) {

    unsafe {
        if let Some(benchmark_ref) = bm.as_mut() {
            benchmark_ref.delete_seq(thread);
        }
    }
}

pub fn bitcoinleveldb_bench_dispatch_delete_random(
    bm:     *mut Benchmark,
    thread: *mut ThreadState) {

    unsafe {
        if let Some(benchmark_ref) = bm.as_mut() {
            benchmark_ref.delete_random(thread);
        }
    }
}

pub fn bitcoinleveldb_bench_dispatch_read_while_writing(
    bm:     *mut Benchmark,
    thread: *mut ThreadState) {

    unsafe {
        if let Some(benchmark_ref) = bm.as_mut() {
            benchmark_ref.read_while_writing(thread);
        }
    }
}

pub fn bitcoinleveldb_bench_dispatch_compact(
    bm:     *mut Benchmark,
    thread: *mut ThreadState) {

    unsafe {
        if let Some(benchmark_ref) = bm.as_mut() {
            benchmark_ref.compact(thread);
        }
    }
}

pub fn bitcoinleveldb_bench_dispatch_crc32c(
    bm:     *mut Benchmark,
    thread: *mut ThreadState) {

    unsafe {
        if let Some(benchmark_ref) = bm.as_mut() {
            benchmark_ref.crc_32c(thread);
        }
    }
}

pub fn bitcoinleveldb_bench_dispatch_snappy_compress(
    bm:     *mut Benchmark,
    thread: *mut ThreadState) {

    unsafe {
        if let Some(benchmark_ref) = bm.as_mut() {
            benchmark_ref.snappy_compress(thread);
        }
    }
}

pub fn bitcoinleveldb_bench_dispatch_snappy_uncompress(
    bm:     *mut Benchmark,
    thread: *mut ThreadState) {

    unsafe {
        if let Some(benchmark_ref) = bm.as_mut() {
            benchmark_ref.snappy_uncompress(thread);
        }
    }
}


/// Helper for quickly generating random data.
pub struct RandomGenerator {
    data: String,
    pos:  i32,
}

impl Default for RandomGenerator {
    
    fn default() -> Self {
        // We use a limited amount of data over and over again and ensure
        // that it is larger than the compression window (32KB), and also
        // large enough to serve all typical value sizes we want to write.
        let mut rnd = Random::new(301);
        let compression_ratio = bitcoinleveldb_bench_flag_compression_ratio_get();

        let raw_fragment_len_f64 = (compression_ratio * 100.0).round();
        let raw_fragment_len = if raw_fragment_len_f64 < 1.0 {
            1usize
        } else if raw_fragment_len_f64 > 100.0 {
            100usize
        } else {
            raw_fragment_len_f64 as usize
        };

        let mut data = String::with_capacity(1_048_576);

        while data.len() < 1_048_576 {
            // Add a short fragment that is as compressible as specified
            // by FLAGS_compression_ratio.
            let mut raw_fragment = String::with_capacity(raw_fragment_len);
            let mut raw_index = 0usize;

            while raw_index < raw_fragment_len {
                let byte = b'a' + (rnd.uniform(26) as u8);
                raw_fragment.push(char::from(byte));
                raw_index += 1;
            }

            let mut piece = String::with_capacity(100);
            while piece.len() < 100 {
                let remaining = 100usize - piece.len();
                if remaining >= raw_fragment.len() {
                    piece.push_str(&raw_fragment);
                } else {
                    piece.push_str(&raw_fragment[..remaining]);
                }
            }

            data.push_str(&piece);
        }

        Self {
            data,
            pos: 0,
        }
    }
}

impl RandomGenerator {

    pub fn generate(&mut self, len: usize) -> Slice {
        let pos = if self.pos < 0 {
            0usize
        } else {
            self.pos as usize
        };

        if pos + len > self.data.len() {
            self.pos = 0;
            assert!(len < self.data.len());
        }

        assert!(len <= i32::MAX as usize);

        let start = if self.pos < 0 {
            0usize
        } else {
            self.pos as usize
        };

        self.pos += len as i32;

        unsafe {
            Slice::from_ptr_len(self.data.as_ptr().add(start), len)
        }
    }
}

#[cfg(target_os = "linux")]
pub fn trim_space(s: Slice) -> Slice {
    let bytes = s.as_bytes();

    let mut start = 0usize;
    while start < bytes.len() && bytes[start].is_ascii_whitespace() {
        start += 1;
    }

    let mut limit = bytes.len();
    while limit > start && bytes[limit - 1].is_ascii_whitespace() {
        limit -= 1;
    }

    unsafe {
        Slice::from_ptr_len(bytes.as_ptr().add(start), limit - start)
    }
}

pub fn append_with_space(
    str_: *mut String,
    msg:  Slice)  {

    if msg.empty() {
        return;
    }

    assert!(!str_.is_null());

    unsafe {
        if let Some(target) = str_.as_mut() {
            if !target.is_empty() {
                target.push(' ');
            }
            target.push_str(&msg.to_string());
        }
    }
}

///--------------------------
pub struct Stats {
    start:          f64,
    finish:         f64,
    seconds:        f64,
    done:           i32,
    next_report:    i32,
    bytes:          i64,
    last_op_finish: f64,
    hist:           Histogram,
    message:        String,
}

impl Default for Stats {
    
    fn default() -> Self {
        let mut result = Self {
            start: 0.0,
            finish: 0.0,
            seconds: 0.0,
            done: 0,
            next_report: 100,
            bytes: 0,
            last_op_finish: 0.0,
            hist: Histogram::default(),
            message: String::new(),
        };

        result.start();
        result
    }
}

impl Stats {

    pub fn start(&mut self)  {
        self.next_report = 100;
        self.hist.clear();
        self.done = 0;
        self.bytes = 0;
        self.seconds = 0.0;
        self.message.clear();
        self.start = bitcoinleveldb_bench_now_seconds();
        self.finish = self.start;
        self.last_op_finish = self.start;
    }
   
    pub fn merge(&mut self, other: &Stats)  {
        self.hist.merge(&other.hist);
        self.done += other.done;
        self.bytes += other.bytes;
        self.seconds += other.seconds;

        if other.start < self.start {
            self.start = other.start;
        }

        if other.finish > self.finish {
            self.finish = other.finish;
        }

        // Just keep the messages from one thread
        if self.message.is_empty() {
            self.message = other.message.clone();
        }
    }
    
    pub fn stop(&mut self)  {
        self.finish = bitcoinleveldb_bench_now_seconds();
        self.seconds = self.finish - self.start;
    }

    pub fn add_message(&mut self, msg: Slice)  {
        append_with_space(&mut self.message as *mut String, msg);
    }

    pub fn finished_single_op(&mut self)  {
        if BITCOINLEVELDB_BENCH_FLAGS_HISTOGRAM.load(Relaxed) {
            let now = bitcoinleveldb_bench_now_seconds();
            let micros = (now - self.last_op_finish) * 1e6;
            self.hist.add(micros);
            if micros > 20000.0 {
                eprint!("long op: {:.1} micros{:30}\r", micros, "");
            }
            self.last_op_finish = now;
        }

        self.done += 1;
        if self.done >= self.next_report {
            if self.next_report < 1000 {
                self.next_report += 100;
            } else if self.next_report < 5000 {
                self.next_report += 500;
            } else if self.next_report < 10000 {
                self.next_report += 1000;
            } else if self.next_report < 50000 {
                self.next_report += 5000;
            } else if self.next_report < 100000 {
                self.next_report += 10000;
            } else if self.next_report < 500000 {
                self.next_report += 50000;
            } else {
                self.next_report += 100000;
            }
            eprint!("... finished {} ops{:30}\r", self.done, "");
        }
    }
    
    pub fn add_bytes(&mut self, n: i64)  {
        self.bytes += n;
    }

    pub fn report(&mut self, name: &Slice)  {
        // Pretend at least one op was done in case we are running a benchmark
        // that does not call FinishedSingleOp().
        if self.done < 1 {
            self.done = 1;
        }

        let mut extra = String::new();
        if self.bytes > 0 {
            // Rate is computed on actual elapsed time, not the sum of per-thread
            // elapsed times.
            let elapsed = self.finish - self.start;
            let rate = format!("{:6.1} MB/s", (self.bytes as f64 / 1048576.0) / elapsed);
            extra = rate;
        }
        append_with_space(&mut extra as *mut String, Slice::from(&self.message));

        println!(
            "{:<12} : {:11.3} micros/op;{}{}",
            name.to_string(),
            self.seconds * 1e6 / self.done as f64,
            if extra.is_empty() { "" } else { " " },
            extra
        );
        if BITCOINLEVELDB_BENCH_FLAGS_HISTOGRAM.load(Relaxed) {
            println!("Microseconds per op:\n{}\n", self.hist.to_string());
        }
    }
}

/**
   State shared by all concurrent executions of
   the same benchmark.
  */
pub struct SharedState {
    mu: Mutex<shared_state::Inner>,
    cv: Condvar,
}

pub mod shared_state {
    use super::*;

    pub struct Inner {
        pub total: i32,

        /*
          | Each thread goes through the following states:
          |
          |    (1) initializing
          |
          |    (2) waiting for others to be initialized
          |
          |    (3) running
          |
          |    (4) done
          */

        pub num_initialized: i32,
        pub num_done:        i32,
        pub start:           bool,
    }
}

impl SharedState {

    pub fn new(total: i32) -> Self {

        Self {
            mu: Mutex::new(shared_state::Inner {
                total,
                num_initialized: 0,
                num_done: 0,
                start: false,
            }),
            cv: Condvar::new(),
        }
    }
}

/**
  | Per-thread state for concurrent executions
  | of the same benchmark.
  |
  */
pub struct ThreadState {

    /**
       0..n-1 when running in n threads
      */
    tid:    i32,

    /**
       Has different seeds for different threads
      */
    rand:   Random,
    stats:  Stats,
    shared: *mut SharedState,
}

impl ThreadState {

    pub fn new(index: i32) -> Self {

        Self {
            tid: index,
            rand: Random::new((1000 + index) as u32),
            stats: Stats::default(),
            shared: 0 as *mut SharedState,
        }
    }
}

///-------------------------
pub struct Benchmark {
    cache:             *mut LevelDBCache,
    filter_policy:     *mut LevelDBFilterPolicy,
    db:                *mut LevelDB,
    num:               i32,
    value_size:        i32,
    entries_per_batch: i32,
    write_options:     WriteOptions,
    reads:             i32,
    heap_counter:      i32,
}

pub mod benchmark {
    use super::*;

    pub struct ThreadArg {
        pub bm:     *mut Benchmark,
        pub shared: *mut SharedState,
        pub thread: *mut ThreadState,
        pub method: fn(_0: *mut Benchmark, _1: *mut ThreadState),
    }
}

impl Default for Benchmark {
    
    fn default() -> Self {
        let cache_size = BITCOINLEVELDB_BENCH_FLAGS_CACHE_SIZE.load(Relaxed);
        let bloom_bits = BITCOINLEVELDB_BENCH_FLAGS_BLOOM_BITS.load(Relaxed);
        let num = BITCOINLEVELDB_BENCH_FLAGS_NUM.load(Relaxed);
        let reads_flag = BITCOINLEVELDB_BENCH_FLAGS_READS.load(Relaxed);

        let cache = if cache_size >= 0 {
            leveldb_cache_create_lru(cache_size as usize)
        } else {
            ptr::null_mut()
        };

        let filter_policy = if bloom_bits >= 0 {
            leveldb_filterpolicy_create_bloom(bloom_bits)
        } else {
            ptr::null_mut()
        };

        if let Some(db_path) = bitcoinleveldb_bench_flag_db_path_get() {
            match fs::read_dir(&db_path) {
                Ok(entries) => {
                    for entry_result in entries {
                        match entry_result {
                            Ok(entry) => {
                                let file_name_os = entry.file_name();
                                match file_name_os.to_str() {
                                    Some(file_name) => {
                                        if file_name.starts_with("heap-") {
                                            let _ = fs::remove_file(entry.path());
                                        }
                                    }
                                    None => {}
                                }
                            }
                            Err(_) => {}
                        }
                    }
                }
                Err(_) => {}
            }

            if !BITCOINLEVELDB_BENCH_FLAGS_USE_EXISTING_DB.load(Relaxed) {
                bitcoinleveldb_bench_destroy_db_ignore_error(&db_path);
            }
        }

        Self {
            cache,
            filter_policy,
            db: ptr::null_mut(),
            num,
            value_size: BITCOINLEVELDB_BENCH_FLAGS_VALUE_SIZE.load(Relaxed),
            entries_per_batch: 1,
            write_options: WriteOptions::default(),
            reads: if reads_flag < 0 { num } else { reads_flag },
            heap_counter: 0,
        }
    }
}

impl Drop for Benchmark {
    fn drop(&mut self) {
        if !self.db.is_null() {
            leveldb_close(self.db);
            self.db = ptr::null_mut();
        }

        if !self.cache.is_null() {
            leveldb_cache_destroy(self.cache);
            self.cache = ptr::null_mut();
        }

        if !self.filter_policy.is_null() {
            leveldb_filterpolicy_destroy(self.filter_policy);
            self.filter_policy = ptr::null_mut();
        }
    }
}

impl Benchmark {

    pub fn print_header(&mut self)  {
        const K_KEY_SIZE: i32 = 16;
        let value_size = BITCOINLEVELDB_BENCH_FLAGS_VALUE_SIZE.load(Relaxed);
        let compression_ratio = bitcoinleveldb_bench_flag_compression_ratio_get();

        self.print_environment();
        println!("Keys:       {} bytes each", K_KEY_SIZE);
        println!(
            "Values:     {} bytes each ({} bytes after compression)",
            value_size,
            ((value_size as f64) * compression_ratio + 0.5) as i32
        );
        println!("Entries:    {}", self.num);
        println!(
            "RawSize:    {:.1} MB (estimated)",
            (((i64::from(K_KEY_SIZE) + i64::from(value_size)) * i64::from(self.num)) as f64)
                / 1048576.0
        );
        println!(
            "FileSize:   {:.1} MB (estimated)",
            (((f64::from(K_KEY_SIZE) + (f64::from(value_size) * compression_ratio))
                * f64::from(self.num))
                / 1048576.0)
        );
        self.print_warnings();
        println!("------------------------------------------------");
    }

    /// Preserves benchmark identity by emitting only environment warnings that
    /// remain valid across refactors: debug assertions and snappy availability.
    pub fn print_warnings(&mut self)  {

        if cfg!(debug_assertions) {
            println!("WARNING: Assertions are enabled; benchmarks unnecessarily slow");
        }

        // See if snappy is working by attempting to compress a compressible string
        let text = b"yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy";
        let mut compressed = String::new();

        if !bitcoinleveldb_compat::snappy_compress(
            text.as_ptr(),
            text.len(),
            &mut compressed as *mut String,
        ) {
            println!("WARNING: Snappy compression is not enabled");
        } else if compressed.len() >= text.len() {
            println!("WARNING: Snappy compression is not effective");
        }
    }
    
    pub fn print_environment(&mut self)  {
        eprintln!(
            "LevelDB:    version {}.{}",
            leveldb_major_version(),
            leveldb_minor_version()
        );

        #[cfg(target_os = "linux")]
        {
            match fs::read_to_string("/proc/cpuinfo") {
                Ok(cpuinfo) => {
                    let mut num_cpus = 0_i32;
                    let mut cpu_type = String::new();
                    let mut cache_size = String::new();

                    for line in cpuinfo.lines() {
                        match line.split_once(':') {
                            Some((key_text, value_text)) => {
                                let key = trim_space(Slice::from_str(key_text)).to_string();
                                let value = trim_space(Slice::from_str(value_text)).to_string();

                                if key == "model name" {
                                    num_cpus += 1;
                                    cpu_type = value;
                                } else if key == "cache size" {
                                    cache_size = value;
                                }
                            }
                            None => {}
                        }
                    }

                    if num_cpus > 0 {
                        eprintln!("CPU:        {} * {}", num_cpus, cpu_type);
                    }
                    if !cache_size.is_empty() {
                        eprintln!("CPUCache:   {}", cache_size);
                    }
                }
                Err(_) => {}
            }
        }
    }

    pub fn run(&mut self)  {
        self.print_header();
        self.open();

        let benchmarks_text = bitcoinleveldb_bench_flag_benchmarks_get();
        for name in benchmarks_text.split(',') {
            // Reset parameters that may be overridden below
            self.num = BITCOINLEVELDB_BENCH_FLAGS_NUM.load(Relaxed);
            self.reads = {
                let reads_flag = BITCOINLEVELDB_BENCH_FLAGS_READS.load(Relaxed);
                if reads_flag < 0 {
                    BITCOINLEVELDB_BENCH_FLAGS_NUM.load(Relaxed)
                } else {
                    reads_flag
                }
            };
            self.value_size = BITCOINLEVELDB_BENCH_FLAGS_VALUE_SIZE.load(Relaxed);
            self.entries_per_batch = 1;
            self.write_options = WriteOptions::default();

            let mut method: Option<fn(*mut Benchmark, *mut ThreadState)> = None;
            let mut fresh_db = false;
            let mut num_threads = BITCOINLEVELDB_BENCH_FLAGS_THREADS.load(Relaxed);

            if name == "open" {
                method = Some(bitcoinleveldb_bench_dispatch_open_bench);
                self.num /= 10000;
                if self.num < 1 {
                    self.num = 1;
                }
            } else if name == "fillseq" {
                fresh_db = true;
                method = Some(bitcoinleveldb_bench_dispatch_write_seq);
            } else if name == "fillbatch" {
                fresh_db = true;
                self.entries_per_batch = 1000;
                method = Some(bitcoinleveldb_bench_dispatch_write_seq);
            } else if name == "fillrandom" {
                fresh_db = true;
                method = Some(bitcoinleveldb_bench_dispatch_write_random);
            } else if name == "overwrite" {
                fresh_db = false;
                method = Some(bitcoinleveldb_bench_dispatch_write_random);
            } else if name == "fillsync" {
                fresh_db = true;
                self.num /= 1000;
                self.write_options.set_sync(true);
                method = Some(bitcoinleveldb_bench_dispatch_write_random);
            } else if name == "fill100K" {
                fresh_db = true;
                self.num /= 1000;
                self.value_size = 100 * 1000;
                method = Some(bitcoinleveldb_bench_dispatch_write_random);
            } else if name == "readseq" {
                method = Some(bitcoinleveldb_bench_dispatch_read_sequential);
            } else if name == "readreverse" {
                method = Some(bitcoinleveldb_bench_dispatch_read_reverse);
            } else if name == "readrandom" {
                method = Some(bitcoinleveldb_bench_dispatch_read_random);
            } else if name == "readmissing" {
                method = Some(bitcoinleveldb_bench_dispatch_read_missing);
            } else if name == "seekrandom" {
                method = Some(bitcoinleveldb_bench_dispatch_seek_random);
            } else if name == "readhot" {
                method = Some(bitcoinleveldb_bench_dispatch_read_hot);
            } else if name == "readrandomsmall" {
                self.reads /= 1000;
                method = Some(bitcoinleveldb_bench_dispatch_read_random);
            } else if name == "deleteseq" {
                method = Some(bitcoinleveldb_bench_dispatch_delete_seq);
            } else if name == "deleterandom" {
                method = Some(bitcoinleveldb_bench_dispatch_delete_random);
            } else if name == "readwhilewriting" {
                num_threads += 1;  // Add extra thread for writing
                method = Some(bitcoinleveldb_bench_dispatch_read_while_writing);
            } else if name == "compact" {
                method = Some(bitcoinleveldb_bench_dispatch_compact);
            } else if name == "crc32c" {
                method = Some(bitcoinleveldb_bench_dispatch_crc32c);
            } else if name == "snappycomp" {
                method = Some(bitcoinleveldb_bench_dispatch_snappy_compress);
            } else if name == "snappyuncomp" {
                method = Some(bitcoinleveldb_bench_dispatch_snappy_uncompress);
            } else if name == "heapprofile" {
                self.heap_profile();
            } else if name == "stats" {
                self.print_stats(b"leveldb.stats\0".as_ptr());
            } else if name == "sstables" {
                self.print_stats(b"leveldb.sstables\0".as_ptr());
            } else {
                if !name.is_empty() {
                    // No error message for empty name
                    eprintln!("unknown benchmark '{}'", name);
                }
            }

            if fresh_db {
                if BITCOINLEVELDB_BENCH_FLAGS_USE_EXISTING_DB.load(Relaxed) {
                    println!("{:<12} : skipped (--use_existing_db is true)", name);
                    method = None;
                } else {
                    if !self.db.is_null() {
                        leveldb_close(self.db);
                        self.db = ptr::null_mut();
                    }

                    if let Some(db_path) = bitcoinleveldb_bench_flag_db_path_get() {
                        bitcoinleveldb_bench_destroy_db_ignore_error(&db_path);
                    }

                    self.open();
                }
            }

            if let Some(method_fn) = method {
                let name_slice = Slice::from_str(name);
                self.run_benchmark(num_threads, name_slice, method_fn);
            }
        }
    }
    
    pub fn thread_body(v: *mut c_void)  {
        unsafe {
            let arg = &mut *(v as *mut benchmark::ThreadArg);
            let shared = &mut *arg.shared;
            let thread = &mut *arg.thread;

            {
                let mut guard = shared.mu.lock();
                guard.num_initialized += 1;

                if guard.num_initialized >= guard.total {
                    shared.cv.notify_all();
                }

                while !guard.start {
                    shared.cv.wait(&mut guard);
                }
            }

            thread.stats.start();
            (arg.method)(arg.bm, arg.thread);
            thread.stats.stop();

            {
                let mut guard = shared.mu.lock();
                guard.num_done += 1;

                if guard.num_done >= guard.total {
                    shared.cv.notify_all();
                }
            }
        }
    }

    /// Preserves the original start-barrier and completion-barrier semantics:
    /// every worker blocks until all workers are initialized, and the reported
    /// stats are the merged result of all threads for the named benchmark.
    pub fn run_benchmark(
        &mut self,
        n:      i32,
        name:   Slice,
        method: fn(_0: *mut Benchmark, _1: *mut ThreadState),
    )  {

        if n <= 0 {
            eprintln!("invalid thread count '{}'", n);
            unsafe {
                exit(1);
            }
        }

        let shared = SharedState::new(n);
        let bm_ptr = self as *mut Benchmark;
        let shared_ptr = (&shared as *const SharedState) as *mut SharedState;

        let mut thread_states: Vec<Box<ThreadState>> = Vec::with_capacity(n as usize);
        let mut i = 0_i32;
        while i < n {
            let mut thread_state = Box::new(ThreadState::new(i));
            thread_state.shared = shared_ptr;
            thread_states.push(thread_state);
            i += 1;
        }

        let mut args: Vec<benchmark::ThreadArg> = Vec::with_capacity(n as usize);
        let mut j = 0_i32;
        while j < n {
            let thread_ptr = thread_states[j as usize].as_mut() as *mut ThreadState;
            args.push(benchmark::ThreadArg {
                bm: bm_ptr,
                shared: shared_ptr,
                thread: thread_ptr,
                method,
            });
            j += 1;
        }

        std::thread::scope(|scope| {
            let mut handles = Vec::with_capacity(n as usize);

            let mut k = 0_i32;
            while k < n {
                let arg_addr = (&mut args[k as usize] as *mut benchmark::ThreadArg) as usize;
                handles.push(scope.spawn(move || {
                    Benchmark::thread_body(arg_addr as *mut c_void);
                }));
                k += 1;
            }

            {
                let mut guard = shared.mu.lock();
                while guard.num_initialized < n {
                    shared.cv.wait(&mut guard);
                }

                guard.start = true;
                shared.cv.notify_all();

                while guard.num_done < n {
                    shared.cv.wait(&mut guard);
                }
            }

            while let Some(handle) = handles.pop() {
                match handle.join() {
                    Ok(()) => {}
                    Err(_) => {
                        eprintln!("benchmark thread terminated unexpectedly");
                        unsafe {
                            exit(1);
                        }
                    }
                }
            }
        });

        let (first_slice, rest) = thread_states.split_at_mut(1);
        let first_thread = &mut first_slice[0];

        let mut index = 0usize;
        while index < rest.len() {
            first_thread.stats.merge(&rest[index].stats);
            index += 1;
        }

        first_thread.stats.report(&name);
    }

    pub fn crc_32c(&mut self, thread: *mut ThreadState)  {
        
        unsafe {
            let thread_ref = &mut *thread;

            // Checksum about 500MB of data total
            const SIZE: usize = 4096;
            let label = Slice::from_str("(4K per op)");
            let data = vec![b'x'; SIZE];
            let mut bytes: i64 = 0;
            let mut crc: u32 = 0;

            while bytes < 500 * 1048576 {
                crc = bitcoinleveldb_bench_crc32c_value(&data);
                thread_ref.stats.finished_single_op();
                bytes += SIZE as i64;
            }

            // Print so result is not dead
            eprint!("... crc=0x{:x}\r", crc);

            thread_ref.stats.add_bytes(bytes);
            thread_ref.stats.add_message(label);
        }
    }
    
    pub fn snappy_compress(&mut self, thread: *mut ThreadState)  {

        unsafe {
            let thread_ref = &mut *thread;
            let mut gen = RandomGenerator::default();
            let block_size = *Options::default().block_size();
            let input = gen.generate(block_size);
            let input_bytes = input.as_bytes();

            let mut bytes: i64 = 0;
            let mut produced: i64 = 0;
            let mut ok = true;
            let mut compressed = String::new();

            while ok && bytes < 1024 * 1048576 {
                ok = bitcoinleveldb_compat::snappy_compress(
                    input_bytes.as_ptr(),
                    input_bytes.len(),
                    &mut compressed as *mut String,
                );
                produced += compressed.len() as i64;
                bytes += input_bytes.len() as i64;
                thread_ref.stats.finished_single_op();
            }

            if !ok {
                thread_ref
                    .stats
                    .add_message(Slice::from_str("(snappy failure)"));
            } else {
                let msg = format!(
                    "(output: {:.1}%)",
                    (produced as f64 * 100.0) / bytes as f64
                );
                thread_ref.stats.add_message(Slice::from(&msg));
                thread_ref.stats.add_bytes(bytes);
            }
        }
    }

    pub fn snappy_uncompress(&mut self, thread: *mut ThreadState)  {

        unsafe {
            let thread_ref = &mut *thread;
            let mut gen = RandomGenerator::default();
            let block_size = *Options::default().block_size();
            let input = gen.generate(block_size);
            let input_bytes = input.as_bytes();

            let mut compressed = String::new();
            let mut ok = bitcoinleveldb_compat::snappy_compress(
                input_bytes.as_ptr(),
                input_bytes.len(),
                &mut compressed as *mut String,
            );

            let mut bytes: i64 = 0;
            let mut uncompressed = vec![0_u8; input_bytes.len()];

            while ok && bytes < 1024 * 1048576 {
                ok = bitcoinleveldb_compat::snappy_uncompress(
                    compressed.as_ptr(),
                    compressed.len(),
                    uncompressed.as_mut_ptr(),
                );
                bytes += input_bytes.len() as i64;
                thread_ref.stats.finished_single_op();
            }

            if !ok {
                thread_ref
                    .stats
                    .add_message(Slice::from_str("(snappy failure)"));
            } else {
                thread_ref.stats.add_bytes(bytes);
            }
        }
    }
    
    pub fn open(&mut self)  {
        
        assert!(self.db.is_null());

        let db_path = match bitcoinleveldb_bench_flag_db_path_get() {
            Some(path) => path,
            None => {
                eprintln!("open error: missing db path");
                unsafe {
                    exit(1);
                }
            }
        };

        let c_db_path = bitcoinleveldb_bench_cstring_or_exit(&db_path);
        let mut options = LevelDBOptions::default();

        unsafe {
            leveldb_options_set_create_if_missing(
                &mut options as *mut LevelDBOptions,
                if BITCOINLEVELDB_BENCH_FLAGS_USE_EXISTING_DB.load(Relaxed) {
                    0
                } else {
                    1
                },
            );

            if !self.cache.is_null() {
                leveldb_options_set_cache(&mut options as *mut LevelDBOptions, self.cache);
            }

            if !self.filter_policy.is_null() {
                leveldb_options_set_filter_policy(
                    &mut options as *mut LevelDBOptions,
                    self.filter_policy,
                );
            }

            leveldb_options_set_write_buffer_size(
                &mut options as *mut LevelDBOptions,
                BITCOINLEVELDB_BENCH_FLAGS_WRITE_BUFFER_SIZE.load(Relaxed).max(0) as usize,
            );
            leveldb_options_set_max_file_size(
                &mut options as *mut LevelDBOptions,
                BITCOINLEVELDB_BENCH_FLAGS_MAX_FILE_SIZE.load(Relaxed).max(0) as usize,
            );
            leveldb_options_set_block_size(
                &mut options as *mut LevelDBOptions,
                BITCOINLEVELDB_BENCH_FLAGS_BLOCK_SIZE.load(Relaxed).max(0) as usize,
            );
            leveldb_options_set_max_open_files(
                &mut options as *mut LevelDBOptions,
                BITCOINLEVELDB_BENCH_FLAGS_OPEN_FILES.load(Relaxed),
            );
        }

        options
            .rep_mut()
            .set_reuse_logs(BITCOINLEVELDB_BENCH_FLAGS_REUSE_LOGS.load(Relaxed));

        let mut errptr: *mut u8 = ptr::null_mut();

        unsafe {
            self.db = leveldb_open(
                &options as *const LevelDBOptions,
                c_db_path.as_ptr() as *const u8,
                &mut errptr,
            );
        }

        bitcoinleveldb_bench_leveldb_error_check_or_exit("open", errptr);

        if self.db.is_null() {
            eprintln!("open error: null db");
            unsafe {
                exit(1);
            }
        }
    }
    
    pub fn open_bench(&mut self, thread: *mut ThreadState)  {

        unsafe {
            let thread_ref = &mut *thread;

            let mut i = 0;
            while i < self.num {
                if !self.db.is_null() {
                    leveldb_close(self.db);
                    self.db = ptr::null_mut();
                }
                self.open();
                thread_ref.stats.finished_single_op();
                i += 1;
            }
        }
    }
    
    pub fn write_seq(&mut self, thread: *mut ThreadState)  {
        self.do_write(thread, true);
    }
    
    pub fn write_random(&mut self, thread: *mut ThreadState)  {
        self.do_write(thread, false);
    }

    /// Preserves write ordering and batch boundaries exactly: keys are generated
    /// in the same order as the source benchmark and each batch is committed by
    /// a single LevelDB write call.
    pub fn do_write(&mut self, 
        thread: *mut ThreadState,
        seq:    bool)  {

        unsafe {
            let thread_ref = &mut *thread;

            if self.num != BITCOINLEVELDB_BENCH_FLAGS_NUM.load(Relaxed) {
                let msg = format!("({} ops)", self.num);
                thread_ref.stats.add_message(Slice::from(&msg));
            }

            let mut gen = RandomGenerator::default();
            let write_options =
                bitcoinleveldb_bench_make_write_options_from_rep(&self.write_options);
            let batch = leveldb_writebatch_create();

            let mut bytes: i64 = 0;
            let mut i = 0_i32;

            while i < self.num {
                leveldb_writebatch_clear(batch);

                let mut j = 0_i32;
                while j < self.entries_per_batch {
                    let k = if seq {
                        i + j
                    } else {
                        (thread_ref.rand.next()
                            % (BITCOINLEVELDB_BENCH_FLAGS_NUM.load(Relaxed) as u32)) as i32
                    };

                    let key = format!("{:016}", k);
                    let value = gen.generate(self.value_size as usize);
                    let value_bytes = value.as_bytes();

                    leveldb_writebatch_put(
                        batch,
                        key.as_ptr(),
                        key.len(),
                        value_bytes.as_ptr(),
                        value_bytes.len(),
                    );

                    bytes += i64::from(self.value_size) + key.len() as i64;
                    thread_ref.stats.finished_single_op();
                    j += 1;
                }

                let mut errptr: *mut u8 = ptr::null_mut();
                leveldb_write(
                    self.db,
                    &write_options as *const LevelDBWriteOptions,
                    batch,
                    &mut errptr,
                );
                bitcoinleveldb_bench_leveldb_error_check_or_exit("put", errptr);

                i += self.entries_per_batch;
            }

            leveldb_writebatch_destroy(batch);
            thread_ref.stats.add_bytes(bytes);
        }
    }

    pub fn read_sequential(&mut self, thread: *mut ThreadState)  {

        unsafe {
            let thread_ref = &mut *thread;
            let options = LevelDBReadOptions::default();
            let iter = leveldb_create_iterator(
                self.db,
                &options as *const LevelDBReadOptions,
            );

            let mut i = 0;
            let mut bytes: i64 = 0;
            leveldb_iter_seek_to_first(iter);

            while i < self.reads && leveldb_iter_valid(iter) != 0 {
                let mut klen: usize = 0;
                let mut vlen: usize = 0;

                let _kptr = leveldb_iter_key(iter, &mut klen as *mut usize);
                let _vptr = leveldb_iter_value(iter, &mut vlen as *mut usize);

                bytes += klen as i64 + vlen as i64;
                thread_ref.stats.finished_single_op();
                i += 1;
                leveldb_iter_next(iter);
            }

            let mut errptr: *mut u8 = ptr::null_mut();
            leveldb_iter_get_error(iter, &mut errptr);
            bitcoinleveldb_bench_leveldb_error_check_or_exit("iterator", errptr);

            leveldb_iter_destroy(iter);
            thread_ref.stats.add_bytes(bytes);
        }
    }

    pub fn read_reverse(&mut self, thread: *mut ThreadState)  {

        unsafe {
            let thread_ref = &mut *thread;
            let options = LevelDBReadOptions::default();
            let iter = leveldb_create_iterator(
                self.db,
                &options as *const LevelDBReadOptions,
            );

            let mut i = 0;
            let mut bytes: i64 = 0;
            leveldb_iter_seek_to_last(iter);

            while i < self.reads && leveldb_iter_valid(iter) != 0 {
                let mut klen: usize = 0;
                let mut vlen: usize = 0;

                let _kptr = leveldb_iter_key(iter, &mut klen as *mut usize);
                let _vptr = leveldb_iter_value(iter, &mut vlen as *mut usize);

                bytes += klen as i64 + vlen as i64;
                thread_ref.stats.finished_single_op();
                i += 1;
                leveldb_iter_prev(iter);
            }

            let mut errptr: *mut u8 = ptr::null_mut();
            leveldb_iter_get_error(iter, &mut errptr);
            bitcoinleveldb_bench_leveldb_error_check_or_exit("iterator", errptr);

            leveldb_iter_destroy(iter);
            thread_ref.stats.add_bytes(bytes);
        }
    }   

    pub fn read_random(&mut self, thread: *mut ThreadState)  {

        unsafe {
            let thread_ref = &mut *thread;
            let options = LevelDBReadOptions::default();
            let mut found = 0;

            let mut i = 0;
            while i < self.reads {
                let k = (thread_ref.rand.next() % (BITCOINLEVELDB_BENCH_FLAGS_NUM.load(Relaxed) as u32)) as i32;
                let key = format!("{:016}", k);

                let mut vallen: usize = 0;
                let mut errptr: *mut u8 = ptr::null_mut();
                let value_ptr = leveldb_get(
                    self.db,
                    &options as *const LevelDBReadOptions,
                    key.as_ptr(),
                    key.len(),
                    &mut vallen as *mut usize,
                    &mut errptr,
                );
                bitcoinleveldb_bench_leveldb_error_check_or_exit("get", errptr);

                if !value_ptr.is_null() {
                    found += 1;
                    leveldb_free(value_ptr as *mut c_void);
                }

                thread_ref.stats.finished_single_op();
                i += 1;
            }

            let msg = format!("({} of {} found)", found, self.num);
            thread_ref.stats.add_message(Slice::from(&msg));
        }
    }

    pub fn read_missing(&mut self, thread: *mut ThreadState)  {

        unsafe {
            let thread_ref = &mut *thread;
            let options = LevelDBReadOptions::default();

            let mut i = 0;
            while i < self.reads {
                let k = (thread_ref.rand.next() % (BITCOINLEVELDB_BENCH_FLAGS_NUM.load(Relaxed) as u32)) as i32;
                let key = format!("{:016}.", k);

                let mut vallen: usize = 0;
                let mut errptr: *mut u8 = ptr::null_mut();
                let value_ptr = leveldb_get(
                    self.db,
                    &options as *const LevelDBReadOptions,
                    key.as_ptr(),
                    key.len(),
                    &mut vallen as *mut usize,
                    &mut errptr,
                );
                bitcoinleveldb_bench_leveldb_error_check_or_exit("get", errptr);

                if !value_ptr.is_null() {
                    leveldb_free(value_ptr as *mut c_void);
                }

                thread_ref.stats.finished_single_op();
                i += 1;
            }
        }
    }

    pub fn read_hot(&mut self, thread: *mut ThreadState)  {

        unsafe {
            let thread_ref = &mut *thread;
            let options = LevelDBReadOptions::default();
            let range = (BITCOINLEVELDB_BENCH_FLAGS_NUM.load(Relaxed) + 99) / 100;

            let mut i = 0;
            while i < self.reads {
                let k = (thread_ref.rand.next() % (range as u32)) as i32;
                let key = format!("{:016}", k);

                let mut vallen: usize = 0;
                let mut errptr: *mut u8 = ptr::null_mut();
                let value_ptr = leveldb_get(
                    self.db,
                    &options as *const LevelDBReadOptions,
                    key.as_ptr(),
                    key.len(),
                    &mut vallen as *mut usize,
                    &mut errptr,
                );
                bitcoinleveldb_bench_leveldb_error_check_or_exit("get", errptr);

                if !value_ptr.is_null() {
                    leveldb_free(value_ptr as *mut c_void);
                }

                thread_ref.stats.finished_single_op();
                i += 1;
            }
        }
    }

     pub fn seek_random(&mut self, thread: *mut ThreadState)  {

        unsafe {
            let thread_ref = &mut *thread;
            let options = LevelDBReadOptions::default();
            let mut found = 0;

            let mut i = 0;
            while i < self.reads {
                let iter = leveldb_create_iterator(
                    self.db,
                    &options as *const LevelDBReadOptions,
                );

                let k = (thread_ref.rand.next() % (BITCOINLEVELDB_BENCH_FLAGS_NUM.load(Relaxed) as u32)) as i32;
                let key = format!("{:016}", k);

                leveldb_iter_seek(iter, key.as_ptr(), key.len());

                if leveldb_iter_valid(iter) != 0 {
                    let mut klen: usize = 0;
                    let kptr = leveldb_iter_key(iter, &mut klen as *mut usize);
                    let iter_key = Slice::from_ptr_len(kptr, klen);
                    if iter_key.as_bytes() == key.as_bytes() {
                        found += 1;
                    }
                }

                let mut errptr: *mut u8 = ptr::null_mut();
                leveldb_iter_get_error(iter, &mut errptr);
                bitcoinleveldb_bench_leveldb_error_check_or_exit("iterator", errptr);

                leveldb_iter_destroy(iter);
                thread_ref.stats.finished_single_op();
                i += 1;
            }

            let msg = format!("({} of {} found)", found, self.num);
            thread_ref.stats.add_message(Slice::from(&msg));
        }
    }   
    
    /// Preserves delete ordering and batch boundaries exactly: keys are generated
    /// in the same sequence as the source benchmark and each batch is committed
    /// by a single LevelDB write call.
    pub fn do_delete(&mut self, 
        thread: *mut ThreadState,
        seq:    bool)  {

        unsafe {
            let thread_ref = &mut *thread;

            let _gen = RandomGenerator::default();
            let write_options =
                bitcoinleveldb_bench_make_write_options_from_rep(&self.write_options);
            let batch = leveldb_writebatch_create();

            let mut i = 0_i32;
            while i < self.num {
                leveldb_writebatch_clear(batch);

                let mut j = 0_i32;
                while j < self.entries_per_batch {
                    let k = if seq {
                        i + j
                    } else {
                        (thread_ref.rand.next()
                            % (BITCOINLEVELDB_BENCH_FLAGS_NUM.load(Relaxed) as u32)) as i32
                    };

                    let key = format!("{:016}", k);
                    leveldb_writebatch_delete(
                        batch,
                        key.as_ptr(),
                        key.len(),
                    );

                    thread_ref.stats.finished_single_op();
                    j += 1;
                }

                let mut errptr: *mut u8 = ptr::null_mut();
                leveldb_write(
                    self.db,
                    &write_options as *const LevelDBWriteOptions,
                    batch,
                    &mut errptr,
                );
                bitcoinleveldb_bench_leveldb_error_check_or_exit("del", errptr);

                i += self.entries_per_batch;
            }

            leveldb_writebatch_destroy(batch);
        }
    }

    pub fn delete_seq(&mut self, thread: *mut ThreadState)  {
        self.do_delete(thread, true);
    }
    
    pub fn delete_random(&mut self, thread: *mut ThreadState)  {
        self.do_delete(thread, false);
    }

    /// Preserves the mixed-mode benchmark invariant: thread zero performs blind
    /// writes until every reader thread has reached completion, and its own
    /// timing is reset before reporting.
    pub fn read_while_writing(&mut self, thread: *mut ThreadState)  {

        unsafe {
            let thread_ref = &mut *thread;

            if thread_ref.tid > 0 {
                self.read_random(thread);
            } else {
                // Special thread that keeps writing until other threads are done.
                let mut gen = RandomGenerator::default();
                let write_options =
                    bitcoinleveldb_bench_make_write_options_from_rep(&self.write_options);

                let shared_ref = match thread_ref.shared.as_ref() {
                    Some(value) => value,
                    None => {
                        eprintln!("readwhilewriting error: missing shared state");
                        exit(1);
                    }
                };

                loop {
                    let should_break = {
                        let guard = shared_ref.mu.lock();
                        guard.num_done + 1 >= guard.num_initialized
                    };

                    if should_break {
                        break;
                    }

                    let k = (thread_ref.rand.next()
                        % (BITCOINLEVELDB_BENCH_FLAGS_NUM.load(Relaxed) as u32)) as i32;
                    let key = format!("{:016}", k);
                    let value = gen.generate(self.value_size as usize);
                    let value_bytes = value.as_bytes();

                    let mut errptr: *mut u8 = ptr::null_mut();
                    leveldb_put(
                        self.db,
                        &write_options as *const LevelDBWriteOptions,
                        key.as_ptr(),
                        key.len(),
                        value_bytes.as_ptr(),
                        value_bytes.len(),
                        &mut errptr,
                    );
                    bitcoinleveldb_bench_leveldb_error_check_or_exit("put", errptr);
                }

                // Do not count any of the preceding work/delay in stats.
                thread_ref.stats.start();
            }
        }
    }

    pub fn compact(&mut self, thread: *mut ThreadState)  {

        let _ = thread;
        leveldb_compact_range(self.db, ptr::null(), 0, ptr::null(), 0);
    }

    pub fn print_stats(&mut self, key_: *const u8)  {

        let stats_ptr = leveldb_property_value(self.db, key_);
        if stats_ptr.is_null() {
            println!("\n(failed)");
        } else {
            let stats = Slice::from(stats_ptr as *const u8).to_string();
            println!("\n{}", stats);
            unsafe {
                leveldb_free(stats_ptr as *mut c_void);
            }
        }
    }

    pub fn write_to_file(
        arg: *mut c_void,
        buf: *const u8,
        n:   i32)  {

        if arg.is_null() {
            return;
        }

        if buf.is_null() {
            return;
        }

        if n <= 0 {
            return;
        }

        unsafe {
            let file_ptr = arg as *mut fs::File;
            if let Some(file_ref) = file_ptr.as_mut() {
                let bytes = Slice::from_ptr_len(buf, n as usize);
                let _ = std::io::Write::write_all(file_ref, bytes.as_bytes());
            }
        }
    }
   
    pub fn heap_profile(&mut self)  {

        let db_path = match bitcoinleveldb_bench_flag_db_path_get() {
            Some(path) => path,
            None => {
                eprintln!("heap profile error: missing db path");
                return;
            }
        };

        self.heap_counter += 1;
        let fname = format!("{}/heap-{:04}", db_path, self.heap_counter);

        match fs::File::create(&fname) {
            Ok(mut file) => {
                let ok = get_heap_profile(
                    Benchmark::write_to_file,
                    (&mut file as *mut fs::File) as *mut c_void,
                );

                drop(file);

                if !ok {
                    eprintln!("heap profiling not supported");
                    let _ = fs::remove_file(&fname);
                }
            }
            Err(error_value) => {
                eprintln!("{}", error_value);
            }
        }
    }
}

pub fn bitcoinleveldb_bench_saturating_i32_from_usize(value: usize) -> i32 {
    if value > i32::MAX as usize {
        i32::MAX
    } else {
        value as i32
    }
}

pub fn benchdb_bench_main(
    argc: i32,
    argv: *mut *mut u8) -> i32 {

    let default_options = Options::default();

    BITCOINLEVELDB_BENCH_FLAGS_WRITE_BUFFER_SIZE.store(
        bitcoinleveldb_bench_saturating_i32_from_usize(*default_options.write_buffer_size()),
        Relaxed,
    );
    BITCOINLEVELDB_BENCH_FLAGS_MAX_FILE_SIZE.store(
        bitcoinleveldb_bench_saturating_i32_from_usize(*default_options.max_file_size()),
        Relaxed,
    );
    BITCOINLEVELDB_BENCH_FLAGS_BLOCK_SIZE.store(
        bitcoinleveldb_bench_saturating_i32_from_usize(*default_options.block_size()),
        Relaxed,
    );
    BITCOINLEVELDB_BENCH_FLAGS_OPEN_FILES.store(*default_options.max_open_files(), Relaxed);

    let mut default_db_path = String::new();

    if !argv.is_null() {
        let mut i = 1_i32;
        while i < argc {
            let arg_ptr = unsafe { *argv.offset(i as isize) };
            let argument = if arg_ptr.is_null() {
                String::new()
            } else {
                Slice::from(arg_ptr as *const u8).to_string()
            };

            if let Some(rest) = argument.strip_prefix("--benchmarks=") {
                bitcoinleveldb_bench_flag_benchmarks_set(rest.to_owned());
            } else if let Some(value) =
                bitcoinleveldb_bench_parse_f64_flag(&argument, "--compression_ratio=")
            {
                bitcoinleveldb_bench_flag_compression_ratio_set(value);
            } else if let Some(value) =
                bitcoinleveldb_bench_parse_bool01_flag(&argument, "--histogram=")
            {
                BITCOINLEVELDB_BENCH_FLAGS_HISTOGRAM.store(value, Relaxed);
            } else if let Some(value) =
                bitcoinleveldb_bench_parse_bool01_flag(&argument, "--use_existing_db=")
            {
                BITCOINLEVELDB_BENCH_FLAGS_USE_EXISTING_DB.store(value, Relaxed);
            } else if let Some(value) =
                bitcoinleveldb_bench_parse_bool01_flag(&argument, "--reuse_logs=")
            {
                BITCOINLEVELDB_BENCH_FLAGS_REUSE_LOGS.store(value, Relaxed);
            } else if let Some(value) =
                bitcoinleveldb_bench_parse_i32_flag(&argument, "--num=")
            {
                BITCOINLEVELDB_BENCH_FLAGS_NUM.store(value, Relaxed);
            } else if let Some(value) =
                bitcoinleveldb_bench_parse_i32_flag(&argument, "--reads=")
            {
                BITCOINLEVELDB_BENCH_FLAGS_READS.store(value, Relaxed);
            } else if let Some(value) =
                bitcoinleveldb_bench_parse_i32_flag(&argument, "--threads=")
            {
                BITCOINLEVELDB_BENCH_FLAGS_THREADS.store(value, Relaxed);
            } else if let Some(value) =
                bitcoinleveldb_bench_parse_i32_flag(&argument, "--value_size=")
            {
                BITCOINLEVELDB_BENCH_FLAGS_VALUE_SIZE.store(value, Relaxed);
            } else if let Some(value) =
                bitcoinleveldb_bench_parse_i32_flag(&argument, "--write_buffer_size=")
            {
                BITCOINLEVELDB_BENCH_FLAGS_WRITE_BUFFER_SIZE.store(value, Relaxed);
            } else if let Some(value) =
                bitcoinleveldb_bench_parse_i32_flag(&argument, "--max_file_size=")
            {
                BITCOINLEVELDB_BENCH_FLAGS_MAX_FILE_SIZE.store(value, Relaxed);
            } else if let Some(value) =
                bitcoinleveldb_bench_parse_i32_flag(&argument, "--block_size=")
            {
                BITCOINLEVELDB_BENCH_FLAGS_BLOCK_SIZE.store(value, Relaxed);
            } else if let Some(value) =
                bitcoinleveldb_bench_parse_i32_flag(&argument, "--cache_size=")
            {
                BITCOINLEVELDB_BENCH_FLAGS_CACHE_SIZE.store(value, Relaxed);
            } else if let Some(value) =
                bitcoinleveldb_bench_parse_i32_flag(&argument, "--bloom_bits=")
            {
                BITCOINLEVELDB_BENCH_FLAGS_BLOOM_BITS.store(value, Relaxed);
            } else if let Some(value) =
                bitcoinleveldb_bench_parse_i32_flag(&argument, "--open_files=")
            {
                BITCOINLEVELDB_BENCH_FLAGS_OPEN_FILES.store(value, Relaxed);
            } else if let Some(rest) = argument.strip_prefix("--db=") {
                bitcoinleveldb_bench_flag_db_path_set(Some(rest.to_owned()));
            } else {
                eprintln!("Invalid flag '{}'", argument);
                unsafe {
                    exit(1);
                }
            }

            i += 1;
        }
    }

    // Choose a location for the test database if none given with --db=<path>
    if bitcoinleveldb_bench_flag_db_path_get().is_none() {
        match bitcoinleveldb_bench_get_test_directory() {
            Some(test_directory) => {
                default_db_path = test_directory;
                default_db_path.push_str("/dbbench");
                bitcoinleveldb_bench_flag_db_path_set(Some(default_db_path));
            }
            None => {}
        }
    }

    let mut benchmark = Benchmark::default();
    benchmark.run();
    0
}
