// ---------------- [ File: bitcoin-aes/src/ctaes.rs ]
/*!
 | Constant time, unoptimized, concise, plain C,
 | AES implementation Based On:
 |
 |   Emilia Kasper and Peter Schwabe, Faster and
 |   Timing-Attack Resistant AES-GCM
 |   http://www.iacr.org/archive/ches2009/57470001/57470001.pdf
 |
 | But using 8 16-bit integers representing
 | a single AES state rather than 8 128-bit
 | integers representing 8 AES states.
 |
 |
 | Slice variable slice_i contains the i'th bit of
 | the 16 state variables in this order:
 |
 |  0  1  2  3
 |  4  5  6  7
 |  8  9 10 11
 | 12 13 14 15
 */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/ctaes/ctaes.h]

#[derive(Default)]
pub struct AES_state {
    slice: [u16; 8],
}

#[derive(Default)]
pub struct AES128_ctx {
    rk: [AES_state; 11],
}

#[derive(Default)]
pub struct AES192_ctx {
    rk: [AES_state; 13],
}

#[derive(Default)]
pub struct AES256_ctx {
    rk: [AES_state; 15],
}

impl From<[u8;32]> for AES256_ctx {
    fn from(x: [u8;32]) -> Self {
        todo!();
    }
}

//-------------------------------------------[.cpp/bitcoin/src/crypto/ctaes/ctaes.c]

/**
  | Convert a byte to sliced form, storing
  | it corresponding to given row and column
  | in s
  |
  */
pub fn load_byte(
    s:        &mut AES_state,
    mut byte: u8,
    r:        i32,
    c:        i32) 
{
    let mut i: i32 = 0;

    for i in 0..8 {

        s.slice[i] |= u16::from((byte & 1) << (r * 4 + c));

        byte >>= 1;
    }
}

/**
  | Load 16 bytes of data into 8 sliced integers
  |
  */
pub fn load_bytes(
        s:      *mut AES_state,
        data16: *const u8)  {
    
    todo!();
        /*
            int c;
        for (c = 0; c < 4; c++) {
            int r;
            for (r = 0; r < 4; r++) {
                LoadByte(s, *(data16++), r, c);
            }
        }
        */
}

/**
  | Convert 8 sliced integers into 16 bytes
  | of data
  |
  */
pub fn save_bytes(
        data16: *mut u8,
        s:      *const AES_state)  {
    
    todo!();
        /*
            int c;
        for (c = 0; c < 4; c++) {
            int r;
            for (r = 0; r < 4; r++) {
                int b;
                uint8_t v = 0;
                for (b = 0; b < 8; b++) {
                    v |= ((s->slice[b] >> (r * 4 + c)) & 1) << b;
                }
                *(data16++) = v;
            }
        }
        */
}

macro_rules! bit_range {
    ($from:ident, 
     $to:ident) => {
        /*
                (((1 << ((to) - (from))) - 1) << (from))
        */
    }
}

macro_rules! bit_range_left {
    ($x:ident, 
     $from:ident, 
     $to:ident, 
     $shift:ident) => {
        /*
                (((x) & BIT_RANGE((from), (to))) << (shift))
        */
    }
}

macro_rules! bit_range_right {
    ($x:ident, 
     $from:ident, 
     $to:ident, 
     $shift:ident) => {
        /*
                (((x) & BIT_RANGE((from), (to))) >> (shift))
        */
    }
}

pub fn shift_rows(s: *mut AES_state)  {
    
    todo!();
        /*
            int i;
        for (i = 0; i < 8; i++) {
            uint16_t v = s->slice[i];
            s->slice[i] =
                (v & BIT_RANGE(0, 4)) |
                BIT_RANGE_LEFT(v, 4, 5, 3) | BIT_RANGE_RIGHT(v, 5, 8, 1) |
                BIT_RANGE_LEFT(v, 8, 10, 2) | BIT_RANGE_RIGHT(v, 10, 12, 2) |
                BIT_RANGE_LEFT(v, 12, 15, 1) | BIT_RANGE_RIGHT(v, 15, 16, 3);
        }
        */
}

pub fn inv_shift_rows(s: *mut AES_state)  {
    
    todo!();
        /*
            int i;
        for (i = 0; i < 8; i++) {
            uint16_t v = s->slice[i];
            s->slice[i] =
                (v & BIT_RANGE(0, 4)) |
                BIT_RANGE_LEFT(v, 4, 7, 1) | BIT_RANGE_RIGHT(v, 7, 8, 3) |
                BIT_RANGE_LEFT(v, 8, 10, 2) | BIT_RANGE_RIGHT(v, 10, 12, 2) |
                BIT_RANGE_LEFT(v, 12, 13, 3) | BIT_RANGE_RIGHT(v, 13, 16, 1);
        }
        */
}

macro_rules! rot {
    ($x:ident, $b:ident) => {
        /*
                (((x) >> ((b) * 4)) | ((x) << ((4-(b)) * 4)))
        */
    }
}

