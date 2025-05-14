// ---------------- [ File: bitcoin-compressor/src/decompress.rs ]
crate::ix!();

pub fn decompress_script(
        script: &mut Script,
        n_size: u32,
        in_:    &CompressedScript) -> bool {
    
    match n_size {

        0x00  => {

            script.resize(25, Default::default());
            script[0] = opcode_type::OP_DUP;
            script[1] = opcode_type::OP_HASH160;
            script[2] = 20;
            unsafe {
                libc::memcpy(
                    as_mut_cvoid!(&mut script[3]), 
                    as_cvoid!(in_.as_ptr()), 
                    20
                );
            }
            script[23] = opcode_type::OP_EQUALVERIFY;
            script[24] = opcode_type::OP_CHECKSIG;
            return true;
        },

        0x01  => {
            script.resize(23, Default::default());
            script[0] = opcode_type::OP_HASH160;
            script[1] = 20;

            unsafe {
                libc::memcpy(
                    as_mut_cvoid!(&mut script[2]), 
                    as_cvoid!(in_.as_ptr()), 
                    20
                );
            }

            script[22] = opcode_type::OP_EQUAL;
            return true;
        },

        0x02 | 0x03  => {

            script.resize(35, Default::default());
            script[0] = 33;
            script[1] = n_size.try_into().unwrap();

            unsafe {
                libc::memcpy(
                    as_mut_cvoid!(&mut script[2]), 
                    as_cvoid!(in_.as_ptr()), 
                    32
                );
            }

            script[34] = opcode_type::OP_CHECKSIG;
            return true;
        },

        0x04 | 0x05  => {

            let mut vch: [u8; 33] = [0; 33];

            vch[0] = (n_size - 2).try_into().unwrap();

            unsafe {
                libc::memcpy(
                    as_mut_cvoid!(&mut vch[1]), 
                    as_cvoid!(in_.as_ptr()), 
                    32
                );
            }

            let mut pubkey: PubKey = PubKey::new(&vch);

            if !pubkey.decompress() {
                return false;
            }

            assert!(pubkey.size() == 65);

            script.resize(67, Default::default());
            script[0] = 65;

            unsafe {
                libc::memcpy(
                    as_mut_cvoid!(&mut script[1]), 
                    as_cvoid!(pubkey.data()), 
                    65
                );
            }

            script[66] = opcode_type::OP_CHECKSIG;
            return true;
        },
        _ => {},
    }

    false
}
