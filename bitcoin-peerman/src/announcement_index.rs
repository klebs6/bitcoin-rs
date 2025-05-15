// ---------------- [ File: bitcoin-peerman/src/announcement_index.rs ]
crate::ix!();

/*
  | Definitions for the 3 indexes used in the main
  | data structure.
  |
  | Each index has a By* type to identify it,
  | a By*View data type to represent the view of
  | announcement it is sorted by, and an
  | By*ViewExtractor type to convert an
  | announcement into the By*View type.
  |
  | See
  | https://www.boost.org/doc/libs/1_58_0/libs/multi_index/doc/reference/key_extraction.html#key_extractors
  | for more information about the key extraction
  | concept.
  */
pub trait Prev: Iterator {
    fn prev(&mut self) -> Option<<Self as Iterator>::Item>;
}

pub mod announcement {

    use super::*;

    /**
      | The ByTime index is sorted by (wait_state,
      | time).
      |
      | All announcements with a timestamp in the
      | future can be found by iterating the index
      | forward from the beginning.
      |
      | All announcements with a timestamp in the past
      | can be found by iterating the index backwards
      | from the end.
      |
      | Uses:
      |
      | * Finding CANDIDATE_DELAYED announcements whose
      |   reqtime has passed, and REQUESTED
      |   announcements whose expiry has passed.
      |
      | * Finding CANDIDATE_READY/BEST announcements
      |   whose reqtime is in the future (when the clock
      |   time went backwards).
      */
    pub struct TimeKey(pub WaitState, pub OffsetDateTime);

    /**
      | The ByPeer index is sorted by (peer, state ==
      | CANDIDATE_BEST, txhash)
      |
      | Uses:
      |
      | - Looking up existing announcements by
      |   peer/txhash, by checking both (peer, false,
      |   txhash) and (peer, true, txhash).
      |
      | - Finding all CANDIDATE_BEST announcements for
      |   a given peer in GetRequestable.
      */
    pub struct PeerKey(pub NodeId, pub bool, pub Arc<u256>);

    /**
      | The ByTxHash index is sorted by (txhash, state,
      | priority).
      |
      | Note: priority == 0 whenever state !=
      | CANDIDATE_READY.
      |
      | Uses:
      |
      | - Deleting all announcements with a given
      |   txhash in ForgetTxHash.
      |
      | - Finding the best CANDIDATE_READY to convert
      |   to CANDIDATE_BEST, when no other
      |   CANDIDATE_READY or REQUESTED announcement
      |   exists for that txhash.
      |
      | - Determining when no more non-COMPLETED
      |   announcements for a given txhash exist, so
      |   the COMPLETED ones can be deleted.
      */
    pub struct TxHashKey(pub Arc<u256>, pub State, pub Priority);
}

impl From<&Announcement> for announcement::TimeKey {

    fn from(ann: &Announcement) -> Self {
        Self(get_wait_state(ann), ann.time)
    }
}

impl From<&Announcement> for announcement::PeerKey {
    
    fn from(ann: &Announcement) -> Self {
        Self(
            ann.peer,
            ann.get_state() == State::CANDIDATE_BEST,
            Arc::new(ann.txhash.clone())
        )
    }
}

impl From<(&Announcement,&PriorityComputer)> for announcement::TxHashKey {
    
    fn from(x: (&Announcement, &PriorityComputer)) -> Self {

        let prio: Priority = match (x.0.get_state() == State::CANDIDATE_READY) {
            true   => (x.1).invoke_announcement(x.0),
            false  => 0
        };

        Self(Arc::new(x.0.txhash.clone()), x.0.get_state(), prio)
    }
}

multidex!{

    name => AnnouncementIndex,

    item => Announcement,

    ordered_unique => [
        (ByPeer, Announcement, ByPeerViewExtractor)
    ]

    ordered_nonunique => [
        (ByTxHash, Announcement, ByTxHashViewExtractor),
        (ByTime,   Announcement, ByTimeViewExtractor)
    ]
}

/**
  | Data type for the main data structure
  | (Announcement objects with ByPeer/ByTxHash/ByTime
  | indexes).
  |
  | using Index = boost::multi_index_container<
  |     Announcement,
  |     boost::multi_index::indexed_by<
  |         boost::multi_index::ordered_unique<    boost::multi_index::tag<ByPeer>,   ByPeerViewExtractor>,
  |         boost::multi_index::ordered_non_unique<boost::multi_index::tag<ByTxHash>, ByTxHashViewExtractor>,
  |         boost::multi_index::ordered_non_unique<boost::multi_index::tag<ByTime>,   ByTimeViewExtractor>
  |     >
  | >;
  */
#[derive(Clone,Debug)]
pub struct AnnouncementIndex {
    pub data:      Vec<Announcement>,
    pub by_peer:   ByPeerIndex,
    pub by_txhash: ByTxHashIndex,
    pub by_time:   ByTimeIndex,
}

