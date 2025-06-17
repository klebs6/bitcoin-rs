// ---------------- [ File: bitcoin-support/src/events.rs ]
crate::ix!();

/// Platform‑independent alias for a socket descriptor in libevent.
pub type EvutilSocket = i32;

/* --------------------------------------------------------------------------
   Declare opaque RAII types **without** colliding with `tracing::Event`.
---------------------------------------------------------------------------*/
macro_rules! declare_event_type {
    ($name:ident, $counter:ident) => {
        static $counter: AtomicUsize = AtomicUsize::new(0);

        #[derive(Debug)]
        pub struct $name {
            id: usize,
        }

        impl $name {
            fn new() -> Self {
                let id = $counter.fetch_add(1, atomic::Ordering::SeqCst) + 1;
                info!(target: "events", ty = stringify!($name), %id, "created");
                Self { id }
            }

            #[cfg(test)]
            pub fn live() -> usize {
                $counter.load(atomic::Ordering::SeqCst)
            }
        }

        impl Default for $name {
            fn default() -> Self { Self::new() }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                info!(target: "events", ty = stringify!($name), id = self.id, "dropped");
                unsafe {
                    memory_cleanse(self as *mut _ as *mut c_void, core::mem::size_of::<Self>());
                }
                $counter.fetch_sub(1, atomic::Ordering::SeqCst);
            }
        }
    };
}

// Create all required opaque handles.
declare_event_type!(EventBase,        EVENT_BASE_COUNT);
declare_event_type!(LibEvent,         EVENT_COUNT);          // was `Event`
declare_event_type!(EvHttp,           EVHTTP_COUNT);
declare_event_type!(EvHttpRequest,    EVHTTP_REQ_COUNT);
declare_event_type!(EvHttpConnection, EVHTTP_CONN_COUNT);

/* --------------------------------------------------------------------------
   Factory helpers – safe Rust stand‑ins for the original C++ RAII creators.
---------------------------------------------------------------------------*/

#[instrument(level = "trace")]
pub fn obtain_event_base() -> Box<EventBase> {
    Box::new(EventBase::default())
}

/// Callback signatures keep C ABI parity but are dummies for now.
pub type EventCallback = unsafe extern "C" fn(*mut LibEvent, *mut c_void);

#[instrument(level = "trace", skip(_cb, _arg))]
pub fn obtain_event(
    _base:   *mut EventBase,
    _s:      EvutilSocket,
    _events: i16,
    _cb:     Option<EventCallback>,
    _arg:    *mut c_void,
) -> Box<LibEvent> {
    Box::new(LibEvent::default())
}

#[instrument(level = "trace", skip(_base))]
pub fn obtain_evhttp(_base: *mut EventBase) -> Box<EvHttp> {
    Box::new(EvHttp::default())
}

pub type EvHttpRequestCallback = unsafe extern "C" fn(*mut EvHttpRequest, *mut c_void);

#[instrument(level = "trace", skip(_cb, _arg))]
pub fn obtain_evhttp_request(
    _cb:  Option<EvHttpRequestCallback>,
    _arg: *mut c_void,
) -> Box<EvHttpRequest> {
    Box::new(EvHttpRequest::default())
}

#[instrument(level = "trace", skip(_base, _host))]
pub fn obtain_evhttp_connection_base(
    _base: *mut EventBase,
    _host: &str,
    _port: u16,
) -> Box<EvHttpConnection> {
    trace!("creating connection to {}:{}", _host, _port);
    Box::new(EvHttpConnection::default())
}

/* --------------------------------------------------------------------------
                                   Tests
---------------------------------------------------------------------------*/
#[cfg(test)]
mod bitcoin_support_tests {
    use super::*;
    use parking_lot::Mutex;

    /// Prevent concurrent execution of these tests so the global
    /// live‑object counters remain deterministic.
    lazy_static! {
        static ref TEST_LOCK: Mutex<()> = Mutex::new(());
    }

    #[traced_test]
    fn test_event_base_lifecycle() {
        let _guard = TEST_LOCK.lock();

        let start = EventBase::live();
        {
            let _eb = obtain_event_base();
            assert_eq!(EventBase::live(), start + 1);
        }
        assert_eq!(EventBase::live(), start);
    }

    #[traced_test]
    fn test_event_lifecycle() {
        let _guard = TEST_LOCK.lock();

        let base = obtain_event_base();
        let base_ptr = Box::into_raw(base);
        let start = LibEvent::live();

        {
            let _ev = obtain_event(base_ptr, 0, 0, None, core::ptr::null_mut());
            assert_eq!(LibEvent::live(), start + 1);
        }
        assert_eq!(LibEvent::live(), start);

        // Safety: reclaim ownership so the EventBase drops.
        unsafe { drop(Box::from_raw(base_ptr)); }
    }

    #[traced_test]
    fn test_evhttp_family_lifecycle() {
        let _guard = TEST_LOCK.lock();

        let eb   = obtain_event_base();
        let base = &*eb as *const EventBase as *mut EventBase;

        let http0 = EvHttp::live();
        let req0  = EvHttpRequest::live();
        let conn0 = EvHttpConnection::live();

        {
            let _http = obtain_evhttp(base);
            let _req  = obtain_evhttp_request(None, core::ptr::null_mut());
            let _conn = obtain_evhttp_connection_base(base, "localhost", 80);

            assert_eq!(EvHttp::live(),           http0 + 1);
            assert_eq!(EvHttpRequest::live(),    req0  + 1);
            assert_eq!(EvHttpConnection::live(), conn0 + 1);
        }
        assert_eq!(EvHttp::live(),           http0);
        assert_eq!(EvHttpRequest::live(),    req0);
        assert_eq!(EvHttpConnection::live(), conn0);
    }
}
