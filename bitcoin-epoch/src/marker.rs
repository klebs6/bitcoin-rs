// ---------------- [ File: bitcoin-epoch/src/marker.rs ]
crate::ix!();

/// Per‑transaction epoch marker.
#[derive(Getters, Builder, Default)]
#[getset(get = "pub")]
pub struct EpochMarker {
    marker: u64,
}

impl EpochMarker {
    /// Update the marker; crate‑internal.
    pub(crate) fn update(&mut self, value: u64) {
        self.marker = value;
    }
}
