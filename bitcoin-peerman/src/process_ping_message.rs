crate::ix!();

impl PeerManager {

    pub fn process_ping_message(
        self:               Arc<Self>, 
        peer:               &Option<Peer>,
        msg_maker:          &NetMsgMaker,
        mut pfrom:          &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        if pfrom.get_common_version() > BIP0031_VERSION {

            let mut nonce: u64 = 0;

            recv.stream_into(&mut nonce);

            /**
              |  Echo the message back with the
              |  nonce. This allows for two useful
              |  features:
              |
              |  1) A remote node can quickly check
              |  if the connection is operational
              |
              |  2) Remote nodes can measure the
              |     latency of the network
              |     thread. If this node is
              |     overloaded it won't respond to
              |     pings quickly and the remote
              |     node can avoid sending us more
              |     work, like chain download
              |     requests.
              |
              |  The nonce stops the remote getting
              |  confused between different pings:
              |  without it, if the remote node
              |  sends a ping once per second and
              |  this node takes 5 seconds to
              |  respond to each, the 5th ping the
              |  remote sends would appear to
              |  return very quickly.
              */
            self.connman.get_mut().push_message(
                &mut *pfrom, 
                msg_maker.make(NetMsgType::PONG, 
                    &[
                        &nonce
                    ]
                )
            );
        }
    }
}
