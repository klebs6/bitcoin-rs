// ---------------- [ File: bitcoin-net-zmq/src/zmqpublishnotifier.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/zmq/zmqpublishnotifier.h]

pub struct ZMQAbstractPublishNotifier {

    base: ZMQAbstractNotifier,

    /**
      | upcounting per message sequence number
      |
      */
    n_sequence: u32, // default = { 0U }
}

impl Initialize for ZMQAbstractPublishNotifier {

    fn initialize(&mut self, pcontext: *mut c_void) -> bool {
        
        todo!();
        /*
            assert(!psocket);

        // check if address is being used by other publish notifier
        std::multimap<std::string, CZMQAbstractPublishNotifier*>::iterator i = mapPublishNotifiers.find(address);

        if (i==mapPublishNotifiers.end())
        {
            psocket = zmq_socket(pcontext, ZMQ_PUB);
            if (!psocket)
            {
                zmqError("Failed to create socket");
                return false;
            }

            LogPrint(LogFlags::ZMQ, "zmq: Outbound message high water mark for %s at %s is %d\n", type, address, outbound_message_high_water_mark);

            int rc = zmq_setsockopt(psocket, ZMQ_SNDHWM, &outbound_message_high_water_mark, sizeof(outbound_message_high_water_mark));
            if (rc != 0)
            {
                zmqError("Failed to set outbound message high water mark");
                zmq_close(psocket);
                return false;
            }

            const int so_keepalive_option {1};
            rc = zmq_setsockopt(psocket, ZMQ_TCP_KEEPALIVE, &so_keepalive_option, sizeof(so_keepalive_option));
            if (rc != 0) {
                zmqError("Failed to set SO_KEEPALIVE");
                zmq_close(psocket);
                return false;
            }

            // On some systems (e.g. OpenBSD) the ZMQ_IPV6 must not be enabled, if the address to bind isn't IPv6
            const int enable_ipv6 { IsZMQAddressIPV6(address) ? 1 : 0};
            rc = zmq_setsockopt(psocket, ZMQ_IPV6, &enable_ipv6, sizeof(enable_ipv6));
            if (rc != 0) {
                zmqError("Failed to set ZMQ_IPV6");
                zmq_close(psocket);
                return false;
            }

            rc = zmq_bind(psocket, address.c_str());
            if (rc != 0)
            {
                zmqError("Failed to bind address");
                zmq_close(psocket);
                return false;
            }

            // register this notifier for the address, so it can be reused for other publish notifier
            mapPublishNotifiers.insert(std::make_pair(address, this));
            return true;
        }
        else
        {
            LogPrint(LogFlags::ZMQ, "zmq: Reusing socket for address %s\n", address);
            LogPrint(LogFlags::ZMQ, "zmq: Outbound message high water mark for %s at %s is %d\n", type, address, outbound_message_high_water_mark);

            psocket = i->second->psocket;
            mapPublishNotifiers.insert(std::make_pair(address, this));

            return true;
        }
        */
    }
}

impl crate::traits::Shutdown for ZMQAbstractPublishNotifier {

    fn shutdown(&mut self)  {
        
        todo!();
        /*
            // Early return if Initialize was not called
        if (!psocket) return;

        int count = mapPublishNotifiers.count(address);

        // remove this notifier from the list of publishers using this address
        typedef std::multimap<std::string, CZMQAbstractPublishNotifier*>::iterator iterator;
        std::pair<iterator, iterator> iterpair = mapPublishNotifiers.equal_range(address);

        for (iterator it = iterpair.first; it != iterpair.second; ++it)
        {
            if (it->second==this)
            {
                mapPublishNotifiers.erase(it);
                break;
            }
        }

        if (count == 1)
        {
            LogPrint(LogFlags::ZMQ, "zmq: Close socket at address %s\n", address);
            int linger = 0;
            zmq_setsockopt(psocket, ZMQ_LINGER, &linger, sizeof(linger));
            zmq_close(psocket);
        }

        psocket = nullptr;
        */
    }
}

pub trait SendZmqMessage {

    fn send_zmq_message(&mut self, 
        command: *const u8,
        data:    *const c_void,
        size:    usize) -> bool;
}

pub struct ZMQPublishHashBlockNotifier {
    base: ZMQAbstractPublishNotifier,
}

pub struct ZMQPublishHashTransactionNotifier { base: ZMQAbstractPublishNotifier, }
pub struct ZMQPublishRawBlockNotifier        { base: ZMQAbstractPublishNotifier, }
pub struct ZMQPublishRawTransactionNotifier  { base: ZMQAbstractPublishNotifier, }
pub struct ZMQPublishSequenceNotifier        { base: ZMQAbstractPublishNotifier, }

