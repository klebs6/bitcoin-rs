// ---------------- [ File: bitcoin-epoch/src/marker.rs ]
crate::ix!();

#[derive(Default)]
pub struct EpochMarker {
    pub(crate) marker: u64,
}
