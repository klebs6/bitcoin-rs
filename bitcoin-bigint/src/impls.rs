crate::ix!();

// ---------------------------------------------------------------------------
// 9) Now we gather the expansions for all macros to produce each needed type:
//
//     BaseUInt32  => BITS=32,  limbs=1
//     BaseUInt64  => BITS=64,  limbs=2
//     BaseUInt160 => BITS=160, limbs=5
//     BaseUInt256 => BITS=256, limbs=8
//
// We expand the macros in one place so that all the methods are defined.
//
// ---------------------------------------------------------------------------
#[macro_export]
macro_rules! bigint {

    ($name:ident, $bits:expr, $limbs:expr) => {
        define_base_uint_struct_and_basic! { $name ,  $bits,  $limbs}
        define_base_uint_conversions!      { $name ,  $bits,  $limbs}
        define_base_uint_mulassign!        { $name ,  $bits,  $limbs}
        define_base_uint_not_neg!          { $name ,  $bits,  $limbs}
        define_base_uint_ord_eq!           { $name ,  $bits,  $limbs}
        define_base_uint_shl_shr!          { $name ,  $bits,  $limbs}
        define_baseuint_addassign!         { $name ,  $bits,  $limbs}
        define_baseuint_subassign!         { $name ,  $bits,  $limbs}
        define_baseuint_divassign!         { $name ,  $bits,  $limbs}
        define_baseuint_add_sub_mul_div!   { $name ,  $bits,  $limbs}
        define_baseuint_bitand!            { $name ,  $bits,  $limbs}
        define_baseuint_bitor!             { $name ,  $bits,  $limbs}
        define_baseuint_bitxor!            { $name ,  $bits,  $limbs}
        define_baseuint_fromstr!           { $name ,  $bits,  $limbs}
        define_baseuint_from_u64!          { $name ,  $bits,  $limbs}
        define_baseuint_get_hex!           { $name ,  $bits,  $limbs}
    }
}

bigint!{BaseUInt32,  32,  1}
bigint!{BaseUInt64,  64,  2}
bigint!{BaseUInt128, 128, 4}
bigint!{BaseUInt160, 160, 5}
bigint!{BaseUInt256, 256, 8}
