crate::ix!();

pub fn parse_double(
        x:   &String,
        out: *mut f64) -> bool {
    
    if !parse_prechecks(x) {
        return false;
    }

    let first  = x.chars().nth(0).unwrap();
    let second = x.chars().nth(1).unwrap();

    if x.len() >= 2 && first == '0' && second == 'x' {

        //  No hexadecimal floats allowed
        return false;
    }

    let result: Result<f64,_> = x.parse::<f64>();

    if out != std::ptr::null_mut() {

        unsafe {

            if let Ok(result) = result {
                *out = result;

            } else {
                *out = 0.0;
            }
        }
    }

    result.is_ok()
}
