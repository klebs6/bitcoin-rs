// ---------------- [ File: bitcoin-settings/src/settings_span.rs ]
crate::ix!();

/**
  | Accessor for list of settings that skips
  | negated values when iterated over.
  |
  | The last boolean `false` value in the list and
  | all earlier values are considered negated.
  */
#[derive(Builder,Getters,MutGetters)]
#[getset(get="pub")]
#[builder(setter(into))]
pub struct SettingsSpan {
    data: *const SettingsValue,
    size: usize,
}

impl Default for SettingsSpan {
    fn default() -> Self {
        Self {
            data: null(),
            size: 0,
        }
    }
}

impl From<&SettingsValue> for SettingsSpan {
    fn from(value: &SettingsValue) -> Self {
        Self {
            data: value as *const SettingsValue,
            size: 1,
        }
    }
}

impl From<&Vec<SettingsValue>> for SettingsSpan {
    fn from(vec: &Vec<SettingsValue>) -> Self {
        Self {
            data: vec.as_ptr(),
            size: vec.len(),
        }
    }
}

impl SettingsSpan {
    /// Construct a new span from a raw pointer + length.
    #[inline]
    pub fn new(data: *const SettingsValue, size: usize) -> Self {
        Self { data, size }
    }

    /// Pointer to the first **non‑negated** value.
    #[inline]
    pub fn begin(&self) -> *const SettingsValue {
        let neg = self.negated();
        trace!("SettingsSpan::begin – negated_count={neg}");
        // SAFETY: Caller guarantees `data` points to an allocation of `size` elements.
        unsafe { self.data.add(neg) }
    }

    /// Pointer one‑past‑the‑last value (standard Rust/C++ end iterator).
    #[inline]
    pub fn end(&self) -> *const SettingsValue {
        // SAFETY: same as above.
        unsafe { self.data.add(self.size) }
    }

    /// `true` when there are no **effective** values (all are negated or span is empty).
    #[inline]
    pub fn empty(&self) -> bool {
        let result = self.size == 0 || self.last_negated();
        debug!(
            "SettingsSpan::empty – size={}, last_negated={}, result={}",
            self.size,
            self.last_negated(),
            result
        );
        result
    }

    /// `true` when the last (highest‑priority) value is an explicit boolean `false`.
    #[inline]
    pub fn last_negated(&self) -> bool {
        if self.size == 0 {
            return false;
        }
        // SAFETY: `size > 0` ⇒ `size - 1` is in‑bounds.
        let is_neg = unsafe { (*self.data.add(self.size - 1)).is_false() };
        trace!("SettingsSpan::last_negated – is_negated={is_neg}");
        is_neg
    }

    /// Number of **consecutive negated** values at the end of the span.
    ///
    /// Mirrors the original C++ logic exactly.
    #[inline]
    pub fn negated(&self) -> usize {
        for i in (1..=self.size).rev() {
            // SAFETY: loop invariant keeps `i‑1` in‑bounds.
            if unsafe { (*self.data.add(i - 1)).is_false() } {
                trace!("SettingsSpan::negated – found negated at index {}", i - 1);
                return i;
            }
        }
        0
    }
}

#[cfg(test)]
mod settings_span_spec {
    use super::*;

    #[traced_test]
    fn negated_detection_and_iteration() {

        // Case 1: No negation.
        let v1 = vec![SettingsValue::from(true), SettingsValue::from(true)];
        let span1 = SettingsSpan::from(&v1);
        assert_eq!(span1.negated(), 0);
        assert!(!span1.empty());
        assert!(!span1.last_negated());
        unsafe {
            assert_eq!(
                span1.begin().offset_from(span1.data),
                0,
                "begin should point to first element"
            );
        }

        // Case 2: Last value negated => entire list negated.
        let v2 = vec![SettingsValue::from(true), SettingsValue::from(false)];
        let span2 = SettingsSpan::from(&v2);
        assert_eq!(span2.negated(), 2);
        assert!(span2.empty());
        assert!(span2.last_negated());
        unsafe {
            assert_eq!(
                span2.begin().offset_from(span2.data),
                2,
                "begin should point to end when all values are negated"
            );
            assert_eq!(
                span2.end().offset_from(span2.data),
                2,
                "end should be two elements ahead (len)"
            );
        }
    }
}

#[cfg(test)]
mod settings_span_more_edges_spec {
    use super::*;

    #[traced_test]
    fn empty_span_reports_empty_and_has_valid_iterators() {
        info!("Empty span should be empty with begin==end");
        let span = SettingsSpan::default();
        assert!(span.empty());
        unsafe {
            assert_eq!(span.begin(), span.end());
        }
    }

    #[traced_test]
    fn multiple_trailing_negations_consume_entire_span() {
        info!("Multiple trailing negations should render the span entirely ineffective");
        let v = vec![
            SettingsValue::from(true),
            SettingsValue::from(false),
            SettingsValue::from(false),
        ];
        let span = SettingsSpan::from(&v);
        debug!("negated count = {}", span.negated());
        assert!(span.empty());
        assert!(span.last_negated());
        unsafe {
            assert_eq!(span.begin().offset_from(span.end()), 0);
        }
    }

    #[traced_test]
    fn interior_negation_masks_all_earlier_values_per_spec() {
        info!("Per spec: the last `false` and all earlier values are considered negated, even if followed by non-negated values");
        let v = vec![
            SettingsValue::from(true),   // idx 0
            SettingsValue::from(false),  // idx 1  ← last negation
            SettingsValue::from(true),   // idx 2  ← effective (only one)
        ];
        let span = SettingsSpan::from(&v);

        // With the semantics "last false masks all earlier", begin must skip the first two.
        assert_eq!(span.negated(), 2, "negated should equal index_of_last_false + 1");
        assert!(!span.empty());
        assert!(!span.last_negated());

        unsafe {
            assert_eq!(
                span.begin().offset_from(*span.data()),
                2,
                "begin should point past the last negation"
            );
            assert_eq!(
                span.end().offset_from(*span.data()),
                3,
                "end should be at len==3"
            );
        }
    }
}
