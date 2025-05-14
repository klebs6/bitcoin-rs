// ---------------- [ File: bitcoin-qt/src/peertablemodel.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/peertablemodel.h]
//-------------------------------------------[.cpp/bitcoin/src/qt/peertablemodel.cpp]

#[Q_METATYPE]
pub struct NodeCombinedStats {
    node_stats:                 NodeStats,
    node_state_stats:           NodeStateStats,
    node_state_stats_available: bool,
}

/**
  | Qt model providing information about
  | connected peers, similar to the "getpeerinfo"
  | RPC call. Used by the rpc console UI.
  |
  */
#[Q_OBJECT]
pub struct PeerTableModel {

    base:       QAbstractTableModel,

    /**
      | Internal peer data structure.
      |
      */
    peers_data: QList<NodeCombinedStats>,

    node:       Rc<RefCell<dyn NodeInterface>>,

    columns:    QStringList,

    timer: *mut QTimer,
}

pub mod peer_table_model {

    use super::*;

    pub enum ColumnIndex {
        NetNodeId = 0,
        Address,
        Direction,
        ConnectionType,
        Network,
        Ping,
        Sent,
        Received,
        Subversion
    }

    pub const StatsRole: i32 = USER_ROLE;
}

impl PeerTableModel {

    pub fn default_columns() -> QStringList {

        todo!();

        /*
        const QStringList columns{
                /*: Title of Peers Table column which contains a
                    unique number used to identify a connection. */
                tr("Peer"),
                /*: Title of Peers Table column which contains the
                    IP/Onion/I2P address of the connected peer. */
                tr("Address"),
                /*: Title of Peers Table column which indicates the direction
                    the peer connection was initiated from. */
                tr("Direction"),
                /*: Title of Peers Table column which describes the type of
                    peer connection. The "type" describes why the connection exists. */
                tr("Type"),
                /*: Title of Peers Table column which states the network the peer
                    connected through. */
                tr("Network"),
                /*: Title of Peers Table column which indicates the current latency
                    of the connection with the peer. */
                tr("Ping"),
                /*: Title of Peers Table column which indicates the total amount of
                    network information we have sent to the peer. */
                tr("Sent"),
                /*: Title of Peers Table column which indicates the total amount of
                    network information we have received from the peer. */
                tr("Received"),
                /*: Title of Peers Table column which contains the peer's
                    User Agent string. */
                tr("User Agent")};
        */
    }

    pub fn new(
        node:   Rc<RefCell<dyn NodeInterface>>,
        parent: *mut QObject) -> Self {
    
        todo!();
        /*
           :
        QAbstractTableModel(parent),
        m_node(node),
        timer(nullptr)
        // set up timer for auto refresh
        timer = new QTimer(this);
        connect(timer, &QTimer::timeout, this, &PeerTableModel::refresh);
        timer->setInterval(MODEL_UPDATE_DELAY);

        // load initial data
        refresh();
        */
    }
    
    pub fn start_auto_refresh(&mut self)  {
        
        todo!();
        /*
            timer->start();
        */
    }
    
    pub fn stop_auto_refresh(&mut self)  {
        
        todo!();
        /*
            timer->stop();
        */
    }
    
    /**
      | @name Methods overridden from QAbstractTableModel
      |
      */
    pub fn row_count(&self, parent: Option<&QModelIndex>) -> i32 {

        let parent: &QModelIndex = unsafe { parent.unwrap_or(&QModelIndex::new()) };
        
        todo!();
        /*
            if (parent.isValid()) {
            return 0;
        }
        return m_peers_data.size();
        */
    }
    
    pub fn column_count(&self, parent: Option<&QModelIndex>) -> i32 {

        let parent: &QModelIndex = unsafe { parent.unwrap_or(&QModelIndex::new()) };
        
        todo!();
        /*
            if (parent.isValid()) {
            return 0;
        }
        return columns.length();
        */
    }
    
    pub fn data(&self, 
        index: &QModelIndex,
        role:  Option<i32>) -> QVariant {

        let role: i32 = role.unwrap_or(QtDisplayRole.try_into().unwrap());
        
        todo!();
        /*
            if(!index.isValid())
            return QVariant();

        NodeCombinedStats *rec = static_cast<NodeCombinedStats*>(index.internalPointer());

        const auto column = static_cast<ColumnIndex>(index.column());
        if (role == QtDisplayRole) {
            switch (column) {
            case NetNodeId:
                return (i64)rec->nodeStats.nodeid;
            case Address:
                return QString::fromStdString(rec->nodeStats.m_addr_name);
            case Direction:
                return QString(rec->nodeStats.fInbound ?
                                   //: An Inbound Connection from a Peer.
                                   tr("Inbound") :
                                   //: An Outbound Connection to a Peer.
                                   tr("Outbound"));
            case ConnectionType:
                return typename gui_util::ConnectionTypeToQString(rec->nodeStats.m_conn_type, /* prepend_direction */ false);
            case Network:
                return typename gui_util::NetworkToQString(rec->nodeStats.m_network);
            case Ping:
                return typename gui_util::formatPingTime(rec->nodeStats.m_min_ping_time);
            case Sent:
                return typename gui_util::formatBytes(rec->nodeStats.nSendBytes);
            case Received:
                return typename gui_util::formatBytes(rec->nodeStats.nRecvBytes);
            case Subversion:
                return QString::fromStdString(rec->nodeStats.cleanSubVer);
            } // no default case, so the compiler can warn about missing cases
            assert(false);
        } else if (role == QtTextAlignmentRole) {
            switch (column) {
            case NetNodeId:
                return QVariant(QtAlignRight | QtAlignVCenter);
            case Address:
                return {};
            case Direction:
            case ConnectionType:
            case Network:
                return QVariant(QtAlignCenter);
            case Ping:
            case Sent:
            case Received:
                return QVariant(QtAlignRight | QtAlignVCenter);
            case Subversion:
                return {};
            } // no default case, so the compiler can warn about missing cases
            assert(false);
        } else if (role == StatsRole) {
            return QVariant::fromValue(rec);
        }

        return QVariant();
        */
    }
    
    pub fn header_data(&self, 
        section:     i32,
        orientation: QtOrientation,
        role:        Option<i32>) -> QVariant {

        let role: i32 = role.unwrap_or(QtDisplayRole.try_into().unwrap());
        
        todo!();
        /*
            if(orientation == QtHorizontal)
        {
            if(role == QtDisplayRole && section < columns.size())
            {
                return columns[section];
            }
        }
        return QVariant();
        */
    }
    
    pub fn flags(&self, index: &QModelIndex) -> QtItemFlags {
        
        todo!();
        /*
            if (!index.isValid()) return QtNoItemFlags;

        QtItemFlags retval = QtItemIsSelectable | QtItemIsEnabled;
        return retval;
        */
    }
    
    pub fn index(&self, 
        row:    i32,
        column: i32,
        parent: Option<&QModelIndex>) -> QModelIndex {

        let parent: &QModelIndex = unsafe { parent.unwrap_or(&QModelIndex::new()) };
        
        todo!();
        /*
            Q_UNUSED(parent);

        if (0 <= row && row < rowCount() && 0 <= column && column < columnCount()) {
            return createIndex(row, column, const_cast<NodeCombinedStats*>(&m_peers_data[row]));
        }

        return QModelIndex();
        */
    }
    
    #[Q_SLOT]
    pub fn refresh(&mut self)  {
        
        todo!();
        /*
            typename NodeInterface::NodesStats nodes_stats;
        m_node.getNodesStats(nodes_stats);
        decltype(m_peers_data) new_peers_data;
        new_peers_data.reserve(nodes_stats.size());
        for (const auto& node_stats : nodes_stats) {
            const NodeCombinedStats stats{std::get<0>(node_stats), std::get<2>(node_stats), std::get<1>(node_stats)};
            new_peers_data.append(stats);
        }

        // Handle peer addition or removal as suggested in Qt Docs. See:
        // - https://doc.qt.io/qt-5/model-view-programming.html#inserting-and-removing-rows
        // - https://doc.qt.io/qt-5/model-view-programming.html#resizable-models
        // We take advantage of the fact that the std::vector returned
        // by typename NodeInterface::getNodesStats is sorted by nodeid.
        for (int i = 0; i < m_peers_data.size();) {
            if (i < new_peers_data.size() && m_peers_data.at(i).nodeStats.nodeid == new_peers_data.at(i).nodeStats.nodeid) {
                ++i;
                continue;
            }
            // A peer has been removed from the table.
            beginRemoveRows(QModelIndex(), i, i);
            m_peers_data.erase(m_peers_data.begin() + i);
            endRemoveRows();
        }

        if (m_peers_data.size() < new_peers_data.size()) {
            // Some peers have been added to the end of the table.
            beginInsertRows(QModelIndex(), m_peers_data.size(), new_peers_data.size() - 1);
            m_peers_data.swap(new_peers_data);
            endInsertRows();
        } else {
            m_peers_data.swap(new_peers_data);
        }

        const auto top_left = index(0, 0);
        const auto bottom_right = index(rowCount() - 1, columnCount() - 1);
        Q_EMIT dataChanged(top_left, bottom_right);
        */
    }
}