impl ProjectIndex for AnnouncementIndex {

    fn project_from_peekable<Src: AnnouncementIterator, Dst: AnnouncementIterator>(&self, x: &CppIter<Src>) -> Option<CppIter<Dst>> {
        todo!();
    }

    fn project<Src: AnnouncementIterator, Dst: AnnouncementIterator>(&self, x: &Src) -> Option<Dst> {
        todo!();
    }
}

impl AnnouncementIndex {

    pub fn len(&self)      -> usize { self.data.len() }

    pub fn is_empty(&self) -> bool  { self.data.is_empty() }

    pub fn get<I: AnnouncementIterator>(&self) -> <<I as AnnouncementIterator>::Tag as AnnouncementIndexTag>::Index {

        <I as AnnouncementIterator>::Tag::get_index(self)
    }

    pub fn get_by_peer(&self) -> ByPeerIndex {
        todo!();
    }

    pub fn get_by_txhash(&self) -> ByTxHashIndex {
        todo!();
    }

    pub fn get_by_time(&self) -> ByTimeIndex {
        todo!();
    }

    //-----------------------------------
    pub fn insert(&mut self, item: Announcement) {
        todo!();
    }

    pub fn empty(&self) -> bool {
        todo!();
    }
}

/*
impl IntoIterator for AnnouncementIndex {

    type Item = Announcement;

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        todo!();
    }
}
*/

pub trait ProjectIndex {

    fn project_from_peekable<Src: AnnouncementIterator, Dst: AnnouncementIterator>(&self, x: &CppIter<Src>) -> Option<CppIter<Dst>>;

    fn project<Src: AnnouncementIterator, Dst: AnnouncementIterator>(&self, x: &Src) -> Option<Dst>;
}

pub trait AnnouncementIndexIndex {

    type IteratorType: AnnouncementIterator;

    type KeyType;

    fn len(&self)      -> usize;

    fn is_empty(&self) -> bool;

    fn lower_bound(&self, x: &Self::KeyType) 
    -> Option<Self::IteratorType> where <Self as AnnouncementIndexIndex>::IteratorType: Iterator 
    {
        todo!();
    }

    fn lower_bound_with_peekable(&self, x: &Self::KeyType) -> CppIter<Self::IteratorType> where <Self as AnnouncementIndexIndex>::IteratorType: Iterator {
        todo!();
    }

    fn push(&mut self, x: &Announcement) -> Result<(), &'static str> {
        todo!();
    }

    fn count(&self, x: &Self::KeyType) -> usize {
        todo!();
    }

    fn contains_key(&self, x: &Self::KeyType) -> bool {
        todo!();
    }

    fn remove(&mut self, x: &Self::KeyType) 
    -> CppIter<Self::IteratorType> where <Self as AnnouncementIndexIndex>::IteratorType: Iterator 
    {
        todo!();
    }

    fn remove_announcement<T>(&mut self, x: &Announcement) -> T 
    {
        todo!();
    }

    fn get(&self, x: &Self::KeyType) -> Option<Self::IteratorType> where <Self as AnnouncementIndexIndex>::IteratorType: Iterator {
        todo!();
    }

    fn get_peekable(&self, x: &Self::KeyType) -> CppIter<Self::IteratorType> where <Self as AnnouncementIndexIndex>::IteratorType: Iterator {
        todo!();
    }

    fn modify_with_peekable<I: AnnouncementIterator>(&mut self, 
        it:       &CppIter<I>,
        modifier: AnnouncementModifier)  {
        todo!();
    }

    fn modify<I: AnnouncementIterator>(&mut self, 
        it:       &I,
        modifier: AnnouncementModifier)  {
        todo!();
    }
}

impl<I: AnnouncementIndexIndex> ProjectIndex for I {

    fn project_from_peekable<Src: AnnouncementIterator, Dst: AnnouncementIterator>(&self, x: &CppIter<Src>) -> Option<CppIter<Dst>> {
        todo!();
    }

    fn project<Src: AnnouncementIterator, Dst: AnnouncementIterator>(&self, x: &Src) -> Option<Dst> {
        todo!();
    }
}

//----------------------------
#[derive(Clone,Debug)]
pub struct ByPeerIndex {
    pub parent: Arc<AnnouncementIndex>,
    pub order:  Vec<usize>,
}

impl AnnouncementIndexIndex for ByPeerIndex {

    type IteratorType = ByPeerIterator;

    type KeyType      = announcement::PeerKey;

    fn len(&self)      -> usize { self.parent.len() }

    fn is_empty(&self) -> bool  { self.parent.is_empty() }
}

#[derive(Debug,Clone)]
pub struct ByPeerIterator {
    pub parent:  Arc<ByPeerIndex>,
    pub current: usize,
}

impl Prev for ByPeerIterator {

