crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/string.cpp]

pub fn legacy_parse_prechecks(str_: &String) -> bool {
    
    todo!();
        /*
            if (str.empty()) // No empty string allowed
            return false;
        if (str.size() >= 1 && (IsSpace(str[0]) || IsSpace(str[str.size() - 1]))) // No padding allowed
            return false;
        if (!ValidAsCString(str)) // No embedded NUL characters allowed
            return false;
        return true;
        */
}

pub fn legacy_parse_int32(
        str_: &String,
        out:  *mut i32) -> bool {
    
    todo!();
        /*
            if (!LegacyParsePrechecks(str))
            return false;
        char* endp = nullptr;
        errno = 0; // strtol will not set errno if valid
        long int n = strtol(str.c_str(), &endp, 10);
        if (out) *out = (int32_t)n;
        // Note that strtol returns a *long int*, so even if strtol doesn't report an over/underflow
        // we still have to check that the returned value is within the range of an *int32_t*. On 64-bit
        // platforms the size of these types may be different.
        return endp && *endp == 0 && !errno &&
               n >= std::numeric_limits<int32_t>::min() &&
               n <= std::numeric_limits<int32_t>::max();
        */
}

pub fn legacy_parse_int64(
        str_: &String,
        out:  *mut i64) -> bool {
    
    todo!();
        /*
            if (!LegacyParsePrechecks(str))
            return false;
        char* endp = nullptr;
        errno = 0; // strtoll will not set errno if valid
        long long int n = strtoll(str.c_str(), &endp, 10);
        if (out) *out = (int64_t)n;
        // Note that strtoll returns a *long long int*, so even if strtol doesn't report an over/underflow
        // we still have to check that the returned value is within the range of an *int64_t*.
        return endp && *endp == 0 && !errno &&
               n >= std::numeric_limits<int64_t>::min() &&
               n <= std::numeric_limits<int64_t>::max();
        */
}

pub fn legacy_parse_uint32(
        str_: &String,
        out:  *mut u32) -> bool {
    
    todo!();
        /*
            if (!LegacyParsePrechecks(str))
            return false;
        if (str.size() >= 1 && str[0] == '-') // Reject negative values, unfortunately strtoul accepts these by default if they fit in the range
            return false;
        char* endp = nullptr;
        errno = 0; // strtoul will not set errno if valid
        unsigned long int n = strtoul(str.c_str(), &endp, 10);
        if (out) *out = (uint32_t)n;
        // Note that strtoul returns a *unsigned long int*, so even if it doesn't report an over/underflow
        // we still have to check that the returned value is within the range of an *uint32_t*. On 64-bit
        // platforms the size of these types may be different.
        return endp && *endp == 0 && !errno &&
               n <= std::numeric_limits<uint32_t>::max();
        */
}

pub fn legacy_parse_uint8(
        str_: &String,
        out:  *mut u8) -> bool {
    
    todo!();
        /*
            uint32_t u32;
        if (!LegacyParseUInt32(str, &u32) || u32 > std::numeric_limits<uint8_t>::max()) {
            return false;
        }
        if (out != nullptr) {
            *out = static_cast<uint8_t>(u32);
        }
        return true;
        */
}

pub fn legacy_parse_uint64(
        str_: &String,
        out:  *mut u64) -> bool {
    
    todo!();
        /*
            if (!LegacyParsePrechecks(str))
            return false;
        if (str.size() >= 1 && str[0] == '-') // Reject negative values, unfortunately strtoull accepts these by default if they fit in the range
            return false;
        char* endp = nullptr;
        errno = 0; // strtoull will not set errno if valid
        unsigned long long int n = strtoull(str.c_str(), &endp, 10);
        if (out) *out = (uint64_t)n;
        // Note that strtoull returns a *unsigned long long int*, so even if it doesn't report an over/underflow
        // we still have to check that the returned value is within the range of an *uint64_t*.
        return endp && *endp == 0 && !errno &&
               n <= std::numeric_limits<uint64_t>::max();
        */
}

