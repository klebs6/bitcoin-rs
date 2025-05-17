// ---------------- [ File: bitcoin-u256/tests/u256.rs ]
use bitcoin_u256::*;
use bitcoin_u160::*;
use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/test/uint256_tests.cpp]

/// Example: A 32-byte array for the 256-bit R1.
pub const R1ARRAY_256: [u8; 32] = [
    0x9c, 0x52, 0x4a, 0xdb, 0xcf, 0x56, 0x11, 0x12,
    0x2b, 0x29, 0x12, 0x5e, 0x5d, 0x35, 0xd2, 0xd2,
    0x22, 0x81, 0xaa, 0xb5, 0x33, 0xf0, 0x08, 0x32,
    0xd5, 0x56, 0xb1, 0xf9, 0xea, 0xe5, 0x1d, 0x7d,
];

/// For `u160`, we need only the first 20 bytes:
pub const R1ARRAY_160: [u8; 20] = [
    0x9c, 0x52, 0x4a, 0xdb, 0xcf,
    0x56, 0x11, 0x12, 0x2b, 0x29,
    0x12, 0x5e, 0x5d, 0x35, 0xd2,
    0xd2, 0x22, 0x81, 0xaa, 0xb5,
];

/// Just a convenience if you want the hex representation as bytes:
pub const R1ARRAY_HEX: &[u8] = b"7D1DE5EAF9B156D53208F033B5AA8122D2D2355D5E12292B121156CFDB4A529C";

// Our 256-bit "R1L" constant:
pub const R1L: u256 = u256::from_bytes_32(R1ARRAY_256);

// Our 160-bit "R1S" constant:
pub const R1S: u160 = u160::from_bytes_20(R1ARRAY_160);

/// Similarly for the second array:
pub const R2ARRAY_256: [u8; 32] = [
    0x70, 0x32, 0x1d, 0x7c, 0x47, 0xa5, 0x6b, 0x40,
    0x26, 0x7e, 0x0a, 0xc3, 0xa6, 0x9c, 0xb6, 0xbf,
    0x13, 0x30, 0x47, 0xa3, 0x19, 0x2d, 0xda, 0x71,
    0x49, 0x13, 0x72, 0xf0, 0xb4, 0xca, 0x81, 0xd7,
];

pub const R2ARRAY_160: [u8; 20] = [
    0x70, 0x32, 0x1d, 0x7c, 0x47,
    0xa5, 0x6b, 0x40, 0x26, 0x7e,
    0x0a, 0xc3, 0xa6, 0x9c, 0xb6,
    0xbf, 0x13, 0x30, 0x47, 0xa3,
];

pub const R2L: u256 = u256::from_bytes_32(R2ARRAY_256);
pub const R2S: u160 = u160::from_bytes_20(R2ARRAY_160);

// Zero, One, Max as 32-byte and 20-byte arrays:

pub const ZERO_ARRAY_256: [u8; 32] = [0u8; 32];
pub const ZERO_ARRAY_160: [u8; 20] = [0u8; 20];
pub const ZEROL: u256 = u256::from_bytes_32(ZERO_ARRAY_256);
pub const ZEROS: u160 = u160::from_bytes_20(ZERO_ARRAY_160);

pub const ONE_ARRAY_256: [u8; 32] = [
    0x01, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];
pub const ONE_ARRAY_160: [u8; 20] = [
    0x01, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];
pub const ONEL: u256 = u256::from_bytes_32(ONE_ARRAY_256);
pub const ONES: u160 = u160::from_bytes_20(ONE_ARRAY_160);

pub const MAX_ARRAY_256: [u8; 32] = [0xFFu8; 32];
pub const MAX_ARRAY_160: [u8; 20] = [0xFFu8; 20];
pub const MAXL: u256 = u256::from_bytes_32(MAX_ARRAY_256);
pub const MAXS: u160 = u160::from_bytes_20(MAX_ARRAY_160);

pub fn array_to_string(
        a:     &[u8],
        width: u32) -> String {
    
    todo!();
        /*
            std::stringstream Stream;
            Stream << std::hex;
            for (unsigned int i = 0; i < width; ++i)
            {
                Stream<<std::setw(2)<<std::setfill('0')<<(unsigned int)A[width-i-1];
            }
            return Stream.str();
        */
}

