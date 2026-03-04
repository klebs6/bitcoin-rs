// ---------------- [ File: bitcoinleveldb-dbimpl/src/test_hooks.rs ]
crate::ix!();

use signal_hook::iterator::Signals;
use bitcoin_imports::{SIGABRT, SIGSEGV, SIGILL, SIGBUS};

use std::sync::Once;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_appender::non_blocking::WorkerGuard;

static INIT: Once = Once::new();
static mut GUARD: Option<WorkerGuard> = None;

pub fn init_tracing_for_tests() {
    INIT.call_once(|| {
        let (writer, guard) = tracing_appender::non_blocking(std::io::stderr());

        unsafe {
            GUARD = Some(guard);
        }

        tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::from_default_env()
                    .add_directive("trace".parse().unwrap()),
            )
            .with_writer(writer)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_file(true)
            .with_line_number(true)
            .init();
    });
}

use std::panic;
use tracing::{error, info};

pub fn install_panic_tracing_hook() {
    panic::set_hook(Box::new(|info| {
        error!("panic occurred: {}", info);

        // Best-effort flush
        info!("flushing tracing on panic");
    }));
}

use std::thread;

pub fn install_abort_signal_handlers() {
    let mut signals = Signals::new(&[
        SIGABRT,
    ])
    .expect("signal registration failed");

    thread::spawn(move || {
        for signal in signals.forever() {
            error!("received fatal signal: {}", signal);

            // Give the tracing worker a chance to flush
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    });
}

pub fn init_test_runtime() {
    init_tracing_for_tests();
    install_panic_tracing_hook();
    install_abort_signal_handlers();
}