//-------------------------------------------[.cpp/bitcoin/src/zmq/zmqpublishnotifier.cpp]

lazy_static!{
    /*
    static std::multimap<std::string, CZMQAbstractPublishNotifier*> mapPublishNotifiers;
    */
}

pub const MSG_HASHBLOCK: &'static str = "hashblock";
pub const MSG_HASHTX:    &'static str = "hashtx";
pub const MSG_RAWBLOCK:  &'static str = "rawblock";
pub const MSG_RAWTX:     &'static str = "rawtx";
pub const MSG_SEQUENCE:  &'static str = "sequence";

/**
  | Internal function to send multipart
  | message
  |
  */
pub fn zmq_send_multipart(
        sock: *mut c_void,
        data: *const c_void,
        size: usize,
        args: &[&str]) -> i32 {
    
    todo!();
        /*
            va_list args;
        va_start(args, size);

        while (1)
        {
            zmq_msg_t msg;

            int rc = zmq_msg_init_size(&msg, size);
            if (rc != 0)
            {
                zmqError("Unable to initialize ZMQ msg");
                va_end(args);
                return -1;
            }

            c_void *buf = zmq_msg_data(&msg);
            memcpy(buf, data, size);

            data = va_arg(args, const c_void*);

            rc = zmq_msg_send(&msg, sock, data ? ZMQ_SNDMORE : 0);
            if (rc == -1)
            {
                zmqError("Unable to send ZMQ msg");
                zmq_msg_close(&msg);
                va_end(args);
                return -1;
            }

            zmq_msg_close(&msg);

            if (!data)
                break;

            size = va_arg(args, size_t);
        }
        va_end(args);
        return 0;
        */
}

pub fn is_zmq_addressipv6(zmq_address: &String) -> bool {
    
    todo!();
        /*
            const std::string tcp_prefix = "tcp://";
        const size_t tcp_index = zmq_address.rfind(tcp_prefix);
        const size_t colon_index = zmq_address.rfind(":");
        if (tcp_index == 0 && colon_index != std::string::npos) {
            const std::string ip = zmq_address.substr(tcp_prefix.length(), colon_index - tcp_prefix.length());
            CNetAddr addr;
            LookupHost(ip, addr, false);
            if (addr.IsIPv6()) return true;
        }
        return false;
        */
}

impl SendZmqMessage for ZMQAbstractPublishNotifier {

    /**
      | send zmq multipart message parts:
      | 
      | - command
      | 
      | - data
      | 
      | - message sequence number
      |
      */
    fn send_zmq_message(&mut self, 
        command: *const u8,
        data:    *const c_void,
        size:    usize) -> bool {
        
        todo!();
        /*
            assert(psocket);

        /* send three parts, command & data & a LE 4byte sequence number */
        unsigned char msgseq[sizeof(uint32_t)];
        WriteLE32(msgseq, nSequence);
        int rc = zmq_send_multipart(psocket, command, strlen(command), data, size, msgseq, (size_t)sizeof(uint32_t), nullptr);
        if (rc == -1)
            return false;

        /* increment memory only sequence number after sending */
        nSequence++;

        return true;
        */
    }
}

impl NotifyBlock for ZMQPublishHashBlockNotifier {
    fn notify_block(&mut self, pindex: *const BlockIndex) -> bool {
        
        todo!();
        /*
            uint256 hash = pindex->GetBlockHash();
        LogPrint(LogFlags::ZMQ, "zmq: Publish hashblock %s to %s\n", hash.GetHex(), this->address);
        char data[32];
        for (unsigned int i = 0; i < 32; i++)
            data[31 - i] = hash.begin()[i];
        return SendZmqMessage(MSG_HASHBLOCK, data, 32);
        */
    }
}

impl NotifyTransaction for ZMQPublishHashTransactionNotifier {
    fn notify_transaction(&mut self, transaction: &Transaction) -> bool {
        
        todo!();
        /*
            uint256 hash = transaction.GetHash();
        LogPrint(LogFlags::ZMQ, "zmq: Publish hashtx %s to %s\n", hash.GetHex(), this->address);
        char data[32];
        for (unsigned int i = 0; i < 32; i++)
            data[31 - i] = hash.begin()[i];
        return SendZmqMessage(MSG_HASHTX, data, 32);
        */
    }
}

