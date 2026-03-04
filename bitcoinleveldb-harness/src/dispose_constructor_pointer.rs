// ---------------- [ File: bitcoinleveldb-harness/src/dispose_constructor_pointer.rs ]
crate::ix!();

impl Harness {
    /// Disposes the currently held tagged constructor pointer (if any) and resets it to null.
    ///
    /// Invariant: this function must never panic; it is used by `Drop`.
    pub fn bitcoinleveldb_harness_dispose_constructor_pointer(&mut self, reason: &'static str) {
        const TAG_MASK: usize = BITCOINLEVELDB_HARNESS_CONSTRUCTOR_PTR_TAG_MASK;

        let tagged: usize = self.constructor as usize;
        let tag: usize = tagged & TAG_MASK;
        let raw: *mut Constructor = (tagged & !TAG_MASK) as *mut Constructor;

        if raw.is_null() {
            self.constructor = 0 as *mut Constructor;
            debug!(
                target: "bitcoinleveldb_harness",
                label = "bitcoinleveldb_harness.harness.constructor.dispose.noop",
                reason = reason,
            );
            return;
        }

        debug!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.harness.constructor.dispose.begin",
            reason = reason,
            tag = tag,
            tag_label = bitcoinleveldb_harness_constructor_tag_machine_label(tag),
            raw_ptr = (raw as usize),
        );

        unsafe {
            match tag {
                0 => {
                    drop(Box::from_raw(raw as *mut TableConstructor));
                }
                1 => {
                    drop(Box::from_raw(raw as *mut BlockConstructor));
                }
                2 => {
                    drop(Box::from_raw(raw as *mut MemTableConstructor));
                }
                3 => {
                    drop(Box::from_raw(raw as *mut DBConstructor));
                }
                _ => {
                    // Invariant: never panic in Drop paths. If tag is corrupted, we cannot safely
                    // recover the concrete type. Leak rather than panic or invoke UB.
                    error!(
                        target: "bitcoinleveldb_harness",
                        label = "bitcoinleveldb_harness.harness.constructor.dispose.invalid_tag",
                        reason = reason,
                        tag = tag,
                        constructor_tagged = tagged,
                        raw_ptr = (raw as usize),
                    );
                }
            }
        }

        self.constructor = 0 as *mut Constructor;

        debug!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.harness.constructor.dispose.end",
            reason = reason,
        );
    }
}
