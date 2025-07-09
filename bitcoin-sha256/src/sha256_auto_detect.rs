crate::ix!();

/**
  | Autodetect the best available Sha256
  | implementation.
  | 
  | Returns the name of the implementation.
  |
  */
pub fn sha256auto_detect() -> String {
    
    todo!();
        /*
            std::string ret = "standard";
    #if defined(USE_ASM) && defined(HAVE_GETCPUID)
        bool have_sse4 = false;
        bool have_xsave = false;
        bool have_avx = false;
        bool have_avx2 = false;
        bool have_shani = false;
        bool enabled_avx = false;

        (c_void)AVXEnabled;
        (c_void)have_sse4;
        (c_void)have_avx;
        (c_void)have_xsave;
        (c_void)have_avx2;
        (c_void)have_shani;
        (c_void)enabled_avx;

        uint32_t eax, ebx, ecx, edx;
        GetCPUID(1, 0, eax, ebx, ecx, edx);
        have_sse4 = (ecx >> 19) & 1;
        have_xsave = (ecx >> 27) & 1;
        have_avx = (ecx >> 28) & 1;
        if (have_xsave && have_avx) {
            enabled_avx = AVXEnabled();
        }
        if (have_sse4) {
            GetCPUID(7, 0, eax, ebx, ecx, edx);
            have_avx2 = (ebx >> 5) & 1;
            have_shani = (ebx >> 29) & 1;
        }

    #if defined(ENABLE_SHANI) && !defined(BUILD_BITCOIN_INTERNAL)
        if (have_shani) {
            Transform = sha256_shani::Transform;
            TransformD64 = TransformD64Wrapper<sha256_shani::Transform>;
            TransformD64_2way = sha256d64_shani::Transform_2way;
            ret = "shani(1way,2way)";
            have_sse4 = false; // Disable SSE4/AVX2;
            have_avx2 = false;
        }
    #endif

        if (have_sse4) {
    #if defined(__x86_64__) || defined(__amd64__)
            Transform = sha256_sse4::Transform;
            TransformD64 = TransformD64Wrapper<sha256_sse4::Transform>;
            ret = "sse4(1way)";
    #endif
    #if defined(ENABLE_SSE41) && !defined(BUILD_BITCOIN_INTERNAL)
            TransformD64_4way = sha256d64_sse41::Transform_4way;
            ret += ",sse41(4way)";
    #endif
        }

    #if defined(ENABLE_AVX2) && !defined(BUILD_BITCOIN_INTERNAL)
        if (have_avx2 && have_avx && enabled_avx) {
            TransformD64_8way = sha256d64_avx2::Transform_8way;
            ret += ",avx2(8way)";
        }
    #endif
    #endif

        assert(SelfTest());
        return ret;
        */
}
