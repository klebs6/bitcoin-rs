crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/clientmodel.h]

pub enum BlockSource {
    NONE,
    REINDEX,
    DISK,
    NETWORK
}

/*TODO
crate::bitflags!{
        const CONNECTIONS_IN   = 1 << 0,
        const CONNECTIONS_OUT  = 1 << 1,
        const CONNECTIONS_ALL  = CONNECTIONS_IN.bits | CONNECTIONS_OUT.bits
    }
}
*/

/**
  | Model for Bitcoin network client.
  |
  */
#[Q_OBJECT]
pub struct ClientModel {
    base:                                   QObject,

    /**
      | caches for the best header: hash, number
      | of blocks and block time
      |
      */
    cached_best_header_height:              RefCell<Atomic<i32>>,

    cached_best_header_time:                RefCell<Atomic<i64>>,
    cached_num_blocks:                      RefCell<Atomic<i32>>, // default = { -1 }
    cached_tip_mutex:                       std::sync::Mutex<client_model::CachedTip>,

    node:                                   Rc<RefCell<dyn NodeInterface>>,
    handler_show_progress:                  Box<dyn Handler>,
    handler_notify_num_connections_changed: Box<dyn Handler>,
    handler_notify_network_active_changed:  Box<dyn Handler>,
    handler_notify_alert_changed:           Box<dyn Handler>,
    handler_banned_list_changed:            Box<dyn Handler>,
    handler_notify_block_tip:               Box<dyn Handler>,
    handler_notify_header_tip:              Box<dyn Handler>,
    options_model:                          *mut OptionsModel,
    peer_table_model:                       *mut PeerTableModel,
    peer_table_sort_proxy:                  *mut PeerTableSortProxy, // default = { nullptr }
    ban_table_model:                        *mut BanTableModel,

    /**
      | A thread to interact with m_node asynchronously
      |
      */
    thread:                                 *const QThread,
}

pub mod client_model {
    use super::*;
    pub struct CachedTip {
        cached_tip_blocks: u256,
    }
}

impl ClientModel {

    pub fn node(&self) -> Rc<RefCell<dyn NodeInterface>> {
        
        todo!();
        /*
            return m_node;
        */
    }
    
