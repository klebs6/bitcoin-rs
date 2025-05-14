// ---------------- [ File: bitcoin-connman/src/dump.rs ]
crate::ix!();

pub fn serialize_filedb<Data>(
    prefix:  &str,
    path:    &Path,
    data:    &Data,
    version: i32) -> bool {

    todo!();
        /*
        // Generate random temporary filename
        uint16_t randv = 0;
        GetRandBytes((unsigned char*)&randv, sizeof(randv));
        std::string tmpfn = strprintf("%s.%04x", prefix, randv);

        // open temp output file, and associate with CAutoFile
        fs::path pathTmp = gArgs.GetDataDirNet() / tmpfn;
        FILE *file = fsbridge::fopen(pathTmp, "wb");
        CAutoFile fileout(file, SER_DISK, version);
        if (fileout.IsNull()) {
            fileout.fclose();
            remove(pathTmp);
            return error("%s: Failed to open file %s", __func__, fs::PathToString(pathTmp));
        }

        // Serialize
        if (!SerializeDB(fileout, data)) {
            fileout.fclose();
            remove(pathTmp);
            return false;
        }

        if (!FileCommit(fileout.Get())) {
            fileout.fclose();
            remove(pathTmp);
            return error("%s: Failed to flush file %s", __func__, fs::PathToString(pathTmp));
        }

        fileout.fclose();

        // replace existing file, if any, with new file
        if (!RenameOver(pathTmp, path)) {
            remove(pathTmp);
            return error("%s: Rename-into-place failed", __func__);
        }

        return true;
        */
}

pub fn dump_peer_addresses(
        args: &ArgsManager,
        addr: &AddrMan) -> bool {
    
    let mut path_addr = args.get_data_dir_net();

    path_addr.push("peers.dat");

    serialize_filedb("peers",&path_addr,addr,CLIENT_VERSION)
}

pub fn subroutine_dump_addresses(connman: Arc<Connman>)  {
    
    let n_start = Instant::now();

    dump_peer_addresses(
        &G_ARGS.lock(), 
        &connman.addrman.get()
    );

    log_print!(
        LogFlags::NET, 
        "Flushed {} addresses to peers.dat  {}ms\n", 
        connman.addrman.size(), 
        Instant::now() - n_start
    );
}
