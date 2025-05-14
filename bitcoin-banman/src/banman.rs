// ---------------- [ File: bitcoin-banman/src/banman.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/banman.h]

/**
  | Banman manages two related but distinct
  | concepts:
  |
  | 1. Banning. This is configured manually by the
  | user, through the setban RPC.  If an address or
  | subnet is banned, we never accept incoming
  | connections from it and never create outgoing
  | connections to it. We won't gossip its address
  | to other peers in addr messages. Banned
  | addresses and subnets are stored to disk on
  | shutdown and reloaded on startup. Banning can
  | be used to prevent connections with spy nodes
  | or other griefers.
  |
  | 2. Discouragement. If a peer misbehaves enough
  | (see Misbehaving() in net_processing.cpp),
  | we'll mark that address as discouraged. We
  | still allow incoming connections from them, but
  | they're preferred for eviction when we receive
  | new incoming connections. We never make
  | outgoing connections to them, and do not gossip
  | their address to other peers. This is
  | implemented as a bloom filter. We can
  | (probabilistically) test for membership, but
  | can't list all discouraged addresses or unmark
  | them as discouraged. Discouragement can prevent
  | our limited connection slots being used up by
  | incompatible or broken peers.
  |
  | Neither banning nor discouragement are
  | protections against denial-of-service attacks,
  | since if an attacker has a way to waste our
  | resources and we disconnect from them and ban
  | that address, it's trivial for them to
  | reconnect from another IP address.
  |
  | Attempting to automatically disconnect or ban
  | any class of peer carries the risk of splitting
  | the network. For example, if we
  | banned/disconnected for a transaction that
  | fails a policy check and a future version
  | changes the policy check so the transaction is
  | accepted, then that transaction could cause the
  | network to split between old nodes and new
  | nodes.
  */
pub struct BanMan {
    pub cs_banned:        ReentrantMutex<BanManInner>,
    pub client_interface: Amo<ClientUIInterface>, // default = None
    pub ban_db:           BanDB,
    pub default_ban_time: OffsetDateTime,
}

//-------------------------------------------[.cpp/bitcoin/src/banman.cpp]
impl Drop for BanMan {
    fn drop(&mut self) {
        self.dump_banlist();
    }
}

fn translated(x: &str) -> String {
    //TODO -- maybe add more here
    x.to_string()
}

impl BanMan {

    pub fn new(
        ban_file:         Box<Path>,
        client_interface: Amo<ClientUIInterface>,
        default_ban_time: OffsetDateTime) -> Self {
    
        let mut x = Self {
            client_interface: client_interface,
            ban_db:           BanDB::new(ban_file.to_path_buf()),
            default_ban_time: default_ban_time,
            cs_banned:        ReentrantMutex::new(BanManInner::default()),
        };

        if x.client_interface.is_some() {
            x.client_interface
                .get_mut()
                .init_message(&translated("Loading banlist…"));
        }

        let n_start = Instant::now();

        if x.ban_db.read(&mut x.cs_banned.get_mut().banned) {

            // sweep out unused entries
            x.sweep_banned();

            log_print!(
                bc_log::net, 
                "Loaded %d banned node addresses/subnets  %dms\n", 
                x.cs_banned.lock().banned.len(), 
                Instant::now() - n_start
            );

        } else {

            log_printf!("Recreating the banlist database\n");

            x.cs_banned.get_mut().banned = Default::default();

            x.cs_banned.get_mut().is_dirty = true;
        }

        x.dump_banlist();

        x
    }

    /**
      | clean unused entries (if bantime has
      | expired)
      |
      */
    pub fn sweep_banned(&mut self)  {
        
        let notify_ui = self.cs_banned.get_mut().do_sweep_banned();

        self.notify_ui_on_sweep(notify_ui);
    }

    pub fn get_banned(&mut self, banmap: &mut BanMap)  {
        
        let notify_ui = self.cs_banned.get_mut().do_get_banned(banmap);

        self.notify_ui_on_sweep(notify_ui);
    }

    pub fn notify_ui_on_sweep(&mut self, notify_ui: bool)  {

        // update UI
        if notify_ui && self.client_interface.is_some() {
            self.client_interface
                .get_mut()
                .banned_list_changed();
        }
    }
}
