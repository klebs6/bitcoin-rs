crate::ix!();

/**
  | Actual implementation for TxRequestTracker's
  | data structure.
  |
  */
pub struct TxRequestTrackerImpl {

    /**
      | The current sequence number. Increases
      | for every announcement. This is used
      | to sort txhashes returned by GetRequestable
      | in announcement order.
      |
      */
    pub current_sequence: SequenceNumber, // default = { 0 }

    /**
      | This tracker's priority computer.
      |
      */
    pub computer:         PriorityComputer,

    /**
      | This tracker's main data structure.
      | See SanityCheck() for the invariants 
      | that apply to it.
      |
      */
    pub index:            Arc<AnnouncementIndex>,

    /**
      | Map with this tracker's per-peer statistics.
      |
      */
    pub peerinfo:         HashMap<NodeId,PeerInfo>,
}

impl TxRequestTrackerImpl {

    pub fn sanity_check(&self)  {
        
        // Recompute m_peerdata from m_index. This
        // verifies the data in it as it should
        // just be caching statistics on
        // m_index. It also verifies the invariant
        // that no PeerInfo announcements with
        // m_total==0 exist.
        assert!(self.peerinfo == recompute_peer_info(self.index.clone()));

        // Calculate per-txhash statistics from
        // m_index, and validate invariants.
        for item in compute_tx_hash_info(&self.index,&self.computer).iter_mut() {

            let info: &mut TxHashInfo = item.1;

            // Cannot have only COMPLETED peer
            // (txhash should have been
            // forgotten already)
            assert!(
                info.candidate_delayed 
                + info.candidate_ready 
                + info.candidate_best 
                + info.requested > 0
            );

            // Can have at most
            // 1 CANDIDATE_BEST/REQUESTED peer
            assert!(
                info.candidate_best + info.requested <= 1
            );

            // If there are any CANDIDATE_READY
            // announcements, there must be
            // exactly one CANDIDATE_BEST or
            // REQUESTED announcement.
            if info.candidate_ready > 0 {
                assert!(info.candidate_best + info.requested == 1);
            }

            // If there is both a CANDIDATE_READY
            // and a CANDIDATE_BEST announcement,
            // the CANDIDATE_BEST one must be at
            // least as good (equal or higher
            // priority) as the best
            // CANDIDATE_READY.
            if info.candidate_ready != 0 && info.candidate_best != 0 {
                assert!(info.priority_candidate_best >= info.priority_best_candidate_ready);
            }

            // No txhash can have been announced
            // by the same peer twice.
            info.peers.sort();

            assert!(
                info.peers.windows(2).find(|slice| slice[0] == slice[1]) == None
            );
        }
    }
    
    pub fn post_get_requestable_sanity_check(&self, 
        now: OffsetDateTime /* micros */)
    {
        for ann in self.index.get_by_time() {

            if ann.is_waiting() {

                // REQUESTED and CANDIDATE_DELAYED
                // must have a time in the future
                // (they should have been
                // converted to
                // COMPLETED/CANDIDATE_READY
                // respectively).
                assert!(ann.time > now);

            } else {

                if ann.is_selectable() {

                    // CANDIDATE_READY and
                    // CANDIDATE_BEST cannot have
                    // a time in the future (they
                    // should have remained
                    // CANDIDATE_DELAYED, or
                    // should have been converted
                    // back to it if time went
                    // backwards).
                    assert!(ann.time <= now);
                }
            }
        }
    }

    /**
      | Wrapper around Index::...::erase
      | that keeps m_peerinfo up to date.
      |
      */
    pub fn erase_with_peekable<I: AnnouncementIterator>(&mut self, 
        it: &mut CppIter<I>) -> CppIter<I> {
    
        let key = it.peek().unwrap().1.peer;

        if let Some(mut peerit) = self.peerinfo.get_mut(&key) {

            let state = it.peek().unwrap().1.get_state();

            peerit.completed -= match state == State::COMPLETED { true => 1, false => 0 };
            peerit.requested -= match state == State::REQUESTED { true => 1, false => 0 };

            if {
                peerit.total -= 1;
                peerit.total
            } == 0 {
                self.peerinfo.remove(&key);
            }
        }

        self.index.get::<I>().remove_announcement(
            &it.peek().unwrap().1
        )
    }

