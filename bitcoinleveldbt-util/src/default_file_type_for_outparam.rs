// ---------------- [ File: bitcoinleveldbt-util/src/default_file_type_for_outparam.rs ]
crate::ix!();

/// Invariant: this placeholder exists only to preserve out-parameter initialization shape for
/// filename parsing; the parsed result overwrites it before observation on the success path.
pub fn dbtest_default_file_type_for_outparam() -> FileType {
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::dbtest",
        label = "dbtest.file_type.default_outparam"
    );

    FileType::LogFile
}
