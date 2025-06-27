// ---------------- [ File: bitcoin-crc32c/src/arm64.rs ]
// -----------------------------------------------------------------------------
//  A R M  /  C R C  +  P M U L L     (hardware‑accelerated Castagnoli CRC‑32)
// -----------------------------------------------------------------------------
//
// A faithful, instruction‑for‑instruction port of
//   crc32c/src/crc32c_arm64.cc  (Bitcoin‑Core fork of  Google‑CRC32C)
// -----------------------------------------------------------------------------

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_arm64.h]
//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_arm64.cc]

/* --------------- * ARM-specific code  --------------- */

/**
   This implementation is based on
   https://github.com/google/leveldb/pull/490.
  */
pub const KBYTES:       usize = 1032;

pub const SEGMENTBYTES: usize = 256;

const K0: u64 = 0x8d96_551c;
const K1: u64 = 0xbd6f_81f8;
const K2: u64 = 0xdcb1_7aa4;

/// 8‑byte step on *one* of the four parallel segments.
#[inline(always)]
unsafe fn step64(crc: &mut u32, p: *const u8) {
    *crc = __crc32cd(*crc, core::ptr::read_unaligned(p as *const u64));
}

/// Process 32 B (4 × 8 B) of *every* segment (former `CRC32C32BYTES`).
///
/// compute 8bytes for each segment parallelly
///
#[inline(always)]
unsafe fn crc32c32bytes(seg0: &mut u32, seg1: &mut u32, seg2: &mut u32, seg3: &mut u32, p: *const u8, ind: usize) {
    step64(seg1, p.add(SEGMENTBYTES + ind * 8));
    step64(seg2, p.add(SEGMENTBYTES * 2 + ind * 8));
    step64(seg3, p.add(SEGMENTBYTES * 3 + ind * 8));
    step64(seg0, p.add(SEGMENTBYTES * 0 + ind * 8));
}

/// Process 256 B ≙ 8× the previous helper (former `CRC32C256BYTES`).
#[inline(always)]
unsafe fn crc32c256bytes(seg0:&mut u32, seg1:&mut u32, seg2:&mut u32, seg3:&mut u32, p:*const u8, ind:usize) {
    crc32c32bytes(seg0, seg1, seg2, seg3, p, ind*8 + 0);
    crc32c32bytes(seg0, seg1, seg2, seg3, p, ind*8 + 1);
    crc32c32bytes(seg0, seg1, seg2, seg3, p, ind*8 + 2);
    crc32c32bytes(seg0, seg1, seg2, seg3, p, ind*8 + 3);
    crc32c32bytes(seg0, seg1, seg2, seg3, p, ind*8 + 4);
    crc32c32bytes(seg0, seg1, seg2, seg3, p, ind*8 + 5);
    crc32c32bytes(seg0, seg1, seg2, seg3, p, ind*8 + 6);
    crc32c32bytes(seg0, seg1, seg2, seg3, p, ind*8 + 7);
}

/// Process a full 1024 B chunk (former `CRC32C1024BYTES`).
#[inline(always)]
unsafe fn crc32c1024bytes(seg0:&mut u32, seg1:&mut u32, seg2:&mut u32, seg3:&mut u32, mut p:*const u8) -> *const u8 {
    crc32c256bytes(seg0, seg1, seg2, seg3, p, 0);
    crc32c256bytes(seg0, seg1, seg2, seg3, p, 1);
    crc32c256bytes(seg0, seg1, seg2, seg3, p, 2);
    crc32c256bytes(seg0, seg1, seg2, seg3, p, 3);
    p.add(4*SEGMENTBYTES)
}

#[inline]
pub unsafe fn crc32c_extend_arm64(mut crc: u32, mut data: *const u8, mut size: usize) -> u32 {

    let mut length = size as isize;

    crc ^= CRC32XOR;

    while length >= KBYTES as isize {
        // --- process 1 KiB in four independent 256 B segments ---------------
        let mut crc0 = crc;
        let mut crc1 = 0u32;
        let mut crc2 = 0u32;
        let mut crc3 = 0u32;

        data = crc32c1024bytes(&mut crc0,&mut crc1,&mut crc2,&mut crc3,data);

        // --- GF(2) polynomial merging of the four partial CRCs --------------
        let t2 = vgetq_lane_u64::<1>(vmull_p64(crc2 as u64, K2));
        let t1 = vgetq_lane_u64::<1>(vmull_p64(crc1 as u64, K1));
        let t0 = vgetq_lane_u64::<1>(vmull_p64(crc0 as u64, K0));

        crc  = __crc32cd(crc3, core::ptr::read_unaligned(data as *const u64));
        data = data.add(core::mem::size_of::<u64>());

        crc ^= __crc32cd(0, t2);
        crc ^= __crc32cd(0, t1);
        crc ^= __crc32cd(0, t0);

        length -= KBYTES as isize;
    }

    // ---------------- fast 8‑byte loop --------------------------------------
    while length >= 8 {
        crc = __crc32cd(crc, core::ptr::read_unaligned(data as *const u64));
        data = data.add(8);
        length -= 8;
    }

    // ---------------- remaining tail  ---------------------------------------
    if (length & 4) != 0 {
        crc = __crc32cw(crc, core::ptr::read_unaligned(data as *const u32));
        data = data.add(4);
    }
    if (length & 2) != 0 {
        crc = __crc32ch(crc, core::ptr::read_unaligned(data as *const u16));
        data = data.add(2);
    }
    if (length & 1) != 0 {
        crc = __crc32cb(crc, *data);
    }

    crc ^ CRC32XOR
}