    /**
      | Wrapper around Index::...::erase
      | that keeps m_peerinfo up to date.
      |
      */
    pub fn erase<I: AnnouncementIterator>(&mut self, 
        mut it: &I) -> I {
    
        let key = it.peer;

        if let Some(mut peerit) = self.peerinfo.get_mut(&key) {

            let state = it.get_state();

            peerit.completed -= match state == State::COMPLETED { true => 1, false => 0 };
            peerit.requested -= match state == State::REQUESTED { true => 1, false => 0 };

            if {
                peerit.total -= 1;
                peerit.total
            } == 0 {
                self.peerinfo.remove(&key);
            }
        }

        self.index.get::<I>().remove_announcement(
            &it
        )
    }

    /**
      | Wrapper around Index::...::modify
      | that keeps m_peerinfo up to date.
      |
      */
    pub fn modify_with_peekable<I: AnnouncementIterator>(&mut self, 
        mut it:   &mut CppIter<I>,
        modifier: AnnouncementModifier)  {

        if let Some(mut peerit) = self.peerinfo.get_mut(&it.peek().unwrap().1.peer) {

            let state = it.peek().unwrap().1.get_state();

            peerit.completed -= match state == State::COMPLETED { true => 1, false => 0 };
            peerit.requested -= match state == State::REQUESTED { true => 1, false => 0 };

            self.index.get::<I>().modify_with_peekable(&it, modifier);

            let state = it.peek().unwrap().1.get_state();

            peerit.completed += match state == State::COMPLETED { true => 1, false => 0 };
            peerit.requested += match state == State::REQUESTED { true => 1, false => 0 };
        }
    }

    /**
      | Wrapper around Index::...::modify
      | that keeps m_peerinfo up to date.
      |
      */
    pub fn modify<I: AnnouncementIterator>(&mut self, 
        it:       &I,
        modifier: AnnouncementModifier)  {

        if let Some(mut peerit) = self.peerinfo.get_mut(&it.peer) {

            let state = it.get_state();

            peerit.completed -= match state == State::COMPLETED { true => 1, false => 0 };
            peerit.requested -= match state == State::REQUESTED { true => 1, false => 0 };

            self.index.get::<I>().modify(it, modifier);

            let state = it.get_state();

            peerit.completed += match state == State::COMPLETED { true => 1, false => 0 };
            peerit.requested += match state == State::REQUESTED { true => 1, false => 0 };
        }
    }

