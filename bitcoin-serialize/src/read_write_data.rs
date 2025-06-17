// ---------------- [ File: bitcoin-serialize/src/read_write_data.rs ]
crate::ix!();

/**
  | Convert the reference base type to X,
  | without changing constness or reference
  | type.
  |
  */
pub fn read_write_as_helper<X>(x: &mut X) -> &mut X {
    x
}

/**
  | Lowest-level serialization and conversion.
  | 
  | -----------
  | @note
  | 
  | Sizes of these types are verified in
  | the tests
  |
  */
#[inline]
pub fn ser_writedata8<Stream: Write>(s: &mut Stream, obj: u8) {
    trace!(value = obj, "ser_writedata8");
    s.write_all(&[obj])
        .expect("I/O error while writing u8");
}

#[inline]
pub fn ser_writedata16<Stream: Write>(s: &mut Stream, obj: u16) {
    trace!(value = obj, "ser_writedata16 (LE)");
    s.write_all(&obj.to_le_bytes())
        .expect("I/O error while writing u16");
}

#[inline]
pub fn ser_writedata_16be<Stream: Write>(s: &mut Stream, obj: u16) {
    trace!(value = obj, "ser_writedata_16be (BE)");
    s.write_all(&obj.to_be_bytes())
        .expect("I/O error while writing u16‑be");
}

#[inline]
pub fn ser_writedata32<Stream: Write>(s: &mut Stream, obj: u32) {
    trace!(value = obj, "ser_writedata32 (LE)");
    s.write_all(&obj.to_le_bytes())
        .expect("I/O error while writing u32");
}

#[inline]
pub fn ser_writedata_32be<Stream: Write>(s: &mut Stream, obj: u32) {
    trace!(value = obj, "ser_writedata_32be (BE)");
    s.write_all(&obj.to_be_bytes())
        .expect("I/O error while writing u32‑be");
}

#[inline]
pub fn ser_writedata64<Stream: Write>(s: &mut Stream, obj: u64) {
    trace!(value = obj, "ser_writedata64 (LE)");
    s.write_all(&obj.to_le_bytes())
        .expect("I/O error while writing u64");
}

#[inline]
pub fn ser_readdata8<Stream: Read>(s: &mut Stream) -> u8 {
    let mut buf = [0u8; 1];
    s.read_exact(&mut buf).expect("I/O error while reading u8");
    let v = buf[0];
    trace!(value = v, "ser_readdata8");
    v
}

#[inline]
pub fn ser_readdata16<Stream: Read>(s: &mut Stream) -> u16 {
    let mut buf = [0u8; 2];
    s.read_exact(&mut buf).expect("I/O error while reading u16");
    let v = u16::from_le_bytes(buf);
    trace!(value = v, "ser_readdata16 (LE)");
    v
}

#[inline]
pub fn ser_readdata_16be<Stream: Read>(s: &mut Stream) -> u16 {
    let mut buf = [0u8; 2];
    s.read_exact(&mut buf).expect("I/O error while reading u16‑be");
    let v = u16::from_be_bytes(buf);
    trace!(value = v, "ser_readdata_16be (BE)");
    v
}

#[inline]
pub fn ser_readdata32<Stream: Read>(s: &mut Stream) -> u32 {
    let mut buf = [0u8; 4];
    s.read_exact(&mut buf).expect("I/O error while reading u32");
    let v = u32::from_le_bytes(buf);
    trace!(value = v, "ser_readdata32 (LE)");
    v
}

#[inline]
pub fn ser_readdata_32be<Stream: Read>(s: &mut Stream) -> u32 {
    let mut buf = [0u8; 4];
    s.read_exact(&mut buf).expect("I/O error while reading u32‑be");
    let v = u32::from_be_bytes(buf);
    trace!(value = v, "ser_readdata_32be (BE)");
    v
}

#[inline]
pub fn ser_readdata64<Stream: Read>(s: &mut Stream) -> u64 {
    let mut buf = [0u8; 8];
    s.read_exact(&mut buf).expect("I/O error while reading u64");
    let v = u64::from_le_bytes(buf);
    trace!(value = v, "ser_readdata64 (LE)");
    v
}
