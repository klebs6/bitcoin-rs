// ---------------- [ File: bitcoin-serialize/src/primitives.rs ]
crate::ix!();

use std::io::{Read, Write, Result as IoResult};

#[inline]
pub fn write_u32_le<W: Write>(w: &mut W, v: u32) -> IoResult<()> {
    w.write_all(&v.to_le_bytes())
}

#[inline]
pub fn read_u32_le<R: Read>(r: &mut R) -> IoResult<u32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

#[inline]
pub fn write_encoded_f64<W: Write>(w: &mut W, v: f64) -> IoResult<()> {
    let bits = v.to_bits();
    w.write_all(&bits.to_be_bytes())
}

#[inline]
pub fn read_encoded_f64<R: Read>(r: &mut R) -> IoResult<f64> {
    let mut buf = [0u8; 8];
    r.read_exact(&mut buf)?;
    let bits = u64::from_be_bytes(buf);
    Ok(f64::from_bits(bits))
}
