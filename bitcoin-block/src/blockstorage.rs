crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/node/blockstorage.h]

pub const DEFAULT_STOPAFTERBLOCKIMPORT: bool = false;

/**
  | The pre-allocation chunk size for blk?????.dat
  | files (since 0.8)
  |
  */
pub const BLOCKFILE_CHUNK_SIZE: u32 = 0x1000000; // 16 MiB

/**
  | The pre-allocation chunk size for rev?????.dat
  | files (since 0.8)
  |
  */
pub const UNDOFILE_CHUNK_SIZE: u32 = 0x100000; // 1 MiB

/**
  | The maximum size of a blk?????.dat file
  | (since 0.8)
  |
  */
pub const MAX_BLOCKFILE_SIZE: u32 = 0x8000000; // 128 MiB

//-------------------------------------------[.cpp/bitcoin/src/node/blockstorage.cpp]


/**
  | Check whether the block associated
  | with this index entry is pruned or not.
  |
  */
pub fn is_block_pruned(pblockindex: *const BlockIndex) -> bool {
    
    todo!();
        /*
            return (fHavePruned && !(pblockindex->nStatus & BLOCK_HAVE_DATA) && pblockindex->nTx > 0);
        */
}

/**
  | If we're using -prune with -reindex, then
  | delete block files that will be ignored by the
  | reindex.  
  |
  | Since reindexing works by starting at block
  | file 0 and looping until a blockfile is
  | missing, do the same here to delete any later
  | block files after a gap.  
  |
  | Also delete all rev files since they'll be
  | rewritten by the reindex anyway.  
  |
  | This ensures that vinfoBlockFile is in sync
  | with what's actually on disk by the time we
  | start downloading, so that pruning works
  | correctly.
  */
pub fn cleanup_block_rev_files()  {
    
    todo!();
        /*
            std::map<std::string, fs::path> mapBlockFiles;

        // Glob all blk?????.dat and rev?????.dat files from the blocks directory.
        // Remove the rev files immediately and insert the blk file paths into an
        // ordered map keyed by block file index.
        LogPrintf("Removing unusable blk?????.dat and rev?????.dat files for -reindex with -prune\n");
        fs::path blocksdir = gArgs.GetBlocksDirPath();
        for (fs::directory_iterator it(blocksdir); it != fs::directory_iterator(); it++) {
            const std::string path = fs::PathToString(it->path().filename());
            if (fs::is_regular_file(*it) &&
                path.length() == 12 &&
                path.substr(8,4) == ".dat")
            {
                if (path.substr(0, 3) == "blk") {
                    mapBlockFiles[path.substr(3, 5)] = it->path();
                } else if (path.substr(0, 3) == "rev") {
                    remove(it->path());
                }
            }
        }

        // Remove all block files that aren't part of a contiguous set starting at
        // zero by walking the ordered map (keys are block file indices) by
        // keeping a separate counter.  Once we hit a gap (or if 0 doesn't exist)
        // start removing block files.
        int nContigCounter = 0;
        for (const std::pair<const std::string, fs::path>& item : mapBlockFiles) {
            if (LocaleIndependentAtoi<int>(item.first) == nContigCounter) {
                nContigCounter++;
                continue;
            }
            remove(item.second);
        }
        */
}

/**
  | Get block file info entry for one block
  | file
  |
  */
pub fn get_block_file_info(n: usize) -> *mut BlockFileInfo {
    
    todo!();
        /*
            LOCK(cs_LastBlockFile);

        return &vinfoBlockFile.at(n);
        */
}

