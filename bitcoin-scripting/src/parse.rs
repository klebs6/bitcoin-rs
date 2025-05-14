// ---------------- [ File: bitcoin-scripting/src/parse.rs ]
crate::ix!();

pub fn parse_script(s: &String) -> Script {
    
    todo!();
        /*
        
        */
}

pub fn script_to_asm_str(
        script:                 &Script,
        attempt_sighash_decode: Option<bool>) -> String {
    let attempt_sighash_decode: bool = attempt_sighash_decode.unwrap_or(false);

    todo!();
        /*
        
        */
}

pub fn format_script(script: &Script) -> String {
    
    todo!();
        /*
        
        */
}

pub fn script_pub_key_to_univ(
        script_pub_key:  &Script,
        out:             &mut UniValue,
        include_hex:     bool,
        include_address: Option<bool>)  {
    let include_address: bool = include_address.unwrap_or(true);

    todo!();
        /*
        
        */
}

pub fn script_to_univ(
        script: &Script,
        out:    &mut UniValue)  {
    
    todo!();
        /*
        
        */
}
