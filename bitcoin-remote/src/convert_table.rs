// ---------------- [ File: bitcoin-remote/src/convert_table.rs ]
crate::ix!();

/// Lookup helper for determining whether a given RPC parameter (identified
/// either by positional *index* or by *name*) must be converted from its
/// textual CLI form to native JSON for the underlying RPC call.
///
/// The two internal `HashSet`s are built **once** at initialization from the
/// canonical `vRPCConvertParams` table generated directly from Bitcoin Core.
/// No mutation occurs afterwards, so external callers only need immutable
/// accessors.
///
/// Robust logging (via the `tracing` crate) is provided at strategic points to
/// aid troubleshooting without polluting the hot path.
///
#[derive(Debug, Getters, MutGetters, Setters, Default, Builder)]
#[builder(pattern = "owned", setter(into), default)]
#[getset(get="pub")]
pub struct RPCConvertTable {
    /// `(method, positional‑index)` pairs needing
    /// conversion (e.g. `("setmocktime", 0)`).
    members: HashSet<(String, i32)>,

    /// `(method, parameter‑name)` pairs needing
    /// conversion (e.g. `("setmocktime", "timestamp")`).
    members_by_name: HashSet<(String, String)>,
}

impl RPCConvertTable {
    /// Build a fully‑populated conversion table using the
    /// static list emitted above.
    pub fn new() -> Self {
        let mut members = HashSet::<(String, i32)>::with_capacity(vRPCConvertParams.len());
        let mut members_by_name =
            HashSet::<(String, String)>::with_capacity(vRPCConvertParams.len());

        for cp in vRPCConvertParams {
            members.insert((cp.method_name().to_owned().to_string(), *cp.param_idx()));
            members_by_name.insert((cp.method_name().to_owned().to_string(), cp.param_name().to_owned().to_string()));
        }

        debug!(
            table_size_idx = members.len(),
            table_size_name = members_by_name.len(),
            "RPCConvertTable initialised"
        );

        Self {
            members,
            members_by_name,
        }
    }

    /// Does *(`method`, `idx`)* require JSON conversion?
    ///
    /// A mutable receiver is accepted for parity with the
    /// original C++ signature; however, no mutation is
    /// performed.
    pub fn convert_with_method_and_idx(&mut self, method: &str, idx: i32) -> bool {
        let hit = self.members.contains(&(method.to_owned(), idx));
        trace!(method, idx, hit, "idx‑based lookup");
        hit
    }

    /// Does *(`method`, `name`)* require JSON conversion?
    ///
    /// A mutable receiver is accepted for parity with the
    /// original C++ signature; however, no mutation is
    /// performed.
    pub fn convert_with_method_and_name(&mut self, method: &str, name: &str) -> bool {
        let hit = self
            .members_by_name
            .contains(&(method.to_owned(), name.to_owned()));
        trace!(method, name, hit, "name‑based lookup");
        hit
    }
}

lazy_static! {
    /// Global, lazily‑initialised conversion table
    /// protected by a `Mutex` for thread‑safe access.
    pub static ref RPC_CVT_TABLE: std::sync::Mutex<RPCConvertTable> =
        std::sync::Mutex::new(RPCConvertTable::new());
}

#[cfg(test)]
mod tests_convert_table {
    use super::*;

    #[traced_test]
    fn finds_known_param_by_idx() {
        let mut tbl = RPCConvertTable::new();
        assert!(
            tbl.convert_with_method_and_idx("setmocktime", 0),
            "Expected positional parameter to be convertible"
        );
    }

    #[traced_test]
    fn finds_known_param_by_name() {
        let mut tbl = RPCConvertTable::new();
        assert!(
            tbl.convert_with_method_and_name("setmocktime", "timestamp"),
            "Expected named parameter to be convertible"
        );
    }

    #[traced_test]
    fn rejects_unknown_param() {
        let mut tbl = RPCConvertTable::new();
        assert!(
            !tbl.convert_with_method_and_idx("does_not_exist", 42),
            "Unknown positional parameter should not be convertible"
        );
        assert!(
            !tbl.convert_with_method_and_name("does_not_exist", "never_happens"),
            "Unknown named parameter should not be convertible"
        );
    }
}
