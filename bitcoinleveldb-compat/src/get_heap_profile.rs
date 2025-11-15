// ---------------- [ File: bitcoinleveldb-compat/src/get_heap_profile.rs ]
crate::ix!();

/**
  | If heap profiling is not supported, returns
  | false.
  |
  | Else repeatedly calls (*func)(arg, data, n) and
  | then returns true.
  |
  | The concatenation of all "data[0,n-1]"
  | fragments is the heap profile.
  */
#[inline]
#[instrument(level = "trace", skip(func, arg))]
pub fn get_heap_profile(
    func: fn(
        _0: *mut c_void,
        _1: *const u8,
        _2: i32,
    ) -> c_void,
    arg: *mut c_void,
) -> bool {
    debug!(
        callback_ptr = ?(func as *const ()),
        arg_ptr = ?arg,
        "get_heap_profile: heap profiling not supported in this build"
    );
    let _ = func;
    let _ = arg;
    false
}
