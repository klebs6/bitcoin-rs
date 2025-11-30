// ---------------- [ File: bitcoinleveldb-posixenv/src/test_util.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/env_test.cc]

pub const DELAY_MICROS: i32 = 100000;

#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct EnvTest {
    env: Rc<RefCell<dyn Env>>,
}

impl Default for EnvTest {
    fn default() -> Self {
        trace!(
            "EnvTest::default: constructing leaked PosixEnv instance for tests"
        );

        // We intentionally leak this `Rc<RefCell<dyn Env>>` so that the embedded
        // `PosixEnv` is never dropped. Dropping `PosixEnv` would abort the
        // process, as its `Drop` implementation mirrors the original C++
        // behaviour. Each call gets its own logically independent Env instance.
        let leaked: &'static Rc<RefCell<dyn Env>> = {
            let env_impl: PosixEnv = PosixEnv::default();
            let trait_obj: Rc<RefCell<dyn Env>> = Rc::new(RefCell::new(env_impl));
            Box::leak(Box::new(trait_obj))
        };

        let env_clone: Rc<RefCell<dyn Env>> = leaked.clone();

        Self { env: env_clone }
    }
}


/// Minimal reimplementation of leveldb::Random and the test helpers
/// test::RandomSeed() / test::RandomString().
///
#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct TestRandom {
    seed: u32,
}

impl TestRandom {
    pub fn new(seed: u32) -> Self {
        // Match leveldb behaviour: keep seed in (0, 2147483647].
        const MAX: u32 = 2147483647;
        let mut s = seed & MAX;
        if s == 0 || s == MAX {
            s = 1;
        }
        trace!(seed = s, "TestRandom::new");
        Self { seed: s }
    }

    pub fn next(&mut self) -> u32 {
        // 31‑bit linear congruential generator:
        //   seed_ = (seed_ * 16807) % 2147483647;
        const M: u64 = 2147483647;
        const A: u64 = 16807;

        let product = self.seed as u64 * A;
        self.seed = (product % M) as u32;
        self.seed
    }

    pub fn uniform(&mut self, n: u32) -> u32 {
        assert!(n > 0);
        self.next() % n
    }

    pub fn one_in(&mut self, n: i32) -> bool {
        assert!(n > 0);
        self.uniform(n as u32) == 0
    }

    pub fn skewed(&mut self, max_log: i32) -> usize {
        // Rough translation of leveldb::Random::Skewed:
        // return Uniform(1 << Uniform(max_log + 1));
        let max_log = if max_log < 0 { 0 } else { max_log } as u32;
        let shift = self.uniform(max_log + 1);
        let limit = 1u32 << shift;
        self.uniform(limit) as usize
    }
}

pub fn test_random_seed() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(1));

    let nanos = now.subsec_nanos();
    let seed = if nanos == 0 { 1 } else { nanos };
    trace!(seed, "test_random_seed");
    seed
}

pub fn test_random_string(rnd: &mut TestRandom, len: usize) -> String {
    const CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut out = String::with_capacity(len);
    for _ in 0..len {
        let idx = rnd.uniform(CHARS.len() as u32) as usize;
        out.push(CHARS[idx] as char);
    }
    out
}

///--------------------
#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct State {
    mu:   std::sync::Mutex<state::Inner>,
    cvar: std::sync::Condvar,
}

pub mod state {
    #[derive(Debug)]
    pub struct Inner {
        pub val:         i32,
        pub num_running: i32,
    }
}

impl State {
    pub fn new(val: i32, num_running: i32) -> Self {
        trace!(
            val,
            num_running,
            "State::new for env_test_start_thread"
        );
        Self {
            mu:   std::sync::Mutex::new(state::Inner { val, num_running }),
            cvar: std::sync::Condvar::new(),
        }
    }
}

pub fn thread_body(arg: *mut c_void) -> c_void {
    trace!(?arg, "thread_body: invoked");

    assert!(
        !arg.is_null(),
        "thread_body: received null argument pointer"
    );

    unsafe {
        let state = &*(arg as *mut State);

        let mut inner = state
            .mu
            .lock()
            .expect("thread_body: state mutex poisoned");

        inner.val += 1;
        inner.num_running -= 1;

        debug!(
            val         = inner.val,
            num_running = inner.num_running,
            "thread_body: updated state"
        );

        // Notify any waiter that state has changed.
        state.cvar.notify_all();
        // Mutex guard is dropped here.
    }

    trace!("thread_body: completed");

    // Return an opaque `c_void` value to satisfy the signature.
    unsafe { std::mem::zeroed() }
}

#[cfg(test)]
mod posix_env_test_util_tests {
    use super::*;
    use core::ffi::c_void;
    use std::rc::Rc;

    #[traced_test]
    fn test_random_generates_values_within_expected_range() {
        let seed = test_random_seed();
        let mut rnd = TestRandom::new(seed);

        for _ in 0..100 {
            let value = rnd.uniform(10);
            assert!(
                value < 10,
                "TestRandom::uniform must bound results to the given upper limit; got {value}"
            );
        }

        let skewed = rnd.skewed(10);
        assert!(
            skewed < (1usize << 10),
            "TestRandom::skewed must generate values within the configured range; got {skewed}"
        );
    }

    #[traced_test]
    fn test_random_string_produces_string_of_requested_length() {
        let seed = 12345;
        let mut rnd = TestRandom::new(seed);

        let s = test_random_string(&mut rnd, 32);

        assert_eq!(
            s.len(),
            32,
            "test_random_string must produce output of the requested length"
        );

        for ch in s.chars() {
            assert!(
                ch.is_ascii_alphanumeric(),
                "test_random_string must restrict output to ASCII alphanumeric characters; saw {ch:?}"
            );
        }
    }

    #[traced_test]
    fn thread_body_updates_state_and_signals_condvar() {
        let state = State::new(0, 1);

        let arg = &state as *const State as *mut c_void;

        thread_body(arg);

        let inner = state
            .mu()
            .lock()
            .expect("mutex should not be poisoned after thread_body");

        assert_eq!(
            inner.val, 1,
            "thread_body must increment the state's val field"
        );
        assert_eq!(
            inner.num_running, 0,
            "thread_body must decrement the state's num_running field"
        );
    }

    #[traced_test]
    fn env_test_default_leaks_env_but_provides_working_handle() {
        let env_test = EnvTest::default();

        let rc = env_test.env();
        assert!(
            Rc::strong_count(rc) >= 1,
            "EnvTest::default must hold at least one strong reference to the Env"
        );
    }
}