    /**
      | Convert a CANDIDATE_DELAYED
      | announcement into
      | a CANDIDATE_READY. If this makes it
      | the new best CANDIDATE_READY (and no
      | REQUESTED exists) and better than the
      | CANDIDATE_BEST (if any), it becomes
      | the new CANDIDATE_BEST.
      */
    pub fn promote_candidate_ready(&mut self, 
        it: Option<ByTxHashIterator>) 
    {
        assert!(it.is_some());
        assert!(it.as_ref().unwrap().get_state() == State::CANDIDATE_DELAYED);

        //  Convert CANDIDATE_DELAYED to CANDIDATE_READY first.
        self.modify::<ByTxHashIterator>(
            it.as_ref().unwrap(),
            Box::new(|ann: &mut Announcement| {
                ann.set_state(State::CANDIDATE_READY);
            })
        );

        /**
          | The following code relies on the fact
          | that the ByTxHash is sorted by txhash,
          | and then by state (first _DELAYED, then
          | _READY, then _BEST/REQUESTED). Within
          | the _READY announcements, the best one
          | (highest priority) comes last. Thus, if
          | an existing _BEST exists for the same
          | txhash that this announcement may be
          | preferred over, it must immediately
          | follow the newly created _READY.
          */
        let mut it_next = {
            let mut x = it.clone();
            x.as_mut().unwrap().next();
            x
        };

        if it_next.is_none()
        || it_next.clone().unwrap().txhash != it.clone().unwrap().txhash 
        || it_next.clone().unwrap().get_state() == State::COMPLETED {

            /**
              | This is the new best
              | CANDIDATE_READY, and there is no
              | IsSelected() announcement for this
              | txhash already.
              */
            self.modify::<ByTxHashIterator>(
                it.as_ref().unwrap(),
                Box::new(|ann: &mut Announcement| {
                    ann.set_state(State::CANDIDATE_BEST);
                })
            );

        } else {

            if it_next.clone().unwrap().get_state() == State::CANDIDATE_BEST {

                let priority_old: Priority = self.computer.invoke_announcement(
                    &it_next.as_ref().unwrap()
                );

                let priority_new: Priority = self.computer.invoke_announcement(
                    &it.as_ref().unwrap()
                );

                if priority_new > priority_old {

                    /**
                      | There is a CANDIDATE_BEST announcement
                      | already, but this one is better.
                      |
                      */
                    self.modify::<ByTxHashIterator>(
                        it_next.as_ref().unwrap(),
                        Box::new(|ann: &mut Announcement| {
                            ann.set_state(State::CANDIDATE_READY);
                        })
                    );

                    self.modify::<ByTxHashIterator>(
                        it.as_ref().unwrap(), 
                        Box::new(|ann: &mut Announcement| {
                            ann.set_state(State::CANDIDATE_BEST);
                        })
                    );
                }
            }
        }
    }

    /**
      | Change the state of an announcement to
      | something non-IsSelected(). If it was
      | IsSelected(), the next best
      | announcement will be marked
      | CANDIDATE_BEST.
      */
    pub fn change_and_reselect(&mut self, 
        it:        Option<ByTxHashIterator>,
        new_state: State)  {
        
        assert!(new_state == State::COMPLETED || new_state == State::CANDIDATE_DELAYED);

        assert!(it.is_some());

        let it = it.unwrap();

        if it.is_selected() 
        && *it != *self.index.get::<ByTxHashIterator>().into_iter() {

            let mut it_prev = {
                let mut x = it.clone();
                x.prev();
                x
            };

            /**
              | The next best CANDIDATE_READY, if
              | any, immediately precedes the
              | REQUESTED or CANDIDATE_BEST
              | announcement in the ByTxHash index.
              */
            if it_prev.txhash == it.txhash 
            && it_prev.get_state() == State::CANDIDATE_READY {

                // If one such CANDIDATE_READY
                // exists (for this txhash),
                // convert it to CANDIDATE_BEST.
                self.modify::<ByTxHashIterator>(
                    &it_prev,
                    Box::new(|ann: &mut Announcement| {
                        ann.set_state(State::CANDIDATE_BEST);
                    })
                );
            }
        }

        self.modify::<ByTxHashIterator>(
            &it, 
            Box::new(move |ann: &mut Announcement| {
                ann.set_state(new_state.clone());
            })
        );
    }

    /**
      | Check if 'it' is the only announcement
      | for a given txhash that isn't COMPLETED.
      |
      */
    pub fn is_only_non_completed(&mut self, it: Option<ByTxHashIterator>) -> bool {
        
        assert!(it.is_some());

        // Not allowed to call this on
        // COMPLETED announcements.
        assert!(it.clone().unwrap().get_state() != State::COMPLETED);

        let mut it_prev = {
            let mut x = it.clone();
            x.as_mut().unwrap().prev();
            x
        };

        // This announcement has a predecessor
        // that belongs to the same
        // txhash. Due to ordering, and the
        // fact that 'it' is not COMPLETED,
        // its predecessor cannot be COMPLETED
        // here.
        if *it.clone().unwrap() != *self.index.get_by_txhash().into_iter() 
        && it_prev.as_ref().unwrap().txhash == it.as_ref().unwrap().txhash {
            return false;
        }

        let it_next = {
            let mut x = it.clone();
            x.as_mut().unwrap().next();
            x
        };

        // This announcement has a successor
        // that belongs to the same txhash,
        // and is not COMPLETED.
        if it_next.is_some()
        && it_next.as_ref().unwrap().txhash == it.as_ref().unwrap().txhash 
        && it_next.as_ref().unwrap().get_state() != State::COMPLETED {
            return false;
        }

        true
    }

