// ---------------- [ File: bitcoinleveldb-dbtest/src/delay.rs ]
crate::ix!();

pub fn delay_milliseconds(millis: i32)  {

    tracing::trace!(
        target: "bitcoinleveldb_dbtest::delay",
        event  = "delay_milliseconds.entry",
        millis
    );

    if millis > 0 {
        // Prefer the LevelDB Env clock/scheduler surface when available.
        // This preserves the semantics of the original code (including test Env wrappers).
        //
        // Note: `<dyn Env>::default()` is expected to be provided by the environment crate
        // as an inherent method on the trait object, mirroring `Env::Default()` in C++.
        let env = PosixEnv::shared();
        let micros: i32 = millis.saturating_mul(1000);

        // Sleep is a side-effect boundary. We trace the derived duration.
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::delay",
            event  = "delay_milliseconds.sleep_for_microseconds",
            micros
        );

        env.borrow_mut().sleep_for_microseconds(micros);
    } else {
        tracing::trace!(
            target: "bitcoinleveldb_dbtest::delay",
            event  = "delay_milliseconds.noop",
            millis
        );
    }

    tracing::trace!(
        target: "bitcoinleveldb_dbtest::delay",
        event  = "delay_milliseconds.exit",
        millis
    );
}
