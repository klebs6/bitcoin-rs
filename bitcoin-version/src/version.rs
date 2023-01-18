/*!
  | network protocol versioning
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/version.h]

pub const PACKAGE_NAME: &'static str = "bitcoin";

pub const PROTOCOL_VERSION: i32 = 70016;

/**
  | initial proto version, to be increased
  | after version/verack negotiation
  |
  */
pub const INIT_PROTO_VERSION: i32 = 209;

/**
  | disconnect from peers older than this
  | proto version
  |
  */
pub const MIN_PEER_PROTO_VERSION: i32 = 31800;

/**
  | BIP 0031, pong message, is enabled for
  | all versions AFTER this one
  |
  */
pub const BIP0031_VERSION: i32 = 60000;

/**
  | "filter*" commands are disabled without
  | NODE_BLOOM after and including this
  | version
  |
  */
pub const NO_BLOOM_VERSION: i32 = 70011;

/**
  | "sendheaders" command and announcing
  | blocks with headers starts with this
  | version
  |
  */
pub const SENDHEADERS_VERSION: i32 = 70012;

/**
  | "feefilter" tells peers to filter invs
  | to you by fee starts with this version
  |
  */
pub const FEEFILTER_VERSION: i32 = 70013;

/**
  | short-id-based block download starts
  | with this version
  |
  */
pub const SHORT_IDS_BLOCKS_VERSION: i32 = 70014;

/**
  | not banning for invalid compact blocks
  | starts with this version
  |
  */
pub const INVALID_CB_NO_BAN_VERSION: i32 = 70015;

/**
  | "wtxidrelay" command for wtxid-based
  | relay starts with this version
  |
  */
pub const WTXID_RELAY_VERSION: i32 = 70016;

/*
  | Make sure that none of the values above
  | collide with `SERIALIZE_TRANSACTION_NO_WITNESS`
  | or `ADDRV2_FORMAT`.
  |
  */
