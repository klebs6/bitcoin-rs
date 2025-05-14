// ---------------- [ File: bitcoin-walletdb/tests/walletdb.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/wallet/test/walletdb_tests.cpp]

#[cfg(test)]
#[fixture(BasicTestingSetup)]
pub mod walletdb_tests {

    #[test] fn walletdb_readkeyvalue() {
        todo!();
        /*
        
            /**
             * When ReadKeyValue() reads from either a "key" or "wkey" it first reads the DataStream steam into a
             * CPrivKey or CWalletKey respectively and then reads a hash of the pubkey and privkey into a uint256.
             * Wallets from 0.8 or before do not store the pubkey/privkey hash, trying to read the hash from old
             * wallets throws an exception, for backwards compatibility this read is wrapped in a try block to
             * silently fail. The test here makes sure the type of exception thrown from DataStream::read()
             * matches the type we expect, otherwise we need to update the "key"/"wkey" exception type caught.
             */
            DataStream ssValue(SER_DISK, CLIENT_VERSION);
            uint256 dummy;
            BOOST_CHECK_THROW(ssValue >> dummy, std::ios_base::failure);

        */
    }
}
