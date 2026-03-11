// ---------------- [ File: bitcoinleveldb-test/src/harness.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/testharness.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/testharness.cc]

/**
  | An instance of Tester is allocated to
  | hold temporary state during the execution
  | of an assertion.
  |
  */
pub struct Tester {
    ok:    bool,
    fname: &'static str,
    line:  i32,
    ss:    String,
}

impl Drop for Tester {
    fn drop(&mut self) {
        if !self.ok {
            error!(
                target: "bitcoinleveldb_test::harness",
                event = "tester_drop_failure",
                file = self.fname,
                line = self.line,
                message = %self.ss
            );

            eprintln!("{}:{}:{}", self.fname, self.line, self.ss);
            exit(1);
        }
    }
}

macro_rules! binary_op {
    ($name:ident, $op:tt) => {
        impl Tester {
            pub fn $name<X, Y>(mut self, x: &X, y: &Y) -> Tester
            where
                X: Display + ?Sized,
                Y: Display + ?Sized,
                for<'a> &'a X: PartialEq<&'a Y> + PartialOrd<&'a Y>,
            {
                if !(x $op y) {
                    self.ss.push_str(" failed: ");
                    self.ss.push_str(format!("{}", x).as_str());
                    self.ss.push_str(concat!(" ", stringify!($op), " "));
                    self.ss.push_str(format!("{}", y).as_str());
                    self.ok = false;
                }

                self
            }
        }
    }
}

binary_op!{ IsEq, == }
binary_op!{ IsNe, != }
binary_op!{ IsGe, >= }
binary_op!{ IsGt, > }
binary_op!{ IsLe, <= }
binary_op!{ IsLt, < }

impl Tester {

    pub fn new(
        f: &'static str,
        l: i32,
    ) -> Self {
        trace!(
            target: "bitcoinleveldb_test::harness",
            event = "tester_new_entry",
            file = f,
            line = l
        );

        let out = Self {
            ok: true,
            fname: f,
            line: l,
            ss: String::new(),
        };

        trace!(
            target: "bitcoinleveldb_test::harness",
            event = "tester_new_exit",
            file = out.fname,
            line = out.line
        );

        out
    }

    pub fn is(
        mut self,
        b:   bool,
        msg: &'static str,
    ) -> Tester {
        if !b {
            self.ss.push_str(" Assertion failure ");
            self.ss.push_str(msg);
            self.ok = false;
        }

        self
    }

    pub fn is_ok(
        mut self,
        s: &Status,
    ) -> Tester {
        if !s.is_ok() {
            self.ss.push(' ');
            self.ss.push_str(s.to_string().as_str());
            self.ok = false;
        }

        self
    }
}

impl<V> Shl<&V> for Tester
where
    V: Display + ?Sized,
{
    type Output = Tester;

    /**
      | Attach the specified value to the error
      | message if an error has occurred
      |
      */
    #[inline]
    fn shl(mut self, rhs: &V) -> Self::Output {
        if !self.ok {
            self.ss.push(' ');
            self.ss.push_str(format!("{}", rhs).as_str());
        }

        self
    }
}

macro_rules! assert_true {
    ($c:ident) => {
        crate::harness::Tester::new(file!(), line!() as i32).is(($c), stringify!($c))
    }
}

macro_rules! assert_ok {
    ($s:ident) => {
        crate::harness::Tester::new(file!(), line!() as i32).is_ok(&($s))
    }
}

macro_rules! assert_eq {
    ($a:ident, $b:ident) => {
        crate::harness::Tester::new(file!(), line!() as i32).IsEq(&($a), &($b))
    }
}

macro_rules! assert_ne {
    ($a:ident, $b:ident) => {
        crate::harness::Tester::new(file!(), line!() as i32).IsNe(&($a), &($b))
    }
}

macro_rules! assert_ge {
    ($a:ident, $b:ident) => {
        crate::harness::Tester::new(file!(), line!() as i32).IsGe(&($a), &($b))
    }
}

macro_rules! assert_gt {
    ($a:ident, $b:ident) => {
        crate::harness::Tester::new(file!(), line!() as i32).IsGt(&($a), &($b))
    }
}

macro_rules! assert_le {
    ($a:ident, $b:ident) => {
        crate::harness::Tester::new(file!(), line!() as i32).IsLe(&($a), &($b))
    }
}

macro_rules! assert_lt {
    ($a:ident, $b:ident) => {
        crate::harness::Tester::new(file!(), line!() as i32).IsLt(&($a), &($b))
    }
}

#[derive(Clone)]
pub struct Test {
    base: String,
    name: String,
    func: fn(),
}

lazy_static!{
    static ref BITCOINLEVELDB_TEST_HARNESS_REGISTERED_TESTS: Mutex<Vec<Test>> =
        Mutex::new(Vec::new());
}

/**
  | Register the specified test. Typically
  | not used directly, but invoked via the
  | macro expansion of TEST.
  |
  */
