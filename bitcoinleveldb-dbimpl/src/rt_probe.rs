crate::ix!();

// NOTE: can reenable this in the future if we ever need it.
// this function is useful when debugging a hang, for exam,ple.
#[cfg(test)]
#[macro_export]
macro_rules! bitcoinleveldb_dbimpl_realtime_probe_20260303 {
    ($label:expr, $($arg:tt)*) => {{ }};
}

#[disable]
#[cfg(test)]
#[macro_export]
macro_rules! bitcoinleveldb_dbimpl_realtime_probe_20260303 {
    ($label:expr, $($arg:tt)*) => {{
        let _ = $crate::bitcoinleveldb_dbimpl_realtime_tracing_subscriber_install_20260303();

        let __pid: u32 = std::process::id();
        let __tid = std::thread::current().id();

        let __ts_ms: u128 = match std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
        {
            Ok(d) => d.as_millis(),
            Err(_e) => 0u128,
        };

        let __args = format_args!($($arg)*);
        let __line = format_args!(
            "bitcoinleveldb-dbimpl realtime ts_ms={} pid={} tid={:?} file={} line={} label={} {}",
            __ts_ms,
            __pid,
            __tid,
            file!(),
            line!(),
            $label,
            __args
        );

        #[cfg(unix)]
        {
            let __tty_res = std::fs::OpenOptions::new().write(true).open("/dev/tty");
            match __tty_res {
                Ok(mut __tty) => {
                    let _ = std::io::Write::write_fmt(&mut __tty, __line);
                    let _ = std::io::Write::write_all(&mut __tty, b"\n");
                    let _ = std::io::Write::flush(&mut __tty);
                }
                Err(_e) => {}
            }

            let __file_res = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open($crate::BITCOINLEVELDB_DBIMPL_REALTIME_TRACING_LOG_PATH_20260303);
            match __file_res {
                Ok(mut __file) => {
                    let _ = std::io::Write::write_fmt(&mut __file, __line);
                    let _ = std::io::Write::write_all(&mut __file, b"\n");
                    let _ = std::io::Write::flush(&mut __file);
                }
                Err(_e) => {}
            }
        }

        eprintln!("{}", __line);
    }};
}

#[cfg(test)]
pub const BITCOINLEVELDB_DBIMPL_REALTIME_TRACING_LOG_PATH_20260303: &str =
    "/tmp/bitcoinleveldb-dbimpl-realtime-tracing.log";

#[cfg(test)]
#[derive(Debug, Clone, Default)]
pub struct BitcoinLeveldbDbimplRealtimeTracingTtyMakeWriter20260303;

#[cfg(test)]
#[derive(Debug, Clone, Default)]
pub struct BitcoinLeveldbDbimplRealtimeTracingTtyWriter20260303;

#[cfg(test)]
impl std::io::Write for BitcoinLeveldbDbimplRealtimeTracingTtyWriter20260303 {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        #[cfg(unix)]
        {
            let __tty_res = std::fs::OpenOptions::new().write(true).open("/dev/tty");
            match __tty_res {
                Ok(mut __tty) => {
                    let _ = std::io::Write::write_all(&mut __tty, buf);
                    let _ = std::io::Write::flush(&mut __tty);
                }
                Err(_e) => {}
            }

            let __file_res = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(BITCOINLEVELDB_DBIMPL_REALTIME_TRACING_LOG_PATH_20260303);
            match __file_res {
                Ok(mut __file) => {
                    let _ = std::io::Write::write_all(&mut __file, buf);
                    let _ = std::io::Write::flush(&mut __file);
                }
                Err(_e) => {}
            }
        }

        let _ = std::io::Write::write_all(&mut std::io::stderr(), buf);
        let _ = std::io::Write::flush(&mut std::io::stderr());

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
impl<'a> tracing_subscriber::fmt::MakeWriter<'a>
    for BitcoinLeveldbDbimplRealtimeTracingTtyMakeWriter20260303
{
    type Writer = BitcoinLeveldbDbimplRealtimeTracingTtyWriter20260303;

    fn make_writer(&'a self) -> Self::Writer {
        BitcoinLeveldbDbimplRealtimeTracingTtyWriter20260303::default()
    }
}

#[cfg(test)]
pub fn bitcoinleveldb_dbimpl_realtime_tracing_subscriber_install_20260303() -> bool {
    static BITCOINLEVELDB_DBIMPL_REALTIME_TRACING_ONCE_20260303: std::sync::Once =
        std::sync::Once::new();
    static BITCOINLEVELDB_DBIMPL_REALTIME_TRACING_INSTALLED_20260303: std::sync::atomic::AtomicBool =
        std::sync::atomic::AtomicBool::new(false);

    BITCOINLEVELDB_DBIMPL_REALTIME_TRACING_ONCE_20260303.call_once(|| {
        let __filter = match tracing_subscriber::EnvFilter::try_from_default_env() {
            Ok(f) => f,
            Err(_e) => tracing_subscriber::EnvFilter::new("debug"),
        };

        let __builder = tracing_subscriber::fmt()
            .with_env_filter(__filter)
            .with_writer(BitcoinLeveldbDbimplRealtimeTracingTtyMakeWriter20260303::default())
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_file(true)
            .with_line_number(true)
            .with_level(true);

        let __init_res = __builder.try_init();
        match __init_res {
            Ok(()) => {
                BITCOINLEVELDB_DBIMPL_REALTIME_TRACING_INSTALLED_20260303
                    .store(true, std::sync::atomic::Ordering::SeqCst);
            }
            Err(_e) => {
                BITCOINLEVELDB_DBIMPL_REALTIME_TRACING_INSTALLED_20260303
                    .store(false, std::sync::atomic::Ordering::SeqCst);
            }
        }
    });

    BITCOINLEVELDB_DBIMPL_REALTIME_TRACING_INSTALLED_20260303
        .load(std::sync::atomic::Ordering::SeqCst)
}

