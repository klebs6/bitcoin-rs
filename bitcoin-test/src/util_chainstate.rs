crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/util/chainstate.h]

pub type NoMalleationCallbackType = fn(file: &AutoFile, meta: &SnapshotMetadata) -> ();

pub const NoMalleation: NoMalleationCallbackType = |file: &AutoFile, meta: &SnapshotMetadata| {};

/**
  | Create and activate a UTXO snapshot,
  | optionally providing a function to
  | malleate the snapshot.
  |
  */
pub fn create_and_activate_utxo_snapshot(
        node:       &mut NodeContext,
        root:       Box<Path>,
        malleation: Option<NoMalleationCallbackType>) -> bool {

    let malleation = malleation.unwrap_or(NoMalleation);

    todo!();
        /*
            // Write out a snapshot to the test's tempdir.
        //
        int height;
        
    [&]() { LOCK(::cs_main);  height = node.chainman->ActiveHeight() }()
    ;
        fs::path snapshot_path = root / tfm::format("test_snapshot.%d.dat", height);
        FILE* outfile{fsbridge::fopen(snapshot_path, "wb")};
        CAutoFile auto_outfile{outfile, SER_DISK, CLIENT_VERSION};

        UniValue result = CreateUTXOSnapshot(node, node.chainman->ActiveChainstate(), auto_outfile);
        BOOST_TEST_MESSAGE(
            "Wrote UTXO snapshot to " << fs::PathToString(snapshot_path.make_preferred()) << ": " << result.write());

        // Read the written snapshot in and then activate it.
        //
        FILE* infile{fsbridge::fopen(snapshot_path, "rb")};
        CAutoFile auto_infile{infile, SER_DISK, CLIENT_VERSION};
        SnapshotMetadata metadata;
        auto_infile >> metadata;

        malleation(auto_infile, metadata);

        return node.chainman->ActivateSnapshot(auto_infile, metadata, /*in_memory*/ true);
        */
}