pub fn undo_write_to_disk(
        blockundo:     &BlockUndo,
        pos:           &mut FlatFilePos,
        hash_block:    &u256,
        message_start: &message_header::MessageStartChars) -> bool {
    
    todo!();
        /*
            // Open history file to append
        CAutoFile fileout(OpenUndoFile(pos), SER_DISK, CLIENT_VERSION);
        if (fileout.IsNull()) {
            return error("%s: OpenUndoFile failed", __func__);
        }

        // Write index header
        unsigned int nSize = GetSerializeSize(blockundo, fileout.GetVersion());
        fileout << messageStart << nSize;

        // Write undo data
        long fileOutPos = ftell(fileout.Get());
        if (fileOutPos < 0) {
            return error("%s: ftell failed", __func__);
        }
        pos.nPos = (unsigned int)fileOutPos;
        fileout << blockundo;

        // calculate & write checksum
        CHashWriter hasher(SER_GETHASH, PROTOCOL_VERSION);
        hasher << hashBlock;
        hasher << blockundo;
        fileout << hasher.GetHash();

        return true;
        */
}

pub fn undo_read_from_disk(
        blockundo: &mut BlockUndo,
        pindex:    *const BlockIndex) -> bool {
    
    todo!();
        /*
            FlatFilePos pos = pindex->GetUndoPos();
        if (pos.IsNull()) {
            return error("%s: no undo data available", __func__);
        }

        // Open history file to read
        CAutoFile filein(OpenUndoFile(pos, true), SER_DISK, CLIENT_VERSION);
        if (filein.IsNull()) {
            return error("%s: OpenUndoFile failed", __func__);
        }

        // Read block
        uint256 hashChecksum;
        CHashVerifier<CAutoFile> verifier(&filein); // We need a CHashVerifier as reserializing may lose data
        try {
            verifier << pindex->pprev->GetBlockHash();
            verifier >> blockundo;
            filein >> hashChecksum;
        } catch (const std::exception& e) {
            return error("%s: Deserialize or I/O error - %s", __func__, e.what());
        }

        // Verify checksum
        if (hashChecksum != verifier.GetHash()) {
            return error("%s: Checksum mismatch", __func__);
        }

        return true;
        */
}

pub fn flush_undo_file(
        block_file: i32,
        finalize:   Option<bool>)  {
    let finalize: bool = finalize.unwrap_or(false);

    todo!();
        /*
            FlatFilePos undo_pos_old(block_file, vinfoBlockFile[block_file].nUndoSize);
        if (!UndoFileSeq().Flush(undo_pos_old, finalize)) {
            AbortNode("Flushing undo file to disk failed. This is likely the result of an I/O error.");
        }
        */
}

pub fn flush_block_file(
        finalize:      Option<bool>,
        finalize_undo: Option<bool>)  {

    let finalize:      bool = finalize.unwrap_or(false);
    let finalize_undo: bool = finalize_undo.unwrap_or(false);

    todo!();
        /*
            LOCK(cs_LastBlockFile);
        FlatFilePos block_pos_old(nLastBlockFile, vinfoBlockFile[nLastBlockFile].nSize);
        if (!BlockFileSeq().Flush(block_pos_old, fFinalize)) {
            AbortNode("Flushing block file to disk failed. This is likely the result of an I/O error.");
        }
        // we do not always flush the undo file, as the chain tip may be lagging behind the incoming blocks,
        // e.g. during IBD or a sync after a node going offline
        if (!fFinalize || finalize_undo) FlushUndoFile(nLastBlockFile, finalize_undo);
        */
}

/**
  | Calculate the amount of disk space the
  | block & undo files currently use
  |
  */
pub fn calculate_current_usage() -> u64 {
    
    todo!();
        /*
            LOCK(cs_LastBlockFile);

        uint64_t retval = 0;
        for (const CBlockFileInfo& file : vinfoBlockFile) {
            retval += file.nSize + file.nUndoSize;
        }
        return retval;
        */
}

/**
  | Actually unlink the specified files
  |
  */
pub fn unlink_pruned_files(set_files_to_prune: &HashSet<i32>)  {
    
    todo!();
        /*
            for (std::set<int>::iterator it = setFilesToPrune.begin(); it != setFilesToPrune.end(); ++it) {
            FlatFilePos pos(*it, 0);
            fs::remove(BlockFileSeq().FileName(pos));
            fs::remove(UndoFileSeq().FileName(pos));
            LogPrint(BCLog::BLOCKSTORE, "Prune: %s deleted blk/rev (%05u)\n", __func__, *it);
        }
        */
}

