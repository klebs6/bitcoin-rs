crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/decode_tx.cpp]

#[fuzz_test] fn decode_tx() {
    todo!();
    /*
    
        const std::string tx_hex = HexStr(buffer);
        CMutableTransaction mtx;
        const bool result_none = DecodeHexTx(mtx, tx_hex, false, false);
        const bool result_try_witness = DecodeHexTx(mtx, tx_hex, false, true);
        const bool result_try_witness_and_maybe_no_witness = DecodeHexTx(mtx, tx_hex, true, true);
        CMutableTransaction no_witness_mtx;
        const bool result_try_no_witness = DecodeHexTx(no_witness_mtx, tx_hex, true, false);
        assert(!result_none);
        if (result_try_witness_and_maybe_no_witness) {
            assert(result_try_no_witness || result_try_witness);
        }
        if (result_try_no_witness) {
            assert(!no_witness_mtx.HasWitness());
            assert(result_try_witness_and_maybe_no_witness);
        }

    */
}
