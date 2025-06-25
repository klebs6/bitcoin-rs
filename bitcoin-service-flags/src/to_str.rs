// ---------------- [ File: bitcoin-service-flags/src/to_str.rs ]
crate::ix!();

/**
  | Convert a service flag (NODE_*) to a
  | human readable string.
  | 
  | It supports unknown service flags which
  | will be returned as "UNKNOWN[...]".
  | 
  | -----------
  | @param[in] bit
  | 
  | the service flag is calculated as (1
  | << bit)
  |
  */
pub fn service_flag_to_str(bit: usize) -> String {
    
    todo!();
        /*
            const uint64_t service_flag = 1ULL << bit;
        switch ((ServiceFlags)service_flag) {
        case NODE_NONE: abort();  // impossible
        case NODE_NETWORK:         return "NETWORK";
        case NODE_BLOOM:           return "BLOOM";
        case NODE_WITNESS:         return "WITNESS";
        case NODE_COMPACT_FILTERS: return "COMPACT_FILTERS";
        case NODE_NETWORK_LIMITED: return "NETWORK_LIMITED";
        // Not using default, so we get warned when a case is missing
        }

        std::ostringstream stream;
        stream.imbue(std::locale::classic());
        stream << "UNKNOWN[";
        stream << "2^" << bit;
        stream << "]";
        return stream.str();
        */
}

/**
  | Convert service flags (a bitmask of
  | NODE_*) to human readable strings.
  | 
  | It supports unknown service flags which
  | will be returned as "UNKNOWN[...]".
  | 
  | -----------
  | @param[in] flags
  | 
  | multiple NODE_* bitwise-OR-ed together
  |
  */
pub fn service_flags_to_str(flags: u64) -> Vec<String> {
    
    todo!();
        /*
            std::vector<std::string> str_flags;

        for (size_t i = 0; i < sizeof(flags) * 8; ++i) {
            if (flags & (1ULL << i)) {
                str_flags.emplace_back(serviceFlagToStr(i));
            }
        }

        return str_flags;
        */
}
