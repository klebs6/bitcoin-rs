// ---------------- [ File: bitcoin-remote/src/get_auth_cookie.rs ]
crate::ix!();

/// Read the cookie from disk into `cookie_out`.  Returns
/// **false** if the file is not present or not readable.
pub fn get_auth_cookie(cookie_out: &mut String) -> bool {
    let filepath = get_auth_cookie_file(None);

    let file = match fs::File::open(&filepath) {
        Ok(f) => f,
        Err(e) => {
            debug!(
                %e,
                "Failed to open auth‑cookie file {:?} for reading",
                filepath
            );
            return false;
        }
    };

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    if let Err(e) = reader.read_line(&mut line) {
        warn!(%e, "Failed to read cookie from {:?}", filepath);
        return false;
    }

    // Trim trailing newline if present (parity with getline)
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }

    *cookie_out = line;
    true
}
