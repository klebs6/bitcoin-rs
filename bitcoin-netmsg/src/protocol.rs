// ---------------- [ File: bitcoin-netmsg/src/protocol.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/protocol.h]

/**
  | Message header.
  | 
  | (4) message start.
  | 
  | (12) command.
  | 
  | (4) size.
  | 
  | (4) checksum.
  |
  */
pub struct MessageHeader {
    pch_message_start: [u8; MESSAGE_HEADER_MESSAGE_START_SIZE],
    pch_command:       [u8; MESSAGE_HEADER_COMMAND_SIZE],
    n_message_size:    u32, //u32::MAX
    pch_checksum:      [u8; MESSAGE_HEADER_CHECKSUM_SIZE],
}

pub const MESSAGE_HEADER_MESSAGE_START_SIZE:  usize = 4;
pub const MESSAGE_HEADER_COMMAND_SIZE:        usize = 12;
pub const MESSAGE_HEADER_MESSAGE_SIZE_SIZE:   usize = 4;
pub const MESSAGE_HEADER_CHECKSUM_SIZE:       usize = 4;
pub const MESSAGE_HEADER_MESSAGE_SIZE_OFFSET: usize = MESSAGE_HEADER_MESSAGE_START_SIZE  + MESSAGE_HEADER_COMMAND_SIZE;
pub const MESSAGE_HEADER_CHECKSUM_OFFSET:     usize = MESSAGE_HEADER_MESSAGE_SIZE_OFFSET + MESSAGE_HEADER_MESSAGE_SIZE_SIZE;
pub const MESSAGE_HEADER_HEADER_SIZE:         usize = MESSAGE_HEADER_MESSAGE_START_SIZE  + MESSAGE_HEADER_COMMAND_SIZE + MESSAGE_HEADER_MESSAGE_SIZE_SIZE + MESSAGE_HEADER_CHECKSUM_SIZE;

pub type MessageHeaderMessageStartChars = [u8; MESSAGE_HEADER_MESSAGE_START_SIZE];

lazy_static!{
    /*
    SERIALIZE_METHODS(CMessageHeader, obj) { 
        READWRITE(obj.pchMessageStart, obj.pchCommand, obj.nMessageSize, obj.pchChecksum); 
    }
    */
}

impl MessageHeader {

    /**
      | Construct a P2P message header from
      | message-start characters, a command
      | and the size of the message.
      | 
      | -----------
      | @note
      | 
      | Passing in a `pszCommand` longer than
      | COMMAND_SIZE will result in a run-time
      | assertion error.
      |
      */
    pub fn new(
        pch_message_start_in: &MessageHeaderMessageStartChars,
        psz_command:          *const u8,
        n_message_size_in:    u32) -> Self {
    
        todo!();
        /*
            memcpy(pchMessageStart, pchMessageStartIn, MESSAGE_START_SIZE);

        // Copy the command name
        size_t i = 0;
        for (; i < COMMAND_SIZE && pszCommand[i] != 0; ++i) pchCommand[i] = pszCommand[i];
        assert(pszCommand[i] == 0); // Assert that the command name passed in is not longer than COMMAND_SIZE

        nMessageSize = nMessageSizeIn;
        */
    }
    
    pub fn get_command(&self) -> String {
        
        todo!();
        /*
            return std::string(pchCommand, pchCommand + strnlen(pchCommand, COMMAND_SIZE));
        */
    }
    
    pub fn is_command_valid(&self) -> bool {
        
        todo!();
        /*
            // Check the command string for errors
        for (const char* p1 = pchCommand; p1 < pchCommand + COMMAND_SIZE; ++p1) {
            if (*p1 == 0) {
                // Must be all zeros after the first zero
                for (; p1 < pchCommand + COMMAND_SIZE; ++p1) {
                    if (*p1 != 0) {
                        return false;
                    }
                }
            } else if (*p1 < ' ' || *p1 > 0x7E) {
                return false;
            }
        }

        return true;
        */
    }
}

/**
  | getdata message type flags
  |
  */
pub const MSG_WITNESS_FLAG: u32 = 1 << 30;
pub const MSG_TYPE_MASK:    u32 = 0xffffffff >> 2;

