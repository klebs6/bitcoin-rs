crate::ix!();

/// Small helper: emit a `TRACE` event whose `step` field is fixed and all
/// remaining tokens are forwarded verbatim to `tracing::trace!`.  This lets
/// you write e.g.  
/// `trace_step!("c‑prop‑0", { c, h_before = h0, h_after = h[0] });`
#[macro_export]
macro_rules! trace_step {
    ($step:expr, { $($field:tt)* }) => {
        tracing::trace!(
            step = $step,
            $($field)*
        );
    };
}

