// ---------------- [ File: bitcoin-version/src/computed_constants.rs ]
crate::ix!();

/*
  | bitcoind-res.rc includes this file,
  | but it cannot cope with real c++ code.
  | 
  | WINDRES_PREPROC is defined to indicate
  | that its pre-processor is running.
  | Anything other than a define should
  | be guarded below.
  |
  */
pub const CLIENT_VERSION_MAJOR: i32 = 22;
pub const CLIENT_VERSION_MINOR: i32 = 99;
pub const CLIENT_VERSION_BUILD: i32 = 0;

#[cfg(not(WINDRES_PREPROC))]
pub const CLIENT_VERSION: i32 = 
10000 * CLIENT_VERSION_MAJOR
+ 100 * CLIENT_VERSION_MINOR
+   1 * CLIENT_VERSION_BUILD;

/**
  | Name of client reported in the 'version'
  | message. Report the same name for both
  | bitcoind and bitcoin-qt, to make it
  | harder for attackers to target servers
  | or GUI users specifically.
  |
  */
#[cfg(not(WINDRES_PREPROC))]
pub const CLIENT_NAME: &'static str = "Satoshi";

pub const PACKAGE_VERSION: &'static str = "22.99.0";

//-------------------------------------------[.cpp/bitcoin/src/clientversion.cpp]

// Tagged build has highest priority
#[cfg(BUILD_GIT_TAG)]
pub const BUILD_DESC: &'static str = BUILD_GIT_TAG;

#[cfg(BUILD_GIT_TAG)]
pub const BUILD_SUFFIX: &'static str = "";

// Untagged builds
#[cfg(not(BUILD_GIT_TAG))]
lazy_static! {
    pub static ref BUILD_DESC: String = format!("v{}", PACKAGE_VERSION);
}

// Untagged but official release: suffix empty
#[cfg(all(not(BUILD_GIT_TAG), CLIENT_VERSION_IS_RELEASE))]
pub const BUILD_SUFFIX: &'static str = "";

// Untagged dev build with commit hash
#[cfg(all(not(BUILD_GIT_TAG), not(CLIENT_VERSION_IS_RELEASE), BUILD_GIT_COMMIT))]
lazy_static! {
    pub static ref BUILD_SUFFIX: String = format!("-{}", BUILD_GIT_COMMIT);
}

// Untagged dev build with commit ID (git archive)
#[cfg(all(not(BUILD_GIT_TAG), not(CLIENT_VERSION_IS_RELEASE), not(BUILD_GIT_COMMIT), GIT_COMMIT_ID))]
lazy_static! {
    pub static ref BUILD_SUFFIX: String = format!("-g{}", GIT_COMMIT_ID);
}

// Untagged, no release, no commit info available (fallback)
#[cfg(all(
    not(BUILD_GIT_TAG),
    not(CLIENT_VERSION_IS_RELEASE),
    not(BUILD_GIT_COMMIT),
    not(GIT_COMMIT_ID)
))]
pub const BUILD_SUFFIX: &'static str = "-unk";

#[cfg(test)]
mod cfg_gate_tests {
    use super::*;

    #[traced_test]
    #[cfg(BUILD_GIT_TAG)]
    fn build_tag_defined() {
        assert_eq!(BUILD_DESC, BUILD_GIT_TAG);
        assert_eq!(BUILD_SUFFIX, "");
    }

    #[traced_test]
    #[cfg(all(not(BUILD_GIT_TAG), CLIENT_VERSION_IS_RELEASE))]
    fn untagged_release_build() {
        assert_eq!(&*BUILD_DESC, &format!("v{}", PACKAGE_VERSION));
        assert_eq!(BUILD_SUFFIX, "");
    }

    #[traced_test]
    #[cfg(all(not(BUILD_GIT_TAG), not(CLIENT_VERSION_IS_RELEASE), BUILD_GIT_COMMIT))]
    fn untagged_dev_build_commit() {
        assert_eq!(&*BUILD_SUFFIX, &format!("-{}", BUILD_GIT_COMMIT));
    }

    #[traced_test]
    #[cfg(all(not(BUILD_GIT_TAG), not(CLIENT_VERSION_IS_RELEASE), not(BUILD_GIT_COMMIT), GIT_COMMIT_ID))]
    fn untagged_dev_build_git_commit_id() {
        assert_eq!(&*BUILD_SUFFIX, &format!("-g{}", GIT_COMMIT_ID));
    }

    #[traced_test]
    #[cfg(all(
        not(BUILD_GIT_TAG),
        not(CLIENT_VERSION_IS_RELEASE),
        not(BUILD_GIT_COMMIT),
        not(GIT_COMMIT_ID)
    ))]
    fn fallback_unknown_suffix() {
        assert_eq!(BUILD_SUFFIX, "-unk");
    }
}
