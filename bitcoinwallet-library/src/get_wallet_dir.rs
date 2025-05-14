// ---------------- [ File: bitcoinwallet-library/src/get_wallet_dir.rs ]
crate::ix!();

/**
  | Get the path of the wallet directory.
  |
  */
pub fn get_wallet_dir() -> Box<Path> {
    
    todo!();
        /*
            fs::path path;

        if (gArgs.IsArgSet("-walletdir")) {
            path = fs::PathFromString(gArgs.GetArg("-walletdir", ""));
            if (!fs::is_directory(path)) {
                // If the path specified doesn't exist, we return the deliberately
                // invalid empty string.
                path = "";
            }
        } else {
            path = gArgs.GetDataDirNet();
            // If a wallets directory exists, use that, otherwise default to GetDataDir
            if (fs::is_directory(path / "wallets")) {
                path /= "wallets";
            }
        }

        return path;
        */
}
