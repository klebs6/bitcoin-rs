crate::ix!();

pub struct BanManInner {
    pub banned:      BanMap,
    pub is_dirty:    bool, 
    pub discouraged: RollingBloomFilter,
}

impl Default for BanManInner {
    fn default() -> Self {
        Self {
            banned:   BanMap::new(),
            is_dirty: false,
            discouraged: RollingBloomFilter::new(50000, 0.000001),
        }
    }
}

pub type ShouldNotifyUi = bool;

impl BanManInner {

    pub fn clear_banned(&mut self) {
        self.banned.clear();
        self.is_dirty = true;
    }

    pub fn do_get_banned(&mut self, banmap: &mut BanMap) -> ShouldNotifyUi {

        // Sweep the banlist so expired bans are
        // not returned
        let notify_ui = self.do_sweep_banned();

        // create a thread safe copy
        *banmap = self.banned.clone();

        notify_ui
    }

    pub fn do_sweep_banned(&mut self) -> ShouldNotifyUi {

        let now = OffsetDateTime::now_utc();

        let mut notify_ui: bool = false;

        self.banned.retain(
            |sub_net, ban_entry| {

                let delete = {
                    let invalid_subnet = !sub_net.is_valid();
                    let past_bantime   = now > ban_entry.n_ban_until;

                    invalid_subnet || past_bantime
                };

                if delete {

                    self.is_dirty  = true;

                    notify_ui = true;

                    log_print!(bc_log::net, 
                        format!{
                            "Removed banned node address/subnet: {}\n", 
                            sub_net.to_string()
                        }
                    );
                }

                !delete
            }
        );

        notify_ui
    }
}
