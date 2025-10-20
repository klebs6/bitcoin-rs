// ---------------- [ File: bitcoin-random/src/state.rs ]
crate::ix!();

/**
  | The RNG state consists of 256 bits of
  | entropy, taken from the output of one
  | operation's SHA512 output, and fed
  | as input to the next one.
  | 
  | Carrying 256 bits of entropy should
  | be sufficient to guarantee unpredictability
  | as long as any entropy source was ever
  | unpredictable to an attacker. To protect
  | against situations where an attacker
  | might observe the RNG's state, fresh
  | entropy is always mixed when
  | 
  | GetStrongRandBytes is called.
  |
  */
pub struct RNGInnerState {
    state:           [u8; 32],
    counter:         u64,
    strongly_seeded: bool,
}

impl Default for RNGInnerState {
    fn default() -> Self {
        Self {
            state:           [0; 32],
            counter:         0,
            strongly_seeded: false,
        }
    }
}

#[derive(Default)]
pub struct RNGStateEvents {
    hasher: Sha256,
}

//--------------------------
pub struct RNGState {
    inner:  Mutex<RNGInnerState>,
    events: Mutex<RNGStateEvents>,
}

impl Default for RNGState {
    
    fn default() -> Self {

        init_hardware_rand();

        Self {
            inner:  Mutex::new(RNGInnerState::default()),
            events: Mutex::new(RNGStateEvents::default()),
        }
    }
}

impl RNGState {
    
    pub fn add_event(&mut self, event_info: u32)  {

        let mut events = self.events.lock();

        events.hasher.write_ptr(
            &event_info as *const _ as *const u8, 
            size_of_val(&event_info)
        );

        /*
        | Get the low four bytes of the
        | performance counter. This
        | translates to roughly the subsecond
        | part.
        */
        let perfcounter: u32 
        = (get_performance_counter() & 0xffffffff).try_into().unwrap();

        events.hasher.write_ptr(
            &perfcounter as *const _ as *const u8, 
            size_of_val(&perfcounter)
        );
    }

    /**
      | Feed (the hash of) all events added through
      | AddEvent() to hasher.
      |
      */
    pub fn seed_events(&mut self, hasher: &mut Sha512)  {

        /*
          | We use only Sha256 for the events
          | hashing to get the ASM speedups we have
          | for Sha256, since we want it to be fast
          | as network peers may be able to trigger
          | it repeatedly.
          */
        let mut events = self.events.lock();

        let mut events_hash: [u8; 32] = [0; 32];

        events.hasher.finalize(&mut events_hash);
        events.hasher.write_ptr(events_hash.as_mut_ptr(), 32);

        // Re-initialize the hasher with the finalized state to use later.
        events.hasher.reset();
        events.hasher.write_ptr(events_hash.as_mut_ptr(), 32);
    }

    /**
      | Extract up to 32 bytes of entropy from
      | the RNG state, mixing in new entropy
      | from hasher.
      | 
      | If this function has never been called
      | with strong_seed = true, false is returned.
      |
      */
    pub fn mix_extract(&mut self, 
        out:         &mut [u8],
        num:         usize,
        mut hasher:  Sha512,
        strong_seed: bool) -> bool {

        assert!{num <= 32};

        type BufType = [u8; 64];

        //Buffer needs to have hasher's output size
        const_assert!{
            size_of::<BufType>() == SHA512_OUTPUT_SIZE, 
        };

        let mut buf: BufType = [0; 64];

        let mut ret: bool = false;

        {
            let mut inner = self.inner.lock();

            inner.strongly_seeded |= strong_seed;

            ret = inner.strongly_seeded;

            /*
              | Write the current state of the RNG into
              | the hasher
              |
              */
            hasher.write(
                inner.state.as_ptr(), 
                32
            );

            /*
              | Write a new counter number into the state
              |
              */
            hasher.write(
                &mut inner.counter as *mut u64 as *mut u8, 
                size_of_val(&inner.counter)
            );

            inner.counter += 1;

            // Finalize the hasher
            hasher.finalize(&mut buf);

            unsafe {

                /*
                  | Store the last 32 bytes of the
                  | hash output as new RNG state.
                  |
                  */
                libc::memcpy(
                    inner.state.as_mut_ptr() as *mut c_void, 
                    buf.as_ptr().add(32) as *const c_void, 
                    32
                );
            }
        }

        /*
           | If desired, copy (up to) the first 32
           | bytes of the hash output as output.
           |
           */
        if num != 0 { 

            assert!{out.len() != 0};

            unsafe {
                libc::memcpy(
                    out.as_mut_ptr() as *mut c_void, 
                    buf.as_ptr() as *const c_void, 
                    num
                );
            }
        }

        // Best effort cleanup of internal state
        hasher.reset();

        memory_cleanse(buf.as_mut_ptr() as *mut c_void, 64);

        ret
    }
}

lazy_static!{
    pub static ref G_RNG: Arc<Mutex<Box<RNGState,SecureAllocator>>> 
        = Arc::new(
            Mutex::new(
                Box::new_in(RNGState::default(), SECURE_ALLOCATOR.clone())
            )
        );
}
