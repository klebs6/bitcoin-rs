// ---------------- [ File: bitcoin-fuzz/src/fuzz_tx_out.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/tx_out.cpp]

#[fuzz] fn tx_out() {
    todo!();
    /*
    
        DataStream ds(buffer, SER_NETWORK, INIT_PROTO_VERSION);
        CTxOut tx_out;
        try {
            int version;
            ds >> version;
            ds.SetVersion(version);
            ds >> tx_out;
        } catch (const std::ios_base::failure&) {
            return;
        }

        const CFeeRate dust_relay_fee{DUST_RELAY_TX_FEE};
        (c_void)GetDustThreshold(tx_out, dust_relay_fee);
        (c_void)IsDust(tx_out, dust_relay_fee);
        (c_void)RecursiveDynamicUsage(tx_out);

        (c_void)tx_out.ToString();
        (c_void)tx_out.IsNull();
        tx_out.SetNull();
        assert(tx_out.IsNull());

    */
}
