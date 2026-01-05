// ---------------- [ File: bitcoinsecp256k1-ecmultgen/src/ecmult_gen.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/ecmult_gen.h]

pub type EcMultGenContextPrec = [[*mut GeStorage; ECMULT_GEN_PREC_N]; ECMULT_GEN_PREC_G];

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/ecmult_gen_impl.h]

/// Multiply with the generator: R = a*G
pub fn ecmult_gen(
        ctx: *const EcMultGenContext,
        r:   *mut Gej,
        gn:  *const Scalar)  {
    
    unsafe {
        let mut add: Ge = core::mem::zeroed();
        let mut adds: GeStorage = core::mem::zeroed();
        let mut gnb: Scalar = Scalar::new();
        let mut bits: u32;

        core::ptr::copy_nonoverlapping(core::ptr::addr_of!((*ctx).initial()), r, 1);

        /* Blind scalar/point multiplication by computing (n-b)G + bG instead of nG. */
        scalar_add(core::ptr::addr_of_mut!(gnb), gn, core::ptr::addr_of!((*ctx).blind()));

        for j in 0..ECMULT_GEN_PREC_N {
            bits = scalar_get_bits(
                core::ptr::addr_of!(gnb),
                (j * ECMULT_GEN_PREC_B) as u32,
                ECMULT_GEN_PREC_B as u32,
            );

            for i in 0..ECMULT_GEN_PREC_G {
                /** This uses a conditional move to avoid any secret data in array indexes.
                 *   _Any_ use of secret indexes has been demonstrated to result in timing
                 *   sidechannels, even when the cache-line access patterns are uniform.
                 *  See also:
                 *   "A word of warning", CHES 2013 Rump Session, by Daniel J. Bernstein and Peter Schwabe
                 *    (https://cryptojedi.org/peter/data/chesrump-20130822.pdf) and
                 *   "Cache Attacks and Countermeasures: the Case of AES", RSA 2006,
                 *    by Dag Arne Osvik, Adi Shamir, and Eran Tromer
                 *    (https://www.tau.ac.il/~tromer/papers/cache.pdf)
                 */
                ge_storage_cmov(
                    core::ptr::addr_of_mut!(adds),
                    core::ptr::addr_of!((*(*ctx).prec())[j][i]),
                    ((i as u32) == bits) as i32,
                );
            }

            ge_from_storage(core::ptr::addr_of_mut!(add), core::ptr::addr_of!(adds));
            gej_add_ge(r, r, core::ptr::addr_of!(add));
        }

        bits = 0;
        ge_clear(core::ptr::addr_of_mut!(add));
        scalar_clear(core::ptr::addr_of_mut!(gnb));
    }
}
