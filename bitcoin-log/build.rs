fn main() {
    bitcoin_cfg::setup();

    usdt::Builder::new("probes.d")
        .build()
        .expect("Failed to build USDT probes");

}