/**
  | constructors, equality, inequality
  |
  */
#[test] fn basics () {
    todo!();
    /*
    
        BOOST_CHECK(1 == 0+1);
        // constructor uint256(vector<char>):
        BOOST_CHECK(R1L.ToString() == ArrayToString(R1Array,32));
        BOOST_CHECK(R1S.ToString() == ArrayToString(R1Array,20));
        BOOST_CHECK(R2L.ToString() == ArrayToString(R2Array,32));
        BOOST_CHECK(R2S.ToString() == ArrayToString(R2Array,20));
        BOOST_CHECK(ZeroL.ToString() == ArrayToString(ZeroArray,32));
        BOOST_CHECK(ZeroS.ToString() == ArrayToString(ZeroArray,20));
        BOOST_CHECK(OneL.ToString() == ArrayToString(OneArray,32));
        BOOST_CHECK(OneS.ToString() == ArrayToString(OneArray,20));
        BOOST_CHECK(MaxL.ToString() == ArrayToString(MaxArray,32));
        BOOST_CHECK(MaxS.ToString() == ArrayToString(MaxArray,20));
        BOOST_CHECK(OneL.ToString() != ArrayToString(ZeroArray,32));
        BOOST_CHECK(OneS.ToString() != ArrayToString(ZeroArray,20));

        // == and !=
        BOOST_CHECK(R1L != R2L && R1S != R2S);
        BOOST_CHECK(ZeroL != OneL && ZeroS != OneS);
        BOOST_CHECK(OneL != ZeroL && OneS != ZeroS);
        BOOST_CHECK(MaxL != ZeroL && MaxS != ZeroS);

        // String Constructor and Copy Constructor
        BOOST_CHECK(uint256S("0x"+R1L.ToString()) == R1L);
        BOOST_CHECK(uint256S("0x"+R2L.ToString()) == R2L);
        BOOST_CHECK(uint256S("0x"+ZeroL.ToString()) == ZeroL);
        BOOST_CHECK(uint256S("0x"+OneL.ToString()) == OneL);
        BOOST_CHECK(uint256S("0x"+MaxL.ToString()) == MaxL);
        BOOST_CHECK(uint256S(R1L.ToString()) == R1L);
        BOOST_CHECK(uint256S("   0x"+R1L.ToString()+"   ") == R1L);
        BOOST_CHECK(uint256S("") == ZeroL);
        BOOST_CHECK(R1L == uint256S(R1ArrayHex));
        BOOST_CHECK(uint256(R1L) == R1L);
        BOOST_CHECK(uint256(ZeroL) == ZeroL);
        BOOST_CHECK(uint256(OneL) == OneL);

        BOOST_CHECK(uint160S("0x"+R1S.ToString()) == R1S);
        BOOST_CHECK(uint160S("0x"+R2S.ToString()) == R2S);
        BOOST_CHECK(uint160S("0x"+ZeroS.ToString()) == ZeroS);
        BOOST_CHECK(uint160S("0x"+OneS.ToString()) == OneS);
        BOOST_CHECK(uint160S("0x"+MaxS.ToString()) == MaxS);
        BOOST_CHECK(uint160S(R1S.ToString()) == R1S);
        BOOST_CHECK(uint160S("   0x"+R1S.ToString()+"   ") == R1S);
        BOOST_CHECK(uint160S("") == ZeroS);
        BOOST_CHECK(R1S == uint160S(R1ArrayHex));

        BOOST_CHECK(u160(R1S) == R1S);
        BOOST_CHECK(u160(ZeroS) == ZeroS);
        BOOST_CHECK(u160(OneS) == OneS);

    */
}

/**
  | <= >= < >
  |
  */
