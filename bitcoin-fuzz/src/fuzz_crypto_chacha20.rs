crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/crypto_chacha20.cpp]

#[fuzz_test] fn crypto_chacha20() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider{buffer.data(), buffer.size()};

        ChaCha20 chacha20;
        if (fuzzed_data_provider.ConsumeBool()) {
            const std::vector<unsigned char> key = ConsumeFixedLengthByteVector(fuzzed_data_provider, fuzzed_data_provider.ConsumeIntegralInRange<size_t>(16, 32));
            chacha20 = ChaCha20{key.data(), key.size()};
        }
        while (fuzzed_data_provider.ConsumeBool()) {
            CallOneOf(
                fuzzed_data_provider,
                [&] {
                    const std::vector<unsigned char> key = ConsumeFixedLengthByteVector(fuzzed_data_provider, fuzzed_data_provider.ConsumeIntegralInRange<size_t>(16, 32));
                    chacha20.SetKey(key.data(), key.size());
                },
                [&] {
                    chacha20.SetIV(fuzzed_data_provider.ConsumeIntegral<uint64_t>());
                },
                [&] {
                    chacha20.Seek(fuzzed_data_provider.ConsumeIntegral<uint64_t>());
                },
                [&] {
                    std::vector<uint8_t> output(fuzzed_data_provider.ConsumeIntegralInRange<size_t>(0, 4096));
                    chacha20.Keystream(output.data(), output.size());
                },
                [&] {
                    std::vector<uint8_t> output(fuzzed_data_provider.ConsumeIntegralInRange<size_t>(0, 4096));
                    const std::vector<uint8_t> input = ConsumeFixedLengthByteVector(fuzzed_data_provider, output.size());
                    chacha20.Crypt(input.data(), output.data(), input.size());
                });
        }

    */
}
