crate::ix!();

/// Delete RPC authentication cookie from disk
///
/// Any I/O error is logged but otherwise ignored (parity with upstream).
///
pub fn delete_auth_cookie() {
    let filepath = get_auth_cookie_file(None);
    if let Err(e) = fs::remove_file(&filepath) {
        // Only log failures that are not “file doesn’t exist”.
        if e.kind() != std::io::ErrorKind::NotFound {
            error!(
                %e,
                "Unable to remove auth‑cookie file {:?} in {}", 
                filepath,
                "delete_auth_cookie"
            );
        }
    } else {
        trace!("Deleted auth‑cookie file {:?}", filepath);
    }
}
