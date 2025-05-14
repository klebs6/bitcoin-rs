// ---------------- [ File: bitcoin-net/src/split.rs ]
crate::ix!();

pub fn split_host_port(
        mut in_:  String,
        port_out: &mut u16,
        host_out: &mut String)  {

    let colon: Option<usize> = in_.rfind(':');

    // if a : is found, and it either follows
    // a [...], or no other : is in the string,
    // treat it as port separator
    let have_colon: bool = colon != None;

    // if there is a colon, and in[0]=='[', colon
    // is not 0, so in[colon-1] is safe
    let bracketed: bool = have_colon && {

        let first = in_.chars().nth(0).unwrap();
        let last  = in_.chars().nth(colon.unwrap() - 1).unwrap();

        let have_brackets = first == '[' && last == ']';

        have_brackets
    };

    let multi_colon: bool = have_colon && {
        in_[0..colon.unwrap()].rfind(':') != None
    };

    if have_colon && {
        colon.unwrap() == 0 
        || bracketed 
        || !multi_colon
    } {
        let mut n = u16::default();

        let val = &in_[(colon.unwrap() + 1)..];

        if parse_uint16(&val,&mut n) {
            in_ = in_[0..colon.unwrap()].to_string();
            *port_out = n;
        }
    }

    if in_.len() > 0 
    && in_.chars().nth(0).unwrap() == '[' 
    && in_.chars().nth(in_.len() - 1).unwrap() == ']' 
    {
        *host_out = in_[1.. 1 + in_.len() - 2].to_string();
    } else {
        *host_out = in_;
    }
}
