crate::ix!();

/// For accelerating the computation of a*G:
/// 
/// To harden against timing attacks, use the following mechanism:
/// 
/// - Break up the multiplicand into groups of PREC_B bits, called n_0, n_1, n_2, ...,
/// n_(PREC_N-1).
/// 
/// - Compute sum(n_i * (PREC_G)^i * G + U_i, i=0 ... PREC_N-1), where:
/// 
///   - U_i = U * 2^i, for i=0 ... PREC_N-2
/// 
///   - U_i = U * (1-2^(PREC_N-1)), for i=PREC_N-1 where U is a point with no known corresponding
///   scalar. Note that sum(U_i, i=0 ... PREC_N-1) = 0.
/// 
/// For each i, and each of the PREC_G possible values of n_i, (n_i * (PREC_G)^i * G + U_i) is
/// precomputed (call it prec(i, n_i)). The formula now becomes sum(prec(i, n_i), i=0
/// ... PREC_N-1).
/// 
/// None of the resulting prec group elements have a known scalar, and neither do any of the
/// intermediate sums while computing a*G.
///
#[derive(Getters)]
#[getset(get="pub")]
pub struct EcMultGenContext {

    /// prec[j][i] = (PREC_G)^j * i * G + U_i
    prec:    EcMultGenContextPrec,

    blind:   Scalar,
    initial: Gej,
}

impl EcMultGenContext {

    pub const fn new() -> Self {
        Self {
            prec:    [[null_mut(); ECMULT_GEN_PREC_N]; ECMULT_GEN_PREC_G],
            blind:   Scalar::new(),
            initial: Gej::new(),
        }
    }
}
