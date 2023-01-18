crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/ipc/context.h]

/**
  | Context struct used to give IPC protocol
  | implementations or implementation hooks access
  | to application state, in case they need to run
  | extra code that isn't needed within a single
  | process, like code copying global state from
  | an existing process to a new process when it's
  | initialized, or code dealing with shared
  | objects that are created or destroyed
  | remotely.
  */
pub struct IpcContext {

}

//-------------------------------------------[.cpp/bitcoin/src/ipc/exception.h]

/**
  | Exception class thrown when a call to remote
  | method fails due to an IPC error, like a socket
  | getting disconnected.
  */
pub struct Exception { }

//-------------------------------------------[.cpp/bitcoin/src/ipc/capnp/context.h]