impl NotifyBlock for ZMQPublishRawBlockNotifier {
    fn notify_block(&mut self, pindex: *const BlockIndex) -> bool {
        
        todo!();
        /*
            LogPrint(LogFlags::ZMQ, "zmq: Publish rawblock %s to %s\n", pindex->GetBlockHash().GetHex(), this->address);

        const ChainConsensusParams& consensusParams = Params().GetConsensus();
        DataStream ss(SER_NETWORK, PROTOCOL_VERSION | RPCSerializationFlags());
        {
            LOCK(cs_main);
            CBlock block;
            if(!ReadBlockFromDisk(block, pindex, consensusParams))
            {
                zmqError("Can't read block from disk");
                return false;
            }

            ss << block;
        }

        return SendZmqMessage(MSG_RAWBLOCK, &(*ss.begin()), ss.size());
        */
    }
}

impl NotifyTransaction for ZMQPublishRawTransactionNotifier {
    fn notify_transaction(&mut self, transaction: &Transaction) -> bool {
        
        todo!();
        /*
        uint256 hash = transaction.GetHash();
        LogPrint(LogFlags::ZMQ, "zmq: Publish rawtx %s to %s\n", hash.GetHex(), this->address);
        DataStream ss(SER_NETWORK, PROTOCOL_VERSION | RPCSerializationFlags());
        ss << transaction;
        return SendZmqMessage(MSG_RAWTX, &(*ss.begin()), ss.size());
        */
    }
}

/**
  | Helper function to send a 'sequence' topic
  | message with the following structure:
  |
  |    <32-byte hash> | <1-byte label> | <8-byte LE
  |    sequence> (optional)
  */
pub fn send_sequence_msg(
        notifier: &mut ZMQAbstractPublishNotifier,
        hash:     u256,
        label:    u8,
        sequence: Option<u64>) -> bool {
    
    todo!();
        /*
            unsigned char data[sizeof(hash) + sizeof(label) + sizeof(uint64_t)];
        for (unsigned int i = 0; i < sizeof(hash); ++i) {
            data[sizeof(hash) - 1 - i] = hash.begin()[i];
        }
        data[sizeof(hash)] = label;
        if (sequence) WriteLE64(data + sizeof(hash) + sizeof(label), *sequence);
        return notifier.SendZmqMessage(MSG_SEQUENCE, data, sequence ? sizeof(data) : sizeof(hash) + sizeof(label));
        */
}

impl NotifyBlockConnect for ZMQPublishSequenceNotifier {
    fn notify_block_connect(&mut self, pindex: *const BlockIndex) -> bool {
        
        todo!();
        /*
            uint256 hash = pindex->GetBlockHash();
        LogPrint(LogFlags::ZMQ, "zmq: Publish sequence block connect %s to %s\n", hash.GetHex(), this->address);
        return SendSequenceMsg(*this, hash, /* Block (C)onnect */ 'C');
        */
    }
}
    
impl NotifyBlockDisconnect for ZMQPublishSequenceNotifier {
    fn notify_block_disconnect(&mut self, pindex: *const BlockIndex) -> bool {
        
        todo!();
        /*
            uint256 hash = pindex->GetBlockHash();
        LogPrint(LogFlags::ZMQ, "zmq: Publish sequence block disconnect %s to %s\n", hash.GetHex(), this->address);
        return SendSequenceMsg(*this, hash, /* Block (D)isconnect */ 'D');
        */
    }
}
    
impl NotifyTransactionAcceptance for ZMQPublishSequenceNotifier {
    fn notify_transaction_acceptance(&mut self, 
        transaction:      &Transaction,
        mempool_sequence: u64) -> bool {
        
        todo!();
        /*
            uint256 hash = transaction.GetHash();
        LogPrint(LogFlags::ZMQ, "zmq: Publish hashtx mempool acceptance %s to %s\n", hash.GetHex(), this->address);
        return SendSequenceMsg(*this, hash, /* Mempool (A)cceptance */ 'A', mempool_sequence);
        */
    }
}
    
impl NotifyTransactionRemoval for ZMQPublishSequenceNotifier {
    fn notify_transaction_removal(&mut self, 
        transaction:      &Transaction,
        mempool_sequence: u64) -> bool {
        
        todo!();
        /*
            uint256 hash = transaction.GetHash();
        LogPrint(LogFlags::ZMQ, "zmq: Publish hashtx mempool removal %s to %s\n", hash.GetHex(), this->address);
        return SendSequenceMsg(*this, hash, /* Mempool (R)emoval */ 'R', mempool_sequence);
        */
    }
}
