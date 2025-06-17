// ---------------- [ File: bitcoin-hash/src/out_point.rs ]
crate::ix!();

/// An outpoint - a combination of a transaction
/// hash and an index n into its vout
#[derive(Clone, Serialize, Deserialize, Getters, MutGetters)]
#[getset(get="pub")]
pub struct OutPoint {
    hash: u256,
    n:    u32,
}

pub const OUT_POINT_NULL_INDEX: u32 = u32::MAX;

impl OutPoint {
    
    #[instrument(level = "debug")]
    pub fn new(hash_in: &u256, n_in: u32) -> Self {
        Self {
            hash: hash_in.clone(),
            n:    n_in,
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub fn set_null(&mut self) {
        self.hash = u256::default();
        self.n = OUT_POINT_NULL_INDEX;
    }

    #[inline]
    #[instrument(level = "trace", skip(self))]
    pub fn is_null(&self) -> bool {
        self.hash == u256::default() && self.n == OUT_POINT_NULL_INDEX
    }

    /// Human‑readable representation identical to Bitcoin Core’s
    /// `OutPoint::ToString()`: the first ten hex digits of the txid
    /// plus the vout index.
    #[instrument(level = "trace", skip(self))]
    pub fn to_string(&self) -> String {
        let h = self.hash.to_string();           // 64‑char hex
        format!("OutPoint({}, {})", &h[..10], self.n)
    }
}

impl fmt::Display for OutPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let h = self.hash.to_string();
        write!(f, "OutPoint({}, {})", &h[..10], self.n)
    }
}

impl Default for OutPoint {
    #[instrument(level = "trace")]
    fn default() -> Self {
        Self {
            hash: u256::default(),
            n:    OUT_POINT_NULL_INDEX,
        }
    }
}

impl PartialEq for OutPoint {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash && self.n == other.n
    }
}

impl Eq for OutPoint {}

impl PartialOrd for OutPoint {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for OutPoint {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hash.cmp(&other.hash) {
            Ordering::Equal => self.n.cmp(&other.n),
            ord => ord,
        }
    }
}

impl Hash for OutPoint {
    #[instrument(level = "trace", skip(self, state))]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.hash.as_ref());
        state.write(&self.n.to_le_bytes());
    }
}

impl RecursiveDynamicUsage for OutPoint {
    #[inline]
    fn recursive_dynamic_usage(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod outpoint_spec {
    use super::*;

    #[traced_test]
    fn display_and_to_string_are_consistent() {
        let h = u256::one();
        let op = OutPoint::new(&h, 7);

        let via_fmt = format!("{}", op);
        let via_method = op.to_string();
        assert_eq!(via_fmt, via_method);

        assert!(via_method.starts_with("OutPoint("));
        assert!(via_method.contains(", 7)"));
    }
}
