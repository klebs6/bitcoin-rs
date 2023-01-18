crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/initexecutor.h]
//-------------------------------------------[.cpp/bitcoin/src/qt/initexecutor.cpp]

/**
  | Class encapsulating Bitcoin Core startup
  | and shutdown.
  | 
  | Allows running startup and shutdown
  | in a different thread from the UI thread.
  |
  */
#[Q_OBJECT]
pub struct InitExecutor {
    base:    QObject,
    node:    Rc<RefCell<dyn NodeInterface>>,
    context: QObject,
    thread:  QThread,
}

impl Drop for InitExecutor {
    fn drop(&mut self) {
        todo!();
        /*
            qDebug() << __func__ << ": Stopping thread";
        m_thread.quit();
        m_thread.wait();
        qDebug() << __func__ << ": Stopped thread";
        */
    }
}

impl InitExecutor {
    
    #[Q_SIGNAL]
    pub fn initialize_result(&mut self, 
        success:  bool,
        tip_info: BlockAndHeaderTipInfo)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn shutdown_result(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn runaway_exception(&mut self, message: &String)  {
        
        todo!();
        /*
        
        */
    }

    pub fn new(node: Rc<RefCell<dyn NodeInterface>>) -> Self {
    
        todo!();
        /*
        : q_object(),
        : node(node),

            m_context.moveToThread(&m_thread);
        m_thread.start();
        */
    }
    
    /**
      | Pass fatal exception message to UI thread
      |
      */
    #[Q_SIGNAL]
    pub fn handle_runaway_exception(&mut self, e: *const Exception)  {
        
        todo!();
        /*
            PrintExceptionContinue(e, "Runaway exception");
        Q_EMIT runawayException(QString::fromStdString(m_node.getWarnings().translated));
        */
    }
    
    #[Q_SLOT]
    pub fn initialize(&mut self)  {
        
        todo!();
        /*
            typename gui_util::ObjectInvoke(&m_context, [this] {
            try {
                util::ThreadRename("qt-init");
                qDebug() << "Running initialization in thread";
                typename interfaces::BlockAndHeaderTipInfo tip_info;
                bool rv = m_node.appInitMain(&tip_info);
                Q_EMIT initializeResult(rv, tip_info);
            } catch (const std::exception& e) {
                handleRunawayException(&e);
            } catch (...) {
                handleRunawayException(nullptr);
            }
        });
        */
    }
    
    #[Q_SLOT]
    pub fn shutdown(&mut self)  {
        
        todo!();
        /*
            typename gui_util::ObjectInvoke(&m_context, [this] {
            try {
                qDebug() << "Running Shutdown in thread";
                m_node.appShutdown();
                qDebug() << "Shutdown finished";
                Q_EMIT shutdownResult();
            } catch (const std::exception& e) {
                handleRunawayException(&e);
            } catch (...) {
                handleRunawayException(nullptr);
            }
        });
        */
    }
}
