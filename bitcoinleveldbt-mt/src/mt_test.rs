// ---------------- [ File: bitcoinleveldb-testmt/src/mt_test.rs ]
crate::ix!();

/* ------------- Multi-threaded test:  ------------- */

pub const NUM_THREADS:  usize = 4;
pub const TEST_SECONDS: usize = 10;
pub const NUM_KEYS:     usize = 1000;

pub struct MTState {
    test:        *mut DBTest,
    stop:        AtomicBool,
    counter:     [AtomicI32;  NUM_THREADS],
    thread_done: [AtomicBool; NUM_THREADS],
}

pub struct MTThread {
    state: *mut MTState,
    id:    i32,
}

/// Invariant: successful parses recover exactly the three decimal fields emitted by
/// `mt_thread_body` before any padding bytes.
///
/// Precondition: `value` is either arbitrary input or one of the strings produced by the writer path.
/// Postcondition: returns `Some((key, writer_id, counter))` iff the prefix matches `"%d.%d.%d"`.
pub fn dbtest_mt_parse_counter_value_prefix(value: &str) -> Option<(i32, i32, i32)> {
    tracing::trace!(
        target: "bitcoinleveldb_dbtest::mt_test",
        label = "dbtest_mt_parse_counter_value_prefix.entry",
        value_len = value.len()
    );

    let mut parts = value.splitn(3, '.');

    let parsed = match (parts.next(), parts.next(), parts.next()) {
        (Some(k_part), Some(w_part), Some(c_part)) => {
            let k_result = k_part.parse::<i32>();
            let w_result = w_part.parse::<i32>();
            let c_result = c_part.trim().parse::<i32>();

            match (k_result, w_result, c_result) {
                (Ok(k), Ok(w), Ok(c)) => Some((k, w, c)),
                (Ok(_), Ok(_), Err(_)) => None,
                (Ok(_), Err(_), Ok(_)) => None,
                (Ok(_), Err(_), Err(_)) => None,
                (Err(_), Ok(_), Ok(_)) => None,
                (Err(_), Ok(_), Err(_)) => None,
                (Err(_), Err(_), Ok(_)) => None,
                (Err(_), Err(_), Err(_)) => None,
            }
        }
        (Some(_), Some(_), None) => None,
        (Some(_), None, Some(_)) => None,
        (Some(_), None, None) => None,
        (None, Some(_), Some(_)) => None,
        (None, Some(_), None) => None,
        (None, None, Some(_)) => None,
        (None, None, None) => None,
    };

    tracing::trace!(
        target: "bitcoinleveldb_dbtest::mt_test",
        label = "dbtest_mt_parse_counter_value_prefix.exit",
        parsed = parsed.is_some()
    );

    parsed
}

pub fn mt_thread_body(arg: *mut c_void)  -> c_void {
    tracing::trace!(
        target: "bitcoinleveldb_dbtest::mt_test",
        label = "mt_thread_body.entry",
        arg_is_null = arg.is_null()
    );

    unsafe {
        let t = arg as *mut MTThread;
        assert!(!t.is_null());

        let id = (*t).id;
        assert!(id >= 0);

        let id_usize = id as usize;
        assert!(id_usize < NUM_THREADS);

        let state = (*t).state;
        assert!(!state.is_null());

        let dbtest_ptr = (*state).test;
        assert!(!dbtest_ptr.is_null());

        let db = (*dbtest_ptr).dbfull();
        assert!(!db.is_null());

        let mut counter: i32 = 0;
        eprintln!("... starting thread {}", id);

        let mut rnd = Random::new(1000 + (id as u32));
        let mut value = String::new();

        while !(*state).stop.load(atomic::Ordering::Acquire) {
            (*state).counter[id_usize].store(counter, atomic::Ordering::Release);

            let rand_key = rnd.uniform(NUM_KEYS as i32) as i32;
            let keybuf = format!("{:016}", rand_key);

            if rnd.one_in(2) {
                // Write values of the form <rand_key, my id, counter>.
                // We add some padding for force compactions.
                let valbuf = format!("{}.{}.{:<1000}", rand_key, id, counter);
                let write_options = WriteOptions::default();
                let key_slice = Slice::from(&keybuf);
                let value_slice = Slice::from(&valbuf);
                let s = (*db).put(&write_options, &key_slice, &value_slice);
                assert!(s.is_ok());
            } else {
                // Read a value and verify that it matches the pattern written above.
                value.clear();
                let read_options = ReadOptions::default();
                let key_slice = Slice::from(&keybuf);
                let s = (*db).get(&read_options, &key_slice, &mut value as *mut String);

                if s.is_not_found() {
                    // Key has not yet been written
                } else {
                    assert!(s.is_ok());

                    let parsed = dbtest_mt_parse_counter_value_prefix(&value);
                    assert!(parsed.is_some());

                    let (k, w, c) = match parsed {
                        Some(tuple) => tuple,
                        None => (0, 0, 0),
                    };

                    assert_eq!(k, rand_key);
                    assert!(w >= 0);
                    assert!(w < NUM_THREADS as i32);

                    let w_usize = w as usize;
                    assert!(c <= (*state).counter[w_usize].load(atomic::Ordering::Acquire));
                }
            }

            counter += 1;
        }

        (*state).thread_done[id_usize].store(true, atomic::Ordering::Release);
        eprintln!("... stopping thread {} after {} ops", id, counter);
    }

    tracing::trace!(
        target: "bitcoinleveldb_dbtest::mt_test",
        label = "mt_thread_body.exit"
    );

    unsafe { std::mem::zeroed() }
}

#[traced_test]
fn db_test_multi_threaded() {
    let mut body = |dbtest: &mut DBTest| {
        let mut mt = MTState {
            test: dbtest as *mut DBTest,
            stop: AtomicBool::new(false),
            counter: [
                AtomicI32::new(0),
                AtomicI32::new(0),
                AtomicI32::new(0),
                AtomicI32::new(0),
            ],
            thread_done: [
                AtomicBool::new(false),
                AtomicBool::new(false),
                AtomicBool::new(false),
                AtomicBool::new(false),
            ],
        };

        let mut threads = [
            MTThread {
                state: &mut mt as *mut MTState,
                id: 0,
            },
            MTThread {
                state: &mut mt as *mut MTState,
                id: 1,
            },
            MTThread {
                state: &mut mt as *mut MTState,
                id: 2,
            },
            MTThread {
                state: &mut mt as *mut MTState,
                id: 3,
            },
        ];

        let env = dbtest.special_env();
        assert!(!env.is_null());

        let mut id: usize = 0;
        while id < NUM_THREADS {
            unsafe {
                (*env)
                    .base_mut()
                    .start_thread(mt_thread_body, (&mut threads[id]) as *mut MTThread as *mut c_void);
            }
            id += 1;
        }

        delay_milliseconds((TEST_SECONDS * 1000) as i32);

        mt.stop.store(true, atomic::Ordering::Release);

        let mut wait_id: usize = 0;
        while wait_id < NUM_THREADS {
            while !mt.thread_done[wait_id].load(atomic::Ordering::Acquire) {
                delay_milliseconds(100);
            }
            wait_id += 1;
        }
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}