#[test] fn comparison () {
    todo!();
    /*
    
        uint256 LastL;
        for (int i = 255; i >= 0; --i) {
            uint256 TmpL;
            *(TmpL.begin() + (i>>3)) |= 1<<(7-(i&7));
            BOOST_CHECK( LastL < TmpL );
            LastL = TmpL;
        }

        BOOST_CHECK( ZeroL < R1L );
        BOOST_CHECK( R2L < R1L );
        BOOST_CHECK( ZeroL < OneL );
        BOOST_CHECK( OneL < MaxL );
        BOOST_CHECK( R1L < MaxL );
        BOOST_CHECK( R2L < MaxL );

        u160 LastS;
        for (int i = 159; i >= 0; --i) {
            u160 TmpS;
            *(TmpS.begin() + (i>>3)) |= 1<<(7-(i&7));
            BOOST_CHECK( LastS < TmpS );
            LastS = TmpS;
        }
        BOOST_CHECK( ZeroS < R1S );
        BOOST_CHECK( R2S < R1S );
        BOOST_CHECK( ZeroS < OneS );
        BOOST_CHECK( OneS < MaxS );
        BOOST_CHECK( R1S < MaxS );
        BOOST_CHECK( R2S < MaxS );

    */
}

/**
  | GetHex SetHex begin() end() size()
  | GetLow64 GetSerializeSize, Serialize, 
  | Unserialize
  |
  */
#[test] fn methods () {
    todo!();
    /*
    
        BOOST_CHECK(R1L.GetHex() == R1L.ToString());
        BOOST_CHECK(R2L.GetHex() == R2L.ToString());
        BOOST_CHECK(OneL.GetHex() == OneL.ToString());
        BOOST_CHECK(MaxL.GetHex() == MaxL.ToString());
        uint256 TmpL(R1L);
        BOOST_CHECK(TmpL == R1L);
        TmpL.SetHex(R2L.ToString());   BOOST_CHECK(TmpL == R2L);
        TmpL.SetHex(ZeroL.ToString()); BOOST_CHECK(TmpL == uint256());

        TmpL.SetHex(R1L.ToString());
        BOOST_CHECK(memcmp(R1L.begin(), R1Array, 32)==0);
        BOOST_CHECK(memcmp(TmpL.begin(), R1Array, 32)==0);
        BOOST_CHECK(memcmp(R2L.begin(), R2Array, 32)==0);
        BOOST_CHECK(memcmp(ZeroL.begin(), ZeroArray, 32)==0);
        BOOST_CHECK(memcmp(OneL.begin(), OneArray, 32)==0);
        BOOST_CHECK(R1L.size() == sizeof(R1L));
        BOOST_CHECK(sizeof(R1L) == 32);
        BOOST_CHECK(R1L.size() == 32);
        BOOST_CHECK(R2L.size() == 32);
        BOOST_CHECK(ZeroL.size() == 32);
        BOOST_CHECK(MaxL.size() == 32);
        BOOST_CHECK(R1L.begin() + 32 == R1L.end());
        BOOST_CHECK(R2L.begin() + 32 == R2L.end());
        BOOST_CHECK(OneL.begin() + 32 == OneL.end());
        BOOST_CHECK(MaxL.begin() + 32 == MaxL.end());
        BOOST_CHECK(TmpL.begin() + 32 == TmpL.end());
        BOOST_CHECK(GetSerializeSize(R1L, PROTOCOL_VERSION) == 32);
        BOOST_CHECK(GetSerializeSize(ZeroL, PROTOCOL_VERSION) == 32);

        DataStream ss(0, PROTOCOL_VERSION);
        ss << R1L;
        BOOST_CHECK(ss.str() == std::string(R1Array,R1Array+32));
        ss >> TmpL;
        BOOST_CHECK(R1L == TmpL);
        ss.clear();
        ss << ZeroL;
        BOOST_CHECK(ss.str() == std::string(ZeroArray,ZeroArray+32));
        ss >> TmpL;
        BOOST_CHECK(ZeroL == TmpL);
        ss.clear();
        ss << MaxL;
        BOOST_CHECK(ss.str() == std::string(MaxArray,MaxArray+32));
        ss >> TmpL;
        BOOST_CHECK(MaxL == TmpL);
        ss.clear();

        BOOST_CHECK(R1S.GetHex() == R1S.ToString());
        BOOST_CHECK(R2S.GetHex() == R2S.ToString());
        BOOST_CHECK(OneS.GetHex() == OneS.ToString());
        BOOST_CHECK(MaxS.GetHex() == MaxS.ToString());
        u160 TmpS(R1S);
        BOOST_CHECK(TmpS == R1S);
        TmpS.SetHex(R2S.ToString());   BOOST_CHECK(TmpS == R2S);
        TmpS.SetHex(ZeroS.ToString()); BOOST_CHECK(TmpS == u160());

        TmpS.SetHex(R1S.ToString());
        BOOST_CHECK(memcmp(R1S.begin(), R1Array, 20)==0);
        BOOST_CHECK(memcmp(TmpS.begin(), R1Array, 20)==0);
        BOOST_CHECK(memcmp(R2S.begin(), R2Array, 20)==0);
        BOOST_CHECK(memcmp(ZeroS.begin(), ZeroArray, 20)==0);
        BOOST_CHECK(memcmp(OneS.begin(), OneArray, 20)==0);
        BOOST_CHECK(R1S.size() == sizeof(R1S));
        BOOST_CHECK(sizeof(R1S) == 20);
        BOOST_CHECK(R1S.size() == 20);
        BOOST_CHECK(R2S.size() == 20);
        BOOST_CHECK(ZeroS.size() == 20);
        BOOST_CHECK(MaxS.size() == 20);
        BOOST_CHECK(R1S.begin() + 20 == R1S.end());
        BOOST_CHECK(R2S.begin() + 20 == R2S.end());
        BOOST_CHECK(OneS.begin() + 20 == OneS.end());
        BOOST_CHECK(MaxS.begin() + 20 == MaxS.end());
        BOOST_CHECK(TmpS.begin() + 20 == TmpS.end());
        BOOST_CHECK(GetSerializeSize(R1S, PROTOCOL_VERSION) == 20);
        BOOST_CHECK(GetSerializeSize(ZeroS, PROTOCOL_VERSION) == 20);

        ss << R1S;
        BOOST_CHECK(ss.str() == std::string(R1Array,R1Array+20));
        ss >> TmpS;
        BOOST_CHECK(R1S == TmpS);
        ss.clear();
        ss << ZeroS;
        BOOST_CHECK(ss.str() == std::string(ZeroArray,ZeroArray+20));
        ss >> TmpS;
        BOOST_CHECK(ZeroS == TmpS);
        ss.clear();
        ss << MaxS;
        BOOST_CHECK(ss.str() == std::string(MaxArray,MaxArray+20));
        ss >> TmpS;
        BOOST_CHECK(MaxS == TmpS);
        ss.clear();

    */
}

