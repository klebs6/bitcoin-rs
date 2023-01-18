crate::ix!();

/**
  | These check for scripts for which a special
  | case with a shorter encoding is defined.
  | 
  | They are implemented separately from
  | the
  | 
  | CScript test, as these test for exact
  | byte sequence correspondences, and
  | are more strict. For example, IsToPubKey
  | also verifies whether the public key
  | is valid (as invalid ones cannot be represented
  | in compressed form).
  |
  */
pub fn is_to_keyid(
        script: &Script,
        hash:   &mut KeyID) -> bool {
    
    if script.base.len() == 25 
    && script.base[0] == opcode_type::OP_DUP 
    && script.base[1] == opcode_type::OP_HASH160 
    && script.base[2] == 20 
    && script.base[23] == opcode_type::OP_EQUALVERIFY 
    && script.base[24] == opcode_type::OP_CHECKSIG 
    {
        unsafe {
            libc::memcpy(as_mut_cvoid![hash], as_cvoid![&script.base[3]], 20);
        }
        return true;
    }

    false
}

pub fn is_to_scriptid(
        script: &Script,
        hash:   &mut ScriptID) -> bool {
    
    if script.base.len() == 23 
    && script.base[0] == opcode_type::OP_HASH160 
    && script.base[1] == 20 
    && script.base[22] == opcode_type::OP_EQUAL 
    {
        unsafe {
            libc::memcpy(as_mut_cvoid![hash], as_cvoid![&script.base[2]], 20);
        }
        return true;
    }

    false
}

pub fn is_to_pub_key(
        script: &Script,
        pubkey: &mut PubKey) -> bool {
    
    if script.base.len() == 35 
    && script.base[0] == 33 
    && script.base[34] == opcode_type::OP_CHECKSIG 
    && (script.base[1] == 0x02 || script.base[1] == 0x03) 
    {
        pubkey.set(&script.base[1], &script.base[34]);
        return true;
    }

    if script.base.len() == 67 
    && script.base[0] == 65 
    && script.base[66] == opcode_type::OP_CHECKSIG 
    && script.base[1] == 0x04 
    {
        pubkey.set(&script.base[1], &script.base[66]);

        // if not fully valid, a case that would
        // not be compressible
        return pubkey.is_fully_valid();
    }

    false
}
