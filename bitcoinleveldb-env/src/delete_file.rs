// ---------------- [ File: bitcoinleveldb-env/src/delete_file.rs ]
crate::ix!();

/**
  | The leveldb::Env class below contains
  | a DeleteFile method.  At the same time,
  | <windows.h>, a fairly popular header file for
  | Windows applications, defines a DeleteFile
  | macro.
  |
  | Without any intervention on our part, the
  | result of this unfortunate coincidence is that
  | the name of the leveldb::Env::DeleteFile method
  | seen by the compiler depends on whether
  | <windows.h> was included before or after the
  | LevelDB headers.
  |
  | To avoid headaches, we undefined DeleteFile (if
  | defined) and redefine it at the bottom of this
  | file. This way <windows.h> can be included
  | before this file (or not at all) and the
  | exported method will always be
  | leveldb::Env::DeleteFile.
  */
#[cfg(_WIN32)]
#[cfg(DeleteFile)]
pub const DeleteFile: bool = false;
pub const LEVELDB_DELETEFILE_UNDEFINED: bool = true;

/**
  | Redefine DeleteFile if necessary.
  |
  */
#[cfg(all(_WIN32,LEVELDB_DELETEFILE_UNDEFINED))]
lazy_static!{
    /*
    #if UNICODE
    #define DeleteFile DeleteFileW
    #else
    #define DeleteFile DeleteFileA
    #endif  // defined(UNICODE)
    */
}
