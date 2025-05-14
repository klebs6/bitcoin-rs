// ---------------- [ File: bitcoin-compat/src/assumptions.rs ]
/*!
  | Compile-time verification of assumptions we
  | make.
  |
  | Some important things we are NOT assuming
  | (non-exhaustive list):
  |
  | * We are NOT assuming a specific value for
  | std::endian::native.
  |
  | * We are NOT assuming a specific value for
  | std::locale("").name().
  |
  | * We are NOT assuming a specific value for
  | std::numeric_limits<char>::is_signed.
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/compat/assumptions.h]

/**
  | Assumption: We assume the floating-point types
  |             to fulfill the requirements of IEC
  |             559 (IEEE 754) standard.
  |
  | Example(s): Floating-point division by zero in
  |             ConnectBlock, CreateTransaction and
  |             EstimateMedianVal.
  */
const_assert!{f32_is_iec559()}// "IEEE 754 float assumed"
const_assert!{f64_is_iec559()} // "IEEE 754 double assumed"

/**
  | the “binary32” type defined in IEEE
  | 754-2008
  |
  */
const fn f32_is_iec559() -> bool { true }

/**
  | the “binary64” type defined in IEEE
  | 754-2008
  |
  */
const fn f64_is_iec559() -> bool { true }

/**
  | Assumption: We assume eight bits per byte
  |             (obviously, but remember: don't
  |             trust -- verify!).
  |
  | Example(s): Everywhere :-)
  */
const_assert!{u8::BITS == 8} //"8-bit byte assumed"

/**
  | Assumption: We assume integer widths.
  |
  | Example(s): GetSizeOfCompactSize and
  |             WriteCompactSize in the
  |             serialization code.
  */
const_assert!{size_of::<u16>() == 2} // "16-bit short assumed"   
const_assert!{size_of::<i32>() == 4} // "32-bit int assumed"     
const_assert!{size_of::<u32>() == 4} // "32-bit unsigned assumed"

/**
  | Assumption: We assume size_t to be 32-bit or
  | 64-bit.
  |
  | Example(s):
  |
  | size_t assumed to be at least 32-bit in
  | ecdsa_signature_parse_der_lax(...).
  |
  | size_t assumed to be 32-bit or 64-bit in
  | MallocUsage(...).
  */
const_assert!{ 
    size_of::<usize>() == 4 || size_of::<usize>() == 8
}

/**
  | "Sizes of size_t and c_void* assumed to
  | be equal"
  |
  */
const_assert!{ 
    size_of::<usize>() == size_of::<*mut c_void>()
}
