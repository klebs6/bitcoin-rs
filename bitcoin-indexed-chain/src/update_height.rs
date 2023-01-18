crate::ix!();

pub fn maybe_update_heights(
    args:      &ArgsManager,
    consensus: &mut ChainConsensusParams)  {

    todo!();
        /*
            for (const std::string& arg : args.GetArgs("-testactivationheight")) {
            const auto found{arg.find('@')};
            if (found == std::string::npos) {
                throw std::runtime_error(strprintf("Invalid format (%s) for -testactivationheight=name@height.", arg));
            }
            const auto name{arg.substr(0, found)};
            const auto value{arg.substr(found + 1)};
            int32_t height;
            if (!ParseInt32(value, &height) || height < 0 || height >= std::numeric_limits<int>::max()) {
                throw std::runtime_error(strprintf("Invalid height value (%s) for -testactivationheight=name@height.", arg));
            }
            if (name == "segwit") {
                consensus.SegwitHeight = int{height};
            } else if (name == "bip34") {
                consensus.BIP34Height = int{height};
            } else if (name == "dersig") {
                consensus.BIP66Height = int{height};
            } else if (name == "cltv") {
                consensus.BIP65Height = int{height};
            } else if (name == "csv") {
                consensus.CSVHeight = int{height};
            } else {
                throw std::runtime_error(strprintf("Invalid name (%s) for -testactivationheight=name@height.", arg));
            }
        }
        */
}

