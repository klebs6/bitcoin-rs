// ---------------- [ File: bitcoinleveldb-harness/src/harness.rs ]
crate::ix!();

pub struct Harness {
    pub(crate) options:     Options,
    pub(crate) constructor: *mut Constructor,
}

impl Default for Harness {
    fn default() -> Self {
        Self {
            options:     Options::default(),
            constructor: 0 as *mut Constructor,
        }
    }
}

impl Drop for Harness {
    fn drop(&mut self) {
        debug!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.harness.drop.entry",
            constructor_tagged = (self.constructor as usize),
        );

        self.bitcoinleveldb_harness_dispose_constructor_pointer("drop");

        debug!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.harness.drop.exit",
        );
    }
}
