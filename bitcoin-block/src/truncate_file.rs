// ---------------- [ File: bitcoin-block/src/truncate_file.rs ]
crate::ix!();

#[cfg(WIN32)]
pub fn truncate_file(
        file:   *mut libc::FILE,
        length: u32) -> bool {
    
    chsize(fileno(file),length) == 0
}

#[cfg(not(WIN32))]
pub fn truncate_file(
        file:   *mut libc::FILE,
        length: u32) -> bool {
    
    unsafe {
        ftruncate(libc::fileno(file),length.into()) == 0
    }
}