/**
  | For backwards compatibility checking.
  |
  */
pub fn atoi64_legacy(str_: &String) -> i64 {
    
    todo!();
        /*
            return strtoll(str.c_str(), nullptr, 10);
        */
}

#[fuzz_test] fn string() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        const std::string random_string_1 = fuzzed_data_provider.ConsumeRandomLengthString(32);
        const std::string random_string_2 = fuzzed_data_provider.ConsumeRandomLengthString(32);
        const std::vector<std::string> random_string_vector = ConsumeRandomLengthStringVector(fuzzed_data_provider);

        (c_void)AmountErrMsg(random_string_1, random_string_2);
        (c_void)AmountHighWarn(random_string_1);
        BlockFilterType block_filter_type;
        (c_void)BlockFilterTypeByName(random_string_1, block_filter_type);
        (c_void)Capitalize(random_string_1);
        (c_void)CopyrightHolders(random_string_1);
        FeeEstimateMode fee_estimate_mode;
        (c_void)FeeModeFromString(random_string_1, fee_estimate_mode);
        (c_void)FormatParagraph(random_string_1, fuzzed_data_provider.ConsumeIntegralInRange<size_t>(0, 1000), fuzzed_data_provider.ConsumeIntegralInRange<size_t>(0, 1000));
        (c_void)FormatSubVersion(random_string_1, fuzzed_data_provider.ConsumeIntegral<int>(), random_string_vector);
        (c_void)GetDescriptorChecksum(random_string_1);
        (c_void)HelpExampleCli(random_string_1, random_string_2);
        (c_void)HelpExampleRpc(random_string_1, random_string_2);
        (c_void)HelpMessageGroup(random_string_1);
        (c_void)HelpMessageOpt(random_string_1, random_string_2);
        (c_void)IsDeprecatedRPCEnabled(random_string_1);
        (c_void)Join(random_string_vector, random_string_1);
        (c_void)JSONRPCError(fuzzed_data_provider.ConsumeIntegral<int>(), random_string_1);
        const util::Settings settings;
        (c_void)OnlyHasDefaultSectionSetting(settings, random_string_1, random_string_2);
        (c_void)ParseNetwork(random_string_1);
        try {
            (c_void)ParseNonRFCJSONValue(random_string_1);
        } catch (const std::runtime_error&) {
        }
        (c_void)ParseOutputType(random_string_1);
        (c_void)RemovePrefix(random_string_1, random_string_2);
        (c_void)ResolveErrMsg(random_string_1, random_string_2);
        try {
            (c_void)RPCConvertNamedValues(random_string_1, random_string_vector);
        } catch (const std::runtime_error&) {
        }
        try {
            (c_void)RPCConvertValues(random_string_1, random_string_vector);
        } catch (const std::runtime_error&) {
        }
        (c_void)SanitizeString(random_string_1);
        (c_void)SanitizeString(random_string_1, fuzzed_data_provider.ConsumeIntegralInRange<int>(0, 3));
    #ifndef WIN32
        (c_void)ShellEscape(random_string_1);
    #endif // WIN32
        uint16_t port_out;
        std::string host_out;
        SplitHostPort(random_string_1, port_out, host_out);
        (c_void)TimingResistantEqual(random_string_1, random_string_2);
        (c_void)ToLower(random_string_1);
        (c_void)ToUpper(random_string_1);
        (c_void)TrimString(random_string_1);
        (c_void)TrimString(random_string_1, random_string_2);
        (c_void)urlDecode(random_string_1);
        (c_void)ValidAsCString(random_string_1);
        (c_void)_(random_string_1.c_str());
        try {
            throw scriptnum_error{random_string_1};
        } catch (const std::runtime_error&) {
        }

        {
            DataStream data_stream{SER_NETWORK, INIT_PROTO_VERSION};
            std::string s;
            auto limited_string = LIMITED_STRING(s, 10);
            data_stream << random_string_1;
            try {
                data_stream >> limited_string;
                assert(data_stream.empty());
                assert(s.size() <= random_string_1.size());
                assert(s.size() <= 10);
                if (!random_string_1.empty()) {
                    assert(!s.empty());
                }
            } catch (const std::ios_base::failure&) {
            }
        }
        {
            DataStream data_stream{SER_NETWORK, INIT_PROTO_VERSION};
            const auto limited_string = LIMITED_STRING(random_string_1, 10);
            data_stream << limited_string;
            std::string deserialized_string;
            data_stream >> deserialized_string;
            assert(data_stream.empty());
            assert(deserialized_string == random_string_1);
        }
        {
            int64_t amount_out;
            (c_void)ParseFixedPoint(random_string_1, fuzzed_data_provider.ConsumeIntegralInRange<int>(0, 1024), &amount_out);
        }
        {
            (c_void)Untranslated(random_string_1);
            const bilingual_str bs1{random_string_1, random_string_2};
            const bilingual_str bs2{random_string_2, random_string_1};
            (c_void)(bs1 + bs2);
        }
        {
            int32_t i32;
            int64_t i64;
            uint32_t u32;
            uint64_t u64;
            uint8_t u8;
            const bool ok_i32 = ParseInt32(random_string_1, &i32);
            const bool ok_i64 = ParseInt64(random_string_1, &i64);
            const bool ok_u32 = ParseUInt32(random_string_1, &u32);
            const bool ok_u64 = ParseUInt64(random_string_1, &u64);
            const bool ok_u8 = ParseUInt8(random_string_1, &u8);

            int32_t i32_legacy;
            int64_t i64_legacy;
            uint32_t u32_legacy;
            uint64_t u64_legacy;
            uint8_t u8_legacy;
            const bool ok_i32_legacy = LegacyParseInt32(random_string_1, &i32_legacy);
            const bool ok_i64_legacy = LegacyParseInt64(random_string_1, &i64_legacy);
            const bool ok_u32_legacy = LegacyParseUInt32(random_string_1, &u32_legacy);
            const bool ok_u64_legacy = LegacyParseUInt64(random_string_1, &u64_legacy);
            const bool ok_u8_legacy = LegacyParseUInt8(random_string_1, &u8_legacy);

            assert(ok_i32 == ok_i32_legacy);
            assert(ok_i64 == ok_i64_legacy);
            assert(ok_u32 == ok_u32_legacy);
            assert(ok_u64 == ok_u64_legacy);
            assert(ok_u8 == ok_u8_legacy);

            if (ok_i32) {
                assert(i32 == i32_legacy);
            }
            if (ok_i64) {
                assert(i64 == i64_legacy);
            }
            if (ok_u32) {
                assert(u32 == u32_legacy);
            }
            if (ok_u64) {
                assert(u64 == u64_legacy);
            }
            if (ok_u8) {
                assert(u8 == u8_legacy);
            }
        }

        {
            const int atoi_result = atoi(random_string_1.c_str());
            const int locale_independent_atoi_result = LocaleIndependentAtoi<int>(random_string_1);
            const int64_t atoi64_result = atoi64_legacy(random_string_1);
            const bool out_of_range = atoi64_result < std::numeric_limits<int>::min() || atoi64_result > std::numeric_limits<int>::max();
            if (out_of_range) {
                assert(locale_independent_atoi_result == 0);
            } else {
                assert(atoi_result == locale_independent_atoi_result);
            }
        }

        {
            const int64_t atoi64_result = atoi64_legacy(random_string_1);
            const int64_t locale_independent_atoi_result = LocaleIndependentAtoi<int64_t>(random_string_1);
            assert(atoi64_result == locale_independent_atoi_result || locale_independent_atoi_result == 0);
        }

    */
}