    #[Q_SIGNAL]
    pub fn num_connections_changed(&mut self, count: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Fired when a message should be reported
      | to the user
      |
      */
    pub fn message(&mut self, 
        title:   &String,
        message: &String,
        style:   u32)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Show progress dialog e.g. for verifychain
      |
      */
    pub fn show_progress(&mut self, 
        title:      &String,
        n_progress: i32)  {
        
        todo!();
        /*
        
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/clientmodel.cpp]

lazy_static!{
    /*
    static int64_t nLastHeaderTipUpdateNotification = 0;
    static int64_t nLastBlockTipUpdateNotification = 0;
    */
}

impl Drop for ClientModel {
    fn drop(&mut self) {
        todo!();
        /*
            unsubscribeFromCoreSignals();

        m_thread->quit();
        m_thread->wait();
        */
    }
}

impl ClientModel {

    pub fn new(
        node:          Rc<RefCell<dyn NodeInterface>>,
        options_model: *mut OptionsModel,
        parent:        *mut QObject) -> Self {
    
        todo!();
        /*


            :
        QObject(parent),
        m_node(node),
        optionsModel(_optionsModel),
        peerTableModel(nullptr),
        banTableModel(nullptr),
        m_thread(new QThread(this))
        cachedBestHeaderHeight = -1;
        cachedBestHeaderTime = -1;

        peerTableModel = new PeerTableModel(m_node, this);
        m_peer_table_sort_proxy = new PeerTableSortProxy(this);
        m_peer_table_sort_proxy->setSourceModel(peerTableModel);

        banTableModel = new BanTableModel(m_node, this);

        QTimer* timer = new QTimer;
        timer->setInterval(MODEL_UPDATE_DELAY);
        connect(timer, &QTimer::timeout, [this] {
            // no locking required at this point
            // the following calls will acquire the required lock
            Q_EMIT mempoolSizeChanged(m_node.getMempoolSize(), m_node.getMempoolDynamicUsage());
            Q_EMIT bytesChanged(m_node.getTotalBytesRecv(), m_node.getTotalBytesSent());
        });
        connect(m_thread, &QThread::finished, timer, &QObject::deleteLater);
        connect(m_thread, &QThread::started, [timer] { timer->start(); });
        // move timer to thread so that polling doesn't disturb main event loop
        timer->moveToThread(m_thread);
        m_thread->start();
        QTimer::singleShot(0, timer, []() {
            util::ThreadRename("qt-clientmodl");
        });

        subscribeToCoreSignals();
        */
    }
    
    /**
      | Return number of connections, default
      | is in- and outbound (total)
      |
      */
    pub fn get_num_connections(&self, flags: Option<u32>) -> i32 {

        //let flags: u32 = flags.unwrap_or(NumConnections::CONNECTIONS_ALL);
        
        todo!();
        /*
            ConnectionDirection connections = ConnectionDirection::None;

        if(flags == NumConnections::CONNECTIONS_IN)
            connections = ConnectionDirection::In;
        else if (flags == NumConnections::CONNECTIONS_OUT)
            connections = ConnectionDirection::Out;
        else if (flags == NumConnections::CONNECTIONS_ALL)
            connections = ConnectionDirection::Both;

        return m_node.getNodeCount(connections);
        */
    }
    
    pub fn get_header_tip_height(&self) -> i32 {
        
        todo!();
        /*
            if (cachedBestHeaderHeight == -1) {
            // make sure we initially populate the cache via a cs_main lock
            // otherwise we need to wait for a tip update
            int height;
            int64_t blockTime;
            if (m_node.getHeaderTip(height, blockTime)) {
                cachedBestHeaderHeight = height;
                cachedBestHeaderTime = blockTime;
            }
        }
        return cachedBestHeaderHeight;
        */
    }
    
    pub fn get_header_tip_time(&self) -> i64 {
        
        todo!();
        /*
            if (cachedBestHeaderTime == -1) {
            int height;
            int64_t blockTime;
            if (m_node.getHeaderTip(height, blockTime)) {
                cachedBestHeaderHeight = height;
                cachedBestHeaderTime = blockTime;
            }
        }
        return cachedBestHeaderTime;
        */
    }
    
    pub fn get_num_blocks(&self) -> i32 {
        
        todo!();
        /*
            if (m_cached_num_blocks == -1) {
            m_cached_num_blocks = m_node.getNumBlocks();
        }
        return m_cached_num_blocks;
        */
    }
    
    pub fn get_best_block_hash(&mut self) -> u256 {
        
        todo!();
        /*
            uint256 tip{
        [&]() { LOCK(m_cached_tip_mutex);  return m_cached_tip_blocks }()
        };

        if (!tip.IsNull()) {
            return tip;
        }

        // Lock order must be: first `cs_main`, then `m_cached_tip_mutex`.
        // The following will lock `cs_main` (and release it), so we must not
        // own `m_cached_tip_mutex` here.
        tip = m_node.getBestBlockHash();

        LOCK(m_cached_tip_mutex);
        // We checked that `m_cached_tip_blocks` is not null above, but then we
        // released the mutex `m_cached_tip_mutex`, so it could have changed in the
        // meantime. Thus, check again.
        if (m_cached_tip_blocks.IsNull()) {
            m_cached_tip_blocks = tip;
        }
        return m_cached_tip_blocks;
        */
    }
    
    #[Q_SLOT]
    pub fn update_num_connections(&mut self, num_connections: i32)  {
        
        todo!();
        /*
            Q_EMIT numConnectionsChanged(numConnections);
        */
    }
    
    pub fn update_network_active(&mut self, network_active: bool)  {
        
        todo!();
        /*
            Q_EMIT networkActiveChanged(networkActive);
        */
    }
    
    pub fn update_alert(&mut self)  {
        
        todo!();
        /*
            Q_EMIT alertsChanged(getStatusBarWarnings());
        */
    }
    
    /**
      | Returns enum BlockSource of the current
      | importing/syncing state
      |
      */
    pub fn get_block_source(&self) -> BlockSource {
        
        todo!();
        /*
            if (m_node.getReindex())
            return BlockSource::REINDEX;
        else if (m_node.getImporting())
            return BlockSource::DISK;
        else if (getNumConnections() > 0)
            return BlockSource::NETWORK;

        return BlockSource::NONE;
        */
    }
    
    /**
      | Return warnings to be displayed in status
      | bar
      |
      */
    pub fn get_status_bar_warnings(&self) -> String {
        
        todo!();
        /*
            return QString::fromStdString(m_node.getWarnings().translated);
        */
    }
    
    pub fn get_options_model(&mut self) -> *mut OptionsModel {
        
        todo!();
        /*
            return optionsModel;
        */
    }
    
    pub fn get_peer_table_model(&mut self) -> *mut PeerTableModel {
        
        todo!();
        /*
            return peerTableModel;
        */
    }
    
    pub fn peer_table_sort_proxy(&mut self) -> *mut PeerTableSortProxy {
        
        todo!();
        /*
            return m_peer_table_sort_proxy;
        */
    }
    
    pub fn get_ban_table_model(&mut self) -> *mut BanTableModel {
        
        todo!();
        /*
            return banTableModel;
        */
    }
    
    pub fn format_full_version(&self) -> String {
        
        todo!();
        /*
            return QString::fromStdString(FormatFullVersion());
        */
    }
    
    pub fn format_sub_version(&self) -> String {
        
        todo!();
        /*
            return QString::fromStdString(strSubVersion);
        */
    }
    
    pub fn is_release_version(&self) -> bool {
        
        todo!();
        /*
            return CLIENT_VERSION_IS_RELEASE;
        */
    }
    
    pub fn format_client_startup_time(&self) -> String {
        
        todo!();
        /*
            return QDateTime::fromSecsSinceEpoch(GetStartupTime()).toString();
        */
    }
    
    pub fn data_dir(&self) -> String {
        
        todo!();
        /*
            return typename gui_util::boostPathToQString(gArgs.GetDataDirNet());
        */
    }
    
    pub fn blocks_dir(&self) -> String {
        
        todo!();
        /*
            return typename gui_util::boostPathToQString(gArgs.GetBlocksDirPath());
        */
    }
    
    pub fn update_banlist(&mut self)  {
        
        todo!();
        /*
            banTableModel->refresh();
        */
    }
    
    pub fn subscribe_to_core_signals(&mut self)  {
        
        todo!();
        /*
            // Connect signals to client
        m_handler_show_progress = m_node.handleShowProgress(std::bind(ShowProgress, this, std::placeholders::_1, std::placeholders::_2));
        m_handler_notify_num_connections_changed = m_node.handleNotifyNumConnectionsChanged(std::bind(NotifyNumConnectionsChanged, this, std::placeholders::_1));
        m_handler_notify_network_active_changed = m_node.handleNotifyNetworkActiveChanged(std::bind(NotifyNetworkActiveChanged, this, std::placeholders::_1));
        m_handler_notify_alert_changed = m_node.handleNotifyAlertChanged(std::bind(NotifyAlertChanged, this));
        m_handler_banned_list_changed = m_node.handleBannedListChanged(std::bind(BannedListChanged, this));
        m_handler_notify_block_tip = m_node.handleNotifyBlockTip(std::bind(BlockTipChanged, this, std::placeholders::_1, std::placeholders::_2, std::placeholders::_3, false));
        m_handler_notify_header_tip = m_node.handleNotifyHeaderTip(std::bind(BlockTipChanged, this, std::placeholders::_1, std::placeholders::_2, std::placeholders::_3, true));
        */
    }
    
    pub fn unsubscribe_from_core_signals(&mut self)  {
        
        todo!();
        /*
            // Disconnect signals from client
        m_handler_show_progress->disconnect();
        m_handler_notify_num_connections_changed->disconnect();
        m_handler_notify_network_active_changed->disconnect();
        m_handler_notify_alert_changed->disconnect();
        m_handler_banned_list_changed->disconnect();
        m_handler_notify_block_tip->disconnect();
        m_handler_notify_header_tip->disconnect();
        */
    }
    
    pub fn get_proxy_info(&self, ip_port: &mut String) -> bool {
        
        todo!();
        /*
            proxyType ipv4, ipv6;
        if (m_node.getProxy((Network) 1, ipv4) && m_node.getProxy((Network) 2, ipv6)) {
          ip_port = ipv4.proxy.ToStringIPPort();
          return true;
        }
        return false;
        */
    }
}

/**
  | Handlers for core signals
  |
  */
pub fn show_progress(
        clientmodel: *mut ClientModel,
        title:       &String,
        n_progress:  i32)  {
    
    todo!();
        /*
            // emits signal "showProgress"
        bool invoked = QMetaObject::invokeMethod(clientmodel, "showProgress", QtQueuedConnection,
                                  Q_ARG(QString, QString::fromStdString(title)),
                                  Q_ARG(int, nProgress));
        assert(invoked);
        */
}

pub fn notify_num_connections_changed(
        clientmodel:         *mut ClientModel,
        new_num_connections: i32)  {
    
    todo!();
        /*
            // Too noisy: qDebug() << "NotifyNumConnectionsChanged: " + QString::number(newNumConnections);
        bool invoked = QMetaObject::invokeMethod(clientmodel, "updateNumConnections", QtQueuedConnection,
                                  Q_ARG(int, newNumConnections));
        assert(invoked);
        */
}

pub fn notify_network_active_changed(
        clientmodel:    *mut ClientModel,
        network_active: bool)  {
    
    todo!();
        /*
            bool invoked = QMetaObject::invokeMethod(clientmodel, "updateNetworkActive", QtQueuedConnection,
                                  Q_ARG(bool, networkActive));
        assert(invoked);
        */
}

pub fn notify_alert_changed(clientmodel: *mut ClientModel)  {
    
    todo!();
        /*
            qDebug() << "NotifyAlertChanged";
        bool invoked = QMetaObject::invokeMethod(clientmodel, "updateAlert", QtQueuedConnection);
        assert(invoked);
        */
}

pub fn banned_list_changed(clientmodel: *mut ClientModel)  {
    
    todo!();
        /*
            qDebug() << QString("%1: Requesting update for peer banlist").arg(__func__);
        bool invoked = QMetaObject::invokeMethod(clientmodel, "updateBanlist", QtQueuedConnection);
        assert(invoked);
        */
}

pub fn block_tip_changed(
        clientmodel:           *mut ClientModel,
        sync_state:            SynchronizationState,
        tip:                   BlockTip,
        verification_progress: f64,
        header:                bool)  {
    
    todo!();
        /*
            if (fHeader) {
            // cache best headers time and height to reduce future cs_main locks
            clientmodel->cachedBestHeaderHeight = tip.block_height;
            clientmodel->cachedBestHeaderTime = tip.block_time;
        } else {
            clientmodel->m_cached_num_blocks = tip.block_height;
            
    [&]() { LOCK(clientmodel->m_cached_tip_mutex);  clientmodel->m_cached_tip_blocks = tip.block_hash; }()
    ;
        }

        // Throttle GUI notifications about (a) blocks during initial sync, and (b) both blocks and headers during reindex.
        const bool throttle = (sync_state != SynchronizationState::POST_INIT && !fHeader) || sync_state == SynchronizationState::INIT_REINDEX;
        const int64_t now = throttle ? GetTimeMillis() : 0;
        int64_t& nLastUpdateNotification = fHeader ? nLastHeaderTipUpdateNotification : nLastBlockTipUpdateNotification;
        if (throttle && now < nLastUpdateNotification + MODEL_UPDATE_DELAY) {
            return;
        }

        bool invoked = QMetaObject::invokeMethod(clientmodel, "numBlocksChanged", QtQueuedConnection,
            Q_ARG(int, tip.block_height),
            Q_ARG(QDateTime, QDateTime::fromSecsSinceEpoch(tip.block_time)),
            Q_ARG(double, verificationProgress),
            Q_ARG(bool, fHeader),
            Q_ARG(SynchronizationState, sync_state));
        assert(invoked);
        nLastUpdateNotification = now;
        */
}
