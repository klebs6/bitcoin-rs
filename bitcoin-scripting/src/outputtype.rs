// ---------------- [ File: bitcoin-scripting/src/outputtype.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/outputtype.h]

pub enum OutputType {
    LEGACY,
    P2SH_SEGWIT,
    BECH32,
    BECH32M,
}

lazy_static!{
    /*
    static constexpr auto OUTPUT_TYPES = std::array{
        OutputType::LEGACY,
        OutputType::P2SH_SEGWIT,
        OutputType::BECH32,
        OutputType::BECH32M,
    };
    */
}

//-------------------------------------------[.cpp/bitcoin/src/outputtype.cpp]

pub const OUTPUT_TYPE_STRING_LEGACY:      &'static str = "legacy";
pub const OUTPUT_TYPE_STRING_P2SH_SEGWIT: &'static str = "p2sh-segwit";
pub const OUTPUT_TYPE_STRING_BECH32:      &'static str = "bech32";
pub const OUTPUT_TYPE_STRING_BECH32M:     &'static str = "bech32m";

pub fn parse_output_type(ty: &String) -> Option<OutputType> {
    
    todo!();
        /*
            if (type == OUTPUT_TYPE_STRING_LEGACY) {
            return OutputType::LEGACY;
        } else if (type == OUTPUT_TYPE_STRING_P2SH_SEGWIT) {
            return OutputType::P2SH_SEGWIT;
        } else if (type == OUTPUT_TYPE_STRING_BECH32) {
            return OutputType::BECH32;
        } else if (type == OUTPUT_TYPE_STRING_BECH32M) {
            return OutputType::BECH32M;
        }
        return std::nullopt;
        */
}

pub fn format_output_type(ty: OutputType) -> &'static str {
    
    todo!();
        /*
            switch (type) {
        case OutputType::LEGACY: return OUTPUT_TYPE_STRING_LEGACY;
        case OutputType::P2SH_SEGWIT: return OUTPUT_TYPE_STRING_P2SH_SEGWIT;
        case OutputType::BECH32: return OUTPUT_TYPE_STRING_BECH32;
        case OutputType::BECH32M: return OUTPUT_TYPE_STRING_BECH32M;
        } // no default case, so the compiler can warn about missing cases
        assert(false);
        */
}
