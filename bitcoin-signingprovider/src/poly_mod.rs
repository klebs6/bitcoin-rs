// ---------------- [ File: bitcoin-signingprovider/src/poly_mod.rs ]
crate::ix!();

/**
  | Interprets c as 8 groups of 5 bits which
  | are the coefficients of a degree 8 polynomial
  | over
  | 
  | GF(32), multiplies that polynomial
  | by x, computes its remainder modulo
  | a generator, and adds the constant term
  | val.
  | 
  | This generator is G(x) = x^8 + {30}x^7
  | + {23}x^6 + {15}x^5 + {14}x^4 + {10}x^3
  | + {6}x^2 + {12}x + {9}.
  | 
  | It is chosen to define an cyclic error
  | detecting code which is selected by:
  | 
  | - Starting from all BCH codes over GF(32)
  | of degree 8 and below, which by construction
  | guarantee detecting 3 errors in windows
  | up to 19000 symbols.
  | 
  | - Taking all those generators, and for
  | degree 7 ones, extend them to degree
  | 8 by adding all degree-1 factors.
  | 
  | - Selecting just the set of generators
  | that guarantee detecting 4 errors in
  | a window of length 512.
  | 
  | - Selecting one of those with best worst-case
  | behavior for 5 errors in windows of length
  | up to 512.
  | 
  | The generator and the constants to implement
  | it can be verified using this Sage code:
  | 
  | -----------
  | @code
  | 
  | B = GF(2) # Binary field
  | BP.<b> = B[] # Polynomials over the binary field
  | F_mod = b**5 + b**3 + 1
  | F.<f> = GF(32, modulus=F_mod, repr='int') # GF(32) definition
  | FP.<x> = F[] # Polynomials over GF(32)
  | E_mod = x**3 + x + F.fetch_int(8)
  | E.<e> = F.extension(E_mod) # Extension field definition
  | alpha = e**2743 # Choice of an element in extension field
  | for p in divisors(E.order() - 1): # Verify alpha has order 32767.
  |     assert((alpha**p == 1) == (p % 32767 == 0))
  | G = lcm([(alpha**i).minpoly() for i in [1056,1057,1058]] + [x + 1])
  | print(G) # Print out the generator
  | for i in [1,2,4,8,16]: # Print out {1,2,4,8,16}*(G mod x^8), packed in hex integers.
  |     v = 0
  |     for coef in reversed((F.fetch_int(i)*(G % x**8)).coefficients(sparse=True)):
  |         v = v*32 + coef.integer_representation()
  |     print("0x%x" % v)
  |
  */
pub fn poly_mod(c: u64, val: i32) -> u64 {
    
    todo!();
        /*
            uint8_t c0 = c >> 35;
        c = ((c & 0x7ffffffff) << 5) ^ val;
        if (c0 & 1) c ^= 0xf5dee51989;
        if (c0 & 2) c ^= 0xa9fdca3312;
        if (c0 & 4) c ^= 0x1bab10e32d;
        if (c0 & 8) c ^= 0x3706b1677a;
        if (c0 & 16) c ^= 0x644d626ffd;
        return c;
        */
}