    /**
      | Convert any announcement to a COMPLETED
      | one. If there are no non-COMPLETED announcements
      | left for this txhash, they are deleted.
      | If this was a REQUESTED announcement,
      | and there are other CANDIDATEs left,
      | the best one is made
      | CANDIDATE_BEST. Returns whether the
      | announcement still exists.
      |
      */
    pub fn make_completed(&mut self, it: &mut Option<ByTxHashIterator>) -> bool {
        
        assert!(it.is_some());

        //  Nothing to be done if it's already COMPLETED.
        if it.as_ref().unwrap().get_state() == State::COMPLETED {
            return true;
        }

        if self.is_only_non_completed(it.clone()) {

            // This is the last non-COMPLETED
            // announcement for this
            // txhash. Delete all.
            let txhash: u256 = it.as_ref().unwrap().txhash.clone();

            loop {

                *it = Some(self.erase::<ByTxHashIterator>(&it.as_ref().unwrap()));

                if it.is_none() && it.as_ref().unwrap().txhash == txhash {
                    break;
                }
            }

            return false;
        }

        // Mark the announcement COMPLETED,
        // and select the next best
        // announcement (the first
        // CANDIDATE_READY) if needed.
        self.change_and_reselect(
            it.clone(),
            State::COMPLETED
        );

        true
    }

    /**
      | Make the data structure consistent
      | with a given point in time:
      |
      | - REQUESTED announcements with expiry
      | <= now are turned into COMPLETED.
      |
      | - CANDIDATE_DELAYED announcements with
      | reqtime <= now are turned into
      | CANDIDATE_{READY,BEST}.
      |
      | - CANDIDATE_{READY,BEST} announcements
      | with reqtime > now are turned into
      | CANDIDATE_DELAYED.
      */
    pub fn set_time_point(&mut self, 
        now:     OffsetDateTime /* micros */,
        expired: Amo<Vec<(NodeId,GenTxId)>>)  {
        
        if expired.is_some() {
            expired.get_mut().clear();
        }

        // Iterate over all CANDIDATE_DELAYED
        // and REQUESTED from old to new, as
        // long as they're in the past, and
        // convert them to CANDIDATE_READY and
        // COMPLETED respectively.
        while !self.index.empty() {

            let it = self.index.get_by_time().into_iter();

            if (*it).get_state() == State::CANDIDATE_DELAYED && (*it).time <= now {

                self.promote_candidate_ready(self.index.project::<ByTimeIterator, ByTxHashIterator>(&it));

            } else {

                if (*it).get_state() == State::REQUESTED && (*it).time <= now {

                    if expired.is_some() {
                        expired.get_mut().push(
                            (
                                (*it).peer, 
                                (*it).clone().into()
                            )
                        );
                    }

                    self.make_completed(&mut self.index.project::<ByTimeIterator,ByTxHashIterator>(&it));

                } else {
                    break;
                }
            }
        }

        while !self.index.empty() {

            // If time went backwards, we may need
            // to demote CANDIDATE_BEST and
            // CANDIDATE_READY announcements back
            // to CANDIDATE_DELAYED. This is an
            // unusual edge case, and unlikely to
            // matter in production. However, it
            // makes it much easier to specify and
            // test TxRequestTracker::Impl's
            // behaviour.
            let idx = self.index.get_by_time();

            let len = idx.len();

            let mut it = idx.into_iter();

            it.advance_by(len - 1);

            it.prev();

            if it.is_selectable() 
            && it.time > now {

                self.change_and_reselect(
                    self.index.project::<ByTimeIterator,ByTxHashIterator>(&it), 
                    State::CANDIDATE_DELAYED
                );

            } else {
                break;
            }
        }
    }
    
