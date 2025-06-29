// ---------------- [ File: bitcoin-version/src/clientversion.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/clientversion.h]

///-----------------
pub fn format_version(n_version: i32) -> String {
    
    format!(
        "{}.{}.{}",
        n_version / 10000,
        (n_version / 100) % 100,
        n_version % 100
    )

}

#[cfg(not(WINDRES_PREPROC))]
pub fn format_full_version() -> String {
    trace!("constructing full version string");

    #[cfg(BUILD_GIT_TAG)]
    {
        let full = format!("{}{}", BUILD_DESC, BUILD_SUFFIX);
        debug!(%full, "full version string (tagged build)");
        full
    }

    #[cfg(not(BUILD_GIT_TAG))]
    {
        let full = format!("{}{}", &*BUILD_DESC, &*BUILD_SUFFIX);
        debug!(%full, "full version string (untagged build)");
        full
    }
}

/**
  | Format the sub‑version field according to BIP 14
  | (https://github.com/bitcoin/bips/blob/master/bip-0014.mediawiki)
  */
#[cfg(not(WINDRES_PREPROC))]
pub fn format_sub_version(
    name:             &String,
    n_client_version: i32,
    comments:         &Vec<String>,
) -> String {
    trace!(
        client = %name,
        version = n_client_version,
        comments = ?comments,
        "format_sub_version invoked"
    );

    let mut out = String::with_capacity(32);
    out.push('/');
    out.push_str(name);
    out.push(':');
    out.push_str(&format_version(n_client_version));

    if !comments.is_empty() {
        out.push('(');
        let mut first = true;
        for c in comments {
            if !first {
                out.push_str("; ");
            }
            out.push_str(c);
            first = false;
        }
        out.push(')');
    }

    out.push('/');
    debug!(formatted = %out, "sub‑version formatted");
    out
}

#[cfg(test)]
mod clientversion_tests {
    use super::*;

    #[traced_test]
    fn format_version_matches_expected() {
        let expected = "22.99.0";
        let actual   = format_version(super::CLIENT_VERSION);
        info!("format_version produced {}", actual);
        assert_eq!(actual, expected);
    }

    #[traced_test]
    fn full_version_non_empty() {
        let full = format_full_version();
        info!("format_full_version produced {}", full);
        assert!(!full.is_empty(), "full version string must not be empty");
    }

    #[traced_test]
    fn sub_version_round_trip() {
        let name      = String::from(super::CLIENT_NAME);
        let comments  = vec![String::from("alpha"), String::from("beta")];

        let expected  = format!(
            "/{}:{}({}; {})/",
            name,
            format_version(super::CLIENT_VERSION),
            comments[0],
            comments[1]
        );

        let actual = format_sub_version(&name, super::CLIENT_VERSION, &comments);
        info!("format_sub_version produced {}", actual);
        assert_eq!(actual, expected);
    }

    #[traced_test]
    fn sub_version_without_comments() {
        let name      = String::from(super::CLIENT_NAME);
        let comments  = Vec::new();

        let expected  = format!(
            "/{}:{}/",
            name,
            format_version(super::CLIENT_VERSION)
        );

        let actual = format_sub_version(&name, super::CLIENT_VERSION, &comments);
        info!("format_sub_version (no comments) produced {}", actual);
        assert_eq!(actual, expected);
    }
}
