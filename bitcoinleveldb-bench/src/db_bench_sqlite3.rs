// ---------------- [ File: bitcoinleveldb-bench/src/db_bench_sqlite3.rs ]
crate::ix!();

use atomic::Ordering;
use std::ptr;
use std::ffi::{c_char};
use std::time::{UNIX_EPOCH,SystemTime};

//-------------------------------------------[.cpp/bitcoin/src/leveldb/benchmarks/db_bench_sqlite3.cc]

/**
  | Comma-separated list of operations
  | to run in the specified order
  | 
  | Actual benchmarks:
  | 
  ----------------------------
  | fillseq -- write N values in sequential
  | key order in async mode
  | 
  ----------------------------
  | fillseqsync -- write N/100 values
  | in sequential key order in sync mode
  | 
  ----------------------------
  | fillseqbatch -- batch write N values
  | in sequential key order in async mode
  | 
  ----------------------------
  | fillrandom -- write N values in random
  | key order in async mode
  | 
  ----------------------------
  | fillrandsync -- write N/100 values
  | in random key order in sync mode
  | 
  ----------------------------
  | fillrandbatch -- batch write N values
  | in sequential key order in async mode
  | 
  ----------------------------
  | overwrite -- overwrite N values in
  | random key order in async mode
  | 
  ----------------------------
  | fillrand100K -- write N/1000 100K
  | values in random order in async mode
  | 
  ----------------------------
  | fillseq100K -- write N/1000 100K values
  | in sequential order in async mode
  | 
  ----------------------------
  | readseq -- read N times sequentially
  | 
  ----------------------------
  | readrandom -- read N times in random
  | order
  | 
  ----------------------------
  | readrand100K -- read N/1000 100K values
  | in sequential order in async mode
  |
  */
pub const FLAGS_benchmarks: &'static str = concat!{
    "fillseq,",
    "fillseqsync,",
    "fillseqbatch,",
    "fillrandom,",
    "fillrandsync,",
    "fillrandbatch,",
    "overwrite,",
    "overwritebatch,",
    "readrandom,",
    "readseq,",
    "fillrand100K,",
    "fillseq100K,",
    "readseq,",
    "readrand100K,"
};

/// Number of key/values to place in database
pub static BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_NUM: AtomicI32 = AtomicI32::new(1_000_000);

/// Number of read operations to do.  If negative, do FLAGS_num reads.
pub static BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_READS: AtomicI32 = AtomicI32::new(-1);

/// Size of each value
pub static BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_VALUE_SIZE: AtomicI32 = AtomicI32::new(100);

/// Print histogram of operation timings
pub static BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_HISTOGRAM: AtomicBool = AtomicBool::new(false);

/// Arrange to generate values that shrink to this fraction of their original size after
/// compression
lazy_static!{
    pub static ref BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_COMPRESSION_RATIO: Mutex<f64> =
        Mutex::new(0.5_f64);
}

/// Page size. Default 1 KB.
pub static BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_PAGE_SIZE: AtomicI32 = AtomicI32::new(1024);

/// Number of pages.
/// 
/// Default cache size = FLAGS_page_size * FLAGS_num_pages = 4 MB.
pub static BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_NUM_PAGES: AtomicI32 = AtomicI32::new(4096);

/// If true, do not destroy the existing database.
///
/// If you set this flag and also specify a benchmark that wants a fresh database, that benchmark
/// will fail.
///
pub static BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_USE_EXISTING_DB: AtomicBool = AtomicBool::new(false);

/// If true, we allow batch writes to occur
pub static BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_TRANSACTION: AtomicBool = AtomicBool::new(true);

/// If true, we enable Write-Ahead Logging
pub static BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_WAL_ENABLED: AtomicBool = AtomicBool::new(true);

/**
   Use the db with the following name.
  */
lazy_static!{
    /*
    static const char* FLAGS_db = nullptr;
    */
}

#[inline] pub fn exec_error_check(
    status:  i32,
    err_msg: *mut u8)  {

    if status != sqlite3_sys::SQLITE_OK {
        let rendered_message = if err_msg.is_null() {
            String::new()
        } else {
            Slice::from(err_msg as *const u8).to_string()
        };

        eprintln!("SQL error: {}", rendered_message);

        unsafe {
            if !err_msg.is_null() {
                sqlite3_sys::sqlite3_free(err_msg as *mut c_void);
            }
        }

        unsafe {
            exit(1);
        }
    }
}

