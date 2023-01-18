crate::ix!();

/**
  | Returns an error string on failure
  |
  */
pub fn load_addrman(
        asmap:   &Vec<bool>,
        args:    &ArgsManager,
        addrman: &mut Box<AddrMan>) -> Option<BilingualStr> {
    
    todo!();
        /*
            auto check_addrman = std::clamp<int32_t>(args.GetIntArg("-checkaddrman", DEFAULT_ADDRMAN_CONSISTENCY_CHECKS), 0, 1000000);
        addrman = std::make_unique<AddrMan>(asmap, /* deterministic */ false, /* consistency_check_ratio */ check_addrman);

        int64_t nStart = GetTimeMillis();
        const auto path_addr{args.GetDataDirNet() / "peers.dat"};
        try {
            DeserializeFileDB(path_addr, *addrman, CLIENT_VERSION);
            LogPrintf("Loaded %i addresses from peers.dat  %dms\n", addrman->size(), GetTimeMillis() - nStart);
        } catch (const DbNotFoundError&) {
            // Addrman can be in an inconsistent state after failure, reset it
            addrman = std::make_unique<AddrMan>(asmap, /* deterministic */ false, /* consistency_check_ratio */ check_addrman);
            LogPrintf("Creating peers.dat because the file was not found (%s)\n", fs::quoted(fs::PathToString(path_addr)));
            DumpPeerAddresses(args, *addrman);
        } catch (const std::exception& e) {
            addrman = nullptr;
            return strprintf(_("Invalid or corrupt peers.dat (%s). If you believe this is a bug, please report it to %s. As a workaround, you can move the file (%s) out of the way (rename, move, or delete) to have a new one created on the next start."),
                             e.what(), PACKAGE_BUGREPORT, fs::quoted(fs::PathToString(path_addr)));
        }
        return std::nullopt;
        */
}




