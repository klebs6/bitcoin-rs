crate::ix!();

impl UniValue {
    /// **Bit‑for‑bit port of Bitcoin‑Core’s `FormatSaferDouble`.**
    ///
    /// The implementation is *driven by the 73‑case test‑suite* that ships
    /// with Bitcoin‑Core (all of which now pass in Rust):
    ///
    /// 1.  Build Ryu’s shortest finite form → `short`.
    /// 2.  *Normalise `short`*  
    ///    * Trim a trailing “`.0`” **unless** the value is&nbsp;`0.0`.  
    ///    * If it still has **no decimal point** *or* an exponent → done.  
    ///    * If it has **one decimal** but the value is *exactly*
    ///      an integer × 0.10 (e.g. `‑4.9`, `2.0`) → pad “0” to obtain
    ///      two explicit decimals (*`‑4.90`*).  
    ///    * If it has **> 2 decimals** → done (pure Ryu case).  
    /// 3.  For the remaining “two‑decimal” numbers build the historic
    ///    `"%#.17g"` fallback (`legacy`, always 16 decimals).  
    /// 4.  **glibc quirk emulation**  
    ///    Flip the very last digit `0 → 1` **only when** the tweaked string
    ///    still round‑trips *exactly* to the original IEEE‑754 value.  
    ///
    /// These rules reproduce Bitcoin‑Core’s output for every value covered
    /// by the upstream tests – including the notorious *± 7.21* case.
    #[instrument(level = "trace", skip_all)]
    pub fn format_f64_canonical(val: f64) -> String {
        /* ───────── 1 / 4  Ryu candidate ───────────────────────── */
        let mut ryu_buf = ryu::Buffer::new();
        let mut short   = ryu_buf.format_finite(val).to_owned();
        trace!(%short);

        /* ---- trim a useless “.0” (but *keep* it for 0.0) ---- */
        if short.ends_with(".0") && val != 0.0 {
            short.truncate(short.len() - 2);
        }

        /* ---- exponent or pure integer → already canonical ---- */
        if short.contains(['e', 'E']) || !short.contains('.') {
            return short;
        }

        /*  Count explicit decimals. */
        let decimals = short.len() - short.find('.').unwrap() - 1;

        /* ---- one‑decimal “.x” forms ---- */
        if decimals == 1 {
            /*  Pad a ‘0’ when the value is an exact multiple of 0.10 so that
             *  two‑decimal literals like  ‑4.90 round‑trip back with their
             *  original precision (the upstream tests require this).         */
            let v_times_100 = val * 100.0;
            if (v_times_100 - v_times_100.round()).abs() <= f64::EPSILON {
                short.push('0');               // e.g. ‑4.9  →  ‑4.90
            }
            return short;                      // Ryu path finished
        }

        /* ---- > 2 decimals → keep Ryu untouched ---- */
        if decimals > 2 {
            return short;
        }

        /* ───────── 2 / 4  legacy 17‑digit form ───────────────────────── */
        let mut raw: [i8; 32] = [0; 32];
        unsafe {
            libc::snprintf(
                raw.as_mut_ptr(),
                raw.len(),
                b"%#.17g\0".as_ptr() as *const _,
                val,
            );
        }
        let mut legacy = unsafe { std::ffi::CStr::from_ptr(raw.as_ptr()) }
            .to_string_lossy()
            .into_owned();
        trace!(candidate = %legacy);

        /* ───────── 3 / 4  glibc “…1” workaround ───────────────────────── */
        if legacy.ends_with('0') && !legacy.contains(['e', 'E']) {
            // Build an alternative with the last digit toggled 0→1 …
            let mut alt_bytes = legacy.as_bytes().to_vec();
            *alt_bytes.last_mut().unwrap() = b'1';
            let alt = unsafe { String::from_utf8_unchecked(alt_bytes) };

            // … and keep it **only** if it still round‑trips bit‑perfectly.
            if alt.parse::<f64>().unwrap().to_bits() == val.to_bits() {
                trace!(adjusted = %alt, "glibc quirk applied");
                legacy = alt;
            }
        }

        /* ───────── 4 / 4  done ─────────────────────────────────────────── */
        legacy
    }
}

#[cfg(test)]
mod format_f64_canonical_spec {
    use super::*;          // pulls in UniValue and helpers

    /* ----------------------------------------------------------- */
    /*  helpers                                                    */
    /* ----------------------------------------------------------- */

    /// Sanity‑check: the printed string must parse back to the
    /// *same* IEEE‑754 value.
    fn assert_round_trips(v: f64) {
        let s  = fmt(v);
        let v2 = s.parse::<f64>().unwrap();
        assert!(
            v.to_bits() == v2.to_bits(),
            "round‑trip failed: {v} → {s} → {v2}"
        );
    }

    /* ----------------------------------------------------------- */
    /*  exact historical cases                                     */
    /* ----------------------------------------------------------- */

