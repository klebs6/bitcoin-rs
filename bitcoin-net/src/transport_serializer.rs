crate::ix!();

pub struct V1TransportSerializer {

}

impl TransportSerializer for V1TransportSerializer {

}

impl PrepareForTransport for V1TransportSerializer {

    fn prepare_for_transport(&mut self, 
        msg:    &mut SerializedNetMsg,
        header: &mut Vec<u8>)  {
        
        todo!();
        /*
        
        */
    }
}

impl V1TransportSerializer {

    pub fn prepare_for_transport(&mut self, 
        msg:    &mut SerializedNetMsg,
        header: &mut Vec<u8>)  {
        
        todo!();
        /*
            // create dbl-sha256 checksum
        uint256 hash = Hash(msg.data);

        // create header
        CMessageHeader hdr(Params().MessageStart(), msg.m_type.c_str(), msg.data.size());
        memcpy(hdr.pchChecksum, hash.begin(), CMessageHeader::CHECKSUM_SIZE);

        // serialize header
        header.reserve(CMessageHeader::HEADER_SIZE);
        CVectorWriter{SER_NETWORK, INIT_PROTO_VERSION, header, 0, hdr};
        */
    }
}

pub type NodeId = i64;

//---------------------------------------------
pub struct V1TransportDeserializer {

    pub chain_params: Rc<RefCell<ChainParams>>,

    /**
      | Only for logging
      |
      */
    pub node_id:      NodeId,

    pub hasher:       Rc<RefCell<Hash256>>,
    pub data_hash:    Rc<RefCell<u256>>,

    /**
      | parsing header (false) or data (true)
      |
      */
    pub in_data:      bool,

    /**
      | partially received header
      |
      */
    pub hdrbuf:       DataStream,

    /**
      | complete header
      |
      */
    pub hdr:          MessageHeader,

    /**
      | received message data
      |
      */
    pub recv:         DataStream,

    pub n_hdr_pos:    u32,
    pub n_data_pos:   u32,

}

impl TransportDeserializer for V1TransportDeserializer {

}

impl Complete for V1TransportDeserializer {
    fn complete(&self) -> bool {
        
        todo!();
        /*
            if (!in_data)
                return false;
            return (hdr.nMessageSize == nDataPos);
        */
    }
}

impl Reset for V1TransportDeserializer {
    fn reset(&mut self)  {
        
        todo!();
        /*
            vRecv.clear();
            hdrbuf.clear();
            hdrbuf.resize(24);
            in_data = false;
            nHdrPos = 0;
            nDataPos = 0;
            data_hash.SetNull();
            hasher.Reset();
        */
    }
}

impl ReadData for V1TransportDeserializer {

    fn read(&mut self, msg_bytes: &mut [u8]) -> i32 {
        
        todo!();
        /*
            int ret = in_data ? readData(msg_bytes) : readHeader(msg_bytes);
            if (ret < 0) {
                Reset();
            } else {
                msg_bytes = msg_bytes.subspan(ret);
            }
            return ret;
        */
    }
}

impl GetMessage for V1TransportDeserializer {
    fn get_message(&mut self, 
        time:             Instant /* micros */,
        out_err_raw_size: &mut u32) -> Option<NetMessage> {
        
        todo!();
        /*
            // decompose a single CNetMessage from the TransportDeserializer
        std::optional<CNetMessage> msg(std::move(vRecv));

        // store command string, time, and sizes
        msg->m_command = hdr.GetCommand();
        msg->m_time = time;
        msg->m_message_size = hdr.nMessageSize;
        msg->m_raw_message_size = hdr.nMessageSize + CMessageHeader::HEADER_SIZE;

        uint256 hash = GetMessageHash();

        // We just received a message off the wire, harvest entropy from the time (and the message checksum)
        RandAddEvent(ReadLE32(hash.begin()));

        // Check checksum and header command string
        if (memcmp(hash.begin(), hdr.pchChecksum, CMessageHeader::CHECKSUM_SIZE) != 0) {
            LogPrint(BCLog::NET, "Header error: Wrong checksum (%s, %u bytes), expected %s was %s, peer=%d\n",
                     SanitizeString(msg->m_command), msg->m_message_size,
                     HexStr(Span<uint8_t>(hash.begin(), hash.begin() + CMessageHeader::CHECKSUM_SIZE)),
                     HexStr(hdr.pchChecksum),
                     m_node_id);
            out_err_raw_size = msg->m_raw_message_size;
            msg = std::nullopt;
        } else if (!hdr.IsCommandValid()) {
            LogPrint(BCLog::NET, "Header error: Invalid message type (%s, %u bytes), peer=%d\n",
                     SanitizeString(hdr.GetCommand()), msg->m_message_size, m_node_id);
            out_err_raw_size = msg->m_raw_message_size;
            msg.reset();
        }

        // Always reset the network deserializer (prepare for the next message)
        Reset();
        return msg;
        */
    }
}

