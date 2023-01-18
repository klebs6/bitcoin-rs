crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/peertablesortproxy.h]
//-------------------------------------------[.cpp/bitcoin/src/qt/peertablesortproxy.cpp]

#[Q_OBJECT]
pub struct PeerTableSortProxy {
    base: QSortFilterProxyModel,
}

impl PeerTableSortProxy {
    
    pub fn new(parent: *mut QObject) -> Self {
    
        todo!();
        /*
            : QSortFilterProxyModel(parent)
        */
    }
    
    pub fn less_than(&self, 
        left_index:  &QModelIndex,
        right_index: &QModelIndex) -> bool {
        
        todo!();
        /*
            const NodeStats left_stats = Assert(sourceModel()->data(left_index, PeerTableModel::StatsRole).value<NodeCombinedStats*>())->nodeStats;
        const NodeStats right_stats = Assert(sourceModel()->data(right_index, PeerTableModel::StatsRole).value<NodeCombinedStats*>())->nodeStats;

        switch (static_cast<PeerTableModel::ColumnIndex>(left_index.column())) {
        case PeerTableModel::NetNodeId:
            return left_stats.nodeid < right_stats.nodeid;
        case PeerTableModel::Address:
            return left_stats.m_addr_name.compare(right_stats.m_addr_name) < 0;
        case PeerTableModel::Direction:
            return left_stats.fInbound > right_stats.fInbound; // default sort Inbound, then Outbound
        case PeerTableModel::ConnectionType:
            return left_stats.m_conn_type < right_stats.m_conn_type;
        case PeerTableModel::Network:
            return left_stats.m_network < right_stats.m_network;
        case PeerTableModel::Ping:
            return left_stats.m_min_ping_time < right_stats.m_min_ping_time;
        case PeerTableModel::Sent:
            return left_stats.nSendBytes < right_stats.nSendBytes;
        case PeerTableModel::Received:
            return left_stats.nRecvBytes < right_stats.nRecvBytes;
        case PeerTableModel::Subversion:
            return left_stats.cleanSubVer.compare(right_stats.cleanSubVer) < 0;
        } // no default case, so the compiler can warn about missing cases
        assert(false);
        */
    }
}