    fn prev(&mut self) -> Option<Self::Item> {
        todo!();
    }
}

impl AnnouncementIterator for ByPeerIterator {
    type Tag   = ByPeer;
    type Index = ByPeerIndex;
}

impl Deref for ByPeerIterator {

    type Target = Announcement;

    fn deref(&self) -> &Self::Target {
        let idx: usize = self.parent.order[self.current];
        &self.parent.parent.data[idx]
    }
}

impl Iterator for ByPeerIterator {
    type Item = Announcement;
    
    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }
}

impl IntoIterator for ByPeerIndex {

    type Item = Announcement;

    type IntoIter = ByPeerIterator;

    fn into_iter(self) -> Self::IntoIter {
        todo!();
    }
}

//---------------------------------
#[derive(Clone,Debug)]
pub struct ByTxHashIndex {
    pub parent:   Arc<AnnouncementIndex>,
    pub order:    Vec<usize>,
    pub computer: Arc<PriorityComputer>,
}

impl AnnouncementIterator for ByTxHashIterator {
    type Tag   = ByTxHash;
    type Index = ByTxHashIndex;
}

impl AnnouncementIndexIndex for ByTxHashIndex {

    type IteratorType = ByTxHashIterator;

    type KeyType      = announcement::TxHashKey;

    fn len(&self)      -> usize { self.parent.len() }

    fn is_empty(&self) -> bool  { self.parent.is_empty() }
}

#[derive(Debug,Clone)]
pub struct ByTxHashIterator {
    pub parent:  Arc<ByTxHashIndex>,
    pub current: usize,
}

impl Deref for ByTxHashIterator {

    type Target = Announcement;

    fn deref(&self) -> &Self::Target {
        let idx: usize = self.parent.order[self.current];
        &self.parent.parent.data[idx]
    }
}

impl Prev for ByTxHashIterator {

    fn prev(&mut self) -> Option<Self::Item> {
        todo!();
    }
}

impl Iterator for ByTxHashIterator {

    type Item = Announcement;
    
    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }
}

impl IntoIterator for ByTxHashIndex {
    type Item     = Announcement;
    type IntoIter = ByTxHashIterator;

    fn into_iter(self) -> Self::IntoIter {
        todo!();
    }
}

//---------------------------------
#[derive(Clone,Debug)]
pub struct ByTimeIndex {
    pub parent: Arc<AnnouncementIndex>,
    pub order:  Vec<usize>,
}

impl AnnouncementIndexIndex for ByTimeIndex {

    type IteratorType = ByTimeIterator;

    type KeyType      = announcement::TimeKey;

    fn len(&self)      -> usize { self.parent.len() }

    fn is_empty(&self) -> bool  { self.parent.is_empty() }
}

#[derive(Debug,Clone)]
pub struct ByTimeIterator {
    pub parent:  Arc<ByTimeIndex>,
    pub current: usize,
}

impl AnnouncementIterator for ByTimeIterator {
    type Tag   = ByTime;
    type Index = ByTimeIndex;
}

impl Prev for ByTimeIterator {

    fn prev(&mut self) -> Option<Self::Item> {
        todo!();
    }
}

impl Deref for ByTimeIterator {

    type Target = Announcement;

    fn deref(&self) -> &Self::Target {
        let idx: usize = self.parent.order[self.current];
        &self.parent.parent.data[idx]
    }
}

impl Iterator for ByTimeIterator {

    type Item = Announcement;
    
    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }
}

impl IntoIterator for ByTimeIndex {
    type Item     = Announcement;
    type IntoIter = ByTimeIterator;

    fn into_iter(self) -> Self::IntoIter {
        todo!();
    }
}

//------------------------

pub trait AnnouncementIndexTag {

    type Index: AnnouncementIndexIndex;

    fn get_index(x: &AnnouncementIndex) -> Self::Index;
}

//------------------------
pub struct ByPeer   {} 

impl AnnouncementIndexTag for ByPeer { 

    type Index = ByPeerIndex;

    fn get_index(x: &AnnouncementIndex) -> Self::Index {
        x.get_by_peer()
    }
}

//------------------------
pub struct ByTxHash {}

impl AnnouncementIndexTag for ByTxHash { 

    type Index = ByTxHashIndex; 

    fn get_index(x: &AnnouncementIndex) -> Self::Index {
        x.get_by_txhash()
    }
}

//------------------------
pub struct ByTime   {} 

impl AnnouncementIndexTag for ByTime { 

    type Index = ByTimeIndex;

    fn get_index(x: &AnnouncementIndex) -> Self::Index {
        x.get_by_time()
    }
}

//---------------------------------

pub trait AnnouncementIterator: Deref<Target=Announcement> + Iterator<Item=Announcement> {
    type Tag: AnnouncementIndexTag;
    type Index: AnnouncementIndexIndex;
}
