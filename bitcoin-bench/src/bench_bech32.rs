crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/bech32.cpp]

#[bench]
fn bech_32encode(b: &mut Bencher)  {
    
    todo!();
        /*
        std::vector<uint8_t> v = ParseHex("c97f5a67ec381b760aeaf67573bc164845ff39a3bb26a1cee401ac67243b48db");
        std::vector<unsigned char> tmp = {0};
        tmp.reserve(1 + 32 * 8 / 5);
        ConvertBits<8, 5, true>([&](unsigned char c) { tmp.push_back(c); }, v.begin(), v.end());
        bench.batch(v.size()).unit("byte").run([&] {
            bech32::Encode(bech32::Encoding::BECH32, "bc", tmp);
        });
        */
}

#[bench]
fn bech_32decode(b: &mut Bencher)  {
    
    todo!();
        /*
        std::string addr = "bc1qkallence7tjawwvy0dwt4twc62qjgaw8f4vlhyd006d99f09";
        bench.batch(addr.size()).unit("byte").run([&] {
            bech32::Decode(addr);
        });
        */
}
