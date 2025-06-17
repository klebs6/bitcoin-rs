crate::ix!();

pub fn gettimedouble() -> f64 {
    
    let mut tv: libc::timeval = unsafe { std::mem::zeroed() };

    unsafe {
        libc::gettimeofday(&mut tv as *mut libc::timeval, null_mut());
    }

    (tv.tv_usec as f64) * 0.000001_f64 + (tv.tv_sec as f64)
}