pub fn block_file_seq() -> FlatFileSeq {
    
    todo!();
        /*
            return FlatFileSeq(gArgs.GetBlocksDirPath(), "blk", gArgs.GetBoolArg("-fastprune", false) ? 0x4000 /* 16kb */ : BLOCKFILE_CHUNK_SIZE);
        */
}

pub fn undo_file_seq() -> FlatFileSeq {
    
    todo!();
        /*
            return FlatFileSeq(gArgs.GetBlocksDirPath(), "rev", UNDOFILE_CHUNK_SIZE);
        */
}

/**
  | Open a block file (blk?????.dat)
  |
  */
pub fn open_block_file(
        pos:       &FlatFilePos,
        read_only: Option<bool>) -> *mut libc::FILE {
    let read_only = read_only.unwrap_or(false);
    
    todo!();
        /*
            return BlockFileSeq().Open(pos, fReadOnly);
        */
}

/**
  | Open an undo file (rev?????.dat)
  |
  */
pub fn open_undo_file(
        pos:       &FlatFilePos,
        read_only: Option<bool>) -> *mut libc::FILE {
    let read_only: bool = read_only.unwrap_or(false);
    
    todo!();
        /*
            return UndoFileSeq().Open(pos, fReadOnly);
        */
}

/**
  | Translation to a filesystem path
  |
  */
pub fn get_block_pos_filename(pos: &FlatFilePos) -> Box<Path> {
    
    todo!();
        /*
            return BlockFileSeq().FileName(pos);
        */
}

pub fn find_block_pos(
        pos:          &mut FlatFilePos,
        n_add_size:   u32,
        n_height:     u32,
        active_chain: &mut Chain,
        n_time:       u64,
        known:        Option<bool>) -> bool {
    let known: bool = known.unwrap_or(false);

    todo!();
        /*
            LOCK(cs_LastBlockFile);

        unsigned int nFile = fKnown ? pos.nFile : nLastBlockFile;
        if (vinfoBlockFile.size() <= nFile) {
            vinfoBlockFile.resize(nFile + 1);
        }

        bool finalize_undo = false;
        if (!fKnown) {
            while (vinfoBlockFile[nFile].nSize + nAddSize >= (gArgs.GetBoolArg("-fastprune", false) ? 0x10000 /* 64kb */ : MAX_BLOCKFILE_SIZE)) {
                // when the undo file is keeping up with the block file, we want to flush it explicitly
                // when it is lagging behind (more blocks arrive than are being connected), we let the
                // undo block write case handle it
                finalize_undo = (vinfoBlockFile[nFile].nHeightLast == (unsigned int)active_chain.Tip()->nHeight);
                nFile++;
                if (vinfoBlockFile.size() <= nFile) {
                    vinfoBlockFile.resize(nFile + 1);
                }
            }
            pos.nFile = nFile;
            pos.nPos = vinfoBlockFile[nFile].nSize;
        }

        if ((int)nFile != nLastBlockFile) {
            if (!fKnown) {
                LogPrint(BCLog::BLOCKSTORE, "Leaving block file %i: %s\n", nLastBlockFile, vinfoBlockFile[nLastBlockFile].ToString());
            }
            FlushBlockFile(!fKnown, finalize_undo);
            nLastBlockFile = nFile;
        }

        vinfoBlockFile[nFile].AddBlock(nHeight, nTime);
        if (fKnown) {
            vinfoBlockFile[nFile].nSize = std::max(pos.nPos + nAddSize, vinfoBlockFile[nFile].nSize);
        } else {
            vinfoBlockFile[nFile].nSize += nAddSize;
        }

        if (!fKnown) {
            bool out_of_space;
            size_t bytes_allocated = BlockFileSeq().Allocate(pos, nAddSize, out_of_space);
            if (out_of_space) {
                return AbortNode("Disk space is too low!", _("Disk space is too low!"));
            }
            if (bytes_allocated != 0 && fPruneMode) {
                fCheckForPruning = true;
            }
        }

        setDirtyFileInfo.insert(nFile);
        return true;
        */
}

