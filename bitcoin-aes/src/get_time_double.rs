// ---------------- [ File: bitcoin-aes/src/get_time_double.rs ]
crate::ix!();

pub fn gettimedouble() -> f64 {
    
    let mut tv: libc::timeval = unsafe { std::mem::zeroed() };

    unsafe {
        libc::gettimeofday(&mut tv as *mut libc::timeval, null_mut());
    }

    (tv.tv_usec as f64) * 0.000001_f64 + (tv.tv_sec as f64)
}

#[cfg(test)]
mod monotonic_time_validation {
    use super::*;

    /// `gettimedouble` should be **monotonic non‑decreasing** within the same
    /// process.
    #[traced_test]
    fn time_value_never_goes_backwards() {
        let first = gettimedouble();
        let second = gettimedouble();
        assert!(
            second >= first,
            "time regressed: first = {first}, second = {second}"
        );
    }
}
