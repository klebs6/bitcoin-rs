#[macro_use] mod imports; use imports::*;

x!{field}

#[cfg(SECP256K1_WIDEMUL_INT128)]
x!{field_5x52}

//x!{asm_field_10x26_arm}

#[cfg(SECP256K1_WIDEMUL_INT64)]
x!{field_10x26}
