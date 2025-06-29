// ---------------- [ File: bitcoin-network/src/ipv6_to_string.rs ]
crate::ix!();

/**
  | Return an IPv6 address text representation with
  | zero compression as described in RFC 5952 ("A
  | Recommendation for IPv6 Address Text
  | Representation").
  */
pub fn ipv6_to_string(
        a:        &[u8],
        scope_id: u32) -> String {
    
    todo!();
        /*
            assert(a.size() == ADDR_IPV6_SIZE);
        const std::array groups{
            ReadBE16(&a[0]),
            ReadBE16(&a[2]),
            ReadBE16(&a[4]),
            ReadBE16(&a[6]),
            ReadBE16(&a[8]),
            ReadBE16(&a[10]),
            ReadBE16(&a[12]),
            ReadBE16(&a[14]),
        };

        // The zero compression implementation is inspired by Rust's std::net::Ipv6Addr, see
        // https://github.com/rust-lang/rust/blob/cc4103089f40a163f6d143f06359cba7043da29b/library/std/src/net/ip.rs#L1635-L1683
        struct ZeroSpan {
            size_t start_index{0};
            size_t len{0};
        };

        // Find longest sequence of consecutive all-zero fields. Use first zero sequence if two or more
        // zero sequences of equal length are found.
        ZeroSpan longest, current;
        for (size_t i{0}; i < groups.size(); ++i) {
            if (groups[i] != 0) {
                current = {i + 1, 0};
                continue;
            }
            current.len += 1;
            if (current.len > longest.len) {
                longest = current;
            }
        }

        std::string r;
        r.reserve(39);
        for (size_t i{0}; i < groups.size(); ++i) {
            // Replace the longest sequence of consecutive all-zero fields with two colons ("::").
            if (longest.len >= 2 && i >= longest.start_index && i < longest.start_index + longest.len) {
                if (i == longest.start_index) {
                    r += "::";
                }
                continue;
            }
            r += strprintf("%s%x", ((!r.empty() && r.back() != ':') ? ":" : ""), groups[i]);
        }

        if (scope_id != 0) {
            r += strprintf("%%%u", scope_id);
        }

        return r;
        */
}
