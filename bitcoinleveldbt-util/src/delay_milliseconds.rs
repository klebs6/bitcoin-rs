// ---------------- [ File: bitcoinleveldbt-util/src/delay_milliseconds.rs ]
crate::ix!();

pub fn delay_milliseconds(millis: i32) {
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::delay",
        event  = "delay_milliseconds.entry",
        millis
    );

    if millis > 0 {
        std::thread::sleep(std::time::Duration::from_millis(millis as u64));
    }

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::delay",
        event  = "delay_milliseconds.exit",
        millis
    );
}
