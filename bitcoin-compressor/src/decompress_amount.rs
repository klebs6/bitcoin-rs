// ---------------- [ File: bitcoin-compressor/src/decompress_amount.rs ]
crate::ix!();

pub fn decompress_amount(mut x: u64) -> u64 {
    
    // x = 0  OR  x = 1+10*(9*n + d - 1) + e  OR  x = 1+10*(n - 1) + 9
    if x == 0 {
        return 0;
    }

    x -= 1;

    // x = 10*(9*n + d - 1) + e
    let mut e: i32 = (x % 10).try_into().unwrap();

    x /= 10;

    let mut n: u64 = 0;

    if e < 9 {

        // x = 9*n + d - 1
        let d: i32 = ((x % 9) + 1).try_into().unwrap();

        x /= 9;

        // x = n
        n = {

            //this might be incorrect, the old way
            //was: n = x * 10 + d;
            u64_plus_i32((x * 10).try_into().unwrap(),d)
        };

    } else {

        n = x + 1;
    }

    while e != 0 {
        n *= 10;
        e -= 1;
    }

    n
}
