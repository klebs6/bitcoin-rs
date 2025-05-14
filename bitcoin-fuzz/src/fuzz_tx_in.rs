// ---------------- [ File: bitcoin-fuzz/src/fuzz_tx_in.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/tx_in.cpp]

#[fuzz] fn tx_in() {
    todo!();
    /*
    
        DataStream ds(buffer, SER_NETWORK, INIT_PROTO_VERSION);
        CTxIn tx_in;
        try {
            int version;
            ds >> version;
            ds.SetVersion(version);
            ds >> tx_in;
        } catch (const std::ios_base::failure&) {
            return;
        }

        (c_void)GetTransactionInputWeight(tx_in);
        (c_void)GetVirtualTransactionInputSize(tx_in);
        (c_void)RecursiveDynamicUsage(tx_in);

        (c_void)tx_in.ToString();

    */
}