#[inline] pub fn step_error_check(status: i32)  {
    if status != sqlite3_sys::SQLITE_DONE {
        eprintln!("SQL step error: status = {}", status);
        unsafe {
            exit(1);
        }
    }
}

#[inline] pub fn error_check(status: i32)  {
    if status != sqlite3_sys::SQLITE_OK {
        eprintln!("sqlite3 error: status = {}", status);
        unsafe {
            exit(1);
        }
    }
}

lazy_static!{
    pub static ref BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_BENCHMARKS_TEXT: Mutex<String> =
        Mutex::new(String::from(FLAGS_benchmarks));
}

lazy_static!{
    pub static ref BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_DB_PATH: Mutex<Option<String>> =
        Mutex::new(None);
}

pub fn bitcoinleveldb_bench_sqlite3_flag_benchmarks_get() -> String {
    BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_BENCHMARKS_TEXT.lock().clone()
}

pub fn bitcoinleveldb_bench_sqlite3_flag_benchmarks_set(value: String) {
    *BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_BENCHMARKS_TEXT.lock() = value;
}

pub fn bitcoinleveldb_bench_sqlite3_flag_compression_ratio_get() -> f64 {
    *BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_COMPRESSION_RATIO.lock()
}

pub fn bitcoinleveldb_bench_sqlite3_flag_compression_ratio_set(value: f64) {
    *BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_COMPRESSION_RATIO.lock() = value;
}

pub fn bitcoinleveldb_bench_sqlite3_flag_db_path_get() -> Option<String> {
    BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_DB_PATH.lock().clone()
}

pub fn bitcoinleveldb_bench_sqlite3_flag_db_path_set(value: Option<String>) {
    *BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_DB_PATH.lock() = value;
}

pub fn bitcoinleveldb_bench_sqlite3_parse_i32_flag(
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

pub fn bitcoinleveldb_bench_sqlite3_parse_f64_flag(
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

pub fn bitcoinleveldb_bench_sqlite3_parse_bool01_flag(
    argument: &str,
    prefix:   &str) -> Option<bool> {

    match bitcoinleveldb_bench_sqlite3_parse_i32_flag(argument, prefix) {
        Some(0) => Some(false),
        Some(1) => Some(true),
        Some(_) => None,
        None => None,
    }
}

pub fn bitcoinleveldb_bench_sqlite3_now_seconds() -> f64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs_f64(),
        Err(_) => 0.0,
    }
}

pub fn bitcoinleveldb_bench_sqlite3_get_test_directory() -> Option<String> {
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

pub fn bitcoinleveldb_bench_sqlite3_cstring_or_exit(input: &str) -> CString {
    match CString::new(input) {
        Ok(value) => value,
        Err(_) => {
            eprintln!("sqlite3 string contains interior NUL");
            unsafe {
                exit(1);
            }
        }
    }
}

pub fn bitcoinleveldb_bench_sqlite3_exec_statement(
    db:        *mut sqlite3_sys::sqlite3,
    statement: &str)  {

    let c_statement = bitcoinleveldb_bench_sqlite3_cstring_or_exit(statement);

    unsafe {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = sqlite3_sys::sqlite3_exec(
            db,
            c_statement.as_ptr(),
            None,
            ptr::null_mut(),
            &mut err_msg,
        );
        exec_error_check(status, err_msg as *mut u8);
    }
}

pub fn bitcoinleveldb_bench_sqlite3_prepare_statement(
    db:        *mut sqlite3_sys::sqlite3,
    statement: &str) -> *mut sqlite3_sys::sqlite3_stmt {

    let c_statement = bitcoinleveldb_bench_sqlite3_cstring_or_exit(statement);

    unsafe {
        let mut stmt: *mut sqlite3_sys::sqlite3_stmt = ptr::null_mut();
        let status = sqlite3_sys::sqlite3_prepare_v2(
            db,
            c_statement.as_ptr(),
            -1,
            &mut stmt,
            ptr::null_mut(),
        );
        error_check(status);
        stmt
    }
}

#[inline] pub fn wal_checkpoint(db: *mut sqlite3_sys::sqlite3)  {
    unsafe {
        // Flush all writes to disk
        if BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_WAL_ENABLED.load(Ordering::Relaxed) {
            let _ = sqlite3_sys::sqlite3_wal_checkpoint_v2(
                db,
                ptr::null(),
                sqlite3_sys::SQLITE_CHECKPOINT_FULL,
                ptr::null_mut(),
                ptr::null_mut(),
            );
        }
    }
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

            // We use a limited amount of data over and over again and ensure
        // that it is larger than the compression window (32KB), and also
        // large enough to serve all typical value sizes we want to write.
        let mut rnd = Random::new(301);
        let compression_ratio = bitcoinleveldb_bench_sqlite3_flag_compression_ratio_get();

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

    pub fn generate(&mut self, len: i32) -> Slice {
        assert!(len >= 0);

        let len_usize = len as usize;
        let pos_usize = if self.pos < 0 {
            0usize
        } else {
            self.pos as usize
        };

        if pos_usize + len_usize > self.data.len() {
            self.pos = 0;
            assert!(len_usize < self.data.len());
        }

        let start = if self.pos < 0 {
            0usize
        } else {
            self.pos as usize
        };

        self.pos += len;

        unsafe {
            Slice::from_ptr_len(self.data.as_ptr().add(start), len_usize)
        }
    }
}

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

