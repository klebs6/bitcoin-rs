crate::ix!();

impl BanMan {

    pub fn ban_netaddr(&mut self, 
        net_addr:         &NetAddr,
        ban_time_offset:  Option<OffsetDateTime>,
        since_unix_epoch: Option<bool>)  {

        let ban_time_offset:   OffsetDateTime = ban_time_offset.unwrap_or(OffsetDateTime::UNIX_EPOCH);
        let since_unix_epoch:            bool = since_unix_epoch.unwrap_or(false);

        let sub_net: SubNet = SubNet::from(net_addr);

        self.ban_subnet(&sub_net, Some(ban_time_offset), Some(since_unix_epoch));
    }

    pub fn ban_subnet(&mut self, 
        sub_net:          &SubNet,
        ban_time_offset:  Option<OffsetDateTime>,
        since_unix_epoch: Option<bool>)  {

        let ban_time_offset:   OffsetDateTime = ban_time_offset.unwrap_or(OffsetDateTime::UNIX_EPOCH);
        let since_unix_epoch:            bool = since_unix_epoch.unwrap_or(false);

        let mut ban_entry: BanEntry = BanEntry::from(&UniValue::from(Instant::now()));

        let mut normalized_ban_time_offset:   OffsetDateTime = ban_time_offset;
        let mut normalized_since_unix_epoch:            bool = since_unix_epoch;

        if ban_time_offset <= OffsetDateTime::UNIX_EPOCH {
            normalized_ban_time_offset = self.default_ban_time;
            normalized_since_unix_epoch = false;
        }

        ban_entry.n_ban_until = {

            let base: OffsetDateTime = match normalized_since_unix_epoch {
                true   => OffsetDateTime::UNIX_EPOCH,
                false  => OffsetDateTime::now_utc(),
            };

            let timestamp = base.unix_timestamp() + normalized_ban_time_offset.unix_timestamp();

            OffsetDateTime::from_unix_timestamp(timestamp).unwrap()
        };

        {
            let inner = self.cs_banned.get_mut();

            if inner.banned.get(sub_net).unwrap().n_ban_until < ban_entry.n_ban_until {
                inner.banned.insert(sub_net.clone(), ban_entry);
                inner.is_dirty = true;
            } else {
                return;
            }
        }

        if self.client_interface.is_some() {
            self.client_interface
                .get_mut()
                .banned_list_changed();
        }

        // store banlist to disk immediately
        self.dump_banlist();
    }
}