pub fn find_undo_pos(
    state:      &mut BlockValidationState,
    n_file:     i32,
    pos:        &mut FlatFilePos,
    n_add_size: u32

) -> bool {
    
    todo!();
        /*
            pos.nFile = nFile;

        LOCK(cs_LastBlockFile);

        pos.nPos = vinfoBlockFile[nFile].nUndoSize;
        vinfoBlockFile[nFile].nUndoSize += nAddSize;
        setDirtyFileInfo.insert(nFile);

        bool out_of_space;
        size_t bytes_allocated = UndoFileSeq().Allocate(pos, nAddSize, out_of_space);
        if (out_of_space) {
            return AbortNode(state, "Disk space is too low!", _("Disk space is too low!"));
        }
        if (bytes_allocated != 0 && fPruneMode) {
            fCheckForPruning = true;
        }

        return true;
        */
}

pub fn write_block_to_disk(
        block:         &Block,
        pos:           &mut FlatFilePos,
        message_start: &message_header::MessageStartChars) -> bool {
    
    todo!();
        /*
            // Open history file to append
        CAutoFile fileout(OpenBlockFile(pos), SER_DISK, CLIENT_VERSION);
        if (fileout.IsNull()) {
            return error("WriteBlockToDisk: OpenBlockFile failed");
        }

        // Write index header
        unsigned int nSize = GetSerializeSize(block, fileout.GetVersion());
        fileout << messageStart << nSize;

        // Write block
        long fileOutPos = ftell(fileout.Get());
        if (fileOutPos < 0) {
            return error("WriteBlockToDisk: ftell failed");
        }
        pos.nPos = (unsigned int)fileOutPos;
        fileout << block;

        return true;
        */
}

pub fn write_undo_data_for_block(
        blockundo:   &BlockUndo,
        state:       &mut BlockValidationState,
        pindex:      *mut BlockIndex,
        chainparams: &ChainParams) -> bool {
    
    todo!();
        /*
            // Write undo information to disk
        if (pindex->GetUndoPos().IsNull()) {
            FlatFilePos _pos;
            if (!FindUndoPos(state, pindex->nFile, _pos, ::GetSerializeSize(blockundo, CLIENT_VERSION) + 40)) {
                return error("ConnectBlock(): FindUndoPos failed");
            }
            if (!UndoWriteToDisk(blockundo, _pos, pindex->pprev->GetBlockHash(), chainparams.MessageStart())) {
                return AbortNode(state, "Failed to write undo data");
            }
            // rev files are written in block height order, whereas blk files are written as blocks come in (often out of order)
            // we want to flush the rev (undo) file once we've written the last block, which is indicated by the last height
            // in the block file info as below; note that this does not catch the case where the undo writes are keeping up
            // with the block writes (usually when a synced up node is getting newly mined blocks) -- this case is caught in
            // the FindBlockPos function
            if (_pos.nFile < nLastBlockFile && static_cast<uint32_t>(pindex->nHeight) == vinfoBlockFile[_pos.nFile].nHeightLast) {
                FlushUndoFile(_pos.nFile, true);
            }

            // update nUndoPos in block index
            pindex->nUndoPos = _pos.nPos;
            pindex->nStatus |= BLOCK_HAVE_UNDO;
            setDirtyBlockIndex.insert(pindex);
        }

        return true;
        */
}

/* ----- Functions for disk access for blocks  ----- */



/**
  | Store block on disk. If dbp is non-nullptr,
  | the file is known to already reside on
  | disk
  |
  */