    pub fn new(deterministic: bool) -> Self {
    
        todo!();

        /*
        // Explicitly initialize m_index as we
        // need to pass a reference to m_computer
        // to ByTxHashViewExtractor.
        self.index(
            (
                (self.by_peer_view_extractor(),            self.less_by_peer_view()),
                (self.by_tx_hash_view_extractor(computer), self.less_by_tx_hash_view()),
                (self.by_time_view_extractor(),            self.less_by_time_view())
            )
        )
        */
    }
    
    pub fn disconnected_peer(&mut self, peer: NodeId)  {
        
        let index = self.index.get_by_peer();

        let mut it = index.lower_bound(
            &announcement::PeerKey(peer,false,Arc::new(u256::ZERO))
        );

        while it.is_some() && it.as_ref().unwrap().peer == peer {

            //  Check what to continue with after
            //  this iteration. 'it' will be
            //  deleted in what follows, so we
            //  need to decide what to continue
            //  with afterwards. There are
            //  a number of cases to consider:
            //
            //  - std::next(it) is end() or
            //    belongs to a different peer. In
            //    that case, this is the last
            //    iteration of the loop (denote
            //    this by setting it_next to
            //    end()).
            //
            //  - 'it' is not the only
            //    non-COMPLETED announcement for
            //    its txhash. This means it will
            //    be deleted, but no other
            //    Announcement objects will be
            //    modified. Continue with
            //    std::next(it) if it belongs to
            //    the same peer, but decide this
            //    ahead of time (as 'it' may
            //    change position in what
            //    follows).
            //
            //  - 'it' is the only non-COMPLETED
            //    announcement for its
            //    txhash. This means it will be
            //    deleted along with all other
            //    announcements for the same
            //    txhash - which may include
            //    std::next(it). However, other
            //    than 'it', no announcements for
            //    the same peer can be affected
            //    (due to (peer, txhash)
            //    uniqueness). In other words, the
            //    situation where std::next(it) is
            //    deleted can only occur if
            //    std::next(it) belongs to
            //    a different peer but the same
            //    txhash as 'it'. This is covered
            //    by the first bulletpoint
            //    already, and we'll have set
            //    it_next to end().
            let it_next = {

                let helper = {
                    let mut x = it.clone();
                    x.as_mut().unwrap().next();
                    x
                };

                match helper.is_none() || helper.as_ref().unwrap().peer != peer {
                    true   => None,
                    false  => Some(helper),
                }
            };

            // If the announcement isn't already
            // COMPLETED, first make it COMPLETED
            // (which will mark other CANDIDATEs
            // as CANDIDATE_BEST, or delete all of
            // a txhash's announcements if no
            // non-COMPLETED ones are left).
            if self.make_completed(&mut index.project::<ByPeerIterator,ByTxHashIterator>(it.as_ref().unwrap())) {

                // Then actually delete the
                // announcement (unless it was
                // already deleted by
                // MakeCompleted).
                self.erase::<ByPeerIterator>(it.as_ref().unwrap());
            }

            it = it_next.unwrap();
        }
    }
    
    pub fn forget_tx_hash(&mut self, txhash: &u256)  {
        
        let mut it = self.index.get_by_txhash().lower_bound(
            &announcement::TxHashKey(Arc::new(txhash.clone()),State::CANDIDATE_DELAYED,0)
        );

        while it.is_some() 
        && it.as_ref().unwrap().txhash == *txhash 
        {
            it = Some(self.erase::<ByTxHashIterator>(it.as_ref().unwrap()));
        }
    }
    
