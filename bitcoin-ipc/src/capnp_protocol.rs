// ---------------- [ File: bitcoin-ipc/src/capnp_protocol.rs ]
crate::ix!();

use crate::protocol::Protocol;

//-------------------------------------------[.cpp/bitcoin/src/ipc/capnp/protocol.h]
//-------------------------------------------[.cpp/bitcoin/src/ipc/capnp/protocol.cpp]

/**
  | Cap'n Proto context struct. Generally the
  | parent ipc::Context struct should be used
  | instead of this struct to give all IPC
  | protocols access to application state, so
  | there aren't unnecessary differences between
  | IPC protocols. But this specialized struct can
  | be used to pass capnp-specific function and
  | object types to capnp hooks.
  */
pub struct CapnpContext {

}

pub fn capnp_ipc_log_fn(
        raise:   bool,
        message: String)  {
    
    todo!();
        /*
        LogPrint(LogFlags::IPC, "%s\n", message);
        if (raise) throw Exception(message);
        */
}

pub type NotLinked                = i32; //TODO
pub type MultiProcessingEventLoop = NotLinked;
pub type TypeIndex                = std::any::TypeId;

pub struct CapnpProtocol {
    context:     CapnpContext,
    loop_thread: Thread,
    loop_:       Option<MultiProcessingEventLoop>,
}

impl Drop for CapnpProtocol {

    fn drop(&mut self) {
        todo!();
        /*
            if (m_loop) {
                std::unique_lock<std::mutex> lock(m_loop->m_mutex);
                m_loop->removeClient(lock);
            }
            if (m_loop_thread.joinable()) m_loop_thread.join();
            assert(!m_loop);
        }{
        */
    }
}

impl Protocol for CapnpProtocol {}

impl Connect for CapnpProtocol {

    fn connect(&mut self, 
        fd:       i32,
        exe_name: *const u8) -> Box<dyn Init> {
        
        todo!();
        /*
            startLoop(exe_name);
            return MultiProcessingConnectStream<messages::Init>(*m_loop, fd);
        */
    }
}

impl Serve for CapnpProtocol {

    fn serve(&mut self, 
        fd:       i32,
        exe_name: *const u8,
        init:     Rc<RefCell<dyn Init>>)  {
        
        todo!();
        /*
            assert(!m_loop);
            mp::g_thread_context.thread_name = MultiProcessingThreadName(exe_name);
            m_loop.emplace(exe_name, &IpcLogFn, &m_context);
            MultiProcessingServeStream<messages::Init>(*m_loop, fd, init);
            m_loop->loop();
            m_loop.reset();
        */
    }
}

impl AddCleanup for CapnpProtocol {

    fn add_cleanup(&mut self, 
        ty:      TypeIndex,
        iface:   *mut c_void,
        cleanup: fn() -> ())  {
        
        todo!();
        /*
            MultiProcessingProxyTypeRegister::types().at(type)(iface).cleanup.emplace_back(std::move(cleanup));
        */
    }
}
    
impl Context for CapnpProtocol {

    fn context(&mut self) -> Rc<RefCell<IpcContext>> {
        
        todo!();
        /*
            return m_context;
        */
    }
}
    
impl CapnpProtocol {
    pub fn start_loop(&mut self, exe_name: *const u8)  {
        
        todo!();
        /*
            if (m_loop) return;
            std::promise<c_void> promise;
            m_loop_thread = std::thread([&] {
                util::ThreadRename("capnp-loop");
                m_loop.emplace(exe_name, &IpcLogFn, &m_context);
                {
                    std::unique_lock<std::mutex> lock(m_loop->m_mutex);
                    m_loop->addClient(lock);
                }
                promise.set_value();
                m_loop->loop();
                m_loop.reset();
            });
            promise.get_future().wait();
        */
    }
}

pub fn make_capnp_protocol() -> Box<dyn Protocol> {
    
    todo!();
        /*
            return std::make_unique<CapnpProtocol>();
        */
}
