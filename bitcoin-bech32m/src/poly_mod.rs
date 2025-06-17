// ---------------- [ File: bitcoin-bech32m/src/poly_mod.rs ]
crate::ix!();

/// This function will compute what 6 5-bit values to XOR into the last 6 input values, in order to
/// make the checksum 0. These
/// 6 values are packed together in a single 30-bit integer. The higher bits correspond to earlier
/// values.
/// 
/// The input is interpreted as a list of coefficients of a polynomial over F = GF(32), with an
/// implicit 1 in front. If the input is [v0,v1,v2,v3,v4], that polynomial is v(x) = 1*x^5 + v0*x^4
/// + v1*x^3 + v2*x^2 + v3*x + v4. The implicit 1 guarantees that [v0,v1,v2,...] has a distinct
/// checksum from [0,v0,v1,v2,...].
/// 
/// The output is a 30-bit integer whose 5-bit groups are the coefficients of the remainder of v(x)
/// mod g(x), where g(x) is the Bech32 generator, x^6 + {29}x^5 + {22}x^4 + {20}x^3 + {21}x^2
/// + {29}x + {18}. g(x) is chosen in such a way that the resulting code is a BCH code,
/// guaranteeing detection of up to 3 errors within a window of 1023 characters. Among the various
/// possible BCH codes, one was selected to in fact guarantee detection of up to 4 errors within
/// a window of 89 characters.
/// 
/// Note that the coefficients are elements of GF(32), here represented as decimal numbers between
/// {}. In this finite field, addition is just XOR of the corresponding numbers. For example, {27}
/// + {13} = {27 ^ 13} = {22}.
/// 
/// Multiplication is more complicated, and requires treating the bits of values themselves as
/// coefficients of a polynomial over a smaller field, GF(2), and multiplying those polynomials 
/// mod a^5 + a^3 + 1. 
///
/// For example, 
/// {5} * {26} = (a^2 + 1) * (a^4 + a^3 + a) = (a^4 + a^3 + a) * a^2 + (a^4 + a^3 + a) = a^6 + a^5 + a^4 + a = a^3 + 1 (mod a^5 + a^3 + 1) = {9}.
/// 
/// During the course of the loop below, `c` contains the bitpacked coefficients of the polynomial
/// constructed from just the values of v that were processed so far, mod g(x).
/// 
/// In the above example, `c` initially corresponds to 1 mod g(x), and after processing 2 inputs of
/// v, it corresponds to x^2 + v0*x + v1 mod g(x). As
/// 1 mod g(x) = 1, that is the starting value for `c`.
/// 
/// The following Sage code constructs the generator used:
/// 
/// B = GF(2) # Binary field
/// BP.<b> = B[] # Polynomials over the binary field
/// F_mod = b**5 + b**3 + 1
/// F.<f> = GF(32, modulus=F_mod, repr='int') # GF(32) definition
/// FP.<x> = F[] # Polynomials over GF(32)
/// E_mod = x**2 + F.fetch_int(9)*x + F.fetch_int(23)
/// E.<e> = F.extension(E_mod) # GF(1024) extension field definition
/// for p in divisors(E.order() - 1): # Verify e has order 1023.
///    assert((e**p == 1) == (p % 1023 == 0))
/// 
/// G = lcm([(e**i).minpoly() for i in range(997,1000)])
/// 
/// print(G) # Print out the generator
/// 
/// It demonstrates that g(x) is the least common multiple of the minimal polynomials of
/// 3 consecutive powers (997,998,999) of a primitive element (e) of GF(1024).
/// 
/// That guarantees it is, in fact, the generator of a primitive BCH code with cycle length 1023
/// and distance 4. See https://en.wikipedia.org/wiki/BCH_code for more details.
//
pub fn poly_mod(v: &Vec<u8>) -> u32 {

    let mut c: u32 = 1;

    // We want to update `c` to correspond to
    // a polynomial with one extra term. If the
    // initial value of `c` consists of the
    // coefficients of c(x) = f(x) mod g(x),
    // we modify it to correspond to c'(x)
    // = (f(x) * x + v_i) mod g(x), where v_i
    // is the next input to
    // process. Simplifying:
    // 
    // c'(x) = (f(x) * x + v_i) mod g(x)
    //         ((f(x) mod g(x)) * x + v_i) mod g(x)
    //         (c(x) * x + v_i) mod g(x)
    // If c(x) = c0*x^5 + c1*x^4 + c2*x^3 + c3*x^2 + c4*x + c5, we want to compute
    // c'(x) = (c0*x^5 + c1*x^4 + c2*x^3 + c3*x^2 + c4*x + c5) * x + v_i mod g(x)
    //       = c0*x^6 + c1*x^5 + c2*x^4 + c3*x^3 + c4*x^2 + c5*x + v_i mod g(x)
    //       = c0*(x^6 mod g(x)) + c1*x^5 + c2*x^4 + c3*x^3 + c4*x^2 + c5*x + v_i
    // If we call (x^6 mod g(x)) = k(x), this can be written as
    // c'(x) = (c1*x^5 + c2*x^4 + c3*x^3 + c4*x^2 + c5*x + v_i) + c0*k(x)
    //
    for v_i in v.iter() {

        // ----------------------------------------------------------------
        //  Update state for the next input coefficient
        // ----------------------------------------------------------------
        //
        // First, determine the value of c0:
        //
        let c0: u8 = (c >> 25) as u8;

        // Then compute 
        // c1*x^5 
        // + c2*x^4 
        // + c3*x^3 
        // + c4*x^2 
        // + c5*x 
        // + v_i:
        c = ((c & 0x1ff_ffff) << 5) ^ (*v_i as u32);

        // Conditional XORs with the generator multiples
        //
        // For each set bit n in c0,
        // conditionally add {2^n}k(x). 
        //
        // These constants can be computed using
        // the following Sage code (continuing the
        // code above):
        //
        // # Print out {1,2,4,8,16}*(g(x) mod
        // # x^6), packed in hex integers.
        // for i in [1,2,4,8,16]: 
        //
        //     v = 0
        //
        //     for coef in reversed((F.fetch_int(i)*(G % x**6)).coefficients(sparse=True)):
        //         v = v*32 + coef.integer_representation()
        //
        //     print("0x%x" % v)
        //
        if (c0 & 1) != 0 {

            // k(x) 
            // = {29}x^5 
            // + {22}x^4
            // + {20}x^3 
            // + {21}x^2 
            // + {29}x
            // + {18}
            c ^= 0x3b6a_57b2;
        }

        if (c0 & 2) != 0 {

            //  {2}k(x) 
            //  = {19}x^5 
            //  + {5}x^4
            //  + x^3 
            //  + {3}x^2 
            //  + {19}x 
            //  + {13}
            c ^= 0x2650_8e6d;
        }

        if (c0 & 4) != 0 {

            // {4}k(x) 
            // = {15}x^5 
            // + {10}x^4
            // + {2}x^3 
            // + {6}x^2 
            // + {15}x 
            // + {26}
            c ^= 0x1ea1_19fa;
        }

        if (c0 & 8) != 0 {

            // {8}k(x) 
            // = {30}x^5 
            // + {20}x^4
            // + {4}x^3 
            // + {12}x^2 
            // + {30}x 
            // + {29}
            c ^= 0x3d42_33dd;
        }

        if (c0 & 16) != 0 {

            // {16}k(x) 
            // = {21}x^5 
            // + x^4
            // + {8}x^3 
            // + {24}x^2 
            // + {21}x 
            // + {19}
            c ^= 0x2a14_62b3;
        }
    }
    c
}
