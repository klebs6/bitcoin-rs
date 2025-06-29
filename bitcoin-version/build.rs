fn main() {
    bitcoin_cfg::setup();
    println!("cargo:rustc-env=COPYRIGHT_YEAR=2025");
    println!("cargo:rustc-env=COPYRIGHT_HOLDERS_FINAL=Bitcoin Developers");
}
