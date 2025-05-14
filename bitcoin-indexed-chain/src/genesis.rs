// ---------------- [ File: bitcoin-indexed-chain/src/genesis.rs ]
crate::ix!();

pub fn create_genesis_block_with_output_script(
    psz_timestamp:         *const u8,
    genesis_output_script: &Script,
    n_time:                u32,
    n_nonce:               u32,
    n_bits:                u32,
    n_version:             i32,
    genesis_reward:        &Amount) -> Block 
{
    todo!();
        /*
            CMutableTransaction txNew;
        txNew.nVersion = 1;
        txNew.vin.resize(1);
        txNew.vout.resize(1);
        txNew.vin[0].scriptSig = CScript() << 486604799 << CScriptNum(4) << std::vector<unsigned char>((const unsigned char*)pszTimestamp, (const unsigned char*)pszTimestamp + strlen(pszTimestamp));
        txNew.vout[0].nValue = genesisReward;
        txNew.vout[0].scriptPubKey = genesisOutputScript;

        CBlock genesis;
        genesis.nTime    = nTime;
        genesis.nBits    = nBits;
        genesis.nNonce   = nNonce;
        genesis.nVersion = nVersion;
        genesis.vtx.push_back(MakeTransactionRef(std::move(txNew)));
        genesis.hashPrevBlock.SetNull();
        genesis.hashMerkleRoot = BlockMerkleRoot(genesis);
        return genesis;
        */
}

/**
  | Build the genesis block. Note that the
  | output of its generation transaction
  | cannot be spent since it did not originally
  | exist in the database.
  | 
  | -----------
  | @code
  | 
  | CBlock(hash=000000000019d6, ver=1, hashPrevBlock=00000000000000, hashMerkleRoot=4a5e1e, nTime=1231006505, nBits=1d00ffff, nNonce=2083236893, vtx=1)
  |   CTransaction(hash=4a5e1e, ver=1, vin.size=1, vout.size=1, nLockTime=0)
  |     CTxIn(OutPoint(000000, -1), coinbase 04ffff001d0104455468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73)
  |     CTxOut(nValue=50.00000000, scriptPubKey=0x5F1DF16B2B704C8A578D0B)
  |   vMerkleTree: 4a5e1e
  |
  */
pub fn create_genesis_block(
    n_time:         u32,
    n_nonce:        u32,
    n_bits:         u32,
    n_version:      i32,
    genesis_reward: &Amount) -> Block {

    todo!();
        /*
            const char* pszTimestamp = "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";
        const CScript genesisOutputScript = CScript() << ParseHex("04678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5f") << OP_CHECKSIG;
        return CreateGenesisBlock(pszTimestamp, genesisOutputScript, nTime, nNonce, nBits, nVersion, genesisReward);
        */
}