#[test] fn conversion () {
    todo!();
    /*
    
        BOOST_CHECK(ArithToUint256(UintToArith256(ZeroL)) == ZeroL);
        BOOST_CHECK(ArithToUint256(UintToArith256(OneL)) == OneL);
        BOOST_CHECK(ArithToUint256(UintToArith256(R1L)) == R1L);
        BOOST_CHECK(ArithToUint256(UintToArith256(R2L)) == R2L);
        BOOST_CHECK(UintToArith256(ZeroL) == 0);
        BOOST_CHECK(UintToArith256(OneL) == 1);
        BOOST_CHECK(ArithToUint256(0) == ZeroL);
        BOOST_CHECK(ArithToUint256(1) == OneL);
        BOOST_CHECK(arith_uint256(R1L.GetHex()) == UintToArith256(R1L));
        BOOST_CHECK(arith_uint256(R2L.GetHex()) == UintToArith256(R2L));
        BOOST_CHECK(R1L.GetHex() == UintToArith256(R1L).GetHex());
        BOOST_CHECK(R2L.GetHex() == UintToArith256(R2L).GetHex());

    */
}

#[test] fn operator_with_self () {
    todo!();
    /*
    
        arith_uint256 v = UintToArith256(uint256S("02"));
        v *= v;
        BOOST_CHECK(v == UintToArith256(uint256S("04")));
        v /= v;
        BOOST_CHECK(v == UintToArith256(uint256S("01")));
        v += v;
        BOOST_CHECK(v == UintToArith256(uint256S("02")));
        v -= v;
        BOOST_CHECK(v == UintToArith256(uint256S("0")));

    */
}

#[test] fn check_one () {
    todo!();
    /*
    
        uint256 one = uint256S("0000000000000000000000000000000000000000000000000000000000000001");
        BOOST_CHECK_EQUAL(one, uint256::ONE);

    */
}
