// ---------------- [ File: bitcoin-string/src/bilingual.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/translation.h]

/// A message that carries both the untranslated English text (`original`)
/// and the user‑language translation (`translated`).
///
/// * In the GUI we show **translated text** and fall back to `original` when
///   no translation is available.
/// * In the log and on `stderr` we show **English only**.
#[derive(Debug, Clone, Default, Getters, Setters)]
#[getset(get = "pub")]
pub struct BilingualStr {
    /// Untranslated, canonical English text.
    original: String,
    /// Localised translation or a copy of `original` when untranslated.
    translated: String,
}

impl AddAssign<&BilingualStr> for BilingualStr {
    #[inline]
    fn add_assign(&mut self, other: &BilingualStr) {
        trace!(
            "BilingualStr::add_assign: self = ({}, {}), other = ({}, {})",
            self.original,
            self.translated,
            other.original,
            other.translated
        );
        self.original += &other.original;
        self.translated += &other.translated;
    }
}

impl BilingualStr {
    /// Returns `true` when the *original* string is empty.
    #[inline]
    pub fn empty(&self) -> bool {
        self.original.is_empty()
    }

    /// Clear both the original and the translated strings.
    #[inline]
    pub fn clear(&mut self) {
        trace!("BilingualStr::clear before = {:?}", self);
        self.original.clear();
        self.translated.clear();
        debug!("BilingualStr::clear after  = {:?}", self);
    }
}

impl Add<&BilingualStr> for BilingualStr {
    type Output = BilingualStr;

    #[inline]
    fn add(mut self, other: &BilingualStr) -> Self::Output {
        self += other;
        self
    }
}

impl From<&str> for BilingualStr {
    #[inline]
    fn from(s: &str) -> Self {
        untranslated(s)
    }
}

/// Mark a [`BilingualStr`] as untranslated: `original == translated`.
#[inline]
pub fn untranslated(original: &str) -> BilingualStr {
    trace!("untranslated: {}", original);
    BilingualStr {
        original: original.to_owned(),
        translated: original.to_owned(),
    }
}

pub mod tinyformat {
    //! Very small subset that we actually use: basic forwarding to `format!`.

    use super::BilingualStr;

    /// Format with `format!`, once for each language half.
    ///
    /// ```ignore
    /// let hello = bilingual_format!("Hello, {}", name);
    /// ```
    #[macro_export]
    macro_rules! bilingual_format {
        ($tmpl:expr $(, $arg:expr)* $(,)?) => {{
            let bs: &BilingualStr = &$tmpl;
            BilingualStr {
                original: format!(bs.original.as_str() $(, $arg)*),
                translated: format!(bs.translated.as_str() $(, $arg)*),
            }
        }};
    }

    /// When direct macro use is infeasible, fall back to this helper.
    pub fn format(bs: &BilingualStr, args_original: String, args_translated: String) -> BilingualStr {
        // Note: buyers beware – the two `String` parameters must already contain
        // all replacements performed externally. This keeps implementation
        // simple while covering the limited in‑tree use‑cases.
        BilingualStr {
            original: args_original,
            translated: args_translated,
        }
    }
}

pub type TranslationFn = fn(&str) -> String;

lazy_static! {
    static ref G_TRANSLATION_FUN_DOES_NOT_ALIAS: RwLock<Option<TranslationFn>> = RwLock::new(None);
}

pub fn set_translation_fn(f: Option<TranslationFn>) {
    trace!("set_translation_fn: {:?}", f.is_some());
    let mut guard = G_TRANSLATION_FUN_DOES_NOT_ALIAS.write();
    *guard = f;
}

pub fn bilingual_tr(psz: &str) -> BilingualStr {
    let guard = G_TRANSLATION_FUN_DOES_NOT_ALIAS.read();

    let translated = guard
        .as_ref()
        .map(|f| f(psz))
        .unwrap_or_else(|| psz.to_owned());

    trace!("_(): \"{}\" → \"{}\"", psz, translated);

    BilingualStr {
        original: psz.to_owned(),
        translated,
    }
}

#[cfg(test)]
mod tests_bilingual {
    use super::*;

    #[traced_test]
    fn untranslated_roundtrip() {
        let msg = BilingualStr::from("abc");
        assert_eq!(msg.original(), "abc");
        assert_eq!(msg.translated(), "abc");
        assert!(msg.empty() == false);
    }

    #[traced_test]
    fn clear_empties() {
        let mut msg = BilingualStr::from("x");
        msg.clear();
        assert!(msg.empty());
        assert_eq!(msg.translated(), "");
    }

    #[traced_test]
    fn add_and_add_assign() {
        let mut a = BilingualStr::from("foo");
        let b = BilingualStr::from("bar");
        a += &b;
        assert_eq!(a.original(), "foobar");
        assert_eq!(a.translated(), "foobar");

        let c = BilingualStr::from("baz");
        let d = a + &c;
        assert_eq!(d.original(), "foobarbaz");
    }

    #[traced_test]
    fn global_translation_function() {
        // Arrange
        set_translation_fn(Some(|s| format!("ES: {}", s)));
        let msg = bilingual_tr("hello");

        // Assert translated
        assert_eq!(msg.original(), "hello");
        assert_eq!(msg.translated(), "ES: hello");

        // Clean‑up for other tests
        set_translation_fn(None);
    }
}