    pub fn received_inv(&mut self, 
        peer:      NodeId,
        gtxid:     &GenTxId,
        preferred: bool,
        reqtime:   OffsetDateTime /* micros */)  {
        
        //  Bail out if we already have
        //  a CANDIDATE_BEST announcement for this
        //  (txhash, peer) combination. The case
        //  where there is a non-CANDIDATE_BEST
        //  announcement already will be caught by
        //  the uniqueness property of the ByPeer
        //  index when we try to emplace the new
        //  object below.
        if self.index
            .get_by_peer()
            .contains_key(
                &announcement::PeerKey(peer,true,Arc::new(gtxid.get_hash().clone()))
            ) 
        {
            return;
        }

        //  Try creating the announcement with
        //  CANDIDATE_DELAYED state (which will
        //  fail due to the uniqueness of the
        //  ByPeer index if a non-CANDIDATE_BEST
        //  announcement already exists with the
        //  same txhash and peer).
        //
        //  Bail out in that case.
        let ret = self.index
            .get_by_peer()
            .push(
                &Announcement::new(
                    gtxid, 
                    peer, 
                    preferred, 
                    reqtime, 
                    self.current_sequence
                )
            );

        if ret.is_err() {
            return;
        }

        // Update accounting metadata.
        self.peerinfo.get_mut(&peer).as_mut().unwrap().total += 1;

        self.current_sequence += 1;
    }

    /**
      | Find the GenTxIds to request now from
      | peer.
      |
      */
    pub fn get_requestable(&mut self, 
        peer:    NodeId,
        now:     OffsetDateTime /* micros */,
        expired: Amo<Vec<(NodeId,GenTxId)>>) -> Vec<GenTxId> {

        //  Move time.
        self.set_time_point(now,expired);

        // Find all CANDIDATE_BEST announcements
        // for this peer.
        let mut selected: Vec<Announcement> = vec![];

        let mut it_peer = self.index.get_by_peer()
            .lower_bound(&announcement::PeerKey(peer,true,Arc::new(u256::ZERO)));

        while it_peer.is_some() 
        && it_peer.as_ref().unwrap().peer == peer 
        && it_peer.as_ref().unwrap().get_state() == State::CANDIDATE_BEST 
        {
            selected.push((**it_peer.as_ref().unwrap()).clone());

            it_peer.as_mut().unwrap().next();
        }

        // Sort by sequence number.
        selected.sort_by(
            |a: &Announcement,b: &Announcement|
                a.sequence().cmp(&b.sequence())
        );

        // Convert to GenTxId and return.
        let mut ret: Vec<GenTxId> = vec![];

        ret.reserve(selected.len());

        ret.extend(selected.iter().map(|ann: &Announcement| {
            (ann.clone()).into()
        }));

        ret
    }
    
