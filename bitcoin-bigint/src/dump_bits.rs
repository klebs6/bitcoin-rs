crate::ix!();

pub fn dump_u64_bits(label: &str, val: u64) {
    debug!("{} = 0x{:016X} => bin={:064b}", label, val, val);
}

pub fn dump_limb_bits(label: &str, limbs: &[u32]) {
    // for a 64-bit BaseUInt, we have 2 limbs:
    //   pn[0] => low 32 bits, pn[1] => high 32 bits
    let low = (limbs[0] as u64) & 0xFFFF_FFFF;
    let high = (limbs[1] as u64) & 0xFFFF_FFFF;
    let combined = (high << 32) | low;
    debug!("{} => pn=[0x{:08X}, 0x{:08X}] => full u64=0x{:016X}", 
           label, limbs[0], limbs[1], combined);
}
