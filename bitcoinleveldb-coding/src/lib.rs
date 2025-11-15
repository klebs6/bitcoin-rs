// ---------------- [ File: bitcoinleveldb-coding/src/lib.rs ]
/*!
  | Endian-neutral encoding:
  |
  | - Fixed-length numbers are encoded with
  | least-significant byte first
  |
  | - In addition we support variable length
  | "varint" encoding
  |
  | - Strings are encoded prefixed by their length
  | in varint format
  */
#[macro_use] mod imports; use imports::*;
//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/coding.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/coding.cc]

x!{decode_fixed}
x!{encode_fixed}
x!{encode_varint}
x!{get_length_prefixed_slice}
x!{get_varint}
x!{put_fixed}
x!{put_length_prefixed_slice}
x!{put_varint}
x!{varint_length}
x!{slice_to_utf8}
