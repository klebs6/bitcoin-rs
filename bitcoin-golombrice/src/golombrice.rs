crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/golombrice.h]

pub fn golomb_rice_encode<OStream>(
        bitwriter: &mut BitStreamWriter<OStream>,
        p:         u8,
        x:         u64)  {

    // Write quotient as unary-encoded: q 1's
    // followed by one 0.
    let mut q: u64 = x >> p;

    while q > 0{

        let nbits: i32 = match q <= 64 {
            true   => q as i32,
            false  => 64
        };

        bitwriter.write(!0, nbits);

        q -= {
            let nbits: u64 = nbits.try_into().unwrap();
            nbits
        };
    }

    bitwriter.write(0, 1);

    // Write the remainder in P bits. Since the
    // remainder is just the bottom P bits of x,
    // there is no need to mask first.
    bitwriter.write(x, p.into());
}

pub fn golomb_rice_decode<IStream>(
        bitreader: &mut BitStreamReader<IStream>,
        p:         u8) -> u64 {

    // Read unary-encoded quotient: q 1's followed
    // by one 0.
    let mut q: u64 = 0;;

    while bitreader.read(1) == 1{
        q += 1;
    }

    let r: u64 = bitreader.read(p.into());

    (q << p) + r
}
