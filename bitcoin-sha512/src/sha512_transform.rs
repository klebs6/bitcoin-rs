// ---------------- [ File: bitcoin-sha512/src/sha512_transform.rs ]
crate::ix!();

/**
  | Perform one SHA-512 transformation,
  | processing a 128-byte chunk.
  |
  */
pub fn sha512_transform(s: *mut u64, chunk: *const u8) {
    // Load state
    let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h);
    unsafe {
        let st = core::slice::from_raw_parts_mut(s, 8);
        a = st[0]; b = st[1]; c = st[2]; d = st[3];
        e = st[4]; f = st[5]; g = st[6]; h = st[7];
    }

    // Message schedule words that we keep cycling
    let (mut w0, mut w1, mut w2, mut w3, mut w4, mut w5, mut w6, mut w7,
        mut w8, mut w9, mut w10, mut w11, mut w12, mut w13, mut w14, mut w15);

    // The explicit flow below mirrors the C++ exactly (no loops),
    // preserving the order of operations.
    // ---- First 16 rounds: direct loads
    w0  = read_be64(unsafe { chunk.add( 0) });  sha512_round(a,b,c,&mut d,e,f,g,&mut h,0x428a2f98d728ae22u64, w0);
    w1  = read_be64(unsafe { chunk.add( 8) });  sha512_round(h,a,b,&mut c,d,e,f,&mut g,0x7137449123ef65cdu64, w1);
    w2  = read_be64(unsafe { chunk.add(16) });  sha512_round(g,h,a,&mut b,c,d,e,&mut f,0xb5c0fbcfec4d3b2fu64, w2);
    w3  = read_be64(unsafe { chunk.add(24) });  sha512_round(f,g,h,&mut a,b,c,d,&mut e,0xe9b5dba58189dbbcu64, w3);
    w4  = read_be64(unsafe { chunk.add(32) });  sha512_round(e,f,g,&mut h,a,b,c,&mut d,0x3956c25bf348b538u64, w4);
    w5  = read_be64(unsafe { chunk.add(40) });  sha512_round(d,e,f,&mut g,h,a,b,&mut c,0x59f111f1b605d019u64, w5);
    w6  = read_be64(unsafe { chunk.add(48) });  sha512_round(c,d,e,&mut f,g,h,a,&mut b,0x923f82a4af194f9bu64, w6);
    w7  = read_be64(unsafe { chunk.add(56) });  sha512_round(b,c,d,&mut e,f,g,h,&mut a,0xab1c5ed5da6d8118u64, w7);
    w8  = read_be64(unsafe { chunk.add(64) });  sha512_round(a,b,c,&mut d,e,f,g,&mut h,0xd807aa98a3030242u64, w8);
    w9  = read_be64(unsafe { chunk.add(72) });  sha512_round(h,a,b,&mut c,d,e,f,&mut g,0x12835b0145706fbeu64, w9);
    w10 = read_be64(unsafe { chunk.add(80) });  sha512_round(g,h,a,&mut b,c,d,e,&mut f,0x243185be4ee4b28cu64, w10);
    w11 = read_be64(unsafe { chunk.add(88) });  sha512_round(f,g,h,&mut a,b,c,d,&mut e,0x550c7dc3d5ffb4e2u64, w11);
    w12 = read_be64(unsafe { chunk.add(96) });  sha512_round(e,f,g,&mut h,a,b,c,&mut d,0x72be5d74f27b896fu64, w12);
    w13 = read_be64(unsafe { chunk.add(104) }); sha512_round(d,e,f,&mut g,h,a,b,&mut c,0x80deb1fe3b1696b1u64, w13);
    w14 = read_be64(unsafe { chunk.add(112) }); sha512_round(c,d,e,&mut f,g,h,a,&mut b,0x9bdc06a725c71235u64, w14);
    w15 = read_be64(unsafe { chunk.add(120) }); sha512_round(b,c,d,&mut e,f,g,h,&mut a,0xc19bf174cf692694u64, w15);

    // ---- Rounds 16..31
    w0  = w0 .wrapping_add(sha512_sigma1(w14)).wrapping_add(w9).wrapping_add(sha512_sigma0(w1));  sha512_round(a,b,c,&mut d,e,f,g,&mut h,0xe49b69c19ef14ad2u64, w0);
    w1  = w1 .wrapping_add(sha512_sigma1(w15)).wrapping_add(w10).wrapping_add(sha512_sigma0(w2));  sha512_round(h,a,b,&mut c,d,e,f,&mut g,0xefbe4786384f25e3u64, w1);
    w2  = w2 .wrapping_add(sha512_sigma1(w0 )) .wrapping_add(w11).wrapping_add(sha512_sigma0(w3));  sha512_round(g,h,a,&mut b,c,d,e,&mut f,0x0fc19dc68b8cd5b5u64, w2);
    w3  = w3 .wrapping_add(sha512_sigma1(w1 )) .wrapping_add(w12).wrapping_add(sha512_sigma0(w4));  sha512_round(f,g,h,&mut a,b,c,d,&mut e,0x240ca1cc77ac9c65u64, w3);
    w4  = w4 .wrapping_add(sha512_sigma1(w2 )) .wrapping_add(w13).wrapping_add(sha512_sigma0(w5));  sha512_round(e,f,g,&mut h,a,b,c,&mut d,0x2de92c6f592b0275u64, w4);
    w5  = w5 .wrapping_add(sha512_sigma1(w3 )) .wrapping_add(w14).wrapping_add(sha512_sigma0(w6));  sha512_round(d,e,f,&mut g,h,a,b,&mut c,0x4a7484aa6ea6e483u64, w5);
    w6  = w6 .wrapping_add(sha512_sigma1(w4 )) .wrapping_add(w15).wrapping_add(sha512_sigma0(w7));  sha512_round(c,d,e,&mut f,g,h,a,&mut b,0x5cb0a9dcbd41fbd4u64, w6);
    w7  = w7 .wrapping_add(sha512_sigma1(w5 )) .wrapping_add(w0 ).wrapping_add(sha512_sigma0(w8));  sha512_round(b,c,d,&mut e,f,g,h,&mut a,0x76f988da831153b5u64, w7);
    w8  = w8 .wrapping_add(sha512_sigma1(w6 )) .wrapping_add(w1 ).wrapping_add(sha512_sigma0(w9));  sha512_round(a,b,c,&mut d,e,f,g,&mut h,0x983e5152ee66dfabu64, w8);
    w9  = w9 .wrapping_add(sha512_sigma1(w7 )) .wrapping_add(w2 ).wrapping_add(sha512_sigma0(w10)); sha512_round(h,a,b,&mut c,d,e,f,&mut g,0xa831c66d2db43210u64, w9);
    w10 = w10.wrapping_add(sha512_sigma1(w8 )) .wrapping_add(w3 ).wrapping_add(sha512_sigma0(w11)); sha512_round(g,h,a,&mut b,c,d,e,&mut f,0xb00327c898fb213fu64, w10);
    w11 = w11.wrapping_add(sha512_sigma1(w9 )) .wrapping_add(w4 ).wrapping_add(sha512_sigma0(w12)); sha512_round(f,g,h,&mut a,b,c,d,&mut e,0xbf597fc7beef0ee4u64, w11);
    w12 = w12.wrapping_add(sha512_sigma1(w10)) .wrapping_add(w5 ).wrapping_add(sha512_sigma0(w13)); sha512_round(e,f,g,&mut h,a,b,c,&mut d,0xc6e00bf33da88fc2u64, w12);
    w13 = w13.wrapping_add(sha512_sigma1(w11)) .wrapping_add(w6 ).wrapping_add(sha512_sigma0(w14)); sha512_round(d,e,f,&mut g,h,a,b,&mut c,0xd5a79147930aa725u64, w13);
    w14 = w14.wrapping_add(sha512_sigma1(w12)) .wrapping_add(w7 ).wrapping_add(sha512_sigma0(w15)); sha512_round(c,d,e,&mut f,g,h,a,&mut b,0x06ca6351e003826fu64, w14);
    w15 = w15.wrapping_add(sha512_sigma1(w13)) .wrapping_add(w8 ).wrapping_add(sha512_sigma0(w0));  sha512_round(b,c,d,&mut e,f,g,h,&mut a,0x142929670a0e6e70u64, w15);

    // ---- Rounds 32..47
    w0  = w0 .wrapping_add(sha512_sigma1(w14)).wrapping_add(w9 ).wrapping_add(sha512_sigma0(w1));  sha512_round(a,b,c,&mut d,e,f,g,&mut h,0x27b70a8546d22ffcu64, w0);
    w1  = w1 .wrapping_add(sha512_sigma1(w15)).wrapping_add(w10).wrapping_add(sha512_sigma0(w2));  sha512_round(h,a,b,&mut c,d,e,f,&mut g,0x2e1b21385c26c926u64, w1);
    w2  = w2 .wrapping_add(sha512_sigma1(w0 )) .wrapping_add(w11).wrapping_add(sha512_sigma0(w3));  sha512_round(g,h,a,&mut b,c,d,e,&mut f,0x4d2c6dfc5ac42aedu64, w2);
    w3  = w3 .wrapping_add(sha512_sigma1(w1 )) .wrapping_add(w12).wrapping_add(sha512_sigma0(w4));  sha512_round(f,g,h,&mut a,b,c,d,&mut e,0x53380d139d95b3dfu64, w3);
    w4  = w4 .wrapping_add(sha512_sigma1(w2 )) .wrapping_add(w13).wrapping_add(sha512_sigma0(w5));  sha512_round(e,f,g,&mut h,a,b,c,&mut d,0x650a73548baf63deu64, w4);
    w5  = w5 .wrapping_add(sha512_sigma1(w3 )) .wrapping_add(w14).wrapping_add(sha512_sigma0(w6));  sha512_round(d,e,f,&mut g,h,a,b,&mut c,0x766a0abb3c77b2a8u64, w5);
    w6  = w6 .wrapping_add(sha512_sigma1(w4 )) .wrapping_add(w15).wrapping_add(sha512_sigma0(w7));  sha512_round(c,d,e,&mut f,g,h,a,&mut b,0x81c2c92e47edaee6u64, w6);
    w7  = w7 .wrapping_add(sha512_sigma1(w5 )) .wrapping_add(w0 ).wrapping_add(sha512_sigma0(w8));  sha512_round(b,c,d,&mut e,f,g,h,&mut a,0x92722c851482353bu64, w7);
    w8  = w8 .wrapping_add(sha512_sigma1(w6 )) .wrapping_add(w1 ).wrapping_add(sha512_sigma0(w9));  sha512_round(a,b,c,&mut d,e,f,g,&mut h,0xa2bfe8a14cf10364u64, w8);
    w9  = w9 .wrapping_add(sha512_sigma1(w7 )) .wrapping_add(w2 ).wrapping_add(sha512_sigma0(w10)); sha512_round(h,a,b,&mut c,d,e,f,&mut g,0xa81a664bbc423001u64, w9);
    w10 = w10.wrapping_add(sha512_sigma1(w8 )) .wrapping_add(w3 ).wrapping_add(sha512_sigma0(w11)); sha512_round(g,h,a,&mut b,c,d,e,&mut f,0xc24b8b70d0f89791u64, w10);
    w11 = w11.wrapping_add(sha512_sigma1(w9 )) .wrapping_add(w4 ).wrapping_add(sha512_sigma0(w12)); sha512_round(f,g,h,&mut a,b,c,d,&mut e,0xc76c51a30654be30u64, w11);
    w12 = w12.wrapping_add(sha512_sigma1(w10)) .wrapping_add(w5 ).wrapping_add(sha512_sigma0(w13)); sha512_round(e,f,g,&mut h,a,b,c,&mut d,0xd192e819d6ef5218u64, w12);
    w13 = w13.wrapping_add(sha512_sigma1(w11)) .wrapping_add(w6 ).wrapping_add(sha512_sigma0(w14)); sha512_round(d,e,f,&mut g,h,a,b,&mut c,0xd69906245565a910u64, w13);
    w14 = w14.wrapping_add(sha512_sigma1(w12)) .wrapping_add(w7 ).wrapping_add(sha512_sigma0(w15)); sha512_round(c,d,e,&mut f,g,h,a,&mut b,0xf40e35855771202au64, w14);
    w15 = w15.wrapping_add(sha512_sigma1(w13)) .wrapping_add(w8 ).wrapping_add(sha512_sigma0(w0));  sha512_round(b,c,d,&mut e,f,g,h,&mut a,0x106aa07032bbd1b8u64, w15);

    // ---- Rounds 48..63
    w0  = w0 .wrapping_add(sha512_sigma1(w14)).wrapping_add(w9 ).wrapping_add(sha512_sigma0(w1));  sha512_round(a,b,c,&mut d,e,f,g,&mut h,0x19a4c116b8d2d0c8u64, w0);
    w1  = w1 .wrapping_add(sha512_sigma1(w15)).wrapping_add(w10).wrapping_add(sha512_sigma0(w2));  sha512_round(h,a,b,&mut c,d,e,f,&mut g,0x1e376c085141ab53u64, w1);
    w2  = w2 .wrapping_add(sha512_sigma1(w0 )) .wrapping_add(w11).wrapping_add(sha512_sigma0(w3));  sha512_round(g,h,a,&mut b,c,d,e,&mut f,0x2748774cdf8eeb99u64, w2);
    w3  = w3 .wrapping_add(sha512_sigma1(w1 )) .wrapping_add(w12).wrapping_add(sha512_sigma0(w4));  sha512_round(f,g,h,&mut a,b,c,d,&mut e,0x34b0bcb5e19b48a8u64, w3);
    w4  = w4 .wrapping_add(sha512_sigma1(w2 )) .wrapping_add(w13).wrapping_add(sha512_sigma0(w5));  sha512_round(e,f,g,&mut h,a,b,c,&mut d,0x391c0cb3c5c95a63u64, w4);
    w5  = w5 .wrapping_add(sha512_sigma1(w3 )) .wrapping_add(w14).wrapping_add(sha512_sigma0(w6));  sha512_round(d,e,f,&mut g,h,a,b,&mut c,0x4ed8aa4ae3418acbu64, w5);
    w6  = w6 .wrapping_add(sha512_sigma1(w4 )) .wrapping_add(w15).wrapping_add(sha512_sigma0(w7));  sha512_round(c,d,e,&mut f,g,h,a,&mut b,0x5b9cca4f7763e373u64, w6);
    w7  = w7 .wrapping_add(sha512_sigma1(w5 )) .wrapping_add(w0 ).wrapping_add(sha512_sigma0(w8));  sha512_round(b,c,d,&mut e,f,g,h,&mut a,0x682e6ff3d6b2b8a3u64, w7);
    w8  = w8 .wrapping_add(sha512_sigma1(w6 )) .wrapping_add(w1 ).wrapping_add(sha512_sigma0(w9));  sha512_round(a,b,c,&mut d,e,f,g,&mut h,0x748f82ee5defb2fcu64, w8);
    w9  = w9 .wrapping_add(sha512_sigma1(w7 )) .wrapping_add(w2 ).wrapping_add(sha512_sigma0(w10)); sha512_round(h,a,b,&mut c,d,e,f,&mut g,0x78a5636f43172f60u64, w9);
    w10 = w10.wrapping_add(sha512_sigma1(w8 )) .wrapping_add(w3 ).wrapping_add(sha512_sigma0(w11)); sha512_round(g,h,a,&mut b,c,d,e,&mut f,0x84c87814a1f0ab72u64, w10);
    w11 = w11.wrapping_add(sha512_sigma1(w9 )) .wrapping_add(w4 ).wrapping_add(sha512_sigma0(w12)); sha512_round(f,g,h,&mut a,b,c,d,&mut e,0x8cc702081a6439ecu64, w11);
    w12 = w12.wrapping_add(sha512_sigma1(w10)) .wrapping_add(w5 ).wrapping_add(sha512_sigma0(w13)); sha512_round(e,f,g,&mut h,a,b,c,&mut d,0x90befffa23631e28u64, w12);
    w13 = w13.wrapping_add(sha512_sigma1(w11)) .wrapping_add(w6 ).wrapping_add(sha512_sigma0(w14)); sha512_round(d,e,f,&mut g,h,a,b,&mut c,0xa4506cebde82bde9u64, w13);
    w14 = w14.wrapping_add(sha512_sigma1(w12)) .wrapping_add(w7 ).wrapping_add(sha512_sigma0(w15)); sha512_round(c,d,e,&mut f,g,h,a,&mut b,0xbef9a3f7b2c67915u64, w14);
    w15 = w15.wrapping_add(sha512_sigma1(w13)) .wrapping_add(w8 ).wrapping_add(sha512_sigma0(w0));  sha512_round(b,c,d,&mut e,f,g,h,&mut a,0xc67178f2e372532bu64, w15);

    // ---- Rounds 64..79
    w0  = w0 .wrapping_add(sha512_sigma1(w14)).wrapping_add(w9 ).wrapping_add(sha512_sigma0(w1));  sha512_round(a,b,c,&mut d,e,f,g,&mut h,0xca273eceea26619cu64, w0);
    w1  = w1 .wrapping_add(sha512_sigma1(w15)).wrapping_add(w10).wrapping_add(sha512_sigma0(w2));  sha512_round(h,a,b,&mut c,d,e,f,&mut g,0xd186b8c721c0c207u64, w1);
    w2  = w2 .wrapping_add(sha512_sigma1(w0 )) .wrapping_add(w11).wrapping_add(sha512_sigma0(w3));  sha512_round(g,h,a,&mut b,c,d,e,&mut f,0xeada7dd6cde0eb1eu64, w2);
    w3  = w3 .wrapping_add(sha512_sigma1(w1 )) .wrapping_add(w12).wrapping_add(sha512_sigma0(w4));  sha512_round(f,g,h,&mut a,b,c,d,&mut e,0xf57d4f7fee6ed178u64, w3);
    w4  = w4 .wrapping_add(sha512_sigma1(w2 )) .wrapping_add(w13).wrapping_add(sha512_sigma0(w5));  sha512_round(e,f,g,&mut h,a,b,c,&mut d,0x06f067aa72176fbau64, w4);
    w5  = w5 .wrapping_add(sha512_sigma1(w3 )) .wrapping_add(w14).wrapping_add(sha512_sigma0(w6));  sha512_round(d,e,f,&mut g,h,a,b,&mut c,0x0a637dc5a2c898a6u64, w5);
    w6  = w6 .wrapping_add(sha512_sigma1(w4 )) .wrapping_add(w15).wrapping_add(sha512_sigma0(w7));  sha512_round(c,d,e,&mut f,g,h,a,&mut b,0x113f9804bef90daeu64, w6);
    w7  = w7 .wrapping_add(sha512_sigma1(w5 )) .wrapping_add(w0 ).wrapping_add(sha512_sigma0(w8));  sha512_round(b,c,d,&mut e,f,g,h,&mut a,0x1b710b35131c471bu64, w7);
    w8  = w8 .wrapping_add(sha512_sigma1(w6 )) .wrapping_add(w1 ).wrapping_add(sha512_sigma0(w9));  sha512_round(a,b,c,&mut d,e,f,g,&mut h,0x28db77f523047d84u64, w8);
    w9  = w9 .wrapping_add(sha512_sigma1(w7 )) .wrapping_add(w2 ).wrapping_add(sha512_sigma0(w10)); sha512_round(h,a,b,&mut c,d,e,f,&mut g,0x32caab7b40c72493u64, w9);
    w10 = w10.wrapping_add(sha512_sigma1(w8 )) .wrapping_add(w3 ).wrapping_add(sha512_sigma0(w11)); sha512_round(g,h,a,&mut b,c,d,e,&mut f,0x3c9ebe0a15c9bebcu64, w10);
    w11 = w11.wrapping_add(sha512_sigma1(w9 )) .wrapping_add(w4 ).wrapping_add(sha512_sigma0(w12)); sha512_round(f,g,h,&mut a,b,c,d,&mut e,0x431d67c49c100d4cu64, w11);
    w12 = w12.wrapping_add(sha512_sigma1(w10)) .wrapping_add(w5 ).wrapping_add(sha512_sigma0(w13)); sha512_round(e,f,g,&mut h,a,b,c,&mut d,0x4cc5d4becb3e42b6u64, w12);
    w13 = w13.wrapping_add(sha512_sigma1(w11)) .wrapping_add(w6 ).wrapping_add(sha512_sigma0(w14)); sha512_round(d,e,f,&mut g,h,a,b,&mut c,0x597f299cfc657e2au64, w13);
    w14 = w14.wrapping_add(sha512_sigma1(w12)) .wrapping_add(w7 ).wrapping_add(sha512_sigma0(w15)); sha512_round(c,d,e,&mut f,g,h,a,&mut b,0x5fcb6fab3ad6faecu64, w14);
    w15 = w15.wrapping_add(sha512_sigma1(w13)) .wrapping_add(w8 ).wrapping_add(sha512_sigma0(w0));  sha512_round(b,c,d,&mut e,f,g,h,&mut a,0x6c44198c4a475817u64, w15);

    // Store back
    unsafe {
        let st = core::slice::from_raw_parts_mut(s, 8);
        st[0] = st[0].wrapping_add(a);
        st[1] = st[1].wrapping_add(b);
        st[2] = st[2].wrapping_add(c);
        st[3] = st[3].wrapping_add(d);
        st[4] = st[4].wrapping_add(e);
        st[5] = st[5].wrapping_add(f);
        st[6] = st[6].wrapping_add(g);
        st[7] = st[7].wrapping_add(h);
    }
}
