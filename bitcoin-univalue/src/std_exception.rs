// ---------------- [ File: bitcoin-univalue/src/std_exception.rs ]
/// Minimal stand‑in so we can faithfully mirror the upstream
/// C++ exception‑based API without adopting `std::error::Error`
/// plumbing throughout the translated crate just yet.
#[derive(Debug, Clone)]
pub struct StdException(pub String);

impl std::fmt::Display for StdException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for StdException {}

#[inline]
pub fn runtime_error<T: Into<String>>(msg: T) -> StdException {
    StdException(msg.into())
}
