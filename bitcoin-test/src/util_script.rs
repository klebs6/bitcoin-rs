// ---------------- [ File: bitcoin-test/src/util_script.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/util/script.h]
//-------------------------------------------[.cpp/bitcoin/src/test/util/script.cpp]

lazy_static!{
    static ref WITNESS_STACK_ELEM_OP_TRUE: Vec<u8> = vec!{opcode_type::OP_TRUE as u8};
}

pub fn p2wsh_op_true() -> Script {

    todo!();

    /*
    CScript{}
    << OP_0
        << ToByteVector([] {
            uint256 hash;
            CSHA256().Write(WITNESS_STACK_ELEM_OP_TRUE.data(), WITNESS_STACK_ELEM_OP_TRUE.size()).Finalize(hash.begin());
            return hash;
        }())
    */
}

lazy_static!{
    static ref P2WSH_OP_TRUE: Script = p2wsh_op_true();
}

/**
  | Flags that are not forbidden by an assert
  | in script validation
  |
  */
pub fn is_valid_flag_combination(flags: u32) -> bool {
    
    todo!();
        /*
            if (flags & SCRIPT_VERIFY_CLEANSTACK && ~flags & (SCRIPT_VERIFY_P2SH | SCRIPT_VERIFY_WITNESS)) return false;
        if (flags & SCRIPT_VERIFY_WITNESS && ~flags & SCRIPT_VERIFY_P2SH) return false;
        return true;
        */
}