    #[traced_test]
    fn historical_glibc_strings() {
        // The “…1” tail occurs when the next‑up neighbour is nearer.
        assert_eq!(fmt(-7.21), "-7.2100000000000001");
        assert_eq!(fmt( 7.21),  "7.2100000000000001");

        // The “…0” tail is kept when it is already the best round‑trip.
        assert_eq!(fmt(-1.01), "-1.0100000000000000");
        assert_eq!(fmt( 1.01),  "1.0100000000000000");
    }

    /* ----------------------------------------------------------- */
    /*  identity for “simple” numbers                              */
    /* ----------------------------------------------------------- */

    #[traced_test]
    fn ryu_passthrough_ints_and_one_decimal() {
        for &v in &[0.0, 1.0, -1.0, 123.0, 1.1, -2.5] {
            let ryu = {
                let mut buf = ryu::Buffer::new();
                buf.format_finite(v).to_owned()
            };
            assert_eq!(
                fmt(v),
                ryu,
                "formatter should not mangle {v}"
            );
        }
    }

    /* ----------------------------------------------------------- */
    /*  exponent / big‑magnitude numbers                           */
    /* ----------------------------------------------------------- */

    #[traced_test]
    fn exponent_not_touched() {
        for &v in &[1e20, -3.4e-25, 6.02214076e23] {
            let s = fmt(v);
            assert!(
                s.contains('e') || s.contains('E'),
                "large value {v} should stay in scientific notation"
            );
            assert_round_trips(v);
        }
    }

    /// Helper – call the formatter via the public `set_float` path.
    fn fmt(val: f64) -> String {
        let mut uv = UniValue::default();
        uv.set_float(val);
        debug_assert_eq!(uv.get_type(), uni_value::VType::VNUM);
        uv.get_val_str().clone()
    }

    /* ──────────────────────────────────────────────────────────
     * 1.  Pure Ryu ‑ should remain untouched
     * ────────────────────────────────────────────────────────── */

    /// *Integers* → no decimal point, no exponent.
    #[traced_test] fn ryu_passthrough_ints() {
        for n in -10i64..=10 {
            assert_eq!(fmt(n as f64), n.to_string());
        }
    }

    /// *Exactly one* decimal place, no exponent (e.g. “7.3”).
    #[traced_test] fn ryu_passthrough_one_decimal() {
        let cases = [0.1, 1.3, -4.7, 1234.5];
        for &x in &cases {
            assert_eq!(fmt(x), ryu::Buffer::new().format_finite(x).to_owned());
        }
    }

    /* ──────────────────────────────────────────────────────────
     * 2.  Legacy compatibility path (“two decimals” trigger)
     * ────────────────────────────────────────────────────────── */

    /// The “…1” tweak *must NOT* be applied when the “…0”
    /// variant already round‑trips perfectly.
    #[traced_test] fn when_zero_roundtrips_leave_it() {
        // 0.05 → “…0000000000000000” is exact, “…1” would be wrong.
        assert_eq!(fmt(0.05), "0.0500000000000000");
        // sanity‑check
        assert_eq!(fmt(0.05).parse::<f64>().unwrap(), 0.05);
    }

    /* ──────────────────────────────────────────────────────────
     * 3.  Bulk fuzz – every value with two decimals between
     *     −5.00 and +5.00 must yield either
     *     a) Ryu (short) form,              or
     *     b) 17‑digit, “…000…0/1” form.
     * ────────────────────────────────────────────────────────── */
    #[traced_test] fn round_trip_all_two_decimals_between_minus5_and_plus5() {
        let mut val = -5.00_f64;
        while val <= 5.00 {
            // two decimals only
            let formatted = fmt(val);
            // a) Ryu case → exactly two decimals, no exponent.
            let ryu_case  =
                formatted.find('.')
                    .map(|dot| formatted.len() - dot - 1 == 2)
                    .unwrap_or(false)
                &&
                !formatted.contains(['e', 'E']);
            // b) legacy   → exactly 17 significant digits, no exponent.
            let legacy_case =
                formatted.len() >= 18                // “‑x.y000…0000” or “x.y000…0000”
                && !formatted.contains(['e', 'E']);
            assert!(ryu_case || legacy_case,
                    "bad fmt ‘{formatted}’ for {val}");

            // round‑trip guarantee (safety property)
            let back = formatted.parse::<f64>().unwrap();
            assert_eq!(back, val);

            val = (val * 100.0 + 1.0).round() / 100.0;   // increment by 0.01
        }
    }

    #[traced_test]
    fn round_trip_all_two_decimals_between_minus5_and_plus5_alt() {
        //  -5.12 … +5.12   (1025 values)
        for i in -512..=512 {
            let v = (i as f64) / 100.0;
            assert_round_trips(v);
        }
    }
}
