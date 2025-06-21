// ---------------- [ File: bitcoin-poly1305/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

x!{accumulate_block}
x!{add_pad_serialize}
x!{compute_g_plus5_minus_p}
x!{ct_select_limbs}
x!{expand_key}
x!{final_carry_and_sub_p}
x!{multiply_and_reduce}
x!{poly1305}
x!{poly1305_auth}
x!{propagate_26bit_carries_once}
x!{read_write}
x!{trace_step}
