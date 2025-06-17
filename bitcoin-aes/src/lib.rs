// ---------------- [ File: bitcoin-aes/src/lib.rs ]
#![feature(test)]

#![allow(soft_unstable)]
#[macro_use] mod imports; use imports::*;

x!{add_round_key}
x!{aes}
x!{aes128}
x!{aes192}
x!{aes256}
x!{aes256_decrypt}
x!{aes256_encrypt}
x!{aes256cbc_decrypt}
x!{aes256cbc_encrypt}
x!{aes_decrypt}
x!{aes_encrypt}
x!{aes_state}
x!{bit_range}
x!{cbc_decrypt}
x!{cbc_encrypt}
x!{ctaes_bench}
x!{ctaes_mix_columns}
x!{ctaes_setup}
x!{ctaes_sub_bytes}
x!{ctaes_test}
x!{get_one_column}
x!{get_time_double}
x!{key_setup}
x!{load_bytes}
x!{multx}
x!{print_number}
x!{run_benchmark}
x!{shift_rows}
