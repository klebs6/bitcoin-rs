crate::ix!();

pub fn print_number(x: f64)  {
    
    let mut y: f64 = x;

    let mut c: i32 = 0;

    if y < 0.0 {
        y = -y;
    }

    while y < 100.0 {
        y *= 10.0;

        c += 1;
    }

    unsafe {
        libc::printf(
            "%.*f".as_ptr() as *const i8, 
            c, 
            x
        );
    }
}
