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
    fname: *const u8,
    line:  i32,
    ss:    Box<dyn std::io::Write>,
}

impl Drop for Tester {
    fn drop(&mut self) {
        todo!();
        /*
            if (!ok_) {
          fprintf(stderr, "%s:%d:%s\n", fname_, line_, ss_.str().c_str());
          exit(1);
        }
        */
    }
}

macro_rules! binary_op {
    ($name:ident, $op:tt) => {
        /*
        
          template <class X, class Y>                        
          Tester& name(const X& x, const Y& y) {             
            if (!(x op y)) {                                 
              ss_ << " failed: " << x << (" " #op " ") << y; 
              ok_ = false;                                   
            }                                                
            return *this;                                    
          }
        */
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
        f: *const u8,
        l: i32) -> Self {
    
        todo!();
        /*
        : ok(true),
        : fname(f),
        : line(l),

        
        */
    }
    
    pub fn is(&mut self, 
        b:   bool,
        msg: *const u8) -> &mut Tester {
        
        todo!();
        /*
            if (!b) {
          ss_ << " Assertion failure " << msg;
          ok_ = false;
        }
        return *this;
        */
    }
    
    pub fn is_ok(&mut self, s: &Status) -> &mut Tester {
        
        todo!();
        /*
            if (!s.ok()) {
          ss_ << " " << s.ToString();
          ok_ = false;
        }
        return *this;
        */
    }
}

impl<V> Shl<&V> for Tester {
    type Output = Tester;
    
    /**
      | Attach the specified value to the error
      | message if an error has occurred
      |
      */
    #[inline] fn shl(self, rhs: &V) -> Self::Output {
        todo!();
        /*
            if (!ok_) {
          ss_ << " " << value;
        }
        return *this;
        */
    }
}

macro_rules! assert_true {
    ($c:ident) => {
        /*
                ::leveldb::test::Tester(__FILE__, __LINE__).Is((c), #c)
        */
    }
}

macro_rules! assert_ok {
    ($s:ident) => {
        /*
                ::leveldb::test::Tester(__FILE__, __LINE__).IsOk((s))
        */
    }
}

macro_rules! assert_eq {
    ($a:ident, $b:ident) => {
        /*
        
          ::leveldb::test::Tester(__FILE__, __LINE__).IsEq((a), (b))
        */
    }
}

macro_rules! assert_ne {
    ($a:ident, $b:ident) => {
        /*
        
          ::leveldb::test::Tester(__FILE__, __LINE__).IsNe((a), (b))
        */
    }
}

macro_rules! assert_ge {
    ($a:ident, $b:ident) => {
        /*
        
          ::leveldb::test::Tester(__FILE__, __LINE__).IsGe((a), (b))
        */
    }
}

macro_rules! assert_gt {
    ($a:ident, $b:ident) => {
        /*
        
          ::leveldb::test::Tester(__FILE__, __LINE__).IsGt((a), (b))
        */
    }
}

macro_rules! assert_le {
    ($a:ident, $b:ident) => {
        /*
        
          ::leveldb::test::Tester(__FILE__, __LINE__).IsLe((a), (b))
        */
    }
}

macro_rules! assert_lt {
    ($a:ident, $b:ident) => {
        /*
        
          ::leveldb::test::Tester(__FILE__, __LINE__).IsLt((a), (b))
        */
    }
}

macro_rules! tconcat {
    ($a:ident, $b:ident) => {
        /*
                TCONCAT1(a, b)
        */
    }
}

macro_rules! tconcat1 {
    ($a:ident, $b:ident) => {
        /*
                a##b
        */
    }
}

macro_rules! test {
    ($base:ident, $name:ident) => {
        /*
        
          class TCONCAT(_Test_, name) : public base {                         
                                                                       
            c_void _Run();                                                      
            static c_void _RunIt() {                                            
              TCONCAT(_Test_, name) t;                                        
              t._Run();                                                       
            }                                                                 
          };                                                                  
          bool TCONCAT(_Test_ignored_, name) = ::leveldb::test::RegisterTest( 
              #base, #name, &TCONCAT(_Test_, name)::_RunIt);                  
          c_void TCONCAT(_Test_, name)::_Run()
        */
    }
}

pub struct Test {
    base: *const u8,
    name: *const u8,
    func: fn() -> c_void,
}

lazy_static!{
    /*
    std::vector<Test>* tests;
    */
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
        func: fn() -> c_void) -> bool {
    
    todo!();
        /*
            if (tests == nullptr) {
        tests = new std::vector<Test>;
      }
      Test t;
      t.base = base;
      t.name = name;
      t.func = func;
      tests->push_back(t);
      return true;
        */
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
    
    todo!();
        /*
            const char* matcher = getenv("LEVELDB_TESTS");

      int num = 0;
      if (tests != nullptr) {
        for (size_t i = 0; i < tests->size(); i++) {
          const Test& t = (*tests)[i];
          if (matcher != nullptr) {
            std::string name = t.base;
            name.push_back('.');
            name.append(t.name);
            if (strstr(name.c_str(), matcher) == nullptr) {
              continue;
            }
          }
          fprintf(stderr, "==== Test %s.%s\n", t.base, t.name);
          (*t.func)();
          ++num;
        }
      }
      fprintf(stderr, "==== PASSED %d tests\n", num);
      return 0;
        */
}

/**
  | Return the directory to use for temporary
  | storage.
  |
  */
pub fn tmp_dir() -> String {
    
    todo!();
        /*
            std::string dir;
      crate::Status s = Env::Default()->GetTestDirectory(&dir);
      ASSERT_TRUE(s.ok()) << s.ToString();
      return dir;
        */
}

/**
  | Return a randomization seed for this run.
  | Typically returns the same number on repeated
  | invocations of this binary, but automated runs
  | may be able to vary the seed.
  */
pub fn random_seed() -> i32 {
    
    todo!();
        /*
            const char* env = getenv("TEST_RANDOM_SEED");
      int result = (env != nullptr ? atoi(env) : 301);
      if (result <= 0) {
        result = 301;
      }
      return result;
        */
}
