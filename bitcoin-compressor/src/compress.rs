crate::ix!();

pub fn compress_script(
        script: &Script,
        out:    &mut CompressedScript) -> bool {
    
    let mut keyid = KeyID::default();

    if is_to_keyid(script,&mut keyid) {

        out.resize(21, Default::default());

        out[0] = 0x00;

        unsafe {
            libc::memcpy(
                as_mut_cvoid!(&mut out[1]), 
                as_cvoid!(&keyid), 
                20
            );
        }

        return true;
    }

    let mut scriptid = ScriptID::default();

    if is_to_scriptid(script,&mut scriptid) {
        out.resize(21, Default::default());
        out[0] = 0x01;

        unsafe {
            libc::memcpy(
                as_mut_cvoid!(&mut out[1]), 
                as_cvoid!(&scriptid), 
                20
            );
        }

        return true;
    }

    let mut pubkey = PubKey::default();

    if is_to_pub_key(script,&mut pubkey) {

        out.resize(33, Default::default());

        unsafe {
            libc::memcpy(
                as_mut_cvoid!(&mut out[1]), 
                as_cvoid!(&pubkey[1]), 
                32
            );
        }

        if pubkey[0] == 0x02 || pubkey[0] == 0x03 {

            out[0] = pubkey[0];
            return true;

        } else {

            if pubkey[0] == 0x04 {
                out[0] = 0x04 | (pubkey[64] & 0x01);
                return true;
            }
        }
    }

    false
}