///-----------------------
pub struct Benchmark {
    db:             *mut sqlite3_sys::sqlite3,
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
        let num = BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_NUM.load(Ordering::Relaxed);
        let reads_flag = BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_READS.load(Ordering::Relaxed);
        let reads = if reads_flag < 0 { num } else { reads_flag };

        let result = Self {
            db: ptr::null_mut(),
            db_num: 0,
            num,
            reads,
            start: 0.0,
            last_op_finish: 0.0,
            bytes: 0,
            message: String::new(),
            hist: Histogram::default(),
            gen: RandomGenerator::default(),
            rand: Random::new(301),
            done: 0,
            next_report: 100,
        };

        if !BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_USE_EXISTING_DB.load(Ordering::Relaxed) {
            match bitcoinleveldb_bench_sqlite3_get_test_directory() {
                Some(test_dir) => {
                    match fs::read_dir(&test_dir) {
                        Ok(entries) => {
                            for entry_result in entries {
                                match entry_result {
                                    Ok(entry) => {
                                        let file_name_os = entry.file_name();
                                        match file_name_os.to_str() {
                                            Some(file_name) => {
                                                if file_name.starts_with("dbbench_sqlite3") {
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
                }
                None => {}
            }
        }

        result
    }
}

impl Drop for Benchmark {
    fn drop(&mut self) {
        let raw_db = self.db;
        if !raw_db.is_null() {
            unsafe {
                let status = sqlite3_sys::sqlite3_close(raw_db);
                error_check(status);
            }
            self.db = ptr::null_mut();
        }
    }
}

impl Benchmark {
    
    pub fn print_header(&mut self)  {
        const K_KEY_SIZE: i32 = 16;
        self.print_environment();
        println!("Keys:       {} bytes each", K_KEY_SIZE);
        println!(
            "Values:     {} bytes each",
            BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_VALUE_SIZE.load(Ordering::Relaxed)
        );
        println!("Entries:    {}", self.num);
        println!(
            "RawSize:    {:.1} MB (estimated)",
            (((i64::from(K_KEY_SIZE)
                + i64::from(BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_VALUE_SIZE.load(Ordering::Relaxed)))
                * i64::from(self.num)) as f64)
                / 1048576.0
        );
        self.print_warnings();
        println!("------------------------------------------------");
    }

    pub fn print_warnings(&mut self)  {
        if cfg!(debug_assertions) {
            println!("WARNING: Assertions are enabled; benchmarks unnecessarily slow");
        }
    }
    
    pub fn print_environment(&mut self)  {
        unsafe {
            let version_ptr = sqlite3_sys::sqlite3_libversion();
            if version_ptr.is_null() {
                eprintln!("SQLite:     version unknown");
            } else {
                let version = CStr::from_ptr(version_ptr).to_string_lossy().into_owned();
                eprintln!("SQLite:     version {}", version);
            }
        }

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
   
    pub fn start(&mut self)  {
        self.start = bitcoinleveldb_bench_sqlite3_now_seconds();
        self.bytes = 0;
        self.message.clear();
        self.last_op_finish = self.start;
        self.hist.clear();
        self.done = 0;
        self.next_report = 100;
    }
   
    pub fn finished_single_op(&mut self)  {
        if BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_HISTOGRAM.load(Ordering::Relaxed) {
            let now = bitcoinleveldb_bench_sqlite3_now_seconds();
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
   
    pub fn stop(&mut self, name: &Slice)  {
        let finish = bitcoinleveldb_bench_sqlite3_now_seconds();

        // Pretend at least one op was done in case we are running a benchmark
        // that does not call FinishedSingleOp().
        if self.done < 1 {
            self.done = 1;
        }

        if self.bytes > 0 {
            let rate = format!(
                "{:6.1} MB/s",
                (self.bytes as f64 / 1048576.0) / (finish - self.start)
            );
            if !self.message.is_empty() {
                self.message = format!("{} {}", rate, self.message);
            } else {
                self.message = rate;
            }
        }

        println!(
            "{:<12} : {:11.3} micros/op;{}{}",
            name.to_string(),
            (finish - self.start) * 1e6 / self.done as f64,
            if self.message.is_empty() { "" } else { " " },
            self.message
        );
        if BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_HISTOGRAM.load(Ordering::Relaxed) {
            println!("Microseconds per op:\n{}\n", self.hist.to_string());
        }
    }
    
    pub fn run(&mut self)  {
        self.print_header();
        self.open();

        let benchmarks_text = bitcoinleveldb_bench_sqlite3_flag_benchmarks_get();
        for name in benchmarks_text.split(',') {
            self.bytes = 0;
            self.start();

            let mut known = true;
            let mut write_sync = false;

            if name == "fillseq" {
                self.write(
                    write_sync,
                    benchmark::Order::SEQUENTIAL,
                    benchmark::DBState::FRESH,
                    self.num,
                    BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_VALUE_SIZE.load(Ordering::Relaxed),
                    1,
                );
                wal_checkpoint(self.db as *mut sqlite3_sys::sqlite3);
            } else if name == "fillseqbatch" {
                self.write(
                    write_sync,
                    benchmark::Order::SEQUENTIAL,
                    benchmark::DBState::FRESH,
                    self.num,
                    BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_VALUE_SIZE.load(Ordering::Relaxed),
                    1000,
                );
                wal_checkpoint(self.db as *mut sqlite3_sys::sqlite3);
            } else if name == "fillrandom" {
                self.write(
                    write_sync,
                    benchmark::Order::RANDOM,
                    benchmark::DBState::FRESH,
                    self.num,
                    BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_VALUE_SIZE.load(Ordering::Relaxed),
                    1,
                );
                wal_checkpoint(self.db as *mut sqlite3_sys::sqlite3);
            } else if name == "fillrandbatch" {
                self.write(
                    write_sync,
                    benchmark::Order::RANDOM,
                    benchmark::DBState::FRESH,
                    self.num,
                    BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_VALUE_SIZE.load(Ordering::Relaxed),
                    1000,
                );
                wal_checkpoint(self.db as *mut sqlite3_sys::sqlite3);
            } else if name == "overwrite" {
                self.write(
                    write_sync,
                    benchmark::Order::RANDOM,
                    benchmark::DBState::EXISTING,
                    self.num,
                    BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_VALUE_SIZE.load(Ordering::Relaxed),
                    1,
                );
                wal_checkpoint(self.db as *mut sqlite3_sys::sqlite3);
            } else if name == "overwritebatch" {
                self.write(
                    write_sync,
                    benchmark::Order::RANDOM,
                    benchmark::DBState::EXISTING,
                    self.num,
                    BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_VALUE_SIZE.load(Ordering::Relaxed),
                    1000,
                );
                wal_checkpoint(self.db as *mut sqlite3_sys::sqlite3);
            } else if name == "fillrandsync" {
                write_sync = true;
                self.write(
                    write_sync,
                    benchmark::Order::RANDOM,
                    benchmark::DBState::FRESH,
                    self.num / 100,
                    BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_VALUE_SIZE.load(Ordering::Relaxed),
                    1,
                );
                wal_checkpoint(self.db as *mut sqlite3_sys::sqlite3);
            } else if name == "fillseqsync" {
                write_sync = true;
                self.write(
                    write_sync,
                    benchmark::Order::SEQUENTIAL,
                    benchmark::DBState::FRESH,
                    self.num / 100,
                    BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_VALUE_SIZE.load(Ordering::Relaxed),
                    1,
                );
                wal_checkpoint(self.db as *mut sqlite3_sys::sqlite3);
            } else if name == "fillrand100K" {
                self.write(
                    write_sync,
                    benchmark::Order::RANDOM,
                    benchmark::DBState::FRESH,
                    self.num / 1000,
                    100 * 1000,
                    1,
                );
                wal_checkpoint(self.db as *mut sqlite3_sys::sqlite3);
            } else if name == "fillseq100K" {
                self.write(
                    write_sync,
                    benchmark::Order::SEQUENTIAL,
                    benchmark::DBState::FRESH,
                    self.num / 1000,
                    100 * 1000,
                    1,
                );
                wal_checkpoint(self.db as *mut sqlite3_sys::sqlite3);
            } else if name == "readseq" {
                self.read_sequential();
            } else if name == "readrandom" {
                self.read(benchmark::Order::RANDOM, 1);
            } else if name == "readrand100K" {
                let n = self.reads;
                self.reads /= 1000;
                self.read(benchmark::Order::RANDOM, 1);
                self.reads = n;
            } else {
                known = false;
                if !name.is_empty() {
                    // No error message for empty name
                    eprintln!("unknown benchmark '{}'", name);
                }
            }

            if known {
                let name_slice = Slice::from_str(name);
                self.stop(&name_slice);
            }
        }
    }

    pub fn open(&mut self)  {
        assert!(self.db.is_null());

        let file_name = match bitcoinleveldb_bench_sqlite3_get_test_directory() {
            Some(mut tmp_dir) => {
                self.db_num += 1;
                tmp_dir.push_str("/");
                tmp_dir.push_str(&format!("dbbench_sqlite3-{}.db", self.db_num));
                tmp_dir
            }
            None => {
                eprintln!("open error: missing test directory");
                unsafe {
                    exit(1);
                }
            }
        };

        let c_file_name = bitcoinleveldb_bench_sqlite3_cstring_or_exit(&file_name);

        unsafe {
            let mut raw_db: *mut sqlite3_sys::sqlite3 = ptr::null_mut();

            // Open database
            let status = sqlite3_sys::sqlite3_open(c_file_name.as_ptr(), &mut raw_db);
            if status != sqlite3_sys::SQLITE_OK {
                let message = if raw_db.is_null() {
                    String::from("sqlite3_open failed")
                } else {
                    let err_ptr = sqlite3_sys::sqlite3_errmsg(raw_db);
                    if err_ptr.is_null() {
                        String::from("sqlite3_open failed")
                    } else {
                        CStr::from_ptr(err_ptr).to_string_lossy().into_owned()
                    }
                };
                eprintln!("open error: {}", message);
                exit(1);
            }

            self.db = raw_db;
        }

        let raw_db = self.db;

        // Change SQLite cache size
        let cache_size = format!(
            "PRAGMA cache_size = {}",
            BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_NUM_PAGES.load(Ordering::Relaxed)
        );
        bitcoinleveldb_bench_sqlite3_exec_statement(raw_db, &cache_size);

        // FLAGS_page_size is defaulted to 1024
        if BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_PAGE_SIZE.load(Ordering::Relaxed) != 1024 {
            let page_size = format!(
                "PRAGMA page_size = {}",
                BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_PAGE_SIZE.load(Ordering::Relaxed)
            );
            bitcoinleveldb_bench_sqlite3_exec_statement(raw_db, &page_size);
        }

        // Change journal mode to WAL if WAL enabled flag is on
        if BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_WAL_ENABLED.load(Ordering::Relaxed) {
            let wal_stmt = "PRAGMA journal_mode = WAL";
            let wal_checkpoint_stmt = "PRAGMA wal_autocheckpoint = 4096";
            bitcoinleveldb_bench_sqlite3_exec_statement(raw_db, wal_stmt);
            bitcoinleveldb_bench_sqlite3_exec_statement(raw_db, wal_checkpoint_stmt);
        }

        // Change locking mode to exclusive and create tables/index for database
        let locking_stmt = "PRAGMA locking_mode = EXCLUSIVE";
        let create_stmt = "CREATE TABLE test (key blob, value blob, PRIMARY KEY(key))";
        let stmt_array = [locking_stmt, create_stmt];

        for statement in stmt_array {
            bitcoinleveldb_bench_sqlite3_exec_statement(raw_db, statement);
        }
    }

    pub fn write(&mut self,
        write_sync:        bool,
        order:             benchmark::Order,
        state:             benchmark::DBState,
        num_entries:       i32,
        value_size:        i32,
        entries_per_batch: i32)  {

        // Create new database if state == FRESH
        match state {
            benchmark::DBState::FRESH => {
                if BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_USE_EXISTING_DB.load(Ordering::Relaxed) {
                    self.message = String::from("skipping (--use_existing_db is true)");
                    return;
                }

                let raw_db = self.db as *mut sqlite3_sys::sqlite3;
                if !raw_db.is_null() {
                    unsafe {
                        let status = sqlite3_sys::sqlite3_close(raw_db);
                        error_check(status);
                    }
                    self.db = ptr::null_mut();
                }

                self.open();
                self.start();
            }
            benchmark::DBState::EXISTING => {}
        }

        if num_entries != self.num {
            self.message = format!("({} ops)", num_entries);
        }

        let raw_db = self.db as *mut sqlite3_sys::sqlite3;

        // Check for synchronous flag in options
        let sync_stmt = if write_sync {
            "PRAGMA synchronous = FULL"
        } else {
            "PRAGMA synchronous = OFF"
        };
        bitcoinleveldb_bench_sqlite3_exec_statement(raw_db, sync_stmt);

        // Preparing sqlite3 statements
        let replace_stmt = bitcoinleveldb_bench_sqlite3_prepare_statement(
            raw_db,
            "REPLACE INTO test (key, value) VALUES (?, ?)",
        );
        let begin_trans_stmt = bitcoinleveldb_bench_sqlite3_prepare_statement(
            raw_db,
            "BEGIN TRANSACTION;",
        );
        let end_trans_stmt = bitcoinleveldb_bench_sqlite3_prepare_statement(
            raw_db,
            "END TRANSACTION;",
        );

        let transaction = entries_per_batch > 1;
        let mut i = 0_i32;

        while i < num_entries {
            // Begin write transaction
            if BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_TRANSACTION.load(Ordering::Relaxed) && transaction {
                unsafe {
                    let status = sqlite3_sys::sqlite3_step(begin_trans_stmt);
                    step_error_check(status);
                    let reset_status = sqlite3_sys::sqlite3_reset(begin_trans_stmt);
                    error_check(reset_status);
                }
            }

            // Create and execute SQL statements
            let mut j = 0_i32;
            while j < entries_per_batch {
                let value_slice = self.gen.generate(value_size);
                let value_bytes = value_slice.as_bytes();

                // Create values for key-value pair
                let k = match order {
                    benchmark::Order::SEQUENTIAL => i + j,
                    benchmark::Order::RANDOM => (self.rand.next() % (num_entries as u32)) as i32,
                };

                let key = format!("{:016}", k);
                let key_bytes = key.as_bytes();

                unsafe {
                    // Bind KV values into replace_stmt
                    let bind_key_status = sqlite3_sys::sqlite3_bind_blob(
                        replace_stmt,
                        1,
                        key_bytes.as_ptr() as *const c_void,
                        16,
                        None,
                    );
                    error_check(bind_key_status);

                    let bind_value_status = sqlite3_sys::sqlite3_bind_blob(
                        replace_stmt,
                        2,
                        value_bytes.as_ptr() as *const c_void,
                        value_size,
                        None,
                    );
                    error_check(bind_value_status);

                    // Execute replace_stmt
                    self.bytes += i64::from(value_size) + key_bytes.len() as i64;
                    let step_status = sqlite3_sys::sqlite3_step(replace_stmt);
                    step_error_check(step_status);

                    // Reset SQLite statement for another use
                    let clear_status = sqlite3_sys::sqlite3_clear_bindings(replace_stmt);
                    error_check(clear_status);
                    let reset_status = sqlite3_sys::sqlite3_reset(replace_stmt);
                    error_check(reset_status);
                }

                self.finished_single_op();
                j += 1;
            }

            // End write transaction
            if BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_TRANSACTION.load(Ordering::Relaxed) && transaction {
                unsafe {
                    let status = sqlite3_sys::sqlite3_step(end_trans_stmt);
                    step_error_check(status);
                    let reset_status = sqlite3_sys::sqlite3_reset(end_trans_stmt);
                    error_check(reset_status);
                }
            }

            i += entries_per_batch;
        }

        unsafe {
            let finalize_replace_status = sqlite3_sys::sqlite3_finalize(replace_stmt);
            error_check(finalize_replace_status);
            let finalize_begin_status = sqlite3_sys::sqlite3_finalize(begin_trans_stmt);
            error_check(finalize_begin_status);
            let finalize_end_status = sqlite3_sys::sqlite3_finalize(end_trans_stmt);
            error_check(finalize_end_status);
        }
    }
    
    pub fn read(&mut self,
        order:             benchmark::Order,
        entries_per_batch: i32)  {

        let raw_db = self.db as *mut sqlite3_sys::sqlite3;

        // Preparing sqlite3 statements
        let begin_trans_stmt = bitcoinleveldb_bench_sqlite3_prepare_statement(
            raw_db,
            "BEGIN TRANSACTION;",
        );
        let end_trans_stmt = bitcoinleveldb_bench_sqlite3_prepare_statement(
            raw_db,
            "END TRANSACTION;",
        );
        let read_stmt = bitcoinleveldb_bench_sqlite3_prepare_statement(
            raw_db,
            "SELECT * FROM test WHERE key = ?",
        );

        let transaction = entries_per_batch > 1;
        let mut i = 0_i32;

        while i < self.reads {
            // Begin read transaction
            if BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_TRANSACTION.load(Ordering::Relaxed) && transaction {
                unsafe {
                    let status = sqlite3_sys::sqlite3_step(begin_trans_stmt);
                    step_error_check(status);
                    let reset_status = sqlite3_sys::sqlite3_reset(begin_trans_stmt);
                    error_check(reset_status);
                }
            }

            // Create and execute SQL statements
            let mut j = 0_i32;
            while j < entries_per_batch {
                // Create key value
                let k = match order {
                    benchmark::Order::SEQUENTIAL => i + j,
                    benchmark::Order::RANDOM => (self.rand.next() % (self.reads as u32)) as i32,
                };

                let key = format!("{:016}", k);
                let key_bytes = key.as_bytes();

                unsafe {
                    // Bind key value into read_stmt
                    let bind_status = sqlite3_sys::sqlite3_bind_blob(
                        read_stmt,
                        1,
                        key_bytes.as_ptr() as *const c_void,
                        16,
                        None,
                    );
                    error_check(bind_status);

                    // Execute read statement
                    let mut step_status = sqlite3_sys::SQLITE_ROW;
                    while step_status == sqlite3_sys::SQLITE_ROW {
                        step_status = sqlite3_sys::sqlite3_step(read_stmt);
                    }
                    step_error_check(step_status);

                    // Reset SQLite statement for another use
                    let clear_status = sqlite3_sys::sqlite3_clear_bindings(read_stmt);
                    error_check(clear_status);
                    let reset_status = sqlite3_sys::sqlite3_reset(read_stmt);
                    error_check(reset_status);
                }

                self.finished_single_op();
                j += 1;
            }

            // End read transaction
            if BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_TRANSACTION.load(Ordering::Relaxed) && transaction {
                unsafe {
                    let status = sqlite3_sys::sqlite3_step(end_trans_stmt);
                    step_error_check(status);
                    let reset_status = sqlite3_sys::sqlite3_reset(end_trans_stmt);
                    error_check(reset_status);
                }
            }

            i += entries_per_batch;
        }

        unsafe {
            let finalize_read_status = sqlite3_sys::sqlite3_finalize(read_stmt);
            error_check(finalize_read_status);
            let finalize_begin_status = sqlite3_sys::sqlite3_finalize(begin_trans_stmt);
            error_check(finalize_begin_status);
            let finalize_end_status = sqlite3_sys::sqlite3_finalize(end_trans_stmt);
            error_check(finalize_end_status);
        }
    }

    pub fn read_sequential(&mut self)  {
        let raw_db = self.db as *mut sqlite3_sys::sqlite3;
        let p_stmt = bitcoinleveldb_bench_sqlite3_prepare_statement(
            raw_db,
            "SELECT * FROM test ORDER BY key",
        );

        let mut i = 0_i32;
        unsafe {
            while i < self.reads && sqlite3_sys::sqlite3_step(p_stmt) == sqlite3_sys::SQLITE_ROW {
                self.bytes += i64::from(sqlite3_sys::sqlite3_column_bytes(p_stmt, 1))
                    + i64::from(sqlite3_sys::sqlite3_column_bytes(p_stmt, 2));
                self.finished_single_op();
                i += 1;
            }

            let finalize_status = sqlite3_sys::sqlite3_finalize(p_stmt);
            error_check(finalize_status);
        }
    }
}

pub fn benchdb_bench_sqlite3_main(
    argc: i32,
    argv: *mut *mut u8) -> i32 {

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
                bitcoinleveldb_bench_sqlite3_flag_benchmarks_set(rest.to_owned());
            } else if let Some(value) =
                bitcoinleveldb_bench_sqlite3_parse_bool01_flag(&argument, "--histogram=")
            {
                BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_HISTOGRAM.store(value, Ordering::Relaxed);
            } else if let Some(value) =
                bitcoinleveldb_bench_sqlite3_parse_f64_flag(&argument, "--compression_ratio=")
            {
                bitcoinleveldb_bench_sqlite3_flag_compression_ratio_set(value);
            } else if let Some(value) =
                bitcoinleveldb_bench_sqlite3_parse_bool01_flag(&argument, "--use_existing_db=")
            {
                BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_USE_EXISTING_DB.store(value, Ordering::Relaxed);
            } else if let Some(value) =
                bitcoinleveldb_bench_sqlite3_parse_i32_flag(&argument, "--num=")
            {
                BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_NUM.store(value, Ordering::Relaxed);
            } else if let Some(value) =
                bitcoinleveldb_bench_sqlite3_parse_i32_flag(&argument, "--reads=")
            {
                BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_READS.store(value, Ordering::Relaxed);
            } else if let Some(value) =
                bitcoinleveldb_bench_sqlite3_parse_i32_flag(&argument, "--value_size=")
            {
                BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_VALUE_SIZE.store(value, Ordering::Relaxed);
            } else if argument == "--no_transaction" {
                BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_TRANSACTION.store(false, Ordering::Relaxed);
            } else if let Some(value) =
                bitcoinleveldb_bench_sqlite3_parse_i32_flag(&argument, "--page_size=")
            {
                BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_PAGE_SIZE.store(value, Ordering::Relaxed);
            } else if let Some(value) =
                bitcoinleveldb_bench_sqlite3_parse_i32_flag(&argument, "--num_pages=")
            {
                BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_NUM_PAGES.store(value, Ordering::Relaxed);
            } else if let Some(value) =
                bitcoinleveldb_bench_sqlite3_parse_bool01_flag(&argument, "--WAL_enabled=")
            {
                BITCOINLEVELDB_BENCH_SQLITE3_FLAGS_WAL_ENABLED.store(value, Ordering::Relaxed);
            } else if let Some(rest) = argument.strip_prefix("--db=") {
                bitcoinleveldb_bench_sqlite3_flag_db_path_set(Some(rest.to_owned()));
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
    if bitcoinleveldb_bench_sqlite3_flag_db_path_get().is_none() {
        match bitcoinleveldb_bench_sqlite3_get_test_directory() {
            Some(test_directory) => {
                default_db_path = test_directory;
                default_db_path.push_str("/dbbench");
                bitcoinleveldb_bench_sqlite3_flag_db_path_set(Some(default_db_path));
            }
            None => {}
        }
    }

    let mut benchmark = Benchmark::default();
    benchmark.run();
    0
}
