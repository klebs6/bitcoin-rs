// ---------------- [ File: bitcoin-univalue/src/parse_int.rs ]
crate::ix!();

pub fn parse_int32(
        str_: &String,
        out:  *mut i32) -> bool {
    
    todo!();
        /*
            if (!ParsePrechecks(str))
            return false;
        char *endp = nullptr;
        errno = 0; // strtol will not set errno if valid
        long int n = strtol(str.c_str(), &endp, 10);
        if(out) *out = (int32_t)n;
        // Note that strtol returns a *long int*, so even if strtol doesn't report an over/underflow
        // we still have to check that the returned value is within the range of an *int32_t*. On 64-bit
        // platforms the size of these types may be different.
        return endp && *endp == 0 && !errno &&
            n >= std::numeric_limits<int32_t>::min() &&
            n <= std::numeric_limits<int32_t>::max();
        */
}

pub fn parse_int64(
        str_: &String,
        out:  *mut i64) -> bool {
    
    todo!();
        /*
            if (!ParsePrechecks(str))
            return false;
        char *endp = nullptr;
        errno = 0; // strtoll will not set errno if valid
        long long int n = strtoll(str.c_str(), &endp, 10);
        if(out) *out = (int64_t)n;
        // Note that strtoll returns a *long long int*, so even if strtol doesn't report a over/underflow
        // we still have to check that the returned value is within the range of an *int64_t*.
        return endp && *endp == 0 && !errno &&
            n >= std::numeric_limits<int64_t>::min() &&
            n <= std::numeric_limits<int64_t>::max();
        */
}
