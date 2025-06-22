// ---------------- [ File: bitcoin-golombrice/src/gcs_filter_params.rs ]
crate::ix!();

pub type GcsFilterElement    = Vec<u8>;
pub type GcsFilterElementSet = HashSet<GcsFilterElement,ByteVectorHash>;

/// Filter parameter set (BIP‑158 §3.1).
#[derive(Debug, Clone, Getters, Builder)]
#[builder(setter(into), default)]
#[getset(get = "pub")]
pub struct GcsFilterParams {
    /// SipHash‑2‑4 key halves.
    siphash_k0: u64,
    siphash_k1: u64,
    /// Golomb–Rice coding parameter *P*.
    p: u8,
    /// Inverse false‑positive rate *M*.
    m: u32,
}

impl Default for GcsFilterParams {
    fn default() -> Self {
        Self {
            siphash_k0: 0,
            siphash_k1: 0,
            p:          0,  // C++ default
            m:          1,  // C++ default
        }
    }
}

impl GcsFilterParams {
    /// Parameter constructor mirroring the C++ `GCSFilter::Params` ctor.
    pub fn new(
        siphash_k0: Option<u64>,
        siphash_k1: Option<u64>,
        p:          Option<u8>,
        m:          Option<u32>,
    ) -> Self {
        let params = Self {
            siphash_k0: siphash_k0.unwrap_or(0),
            siphash_k1: siphash_k1.unwrap_or(0),
            p:          p.unwrap_or(0),
            m:          m.unwrap_or(1),
        };
        trace!(target: "gcsfilter", ?params, "constructed GcsFilterParams");
        params
    }
}
