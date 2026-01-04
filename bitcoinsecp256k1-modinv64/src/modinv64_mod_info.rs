// ---------------- [ File: bitcoinsecp256k1-modinv64/src/modinv64_mod_info.rs ]
crate::ix!();

pub struct ModInv64ModInfo {

    /// The modulus in signed62 notation, must be odd and in [3, 2^256].
    ///
    pub modulus:       ModInv64Signed62,

    /// modulus^{-1} mod 2^62
    /// 
    pub modulus_inv62: u64,
}

impl ModInv64ModInfo {
    #[inline]
    pub(crate) fn modulus(&self) -> &ModInv64Signed62 {
        &self.modulus
    }

    #[inline]
    pub(crate) fn modulus_inv62(&self) -> u64 {
        self.modulus_inv62
    }

    #[cfg(test)]
    #[inline]
    pub(crate) fn from_modulus_and_inv62(modulus: ModInv64Signed62, modulus_inv62: u64) -> Self {
        Self { modulus, modulus_inv62 }
    }
}
