crate::ix!();

pub trait MaybeSendFeeFilter {

    fn maybe_send_feefilter(self: Arc<Self>, 
        pto:          Amo<Box<dyn NodeInterface>>,
        current_time: OffsetDateTime /* micros */);
}

impl MaybeSendFeeFilter for PeerManager {

    /**
      | Send `feefilter` message.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
    fn maybe_send_feefilter(self: Arc<Self>, 
        pto:          Amo<Box<dyn NodeInterface>>,
        current_time: OffsetDateTime /* micros */)  {

        assert_lock_held!(CS_MAIN);

        if self.ignore_incoming_txs {
            return;
        }

        if !pto.get().has_tx_relay() {
            return;
        }

        if pto.get().get_common_version() < FEEFILTER_VERSION {
            return;
        }

        // peers with the forcerelay permission should not filter txs to us
        if pto.get().has_permission(NetPermissionFlags::ForceRelay) {
            return;
        }

        let mut current_filter: Amount = self.mempool.get().get_min_fee(
            (
                G_ARGS.lock()
                    .get_int_arg(
                        "-maxmempool", 
                        DEFAULT_MAX_MEMPOOL_SIZE.try_into().unwrap()
                    ) * 1000000

            ).try_into().unwrap()
        ).get_fee_perk();

        lazy_static!{
            static ref G_FILTER_ROUNDER: Amo<FeeFilterRounder> 
            = Amo::<FeeFilterRounder>::from(FeeFilterRounder::new(&FeeRate::new(DEFAULT_MIN_RELAY_TX_FEE)));
        }

        if self.chainman.get().active_chainstate().is_initial_block_download() {

            // Received tx-inv messages are
            // discarded when the active
            // chainstate is in IBD, so tell the
            // peer to not send them.
            current_filter = MAX_MONEY;

        } else {

            lazy_static!{
                static ref MAX_FILTER: Amount = G_FILTER_ROUNDER.get_mut().round(MAX_MONEY);
            }

            if pto.get().get_tx_relay().last_sent_fee_filter == *MAX_FILTER {

                // Send the current filter if we
                // sent MAX_FILTER previously and
                // made it out of IBD.
                pto.get().get_tx_relay_mut().next_send_feefilter = None;
            }
        }

        if current_time > pto.get().get_tx_relay().next_send_feefilter.unwrap() {

            let mut filter_to_send: Amount = G_FILTER_ROUNDER.get_mut().round(current_filter).clone();

            // We always have a fee filter of at
            // least minRelayTxFee
            filter_to_send = max(
                filter_to_send,
                MIN_RELAY_TX_FEE.get_fee_perk()
            );

            if filter_to_send != pto.get().get_tx_relay().last_sent_fee_filter {

                let common_version = pto.get().get_common_version();
                let msg_maker      = NetMsgMaker::new(common_version);

                self.connman.get_mut().push_message(
                    &mut pto.get_mut(), 
                    msg_maker 
                    .make(
                        NetMsgType::FEEFILTER, 
                        &[
                        &filter_to_send
                        ]
                    )
                );

                pto.get().get_tx_relay_mut().last_sent_fee_filter = filter_to_send;
            }

            pto.get().get_tx_relay_mut().next_send_feefilter = Some(
                poisson_next_send(
                    current_time,
                    AVG_FEEFILTER_BROADCAST_INTERVAL
                )
            );

        } else {

            // If the fee filter has changed
            // substantially and it's still more
            // than MAX_FEEFILTER_CHANGE_DELAY
            // until scheduled broadcast, then
            // move the broadcast to within
            // MAX_FEEFILTER_CHANGE_DELAY.
            if {

                let last_sent_fee_filter = pto.get().get_tx_relay().last_sent_fee_filter;

                let gate0 = current_time + MAX_FEEFILTER_CHANGE_DELAY < pto.get().get_tx_relay().next_send_feefilter.unwrap();

                let gate1a = current_filter < 3 * last_sent_fee_filter / 4;
                let gate1b = current_filter > 4 * last_sent_fee_filter / 3;

                let gate1 = gate1a || gate1b;

                 
                gate0 && gate1 
            }
            {
                let change_delay = get_random_duration(MAX_FEEFILTER_CHANGE_DELAY);

                pto.get().get_tx_relay_mut().next_send_feefilter = Some(current_time + change_delay);
            }
        }
    }
}

