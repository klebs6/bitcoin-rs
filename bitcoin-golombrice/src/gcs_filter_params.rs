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
            p: 19,
            m: 784_931, // values per BIP‑158 for basic block filters
        }
    }
}

impl GcsFilterParams {
    /// Parameter constructor mirroring the C++ `GcsFilterParams` ctor.
    pub fn new(
        siphash_k0: Option<u64>,
        siphash_k1: Option<u64>,
        p:          Option<u8>,
        m:          Option<u32>,
    ) -> Self {
        let params = Self {
            siphash_k0: siphash_k0.unwrap_or(0),
            siphash_k1: siphash_k1.unwrap_or(0),
            p:          p.unwrap_or(19),
            m:          m.unwrap_or(784_931),
        };
        trace!(target: "gcsfilter", ?params, "constructed GcsFilterParams");
        params
    }
}
