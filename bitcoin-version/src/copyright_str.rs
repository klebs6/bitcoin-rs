// ---------------- [ File: bitcoin-version/src/copyright_str.rs ]
crate::ix!();

#[macro_export]
macro_rules! copyright_str {
    () => {
        concat!("2009-", env!("COPYRIGHT_YEAR"), " ", env!("COPYRIGHT_HOLDERS_FINAL"))
    };
}

#[cfg(test)]
mod copyright_str_tests {
    use super::*;

    #[traced_test]
    fn macro_generates_correct_copyright() {
        std::env::set_var("COPYRIGHT_YEAR", "2025");
        std::env::set_var("COPYRIGHT_HOLDERS_FINAL", "Bitcoin Developers");

        let expected = "2009-2025 Bitcoin Developers";
        let actual = copyright_str!();

        info!("macro output: {}", actual);
        assert_eq!(actual, expected);
    }
}
