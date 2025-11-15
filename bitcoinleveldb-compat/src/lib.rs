// ---------------- [ File: bitcoinleveldb-compat/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

x!{port_stdcxx}
x!{port_config}
x!{snappy_compress}
x!{snappy_uncompress}
x!{snappy_get_uncompressed_length}
x!{get_heap_profile}
x!{accelerated_crc32c}
