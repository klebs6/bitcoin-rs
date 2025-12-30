// ---------------- [ File: bitcoinleveldb-repair/src/test_harness.rs ]
crate::ix!();

#[cfg(test)]
pub(crate) mod repairer_test_harness {
    use super::*;
    use std::fs;
    use std::io::{Read, Write};
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};
    use tracing::{debug, error, info, trace, warn};

    static TEST_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

    pub(crate) struct EphemeralDbDir {
        path: PathBuf,
    }

    impl EphemeralDbDir {
        pub(crate) fn new(prefix: &str) -> Self {
            let pid = std::process::id() as u64;
            let ctr = TEST_DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_else(|_| std::time::Duration::from_secs(0));
            let nanos = now.as_nanos();

            let mut path = std::env::temp_dir();
            path.push(format!(
                "bitcoinleveldb-repair-{}-pid{}-ctr{}-ns{}",
                prefix, pid, ctr, nanos
            ));

            trace!(dir = %path.to_string_lossy(), "EphemeralDbDir::new: creating temp dir");
            fs::create_dir_all(&path).unwrap_or_else(|e| {
                panic!(
                    "failed to create temp db dir {}: {}",
                    path.to_string_lossy(),
                    e
                )
            });

            Self { path }
        }

        pub(crate) fn path(&self) -> &Path {
            &self.path
        }

        pub(crate) fn path_string(&self) -> String {
            self.path.to_string_lossy().into_owned()
        }

        pub(crate) fn lost_dir_string(&self) -> String {
            let mut s = self.path_string();
            s.push_str("/lost");
            s
        }
    }

    impl Drop for EphemeralDbDir {
        fn drop(&mut self) {
            let dir = self.path.to_string_lossy().into_owned();
            trace!(dir = %dir, "EphemeralDbDir::drop: removing temp dir");
            match fs::remove_dir_all(&self.path) {
                Ok(_) => {
                    debug!(dir = %dir, "EphemeralDbDir::drop: removed temp dir");
                }
                Err(e) => {
                    warn!(dir = %dir, err = %e, "EphemeralDbDir::drop: failed to remove temp dir (ignored)");
                }
            }
        }
    }

    pub(crate) fn touch_file(path: &str) {
        let p = Path::new(path);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).unwrap_or_else(|e| {
                panic!(
                    "touch_file: failed to create parent dir {} for {}: {}",
                    parent.to_string_lossy(),
                    path,
                    e
                )
            });
        }

        trace!(path = %path, "touch_file: creating/truncating");
        let mut f = fs::File::create(p).unwrap_or_else(|e| {
            panic!(
                "touch_file: failed to create/truncate file {}: {}",
                path, e
            )
        });
        f.flush().unwrap_or_else(|e| {
            panic!(
                "touch_file: failed to flush file {}: {}",
                path, e
            )
        });
    }

    pub(crate) fn write_text_file(path: &str, text: &str) {
        let p = Path::new(path);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).unwrap_or_else(|e| {
                panic!(
                    "write_text_file: failed to create parent dir {} for {}: {}",
                    parent.to_string_lossy(),
                    path,
                    e
                )
            });
        }

        trace!(path = %path, bytes = text.as_bytes().len(), "write_text_file: writing");
        let mut f = fs::File::create(p).unwrap_or_else(|e| {
            panic!(
                "write_text_file: failed to create/truncate file {}: {}",
                path, e
            )
        });
        f.write_all(text.as_bytes()).unwrap_or_else(|e| {
            panic!("write_text_file: failed to write {}: {}", path, e)
        });
        f.flush().unwrap_or_else(|e| {
            panic!("write_text_file: failed to flush {}: {}", path, e)
        });
    }

    pub(crate) fn read_text_file(path: &str) -> String {
        trace!(path = %path, "read_text_file: reading");
        fs::read_to_string(Path::new(path)).unwrap_or_else(|e| {
            panic!("read_text_file: failed to read {}: {}", path, e)
        })
    }

    pub(crate) fn read_bytes_file(path: &str) -> Vec<u8> {
        trace!(path = %path, "read_bytes_file: reading");
        fs::read(Path::new(path)).unwrap_or_else(|e| {
            panic!("read_bytes_file: failed to read {}: {}", path, e)
        })
    }

    pub(crate) fn path_exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    pub(crate) fn is_directory(path: &str) -> bool {
        Path::new(path).is_dir()
    }

    pub(crate) fn directory_basenames(dir: &str) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let rd = match fs::read_dir(dir) {
            Ok(rd) => rd,
            Err(e) => {
                warn!(dir = %dir, err = %e, "directory_basenames: read_dir failed");
                return out;
            }
        };

        for ent in rd.flatten() {
            if let Some(name) = ent.file_name().to_str() {
                out.push(name.to_owned());
            }
        }

        out.sort();
        out
    }

    pub(crate) fn expected_archive_destination(fname: &str) -> String {
        let slash_pos = fname.rfind('/');

        let mut new_dir = String::new();
        if let Some(pos) = slash_pos {
            new_dir.push_str(&fname[..pos]);
        }
        new_dir.push_str("/lost");

        let base = match slash_pos {
            Some(pos) => &fname[(pos + 1)..],
            None => fname,
        };

        let mut new_file = new_dir;
        new_file.push('/');
        new_file.push_str(base);
        new_file
    }

    pub(crate) fn expected_archive_dir(fname: &str) -> String {
        let slash_pos = fname.rfind('/');

        let mut new_dir = String::new();
        if let Some(pos) = slash_pos {
            new_dir.push_str(&fname[..pos]);
        }
        new_dir.push_str("/lost");
        new_dir
    }

    pub(crate) fn assert_archived(src: &str) -> String {
        let dst = expected_archive_destination(src);
        info!(src = %src, dst = %dst, "assert_archived: verifying");
        assert!(
            !path_exists(src),
            "expected source to be archived (missing): src={}",
            src
        );
        assert!(
            path_exists(&dst),
            "expected archived file to exist: dst={}",
            dst
        );
        dst
    }

    pub(crate) fn assert_not_archived(src: &str) -> String {
        let dst = expected_archive_destination(src);
        info!(src = %src, dst = %dst, "assert_not_archived: verifying");
        assert!(
            path_exists(src),
            "expected source to still exist: src={}",
            src
        );
        assert!(
            !path_exists(&dst),
            "expected archived destination to not exist: dst={}",
            dst
        );
        dst
    }

    pub(crate) fn read_current_file_guess(dbname: &str) -> Option<String> {
        let current = format!("{}/CURRENT", dbname);
        if !path_exists(&current) {
            return None;
        }
        let mut s = String::new();
        let mut f = fs::File::open(&current).ok()?;
        f.read_to_string(&mut s).ok()?;
        Some(s)
    }
}