pub fn save_block_to_disk(
        block:        &Block,
        n_height:     i32,
        active_chain: &mut Chain,
        chainparams:  &ChainParams,
        dbp:          *const FlatFilePos) -> FlatFilePos {
    
    todo!();
        /*
            unsigned int nBlockSize = ::GetSerializeSize(block, CLIENT_VERSION);
        FlatFilePos blockPos;
        if (dbp != nullptr) {
            blockPos = *dbp;
        }
        if (!FindBlockPos(blockPos, nBlockSize + 8, nHeight, active_chain, block.GetBlockTime(), dbp != nullptr)) {
            error("%s: FindBlockPos failed", __func__);
            return FlatFilePos();
        }
        if (dbp == nullptr) {
            if (!WriteBlockToDisk(block, blockPos, chainparams.MessageStart())) {
                AbortNode("Failed to write block");
                return FlatFilePos();
            }
        }
        return blockPos;
        */
}

///-----------------------------
pub struct ImportingNow { }

impl Default for ImportingNow {
    
    fn default() -> Self {
        todo!();
        /*


            assert(fImporting == false);
            fImporting = true;
        */
    }
}

impl Drop for ImportingNow {
    fn drop(&mut self) {
        todo!();
        /*
            assert(fImporting == true);
            fImporting = false;
        */
    }
}

///-----------------------------
pub fn thread_import(
        chainman:     &mut ChainstateManager,
        import_files: Vec<Box<Path>>,
        args:         &ArgsManager)  {
    
    todo!();
        /*
            SetSyscallSandboxPolicy(SyscallSandboxPolicy::INITIALIZATION_LOAD_BLOCKS);
        ScheduleBatchPriority();

        {
            CImportingNow imp;

            // -reindex
            if (fReindex) {
                int nFile = 0;
                while (true) {
                    FlatFilePos pos(nFile, 0);
                    if (!fs::exists(GetBlockPosFilename(pos))) {
                        break; // No block files left to reindex
                    }
                    FILE* file = OpenBlockFile(pos, true);
                    if (!file) {
                        break; // This error is logged in OpenBlockFile
                    }
                    LogPrintf("Reindexing block file blk%05u.dat...\n", (unsigned int)nFile);
                    chainman.ActiveChainstate().LoadExternalBlockFile(file, &pos);
                    if (ShutdownRequested()) {
                        LogPrintf("Shutdown requested. Exit %s\n", __func__);
                        return;
                    }
                    nFile++;
                }
                
    [&]() { LOCK(::cs_main);  chainman.m_blockman.m_block_tree_db->WriteReindexing(false) }()
    ;
                fReindex = false;
                LogPrintf("Reindexing finished\n");
                // To avoid ending up in a situation without genesis block, re-try initializing (no-op if reindexing worked):
                chainman.ActiveChainstate().LoadGenesisBlock();
            }

            // -loadblock=
            for (const fs::path& path : vImportFiles) {
                FILE* file = fsbridge::fopen(path, "rb");
                if (file) {
                    LogPrintf("Importing blocks file %s...\n", fs::PathToString(path));
                    chainman.ActiveChainstate().LoadExternalBlockFile(file);
                    if (ShutdownRequested()) {
                        LogPrintf("Shutdown requested. Exit %s\n", __func__);
                        return;
                    }
                } else {
                    LogPrintf("Warning: Could not open blocks file %s\n", fs::PathToString(path));
                }
            }

            // scan for better chains in the block chain database, that are not yet connected in the active best chain

            // We can't hold cs_main during ActivateBestChain even though we're accessing
            // the chainman unique_ptrs since ABC requires us not to be holding cs_main, so retrieve
            // the relevant pointers before the ABC call.
            for (CChainState* chainstate : 
    [&]() { LOCK(::cs_main);  return chainman.GetAll() }()
    ) {
                BlockValidationState state;
                if (!chainstate->ActivateBestChain(state, nullptr)) {
                    LogPrintf("Failed to connect best block (%s)\n", state.ToString());
                    StartShutdown();
                    return;
                }
            }

            if (args.GetBoolArg("-stopafterblockimport", DEFAULT_STOPAFTERBLOCKIMPORT)) {
                LogPrintf("Stopping after block import\n");
                StartShutdown();
                return;
            }
        } // End scope of CImportingNow
        chainman.ActiveChainstate().LoadMempool(args);
        */
}
