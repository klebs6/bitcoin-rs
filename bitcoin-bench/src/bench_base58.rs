crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/base58.cpp]

#[bench]
pub fn base_58encode(b: &mut Bencher)  {
    
    todo!();
        /*
            static const std::array<unsigned char, 32> buff = {
            {
                17, 79, 8, 99, 150, 189, 208, 162, 22, 23, 203, 163, 36, 58, 147,
                227, 139, 2, 215, 100, 91, 38, 11, 141, 253, 40, 117, 21, 16, 90,
                200, 24
            }
        };
        bench.batch(buff.size()).unit("byte").run([&] {
            EncodeBase58(buff);
        });
        */
}

#[bench]
pub fn base_58check_encode(b: &mut Bencher)  {
    
    todo!();
        /*
            static const std::array<unsigned char, 32> buff = {
            {
                17, 79, 8, 99, 150, 189, 208, 162, 22, 23, 203, 163, 36, 58, 147,
                227, 139, 2, 215, 100, 91, 38, 11, 141, 253, 40, 117, 21, 16, 90,
                200, 24
            }
        };
        bench.batch(buff.size()).unit("byte").run([&] {
            EncodeBase58Check(buff);
        });
        */
}

#[bench]
pub fn base_58decode(b: &mut Bencher)  {
    
    todo!();
        /*
            const char* addr = "17VZNX1SN5NtKa8UQFxwQbFeFc3iqRYhem";
        std::vector<unsigned char> vch;
        bench.batch(strlen(addr)).unit("byte").run([&] {
            (c_void) DecodeBase58(addr, vch, 64);
        });
        */
}