pub fn add_round_key(
        s:     *mut AES_state,
        round: *const AES_state)  {
    
    todo!();
        /*
            int b;
        for (b = 0; b < 8; b++) {
            s->slice[b] ^= round->slice[b];
        }
        */
}

/**
  | column_0(s) = column_c(a)
  |
  */
pub fn get_one_column(
        s: *mut AES_state,
        a: *const AES_state,
        c: i32)  {
    
    todo!();
        /*
            int b;
        for (b = 0; b < 8; b++) {
            s->slice[b] = (a->slice[b] >> c) & 0x1111;
        }
        */
}

/**
  | column_c1(r) |= (column_0(s) ^= column_c2(a))
  |
  */
pub fn key_setup_column_mix(
        s:  *mut AES_state,
        r:  *mut AES_state,
        a:  *const AES_state,
        c1: i32,
        c2: i32)  {
    
    todo!();
        /*
            int b;
        for (b = 0; b < 8; b++) {
            r->slice[b] |= ((s->slice[b] ^= ((a->slice[b] >> c2) & 0x1111)) & 0x1111) << c1;
        }
        */
}

/**
  | Rotate the rows in s one position upwards,
  | and xor in r
  |
  */
pub fn key_setup_transform(
        s: *mut AES_state,
        r: *const AES_state)  {
    
    todo!();
        /*
            int b;
        for (b = 0; b < 8; b++) {
            s->slice[b] = ((s->slice[b] >> 4) | (s->slice[b] << 12)) ^ r->slice[b];
        }
        */
}

/**
  | Multiply the cells in s by x, as polynomials
  | over GF(2) mod x^8 + x^4 + x^3 + x + 1
  |
  */
pub fn multx(s: *mut AES_state)  {
    
    todo!();
        /*
            uint16_t top = s->slice[7];
        s->slice[7] = s->slice[6];
        s->slice[6] = s->slice[5];
        s->slice[5] = s->slice[4];
        s->slice[4] = s->slice[3] ^ top;
        s->slice[3] = s->slice[2] ^ top;
        s->slice[2] = s->slice[1];
        s->slice[1] = s->slice[0] ^ top;
        s->slice[0] = top;
        */
}

pub fn aes128_init(
        ctx:   *mut AES128_ctx,
        key16: *const u8)  {
    
    todo!();
        /*
            AES_setup(ctx->rk, key16, 4, 10);
        */
}

pub fn aes128_encrypt(
        ctx:      *const AES128_ctx,
        blocks:   usize,
        cipher16: *mut u8,
        plain16:  *const u8)  {
    
    todo!();
        /*
            while (blocks--) {
            AES_encrypt(ctx->rk, 10, cipher16, plain16);
            cipher16 += 16;
            plain16 += 16;
        }
        */
}

pub fn aes128_decrypt(
        ctx:      *const AES128_ctx,
        blocks:   usize,
        plain16:  *mut u8,
        cipher16: *const u8)  {
    
    todo!();
        /*
            while (blocks--) {
            AES_decrypt(ctx->rk, 10, plain16, cipher16);
            cipher16 += 16;
            plain16 += 16;
        }
        */
}

pub fn aes192_init(
        ctx:   *mut AES192_ctx,
        key24: *const u8)  {
    
    todo!();
        /*
            AES_setup(ctx->rk, key24, 6, 12);
        */
}

pub fn aes192_encrypt(
        ctx:      *const AES192_ctx,
        blocks:   usize,
        cipher16: *mut u8,
        plain16:  *const u8)  {
    
    todo!();
        /*
            while (blocks--) {
            AES_encrypt(ctx->rk, 12, cipher16, plain16);
            cipher16 += 16;
            plain16 += 16;
        }
        */
}

pub fn aes192_decrypt(
        ctx:      *const AES192_ctx,
        blocks:   usize,
        plain16:  *mut u8,
        cipher16: *const u8)  {
    
    todo!();
        /*
            while (blocks--) {
            AES_decrypt(ctx->rk, 12, plain16, cipher16);
            cipher16 += 16;
            plain16 += 16;
        }
        */
}

pub fn aes256_init(
        ctx:   *mut AES256_ctx,
        key32: *const u8)  {
    
    todo!();
        /*
            AES_setup(ctx->rk, key32, 8, 14);
        */
}

pub fn aes256_encrypt(
        ctx:      *const AES256_ctx,
        blocks:   usize,
        cipher16: *mut u8,
        plain16:  *const u8)  {
    
    todo!();
        /*
            while (blocks--) {
            AES_encrypt(ctx->rk, 14, cipher16, plain16);
            cipher16 += 16;
            plain16 += 16;
        }
        */
}

pub fn aes256_decrypt(
        ctx:      *const AES256_ctx,
        blocks:   usize,
        plain16:  *mut u8,
        cipher16: *const u8)  {
    
    todo!();
        /*
            while (blocks--) {
            AES_decrypt(ctx->rk, 14, plain16, cipher16);
            cipher16 += 16;
            plain16 += 16;
        }
        */
}
