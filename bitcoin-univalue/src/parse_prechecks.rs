crate::ix!();

pub fn parse_prechecks(x: &String) -> bool {
    
    if x.is_empty() {
        //  No empty string allowed
        return false;
    }

    let first  = x.chars().nth(0).unwrap() as i32;
    let last   = x.chars().nth(x.len() - 1).unwrap() as i32;

    let padded = json_isspace(first) || json_isspace(last);

    if x.len() >= 1 && padded {
        //  No padding allowed
        return false;
    }

    if x.len() != unsafe { libc::strlen(x.as_ptr() as *const i8) } {
        //  No embedded NUL characters allowed
        return false;
    }

    true
}
