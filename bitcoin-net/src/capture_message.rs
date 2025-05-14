// ---------------- [ File: bitcoin-net/src/capture_message.rs ]
crate::ix!();

/**
  | Dump binary message to file, with timestamp
  |
  */
pub fn capture_message(
    addr:        &Address,
    msg_type:    &String,
    data:        &[u8],
    is_incoming: bool)  {
    
    todo!();
        /*
            // Note: This function captures the message at the time of processing,
        // not at socket receive/send time.
        // This ensures that the messages are always in order from an application
        // layer (processing) perspective.
        auto now = GetTime<microseconds>();

        // Windows folder names can not include a colon
        std::string clean_addr = addr.ToString();
        std::replace(clean_addr.begin(), clean_addr.end(), ':', '_');

        fs::path base_path = gArgs.GetDataDirNet() / "message_capture" / clean_addr;
        fs::create_directories(base_path);

        fs::path path = base_path / (is_incoming ? "msgs_recv.dat" : "msgs_sent.dat");
        CAutoFile f(fsbridge::fopen(path, "ab"), SER_DISK, CLIENT_VERSION);

        ser_writedata64(f, now.count());
        f.write(msg_type.data(), msg_type.length());
        for (auto i = msg_type.length(); i < CMessageHeader::COMMAND_SIZE; ++i) {
            f << uint8_t{'\0'};
        }
        uint32_t size = data.size();
        ser_writedata32(f, size);
        f.write((const char*)data.data(), data.size());
        */
}