/**
  | getdata / inv message types.
  | 
  | These numbers are defined by the protocol.
  | When adding a new value, be sure to mention
  | it in the respective BIP.
  |
  */
bitflags!{
    pub struct GetDataMsg: u32 {
        const UNDEFINED = 0;
        const MSG_TX    = 1;
        const MSG_BLOCK = 2;
        const MSG_WTX   = 5; // Defined in BIP 339

        /*
           | The following can only occur in getdata.
           | Invs always use TX/WTX or BLOCK.
           |
           */

        /**
          | Defined in BIP37
          |
          */
        const MSG_FILTERED_BLOCK = 3;                            

        /**
          | Defined in BIP152
          |
          */
        const MSG_CMPCT_BLOCK    = 4;                            

        /**
          | Defined in BIP144
          |
          */
        const MSG_WITNESS_BLOCK  = Self::MSG_BLOCK.bits | MSG_WITNESS_FLAG; 

        /**
          | Defined in BIP144
          |
          */
        const MSG_WITNESS_TX     = Self::MSG_TX.bits | MSG_WITNESS_FLAG;

        /*
           | MSG_FILTERED_WITNESS_BLOCK is defined in
           | BIP144 as reserved for future use and
           | remains unused.
           |
           | MSG_FILTERED_WITNESS_BLOCK
           | = MSG_FILTERED_BLOCK | MSG_WITNESS_FLAG,
           */
    }
}

/**
  | inv message data
  |
  */
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Inv {
    pub ty:   u32,
    pub hash: u256,
}

impl Into<GenTxId> for Inv {

    /**
      | Convert a TX/WITNESS_TX/WTX CInv to
      | a GenTxId.
      |
      */
    fn into(self) -> GenTxId {

        todo!();
            /*
                assert(inv.IsGenTxMsg());
            return inv.IsMsgWtx() ? GenTxId::Wtxid(inv.hash) : GenTxId::Txid(inv.hash);
            */
    }
}

impl Inv {

    pub fn new(
        type_in: u32,
        hash_in: &u256) -> Self {
    
        todo!();
        /*
        : ty(typeIn),
        : hash(hashIn),
        */
    }
    
