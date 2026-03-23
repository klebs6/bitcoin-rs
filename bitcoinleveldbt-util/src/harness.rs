// ---------------- [ File: bitcoinleveldbt-util/src/harness.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/testharness.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/testharness.cc]

/**
  | An instance of Tester is allocated to
  | hold temporary state during the execution
  | of an assertion.
  |
  */
#[derive(Getters,Builder)]
#[getset(get="pub")]
#[builder(setter(into))]
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
                target: "bitcoinleveldbt_util::harness",
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
            target: "bitcoinleveldbt_util::harness",
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
            target: "bitcoinleveldbt_util::harness",
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

#[derive(Clone,Getters,Builder)]
#[getset(get="pub")]
#[builder(setter(into))]
pub struct Test {
    base: String,
    name: String,
    func: fn(),
}

lazy_static!{
    pub static ref BITCOINLEVELDB_TEST_HARNESS_REGISTERED_TESTS: Mutex<Vec<Test>> =
        Mutex::new(Vec::new());
}
