// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/imports.rs ]
pub(crate) use bitcoin_derive::*;
pub(crate) use bitcoin_imports::*;
pub(crate) use bitcoinsecp256k1_modinv::*;
pub(crate) use bitcoinsecp256k1_scratch::*;

#[cfg(test)]
mod imports_surface_contract_suite {
    use super::*;
    use tracing::{debug, info};

    #[traced_test]
    fn imports_module_reexports_expected_core_types_for_downstream_use() {
        info!("smoke: ensure key reexported types resolve and can be constructed");
        let s = ModInv32Signed30 { v: [0i32; 9] };
        debug!(?s.v, "ModInv32Signed30");
    }
}
