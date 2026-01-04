// ---------------- [ File: bitcoinsecp256k1-group/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/group.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/group_impl.h]

x!{constants}
x!{ge_const}
x!{ge}
x!{ge_clear}
x!{ge_from_storage}
x!{ge_globalz_set_table_gej}
x!{ge_is_in_correct_subgroup}
x!{ge_is_infinity}
x!{ge_is_valid_var}
x!{ge_mul_lambda}
x!{ge_neg}
x!{ge_set_all_gej_var}
x!{ge_set_gej}
x!{ge_set_gej_var}
x!{ge_set_gej_zinv}
x!{ge_set_infinity}
x!{ge_set_xo_var}
x!{ge_set_xy}
x!{ge_storage}
x!{ge_storage_cmov}
x!{ge_to_storage}
x!{gej}
x!{gej_add_ge}
x!{gej_add_ge_var}
x!{gej_add_var}
x!{gej_add_zinv_var}
x!{gej_clear}
x!{gej_const}
x!{gej_double}
x!{gej_double_var}
x!{gej_eq_x_var}
x!{gej_is_infinity}
x!{gej_neg}
x!{gej_rescale}
x!{gej_set_ge}
x!{gej_set_infinity}