pub fn register_test(
    base: *const u8,
    name: *const u8,
    func: fn(),
) -> bool {
    trace!(
        target: "bitcoinleveldb_test::harness",
        event = "register_test_entry",
        base_ptr = (base as usize),
        name_ptr = (name as usize)
    );

    let base_label = if base.is_null() {
        String::new()
    } else {
        unsafe {
            CStr::from_ptr(base as *const c_char)
                .to_string_lossy()
                .into_owned()
        }
    };

    let name_label = if name.is_null() {
        String::new()
    } else {
        unsafe {
            CStr::from_ptr(name as *const c_char)
                .to_string_lossy()
                .into_owned()
        }
    };

    let mut guard = BITCOINLEVELDB_TEST_HARNESS_REGISTERED_TESTS.lock();
    guard.push(Test {
        base: base_label,
        name: name_label,
        func,
    });

    trace!(
        target: "bitcoinleveldb_test::harness",
        event = "register_test_exit",
        registered_count = guard.len()
    );

    true
}

/**
  | Run some of the tests registered by the TEST()
  | macro.  If the environment variable
  | "LEVELDB_TESTS" is not set, runs all tests.
  |
  | Otherwise, runs only the tests whose name
  | contains the value of "LEVELDB_TESTS" as
  | a substring.  E.g., suppose the tests are:
  |
  |    TEST(Foo, Hello) { ... }
  |    TEST(Foo, World) { ... }
  |
  | LEVELDB_TESTS=Hello will run the first test
  | LEVELDB_TESTS=o     will run both tests
  | LEVELDB_TESTS=Junk  will run no tests
  |
  | Returns 0 if all tests pass.
  |
  | Dies or returns a non-zero value if some test
  | fails.
  */
pub fn run_all_tests() -> i32 {
    trace!(
        target: "bitcoinleveldb_test::harness",
        event = "run_all_tests_entry"
    );

    let matcher: Option<String> = unsafe {
        let p = libc::getenv(b"LEVELDB_TESTS\0".as_ptr() as *const c_char);
        if p.is_null() {
            None
        } else {
            Some(CStr::from_ptr(p).to_string_lossy().into_owned())
        }
    };

    let tests_snapshot: Vec<Test> = {
        let guard = BITCOINLEVELDB_TEST_HARNESS_REGISTERED_TESTS.lock();
        guard.clone()
    };

    let mut num: i32 = 0;

    for t in tests_snapshot.iter() {
        let mut full_name = t.base.clone();
        full_name.push('.');
        full_name.push_str(t.name.as_str());

        match matcher.as_ref() {
            Some(m) => {
                if !full_name.contains(m.as_str()) {
                    trace!(
                        target: "bitcoinleveldb_test::harness",
                        event = "run_all_tests_skip",
                        test_name = %full_name,
                        matcher = %m
                    );
                    continue;
                }
            }
            None => {}
        }

        eprintln!("==== Test {}.{}", t.base, t.name);

        trace!(
            target: "bitcoinleveldb_test::harness",
            event = "run_all_tests_invoke",
            test_name = %full_name
        );

        (t.func)();
        num += 1;
    }

    eprintln!("==== PASSED {} tests", num);

    trace!(
        target: "bitcoinleveldb_test::harness",
        event = "run_all_tests_exit",
        executed = num
    );

    0
}

/// Return the directory to use for temporary storage.
///
/// Invariant: the returned path is copied into owned Rust storage before any
/// C-allocated buffer is released, so callers never observe borrowed storage.
pub fn tmp_dir() -> String {
    trace!(
        target: "bitcoinleveldb_test::harness",
        event = "tmp_dir_entry"
    );

    unsafe {
        let env: *mut LevelDBEnv = leveldb_create_default_env();
        if env.is_null() {
            error!(
                target: "bitcoinleveldb_test::harness",
                event = "tmp_dir_env_create_failed"
            );
            panic!("bitcoinleveldb_test__harness_rs__tmp_dir_env_create_failed");
        }

        let p: *mut u8 = leveldb_env_get_test_directory(env);
        leveldb_env_destroy(env);

        if p.is_null() {
            error!(
                target: "bitcoinleveldb_test::harness",
                event = "tmp_dir_get_test_directory_failed"
            );
            panic!("bitcoinleveldb_test__harness_rs__tmp_dir_get_test_directory_failed");
        }

        let dir = CStr::from_ptr(p as *const c_char)
            .to_string_lossy()
            .into_owned();

        leveldb_free(p as *mut c_void);

        trace!(
            target: "bitcoinleveldb_test::harness",
            event = "tmp_dir_exit",
            dir_len = dir.len()
        );

        dir
    }
}

/// Return a randomization seed for this run.
///
/// Invariant: non-positive or unparsable environment overrides collapse to the
/// stable fallback value `301`, preserving deterministic behavior.
pub fn random_seed() -> i32 {
    trace!(
        target: "bitcoinleveldb_test::harness",
        event = "random_seed_entry"
    );

    let result = unsafe {
        let env_ptr = libc::getenv(b"TEST_RANDOM_SEED\0".as_ptr() as *const c_char);
        if env_ptr.is_null() {
            301
        } else {
            let raw = CStr::from_ptr(env_ptr).to_string_lossy().into_owned();
            match raw.parse::<i32>() {
                Ok(parsed) => {
                    if parsed <= 0 {
                        301
                    } else {
                        parsed
                    }
                }
                Err(_) => 301,
            }
        }
    };

    trace!(
        target: "bitcoinleveldb_test::harness",
        event = "random_seed_exit",
        result = result
    );

    result
}
