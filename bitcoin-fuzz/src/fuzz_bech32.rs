// ---------------- [ File: bitcoin-fuzz/src/fuzz_bech32.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/bech32.cpp]

#[fuzz_test] fn bech32() {
    todo!();
    /*
    
        const std::string random_string(buffer.begin(), buffer.end());
        const auto r1 = bech32::Decode(random_string);
        if (r1.hrp.empty()) {
            assert(r1.encoding == bech32::Encoding::INVALID);
            assert(r1.data.empty());
        } else {
            assert(r1.encoding != bech32::Encoding::INVALID);
            const std::string reencoded = bech32::Encode(r1.encoding, r1.hrp, r1.data);
            assert(CaseInsensitiveEqual(random_string, reencoded));
        }

        std::vector<unsigned char> input;
        ConvertBits<8, 5, true>([&](unsigned char c) { input.push_back(c); }, buffer.begin(), buffer.end());

        if (input.size() + 3 + 6 <= 90) {
            // If it's possible to encode input in Bech32(m) without exceeding the 90-character limit:
            for (auto encoding : {bech32::Encoding::BECH32, bech32::Encoding::BECH32M}) {
                const std::string encoded = bech32::Encode(encoding, "bc", input);
                assert(!encoded.empty());
                const auto r2 = bech32::Decode(encoded);
                assert(r2.encoding == encoding);
                assert(r2.hrp == "bc");
                assert(r2.data == input);
            }
        }

    */
}
