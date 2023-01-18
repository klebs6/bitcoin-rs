#![feature(allocator_api)]

macro_rules! ternary {
    ($condition:expr,$if_true:expr,$if_false:expr) => {
        match $condition {
            true  => $if_true,
            false => $if_false,
        }
    }
}

#[macro_use] mod imports; use imports::*;

x!{bitcoinconsensus}
x!{script_error}
x!{interpreter}
x!{script}
x!{sigcache}
x!{sign}
x!{standard}
x!{outputtype}
x!{gen_txid}
x!{signature_checker}
x!{parse}