impl SetVersion for V1TransportDeserializer {
    fn set_version(&mut self, n_version_in: i32)  {
        
        todo!();
        /*
            hdrbuf.SetVersion(nVersionIn);
            vRecv.SetVersion(nVersionIn);
        */
    }
}

impl V1TransportDeserializer {

    pub fn new(
        chain_params: &ChainParams,
        node_id:      NodeId,
        n_type_in:    i32,
        n_version_in: i32) -> Self {
    
        todo!();
        /*
        : chain_params(chain_params),
        : node_id(node_id),
        : hdrbuf(nTypeIn, nVersionIn),
        : recv(nTypeIn, nVersionIn),

            Reset();
        */
    }
    
    pub fn read_header(&mut self, msg_bytes: &[u8]) -> i32 {
        
        todo!();
        /*
            // copy data to temporary parsing buffer
        unsigned int nRemaining = CMessageHeader::HEADER_SIZE - nHdrPos;
        unsigned int nCopy = std::min<unsigned int>(nRemaining, msg_bytes.size());

        memcpy(&hdrbuf[nHdrPos], msg_bytes.data(), nCopy);
        nHdrPos += nCopy;

        // if header incomplete, exit
        if (nHdrPos < CMessageHeader::HEADER_SIZE)
            return nCopy;

        // deserialize to CMessageHeader
        try {
            hdrbuf >> hdr;
        }
        catch (const std::exception&) {
            LogPrint(BCLog::NET, "Header error: Unable to deserialize, peer=%d\n", m_node_id);
            return -1;
        }

        // Check start string, network magic
        if (memcmp(hdr.pchMessageStart, m_chain_params.MessageStart(), CMessageHeader::MESSAGE_START_SIZE) != 0) {
            LogPrint(BCLog::NET, "Header error: Wrong MessageStart %s received, peer=%d\n", HexStr(hdr.pchMessageStart), m_node_id);
            return -1;
        }

        // reject messages larger than MAX_SIZE or MAX_PROTOCOL_MESSAGE_LENGTH
        if (hdr.nMessageSize > MAX_SIZE || hdr.nMessageSize > MAX_PROTOCOL_MESSAGE_LENGTH) {
            LogPrint(BCLog::NET, "Header error: Size too large (%s, %u bytes), peer=%d\n", SanitizeString(hdr.GetCommand()), hdr.nMessageSize, m_node_id);
            return -1;
        }

        // switch state to reading message data
        in_data = true;

        return nCopy;
        */
    }
    
    pub fn read_data(&mut self, msg_bytes: &[u8]) -> i32 {
        
        todo!();
        /*
            unsigned int nRemaining = hdr.nMessageSize - nDataPos;
        unsigned int nCopy = std::min<unsigned int>(nRemaining, msg_bytes.size());

        if (vRecv.size() < nDataPos + nCopy) {
            // Allocate up to 256 KiB ahead, but never more than the total message size.
            vRecv.resize(std::min(hdr.nMessageSize, nDataPos + nCopy + 256 * 1024));
        }

        hasher.Write(msg_bytes.first(nCopy));
        memcpy(&vRecv[nDataPos], msg_bytes.data(), nCopy);
        nDataPos += nCopy;

        return nCopy;
        */
    }
    
    pub fn get_message_hash(&self) -> &u256 {
        
        todo!();
        /*
            assert(Complete());
        if (data_hash.IsNull())
            hasher.Finalize(data_hash);
        return data_hash;
        */
    }
}