    pub fn get_command(&self) -> String {
        
        todo!();
        /*
            std::string cmd;
        if (type & MSG_WITNESS_FLAG)
            cmd.append("witness-");
        int masked = type & MSG_TYPE_MASK;
        switch (masked)
        {
        case MSG_TX:             return cmd.append(NetMsgType::TX);
        // WTX is not a message type, just an inv type
        case MSG_WTX:            return cmd.append("wtx");
        case MSG_BLOCK:          return cmd.append(NetMsgType::BLOCK);
        case MSG_FILTERED_BLOCK: return cmd.append(NetMsgType::MERKLEBLOCK);
        case MSG_CMPCT_BLOCK:    return cmd.append(NetMsgType::CMPCTBLOCK);
        default:
            throw std::out_of_range(strprintf("CInv::GetCommand(): type=%d unknown type", type));
        }
        */
    }
    
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            try {
            return strprintf("%s %s", GetCommand(), hash.ToString());
        } catch(const std::out_of_range &) {
            return strprintf("0x%08x %s", type, hash.ToString());
        }
        */
    }
    
    /**
      | Single-message helper methods
      |
      */
    pub fn is_msg_tx(&self) -> bool {
        
        todo!();
        /*
            return type == MSG_TX;
        */
    }
    
    pub fn is_msg_blk(&self) -> bool {
        
        todo!();
        /*
            return type == MSG_BLOCK;
        */
    }
    
    pub fn is_msg_wtx(&self) -> bool {
        
        todo!();
        /*
            return type == MSG_WTX;
        */
    }
    
    pub fn is_msg_filtered_blk(&self) -> bool {
        
        todo!();
        /*
            return type == MSG_FILTERED_BLOCK;
        */
    }
    
    pub fn is_msg_cmpct_blk(&self) -> bool {
        
        todo!();
        /*
            return type == MSG_CMPCT_BLOCK;
        */
    }
    
    pub fn is_msg_witness_blk(&self) -> bool {
        
        todo!();
        /*
            return type == MSG_WITNESS_BLOCK;
        */
    }

    /**
      | Combined-message helper methods
      |
      */
    pub fn is_gen_tx_msg(&self) -> bool {
        
        todo!();
        /*
            return type == MSG_TX || type == MSG_WTX || type == MSG_WITNESS_TX;
        */
    }
    
    pub fn is_gen_blk_msg(&self) -> bool {
        
        todo!();
        /*
            return type == MSG_BLOCK || type == MSG_FILTERED_BLOCK || type == MSG_CMPCT_BLOCK || type == MSG_WITNESS_BLOCK;
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/protocol.cpp]

lazy_static!{
    static ref INITIAL_BLOCK_DOWNLOAD_COMPLETED: AtomicBool = AtomicBool::new(false);
}

/**
  | Bitcoin protocol message types. When
  | adding new message types, don't forget
  | to update allNetMessageTypes in protocol.cpp.
  |
  */
pub mod NetMsgType {

    /**
      | The version message provides information
      | about the transmitting node to the receiving
      | node at the beginning of a connection.
      |
      */
    pub const VERSION:      &'static str = "version";

    /**
      | The verack message acknowledges a previously-received
      | version message, informing the connecting
      | node that it can begin to send other messages.
      |
      */
    pub const VERACK:       &'static str = "verack";

    /**
      | The addr (IP address) message relays
      | connection information for peers on
      | the network.
      |
      */
    pub const ADDR:         &'static str = "addr";

    /**
      | The addrv2 message relays connection
      | information for peers on the network
      | just like the addr message, but is extended
      | to allow gossiping of longer node addresses
      | (see BIP155).
      |
      */
    pub const ADDRV2:       &'static str = "addrv2";

    /**
      | The sendaddrv2 message signals support
      | for receiving ADDRV2 messages (BIP155).
      | 
      | It also implies that its sender can encode
      | as ADDRV2 and would send ADDRV2 instead
      | of ADDR to a peer that has signaled ADDRV2
      | support by sending SENDADDRV2.
      |
      */
    pub const SENDADDRV2:   &'static str = "sendaddrv2";

    /**
      | The inv message (inventory message)
      | transmits one or more inventories of
      | objects known to the transmitting peer.
      |
      */
    pub const INV:          &'static str = "inv";

    /**
      | The getdata message requests one or
      | more data objects from another node.
      |
      */
    pub const GETDATA:      &'static str = "getdata";

    /**
      | The merkleblock message is a reply to
      | a getdata message which requested a
      | block using the inventory type MSG_MERKLEBLOCK.
      | @since protocol version 70001 as described
      | by BIP37.
      |
      */
    pub const MERKLEBLOCK:  &'static str = "merkleblock";

    /**
      | The getblocks message requests an inv
      | message that provides block header
      | hashes starting from a particular point
      | in the block chain.
      |
      */
    pub const GETBLOCKS:    &'static str = "getblocks";

    /**
      | The getheaders message requests a headers
      | message that provides block headers
      | starting from a particular point in
      | the block chain. @since protocol version
      | 31800.
      |
      */
    pub const GETHEADERS:   &'static str = "getheaders";

    /**
      | The tx message transmits a single transaction.
      |
      */
    pub const TX:           &'static str = "tx";

    /**
      | The headers message sends one or more
      | block headers to a node which previously
      | requested certain headers with a getheaders
      | message. @since protocol version 31800.
      |
      */
    pub const HEADERS:      &'static str = "headers";

    /**
      | The block message transmits a single
      | serialized block.
      |
      */
    pub const BLOCK:        &'static str = "block";

    /**
      | The getaddr message requests an addr
      | message from the receiving node, preferably
      | one with lots of IP addresses of other
      | receiving nodes.
      |
      */
    pub const GETADDR:      &'static str = "getaddr";

    /**
      | The mempool message requests the TXIDs
      | of transactions that the receiving
      | node has verified as valid but which
      | have not yet appeared in a block. @since
      | protocol version 60002.
      |
      */
    pub const MEMPOOL:      &'static str = "mempool";

    /**
      | The ping message is sent periodically
      | to help confirm that the receiving peer
      | is still connected.
      |
      */
    pub const PING:         &'static str = "ping";

    /**
      | The pong message replies to a ping message,
      | proving to the pinging node that the
      | ponging node is still alive. @since
      | protocol version 60001 as described
      | by BIP31.
      |
      */
    pub const PONG:         &'static str = "pong";

    /**
      | The notfound message is a reply to a getdata
      | message which requested an object the
      | receiving node does not have available
      | for relay. @since protocol version
      | 70001.
      |
      */
    pub const NOTFOUND:     &'static str = "notfound";

    /**
      | The filterload message tells the receiving
      | peer to filter all relayed transactions
      | and requested merkle blocks through
      | the provided filter. @since protocol
      | version 70001 as described by BIP37.
      | 
      | Only available with service bit NODE_BLOOM
      | since protocol version 70011 as described
      | by BIP111.
      |
      */
    pub const FILTERLOAD:   &'static str = "filterload";

    /**
      | The filteradd message tells the receiving
      | peer to add a single element to a previously-set
      | bloom filter, such as a new public key.
      | @since protocol version 70001 as described
      | by BIP37.
      | 
      | Only available with service bit NODE_BLOOM
      | since protocol version 70011 as described
      | by BIP111.
      |
      */
    pub const FILTERADD:    &'static str = "filteradd";

    /**
      | The filterclear message tells the receiving
      | peer to remove a previously-set bloom
      | filter. @since protocol version 70001
      | as described by BIP37.
      | 
      | Only available with service bit NODE_BLOOM
      | since protocol version 70011 as described
      | by BIP111.
      |
      */
    pub const FILTERCLEAR:  &'static str = "filterclear";

    /**
      | Indicates that a node prefers to receive
      | new block announcements via a "headers"
      | message rather than an "inv". @since
      | protocol version 70012 as described
      | by BIP130.
      |
      */
    pub const SENDHEADERS:  &'static str = "sendheaders";

    /**
      | The feefilter message tells the receiving
      | peer not to inv us any txs which do not
      | meet the specified min fee rate. @since
      | protocol version 70013 as described
      | by BIP133
      |
      */
    pub const FEEFILTER:    &'static str = "feefilter";

    /**
      | Contains a 1-byte bool and 8-byte LE
      | version number.
      | 
      | Indicates that a node is willing to provide
      | blocks via "cmpctblock" messages.
      | 
      | May indicate that a node prefers to receive
      | new block announcements via a "cmpctblock"
      | message rather than an "inv", depending
      | on message contents. @since protocol
      | version 70014 as described by BIP 152
      |
      */
    pub const SENDCMPCT:    &'static str = "sendcmpct";

    /**
      | Contains a CBlockHeaderAndShortTxIDs
      | object - providing a header and list
      | of "short txids". @since protocol version
      | 70014 as described by BIP 152
      |
      */
    pub const CMPCTBLOCK:   &'static str = "cmpctblock";

    /**
      | Contains a BlockTransactionsRequest
      | 
      | Peer should respond with "blocktxn"
      | message. @since protocol version 70014
      | as described by BIP 152
      |
      */
    pub const GETBLOCKTXN:  &'static str = "getblocktxn";

    /**
      | Contains a BlockTransactions.
      | 
      | Sent in response to a "getblocktxn"
      | message. @since protocol version 70014
      | as described by BIP 152
      |
      */
    pub const BLOCKTXN:     &'static str = "blocktxn";

    /**
      | getcfilters requests compact filters
      | for a range of blocks.
      | 
      | Only available with service bit NODE_COMPACT_FILTERS
      | as described by BIP 157 & 158.
      |
      */
    pub const GETCFILTERS:  &'static str = "getcfilters";

    /**
      | cfilter is a response to a getcfilters
      | request containing a single compact
      | filter.
      |
      */
    pub const CFILTER:      &'static str = "cfilter";

    /**
      | getcfheaders requests a compact filter
      | header and the filter hashes for a range
      | of blocks, which can then be used to reconstruct
      | the filter headers for those blocks.
      | 
      | Only available with service bit NODE_COMPACT_FILTERS
      | as described by BIP 157 & 158.
      |
      */
    pub const GETCFHEADERS: &'static str = "getcfheaders";

    /**
      | cfheaders is a response to a getcfheaders
      | request containing a filter header
      | and a vector of filter hashes for each
      | subsequent block in the requested range.
      |
      */
    pub const CFHEADERS:    &'static str = "cfheaders";

    /**
      | getcfcheckpt requests evenly spaced
      | compact filter headers, enabling parallelized
      | download and validation of the headers
      | between them.
      | 
      | Only available with service bit NODE_COMPACT_FILTERS
      | as described by BIP 157 & 158.
      |
      */
    pub const GETCFCHECKPT: &'static str = "getcfcheckpt";

    /**
      | cfcheckpt is a response to a getcfcheckpt
      | request containing a vector of evenly
      | spaced filter headers for blocks on
      | the requested chain.
      |
      */
    pub const CFCHECKPT:    &'static str = "cfcheckpt";

    /**
      | Indicates that a node prefers to relay
      | transactions via wtxid, rather than
      | txid. @since protocol version 70016
      | as described by BIP 339.
      |
      */
    pub const WTXIDRELAY:   &'static str = "wtxidrelay";
}

/**
  | All known message types. Keep this in
  | the same order as the list of messages
  | above and in protocol.h.
  |
  */
lazy_static!{
    /*
    const static std::string allNetMessageTypes[] = {
        NetMsgType::VERSION,
        NetMsgType::VERACK,
        NetMsgType::ADDR,
        NetMsgType::ADDRV2,
        NetMsgType::SENDADDRV2,
        NetMsgType::INV,
        NetMsgType::GETDATA,
        NetMsgType::MERKLEBLOCK,
        NetMsgType::GETBLOCKS,
        NetMsgType::GETHEADERS,
        NetMsgType::TX,
        NetMsgType::HEADERS,
        NetMsgType::BLOCK,
        NetMsgType::GETADDR,
        NetMsgType::MEMPOOL,
        NetMsgType::PING,
        NetMsgType::PONG,
        NetMsgType::NOTFOUND,
        NetMsgType::FILTERLOAD,
        NetMsgType::FILTERADD,
        NetMsgType::FILTERCLEAR,
        NetMsgType::SENDHEADERS,
        NetMsgType::FEEFILTER,
        NetMsgType::SENDCMPCT,
        NetMsgType::CMPCTBLOCK,
        NetMsgType::GETBLOCKTXN,
        NetMsgType::BLOCKTXN,
        NetMsgType::GETCFILTERS,
        NetMsgType::CFILTER,
        NetMsgType::GETCFHEADERS,
        NetMsgType::CFHEADERS,
        NetMsgType::GETCFCHECKPT,
        NetMsgType::CFCHECKPT,
        NetMsgType::WTXIDRELAY,
    };
    */
}

lazy_static!{
    /*
    const static std::vector<std::string> 
    allNetMessageTypesVec(std::begin(allNetMessageTypes), std::end(allNetMessageTypes));
    */
}


///-----------------------------
impl PartialEq<Inv> for Inv {
    #[inline] fn eq(&self, other: &Inv) -> bool {
        todo!();
    }
}

impl Eq for Inv {}

impl Ord for Inv {
    
    #[inline] fn cmp(&self, other: &Inv) -> Ordering {
        todo!();
        /*
            return (a.type < b.type || (a.type == b.type && a.hash < b.hash));
        */
    }
}

impl PartialOrd<Inv> for Inv {
    #[inline] fn partial_cmp(&self, other: &Inv) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for Inv {
    fn default() -> Self {
    
        todo!();
        /*


            type = 0;
        hash.SetNull();
        */
    }
}

/**
  | Get a vector of all valid message types
  | (see above)
  |
  */
pub fn get_all_net_message_types() -> &'static Vec<String> {
    
    todo!();
        /*
            return allNetMessageTypesVec;
        */
}
