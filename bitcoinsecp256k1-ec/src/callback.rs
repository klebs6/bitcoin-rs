crate::ix!();

#[cfg(not(USE_EXTERNAL_DEFAULT_CALLBACKS))]
pub fn default_illegal_callback_fn(
        str_: *const u8,
        data: *mut c_void)  {
    
    todo!();
        /*
            (c_void)data;
        fprintf(stderr, "[libsecp256k1] illegal argument: %s\n", str);
        abort();
        */
}

#[cfg(not(USE_EXTERNAL_DEFAULT_CALLBACKS))]
pub fn default_error_callback_fn(
        str_: *const u8,
        data: *mut c_void)  {
    
    todo!();
        /*
            (c_void)data;
        fprintf(stderr, "[libsecp256k1] internal consistency check failed: %s\n", str);
        abort();
        */
}

#[cfg(USE_EXTERNAL_DEFAULT_CALLBACKS)]
pub fn default_illegal_callback_fn(
        str_: *const u8,
        data: *mut c_void)  {
    
    todo!();
        /*
        
        */
}

#[cfg(USE_EXTERNAL_DEFAULT_CALLBACKS)]
pub fn default_error_callback_fn(
        str_: *const u8,
        data: *mut c_void)  {
    
    todo!();
        /*
        
        */
}

lazy_static!{
    /*
    static const callback default_illegal_callback = {
        default_illegal_callback_fn,
        NULL
    };
    */
}

lazy_static!{
    /*
    static const callback default_error_callback = {
        default_error_callback_fn,
        NULL
    };
    */
}


