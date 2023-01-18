crate::ix!();

/**
  | Compact serializer for scripts.
  | 
  | It detects common cases and encodes
  | them much more efficiently. 3 special
  | cases are defined:
  | 
  | - Pay to pubkey hash (encoded as 21 bytes)
  | 
  | - Pay to script hash (encoded as 21 bytes)
  | 
  | - Pay to pubkey starting with 0x02, 0x03
  | or 0x04 (encoded as 33 bytes)
  | 
  | Other scripts up to 121 bytes require
  | 1 byte + script length. Above that, scripts
  | up to 16505 bytes require 2 bytes + script
  | length.
  |
  */
pub struct ScriptCompression {

}

/**
  | make this static for now (there are only
  | 6 special scripts defined) this can
  | potentially be extended together with
  | a new nVersion for transactions, in
  | which case this value becomes dependent
  | on nVersion and nHeight of the enclosing
  | transaction.
  |
  */
pub const SCRIPT_COMPRESSION_N_SPECIAL_SCRIPTS: u32 = 6;

impl ScriptCompression {
    
    pub fn ser<Stream: Write + VarIntWriter>(&mut self, 
        stream: &mut Stream,
        script: &Script)  {
    
        let mut compr = CompressedScript::default();

        if compress_script(script,&mut compr) {
            stream.write(&compr);
            return;
        }

        let n_size: u32 = script.len() as u32 + SCRIPT_COMPRESSION_N_SPECIAL_SCRIPTS;

        //encode_var_vec encodes in VarInt
        stream.write_varint(n_size);

        stream.write(script);
    }
    
    pub fn unser<Stream: Read + VarIntReader>(&mut self, 
        stream: &mut Stream,
        script: &mut Script)  {
    
        let mut n_size: u32 = stream.read_varint().unwrap();

        if n_size < SCRIPT_COMPRESSION_N_SPECIAL_SCRIPTS {

            let mut vch: CompressedScript = CompressedScript::new();

            vch.resize(get_special_script_size(n_size), 0x00);

            stream.read(&mut vch);

            decompress_script(script, n_size, &vch);

            return;
        }

        n_size -= SCRIPT_COMPRESSION_N_SPECIAL_SCRIPTS;

        if n_size > MAX_SCRIPT_SIZE.try_into().unwrap() {

            // Overly long script, replace with
            // a short invalid one
            script.write(&[opcode_type::OP_RETURN]);

            {
                //this block replaces the call:
                //stream.ignore(n_size);
                //
                let mut buf = vec![];

                stream
                    .take(n_size.into())
                    .read_to_end(&mut buf)
                    .unwrap(); 
            }

        } else {

            script.resize(n_size.try_into().unwrap(), Default::default());

            stream.read(script);
        }
    }
}