    pub fn requested_tx(&mut self, 
        peer:   NodeId,
        txhash: &u256,
        expiry: OffsetDateTime /* micros */)  {
        
        let mut it = self.index.get_by_peer()
            .get(&announcement::PeerKey(peer,true,Arc::new(txhash.clone())));

        if it.is_none() {

            // There is no CANDIDATE_BEST
            // announcement, look for a _READY or
            // _DELAYED instead. If the caller
            // only ever invokes RequestedTx with
            // the values returned by
            // GetRequestable, and no other
            // non-const functions other than
            // ForgetTxHash and GetRequestable in
            // between, this branch will never
            // execute (as txhashes returned by
            // GetRequestable always correspond to
            // CANDIDATE_BEST announcements).
            it = self.index.get_by_peer().get(&announcement::PeerKey(peer,false,Arc::new(txhash.clone())));

            if it.is_none()
            || (
                it.as_ref().unwrap().get_state() != State::CANDIDATE_DELAYED 
                && it.as_ref().unwrap().get_state() != State::CANDIDATE_READY
            ) 
            {
                // There is no CANDIDATE
                // announcement tracked for this
                // peer, so we have nothing to
                // do. Either this txhash wasn't
                // tracked at all (and the caller
                // should have called
                // ReceivedInv), or it was already
                // requested and/or completed for
                // other reasons and this is just
                // a superfluous RequestedTx call.
                return;
            }

            // Look for an existing CANDIDATE_BEST
            // or REQUESTED with the same
            // txhash. We only need to do this if
            // the found announcement had
            // a different state than
            // CANDIDATE_BEST. If it did,
            // invariants guarantee that no other
            // CANDIDATE_BEST or REQUESTED can
            // exist.
            let it_old = self.index.get_by_txhash()
                .lower_bound(
                    &announcement::TxHashKey(Arc::new(txhash.clone()),State::CANDIDATE_BEST,0)
                );

            if it_old.is_some()
            && it_old.as_ref().unwrap().txhash == *txhash {

                if it_old.as_ref().unwrap().get_state() == State::CANDIDATE_BEST {

                    // The data structure's
                    // invariants require that
                    // there can be at most one
                    // CANDIDATE_BEST or one
                    // REQUESTED announcement per
                    // txhash (but not both
                    // simultaneously), so we have
                    // to convert any existing
                    // CANDIDATE_BEST to another
                    // CANDIDATE_* when
                    // constructing another
                    // REQUESTED.
                    //
                    // It doesn't matter whether
                    // we pick CANDIDATE_READY or
                    // _DELAYED here, as
                    // SetTimePoint() will correct
                    // it at GetRequestable()
                    // time. If time only goes
                    // forward, it will always be
                    // _READY, so pick that to
                    // avoid extra work in
                    // SetTimePoint().
                    self.modify::<ByTxHashIterator>(
                        it_old.as_ref().unwrap(),
                        Box::new(|ann: &mut Announcement| {
                            ann.set_state(State::CANDIDATE_READY);
                        })
                    );

                } else {

                    if it_old.as_ref().unwrap().get_state() == State::REQUESTED {

                        // As we're no longer
                        // waiting for a response
                        // to the previous
                        // REQUESTED announcement,
                        // convert it to
                        // COMPLETED. This also
                        // helps guaranteeing
                        // progress.
                        self.modify::<ByTxHashIterator>(
                            it_old.as_ref().unwrap(),
                            Box::new(|ann: &mut Announcement| {
                                ann.set_state(State::COMPLETED);
                            })
                        );
                    }
                }
            }
        }

        self.modify::<ByPeerIterator>(
            it.as_ref().unwrap(), 
            Box::new(move |ann: &mut Announcement| {
                ann.set_state(State::REQUESTED);
                ann.time = expiry;
            })
        );
    }
    
    pub fn received_response(&mut self, 
        peer:   NodeId,
        txhash: &u256)  {
        
        // We need to search the ByPeer index for
        // both (peer, false, txhash) and (peer,
        // true, txhash).
        let mut it = self.index.get_by_peer()
            .get(&announcement::PeerKey(peer,false,Arc::new(txhash.clone())));

        if it.is_none() {

            it = self.index.get_by_peer()
                .get(&announcement::PeerKey(peer,true,Arc::new(txhash.clone())));
        }

        if let Some(ref it) = it {
            self.make_completed(&mut self.index.project::<ByPeerIterator,ByTxHashIterator>(it));
        }
    }
    
    pub fn count_in_flight(&self, peer: NodeId) -> usize {
        
        let it = self.peerinfo.get(&peer);

        if it.is_some() {
            return it.as_ref().unwrap().requested;
        }

        0
    }
    
    pub fn count_candidates(&self, peer: NodeId) -> usize {
        
        let it = self.peerinfo.get(&peer);

        if it.is_some() {

            let it = it.as_ref().unwrap();

            return it.total - it.requested - it.completed;
        }

        0
    }
    
    pub fn count(&self, peer: NodeId) -> usize {
        
        let it = self.peerinfo.get(&peer);

        if it.is_some() {
            return it.as_ref().unwrap().total;
        }

        0
    }

    /**
      | Count how many announcements are being
      | tracked in total across all peers and
      | transactions.
      |
      */
    pub fn size(&self) -> usize {
        
        self.index.len()
    }
    
    pub fn compute_priority(&self, 
        txhash:    &u256,
        peer:      NodeId,
        preferred: bool) -> u64 {
        
        // Return Priority as a uint64_t as
        // Priority is internal.
        self.computer.invoke(txhash,peer,preferred)
    }
}
