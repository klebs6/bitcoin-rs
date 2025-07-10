// ---------------- [ File: bitcoin-remote/src/generate_auth_cookie.rs ]
crate::ix!();

/// Generate a brand‑new cookie and *atomically* write it to disk.  
///
/// Returns **true** on success and populates `cookie_out` with the newly‑minted value.
///
pub fn generate_auth_cookie(cookie_out: &mut String) -> bool {
    const COOKIE_SIZE: usize = 32;

    // 1. Create random password
    let mut rand_pwd = [0u8; COOKIE_SIZE];
    get_rand_bytes(&mut rand_pwd.as_slice(), COOKIE_SIZE);

    let cookie: String = format!(
        "{}:{}",
        COOKIEAUTH_USER,
        rand_pwd.encode_hex::<String>()
    );

    // 2. Write to temporary file first
    let filepath_tmp = get_auth_cookie_file(Some(true));
    if let Err(e) = {
        if let Some(parent) = filepath_tmp.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                error!(%e, "Unable to create parent directory for cookie");
                return false;
            }
        }
        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            // rely on process umask as per Core comment
            .open(&filepath_tmp)
        {
            Ok(f) => f,
            Err(e) => {
                error!(
                    %e,
                    "Unable to open cookie authentication file {:?} for writing",
                    filepath_tmp
                );
                return false;
            }
        };
        file.write_all(cookie.as_bytes())
            .and_then(|_| file.flush())
    } {
        error!(
            %e,
            "Failed to write temporary cookie file {:?}",
            filepath_tmp
        );
        // best‑effort clean‑up
        let _ = fs::remove_file(&filepath_tmp);
        return false;
    }

    // 3. Atomically rename to final destination
    let filepath_final = get_auth_cookie_file(Some(false));
    if let Err(e) = fs::rename(&filepath_tmp, &filepath_final) {
        error!(
            %e,
            "Unable to rename cookie authentication file {:?} to {:?}",
            filepath_tmp, filepath_final
        );
        let _ = fs::remove_file(&filepath_tmp);
        return false;
    }

    info!(
        "Generated RPC authentication cookie at {:?}",
        filepath_final
    );

    *cookie_out = cookie;
    true
}
