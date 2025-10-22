// ---------------- [ File: bitcoin-tx-confirm-stats/src/read_write.rs ]
crate::ix!();

impl TxConfirmStats {

    /// Read saved state; panics on corruption (Core throws).
    ///
    /// Read data file and do some very basic sanity checking buckets and bucketMap are not updated
    /// yet, so don't access them
    ///
    /// If there is a read failure, we'll just discard this entire object anyway
    ///
    /// The current version will store the decay with each individual TxConfirmStats and also keep
    /// a scale factor
    ///
    pub fn read<R: Read>(
        &mut self,
        filein:         &mut R,
        _n_file_version: i32,
        num_buckets:    usize,
    ) {
        // decay (EncodedDoubleFormatter, LE)
        let mut df = EncodedDoubleFormatter::default();
        let mut decay = 0.0;
        df.unser(filein, &mut decay);
        if !(decay > 0.0 && decay < 1.0) {
            panic!("Corrupt estimates file. Decay must be between 0 and 1 (non-inclusive)");
        }

        // scale (u32, LE)
        let scale = read_u32_le(filein).expect("read(scale): io error");
        if scale == 0 {
            panic!("Corrupt estimates file. Scale must be non-zero");
        }

        // vectors of f64 via VectorFormatter<EncodedDoubleFormatter>
        let mut vf = VectorFormatter::<EncodedDoubleFormatter>::default();

        let mut feerate_avg = Vec::<f64>::new();
        vf.unser(filein, &mut feerate_avg);
        if feerate_avg.len() != num_buckets {
            panic!("Corrupt estimates file. Mismatch in feerate average bucket count");
        }

        let mut tx_ct_avg = Vec::<f64>::new();
        vf.unser(filein, &mut tx_ct_avg);
        if tx_ct_avg.len() != num_buckets {
            panic!("Corrupt estimates file. Mismatch in tx count bucket count");
        }

        // matrices of f64 via VectorFormatter<VectorFormatter<EncodedDoubleFormatter>>
        let mut mf = VectorFormatter::<VectorFormatter<EncodedDoubleFormatter>>::default();

        let mut conf_avg: Vec<Vec<f64>> = Vec::new();
        mf.unser(filein, &mut conf_avg);
        let max_periods  = conf_avg.len();
        let max_confirms = (scale as usize).saturating_mul(max_periods);

        if max_confirms == 0 || max_confirms > 6 * 24 * 7 {
            panic!("Corrupt estimates file.  Must maintain estimates for between 1 and 1008 (one week) confirms");
        }
        for (i, row) in conf_avg.iter().enumerate() {
            if row.len() != num_buckets {
                panic!("Corrupt estimates file. Mismatch in feerate conf average bucket count (row {i})");
            }
        }

        let mut fail_avg: Vec<Vec<f64>> = Vec::new();
        mf.unser(filein, &mut fail_avg);
        if fail_avg.len() != max_periods {
            panic!("Corrupt estimates file. Mismatch in confirms tracked for failures");
        }
        for (i, row) in fail_avg.iter().enumerate() {
            if row.len() != num_buckets {
                panic!("Corrupt estimates file. Mismatch in one of failure average bucket counts (row {i})");
            }
        }

        // Commit to self only after successful read/validation
        self.set_decay(decay);
        self.set_scale(scale);
        self.set_feerate_avg(feerate_avg);
        self.set_tx_ct_avg(tx_ct_avg);
        self.set_conf_avg(conf_avg);
        self.set_fail_avg(fail_avg);

        // Resize in-memory rolling structures to match the file
        self.resize_in_memory_counters(num_buckets);

        trace!(
            target: "estimatefee",
            "Reading estimates: {} buckets counting confirms up to {} blocks",
            num_buckets, max_confirms
        );
    }

    /// Write saved state; panics on IO error (Core throws).
    pub fn write<W: Write>(&self, fileout: &mut W) {
        // decay (EncodedDoubleFormatter, LE)
        let mut df = EncodedDoubleFormatter::default();
        df.ser(fileout, &self.decay());

        // scale (u32, LE)
        write_u32_le(fileout, *self.scale()).expect("write(scale): io error");

        // vectors of f64
        let mut vf = VectorFormatter::<EncodedDoubleFormatter>::default();
        vf.ser(fileout, &self.feerate_avg());
        vf.ser(fileout, &self.tx_ct_avg());

        // matrices of f64
        let mut mf = VectorFormatter::<VectorFormatter<EncodedDoubleFormatter>>::default();
        mf.ser(fileout, &self.conf_avg());
        mf.ser(fileout, &self.fail_avg());
    }
}

#[cfg(test)]
mod tx_confirm_stats_read_write_roundtrip {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn write_then_read_roundtrip() {
        let buckets = vec![1.0, 2.0, 3.0];
        let mut stats = TxConfirmStats::new(&buckets, &Default::default(), 3, 0.998, 2);

        stats.set_tx_ct_avg(vec![10.0, 20.0, 30.0]);
        stats.set_feerate_avg(vec![100.0, 200.0, 300.0]);
        stats.set_conf_avg(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ]);
        stats.set_fail_avg(vec![
            vec![0.1, 0.2, 0.3],
            vec![0.4, 0.5, 0.6],
            vec![0.7, 0.8, 0.9],
        ]);

        let mut buf = Cursor::new(Vec::<u8>::new());
        stats.write(&mut buf);

        buf.set_position(0);
        let mut stats2 = TxConfirmStats::new(&buckets, &Default::default(), 3, 0.5, 1);
        stats2.read(&mut buf, 0, buckets.len());

        assert_eq!(stats2.decay(), stats.decay());
        assert_eq!(stats2.scale(), stats.scale());
        assert_eq!(stats2.tx_ct_avg(), stats.tx_ct_avg());
        assert_eq!(stats2.feerate_avg(), stats.feerate_avg());
        assert_eq!(stats2.conf_avg(), stats.conf_avg());
        assert_eq!(stats2.fail_avg(), stats.fail_avg());
    }

    /* ----------------------- test scaffolding ----------------------- */

    fn mk_buckets(nb: usize) -> Vec<f64> {
        (1..=nb).map(|i| i as f64).collect()
    }

    fn mk_stats(nb: usize, periods: usize, scale: u32, decay: f64) -> TxConfirmStats {
        let buckets = mk_buckets(nb);
        let mut s = TxConfirmStats::new(&buckets, &Default::default(), periods as u32, decay, scale);
        // Fill persisted fields deterministically
        // tx_ct_avg, feerate_avg: length = nb
        s.set_tx_ct_avg((0..nb).map(|j| 10.0 + j as f64).collect());
        s.set_feerate_avg((0..nb).map(|j| 100.0 + 2.0 * j as f64).collect());
        // conf_avg, fail_avg: [periods][nb]
        s.set_conf_avg((0..periods)
            .map(|i| (0..nb).map(|j| (i as f64 + 1.0) * 1.25 + j as f64 * 0.5).collect())
            .collect());
        s.set_fail_avg((0..periods)
            .map(|i| (0..nb).map(|j| (i as f64 + 1.0) * 0.125 + j as f64 * 0.05).collect())
            .collect());
        s
    }

    fn assert_persisted_equal(a: &TxConfirmStats, b: &TxConfirmStats) {
        assert_eq!(a.decay(), b.decay(), "decay mismatch");
        assert_eq!(a.scale(), b.scale(), "scale mismatch");
        assert_eq!(a.tx_ct_avg(), b.tx_ct_avg(), "tx_ct_avg mismatch");
        assert_eq!(a.feerate_avg(), b.feerate_avg(), "feerate_avg mismatch");
        assert_eq!(a.conf_avg().len(), b.conf_avg().len(), "conf_avg periods mismatch");
        assert_eq!(a.fail_avg().len(), b.fail_avg().len(), "fail_avg periods mismatch");
        for (ra, rb) in a.conf_avg().iter().zip(b.conf_avg().iter()) {
            assert_eq!(ra, rb, "conf_avg row mismatch");
        }
        for (ra, rb) in a.fail_avg().iter().zip(b.fail_avg().iter()) {
            assert_eq!(ra, rb, "fail_avg row mismatch");
        }
    }

    fn assert_rolling_buffers_sized(s: &TxConfirmStats, nb: usize) {
        let expected_maxc = (*s.scale() as usize) * s.conf_avg().len();
        assert_eq!(s.unconf_txs().len(), expected_maxc, "unconf_txs rows (=get_max_confirms) mismatch");
        for row in s.unconf_txs().iter() {
            assert_eq!(row.len(), nb, "unconf_txs cols (=num_buckets) mismatch");
            // read() initializes these freshly
            assert!(row.iter().all(|&x| x == 0), "unconf_txs should be zeroed on read()");
        }
        assert_eq!(s.old_unconf_txs().len(), nb, "old_unconf_txs length mismatch");
        assert!(s.old_unconf_txs().iter().all(|&x| x == 0), "old_unconf_txs should be zeroed on read()");
    }

    /// Emit a raw file using the same formatters as production read()/write().
    fn emit_file_bytes(
        decay: f64,
        scale: u32,
        feerate_avg: &Vec<f64>,
        tx_ct_avg: &Vec<f64>,
        conf_avg: &Vec<Vec<f64>>,
        fail_avg: &Vec<Vec<f64>>,
    ) -> Vec<u8> {
        let mut buf = Cursor::new(Vec::<u8>::new());

        // decay (LE) via EncodedDoubleFormatter
        let mut df = EncodedDoubleFormatter::default();
        df.ser(&mut buf, &decay);

        // scale (u32 LE)
        write_u32_le(&mut buf, scale).expect("write(scale)");

        // vectors of f64 via VectorFormatter<EncodedDoubleFormatter>
        let mut vf = VectorFormatter::<EncodedDoubleFormatter>::default();
        vf.ser(&mut buf, feerate_avg);
        vf.ser(&mut buf, tx_ct_avg);

        // matrices of f64 via VectorFormatter<VectorFormatter<EncodedDoubleFormatter>>
        let mut mf = VectorFormatter::<VectorFormatter<EncodedDoubleFormatter>>::default();
        mf.ser(&mut buf, conf_avg);
        mf.ser(&mut buf, fail_avg);

        buf.into_inner()
    }

    /* ----------------------- positive round-trips ----------------------- */

    #[test]
    fn roundtrip_small() {
        let mut stats = mk_stats(/*nb*/3, /*periods*/3, /*scale*/2, /*decay*/0.998);
        let mut buf = Cursor::new(Vec::<u8>::new());
        stats.write(&mut buf);

        buf.set_position(0);
        let mut out = TxConfirmStats::new(&mk_buckets(3), &Default::default(), 3, 0.5, 1);
        out.read(&mut buf, 0, /*num_buckets*/3);

        assert_persisted_equal(&stats, &out);
        assert_rolling_buffers_sized(&out, 3);
        assert_eq!(out.get_max_confirms(), 2 * 3);
    }

    #[test]
    fn roundtrip_various_shapes() {
        let cases = [
            (1usize, 1usize, 1u32, 0.5f64),
            (3, 2, 2, 0.7),
            (8, 5, 3, 0.95),
            (16, 4, 10, 0.999_5),
            // boundary: exactly one week (1008) = 6*24*7
            (7, 7, 144, 0.999),
        ];
        for &(nb, periods, scale, decay) in &cases {
            let mut stats = mk_stats(nb, periods, scale, decay);
            let mut buf = Cursor::new(Vec::<u8>::new());
            stats.write(&mut buf);

            buf.set_position(0);
            let mut out = TxConfirmStats::new(&mk_buckets(nb), &Default::default(), periods as u32, 0.1, 1);
            out.read(&mut buf, 0, nb);

            assert_persisted_equal(&stats, &out);
            assert_rolling_buffers_sized(&out, nb);
            assert_eq!(out.get_max_confirms() as usize, scale as usize * periods);
        }
    }

    /* ----------------------- corruption & boundary tests ----------------------- */

    #[test]
    #[should_panic(expected = "Decay must be between 0 and 1")]
    fn read_rejects_decay_zero() {
        let nb = 3;
        let periods = 2;
        let bytes = emit_file_bytes(
            /*decay*/ 0.0,
            /*scale*/ 2,
            /*feerate*/ &vec![0.0; nb],
            /*tx*/      &vec![0.0; nb],
            /*conf*/    &vec![vec![0.0; nb]; periods],
            /*fail*/    &vec![vec![0.0; nb]; periods],
        );
        let mut buf = Cursor::new(bytes);
        let mut out = TxConfirmStats::new(&mk_buckets(nb), &Default::default(), 1, 0.5, 1);
        out.read(&mut buf, 0, nb);
    }

    #[test]
    #[should_panic(expected = "Decay must be between 0 and 1")]
    fn read_rejects_decay_one() {
        let nb = 3;
        let periods = 2;
        let bytes = emit_file_bytes(1.0, 2, &vec![0.0; nb], &vec![0.0; nb], &vec![vec![0.0; nb]; periods], &vec![vec![0.0; nb]; periods]);
        let mut buf = Cursor::new(bytes);
        let mut out = TxConfirmStats::new(&mk_buckets(nb), &Default::default(), 1, 0.5, 1);
        out.read(&mut buf, 0, nb);
    }

    #[test]
    #[should_panic(expected = "Scale must be non-zero")]
    fn read_rejects_scale_zero() {
        let nb = 2;
        let periods = 1;
        let bytes = emit_file_bytes(0.9, 0, &vec![0.0; nb], &vec![0.0; nb], &vec![vec![0.0; nb]; periods], &vec![vec![0.0; nb]; periods]);
        let mut buf = Cursor::new(bytes);
        let mut out = TxConfirmStats::new(&mk_buckets(nb), &Default::default(), 1, 0.5, 1);
        out.read(&mut buf, 0, nb);
    }

    #[test]
    #[should_panic(expected = "Must maintain estimates for between 1 and 1008")]
    fn read_rejects_max_confirms_zero() {
        // maxPeriods = 0 -> maxConfirms = scale*0 = 0
        let nb = 3;
        let periods = 0;
        let bytes = emit_file_bytes(0.9, 2, &vec![0.0; nb], &vec![0.0; nb], &vec![], &vec![]);
        let mut buf = Cursor::new(bytes);
        let mut out = TxConfirmStats::new(&mk_buckets(nb), &Default::default(), 1, 0.5, 1);
        out.read(&mut buf, 0, nb);
    }

    #[test]
    #[should_panic(expected = "Must maintain estimates for between 1 and 1008")]
    fn read_rejects_max_confirms_too_large() {
        // scale*periods > 1008 (one week)
        let nb = 3;
        let periods = 7;
        let bytes = emit_file_bytes(0.9, /*scale*/ 145, &vec![0.0; nb], &vec![0.0; nb],
                                    &vec![vec![0.0; nb]; periods], &vec![vec![0.0; nb]; periods]);
        let mut buf = Cursor::new(bytes);
        let mut out = TxConfirmStats::new(&mk_buckets(nb), &Default::default(), 1, 0.5, 1);
        out.read(&mut buf, 0, nb);
    }

    #[test]
    #[should_panic(expected = "Mismatch in feerate average bucket count")]
    fn read_rejects_feerate_len_mismatch() {
        // File has nb=3 but we pass num_buckets=4
        let nb_file = 3;
        let periods = 2;
        let bytes = emit_file_bytes(0.9, 2, &vec![0.0; nb_file], &vec![0.0; nb_file],
                                    &vec![vec![0.0; nb_file]; periods], &vec![vec![0.0; nb_file]; periods]);
        let mut buf = Cursor::new(bytes);
        let mut out = TxConfirmStats::new(&mk_buckets(4), &Default::default(), 1, 0.5, 1);
        out.read(&mut buf, 0, /*num_buckets*/ 4);
    }

    #[test]
    #[should_panic(expected = "Mismatch in tx count bucket count")]
    fn read_rejects_txct_len_mismatch() {
        // Build custom file: make txCtAvg of len 2, feerate len 3, pass num_buckets=3
        let feerate = vec![0.0; 3];
        let txct    = vec![0.0; 2];
        let conf    = vec![vec![0.0; 3]; 1];
        let fail    = vec![vec![0.0; 3]; 1];
        let bytes = emit_file_bytes(0.9, 2, &feerate, &txct, &conf, &fail);
        let mut buf = Cursor::new(bytes);
        let mut out = TxConfirmStats::new(&mk_buckets(3), &Default::default(), 1, 0.5, 1);
        out.read(&mut buf, 0, 3);
    }

    #[test]
    #[should_panic(expected = "Mismatch in feerate conf average bucket count")]
    fn read_rejects_confavg_row_len_mismatch() {
        // One row with wrong length (nb-1)
        let nb = 4;
        let periods = 3;
        let mut conf = vec![vec![0.0; nb]; periods];
        conf[1] = vec![0.0; nb - 1];
        let fail = vec![vec![0.0; nb]; periods];
        let bytes = emit_file_bytes(0.9, 2, &vec![0.0; nb], &vec![0.0; nb], &conf, &fail);
        let mut buf = Cursor::new(bytes);
        let mut out = TxConfirmStats::new(&mk_buckets(nb), &Default::default(), 1, 0.5, 1);
        out.read(&mut buf, 0, nb);
    }

    #[test]
    #[should_panic(expected = "Mismatch in confirms tracked for failures")]
    fn read_rejects_failavg_period_count_mismatch() {
        // failAvg has different number of rows than confAvg
        let nb = 3;
        let conf = vec![vec![0.0; nb]; 2];
        let fail = vec![vec![0.0; nb]; 1]; // fewer rows
        let bytes = emit_file_bytes(0.9, 2, &vec![0.0; nb], &vec![0.0; nb], &conf, &fail);
        let mut buf = Cursor::new(bytes);
        let mut out = TxConfirmStats::new(&mk_buckets(nb), &Default::default(), 1, 0.5, 1);
        out.read(&mut buf, 0, nb);
    }

    #[test]
    #[should_panic] // IO error panics (exact message depends on which read fails)
    fn read_rejects_truncated_stream() {
        let mut stats = mk_stats(3, 2, 2, 0.9);
        let mut buf = Cursor::new(Vec::<u8>::new());
        stats.write(&mut buf);
        let mut bytes = buf.into_inner();
        bytes.truncate(bytes.len() - 5); // chop tail

        let mut rd = Cursor::new(bytes);
        let mut out = TxConfirmStats::new(&mk_buckets(3), &Default::default(), 1, 0.5, 1);
        out.read(&mut rd, 0, 3);
    }

    #[test]
    #[should_panic(expected = "Decay must be between 0 and 1")]
    fn read_rejects_decay_nan() {
        let nb = 2;
        let periods = 1;
        let bytes = emit_file_bytes(f64::NAN, 2, &vec![0.0; nb], &vec![0.0; nb], &vec![vec![0.0; nb]; periods], &vec![vec![0.0; nb]; periods]);
        let mut buf = Cursor::new(bytes);
        let mut out = TxConfirmStats::new(&vec![1.0, 2.0], &Default::default(), 1, 0.5, 1);
        out.read(&mut buf, 0, nb);
    }

    #[test]
    #[should_panic(expected = "Decay must be between 0 and 1")]
    fn read_rejects_decay_inf() {
        let nb = 2;
        let periods = 1;
        let bytes = emit_file_bytes(f64::INFINITY, 2, &vec![0.0; nb], &vec![0.0; nb], &vec![vec![0.0; nb]; periods], &vec![vec![0.0; nb]; periods]);
        let mut buf = Cursor::new(bytes);
        let mut out = TxConfirmStats::new(&vec![1.0, 2.0], &Default::default(), 1, 0.5, 1);
        out.read(&mut buf, 0, nb);
    }

    /* ----------------------- dimensioning after read ----------------------- */

    #[test]
    fn resize_in_memory_counters_matches_file() {
        let nb = 5;
        let periods = 6;
        let scale = 7; // 42 = get_max_confirms
        let mut stats = mk_stats(nb, periods, scale, 0.99);
        let mut buf = Cursor::new(Vec::<u8>::new());
        stats.write(&mut buf);

        buf.set_position(0);
        let mut out = TxConfirmStats::new(&mk_buckets(nb), &Default::default(), 1, 0.1, 1);
        out.read(&mut buf, 0, nb);

        assert_eq!(out.get_max_confirms() as usize, scale as usize * periods);
        assert_rolling_buffers_sized(&out, nb);
    }

    /* ----------------------- optional: byte-level parity vs bitcoin-serialize -----------------------
       Enable with: --features serialize_parity  (once read/write switch to formatters)
    */
    #[cfg(feature = "serialize_parity")]
    #[test]
    fn bytes_match_vectorformatter_path() {
        use bitcoin_serialize::{EncodedDoubleFormatter, VectorFormatter, write_u32_le};

        let nb = 4;
        let periods = 3;
        let scale = 11;
        let decay = 0.987_654_321;

        // Build stats
        let mut stats = mk_stats(nb, periods, scale, decay);

        // Bytes from TxConfirmStats::write (DUT)
        let mut buf_dut = Cursor::new(Vec::<u8>::new());
        stats.write(&mut buf_dut);
        let bytes_dut = buf_dut.into_inner();

        // Bytes from canonical formatter path
        let mut buf_ref = Cursor::new(Vec::<u8>::new());
        let mut df = EncodedDoubleFormatter::default();
        df.ser(&mut buf_ref, &decay);
        write_u32_le(&mut buf_ref, scale).unwrap();

        let mut vf = VectorFormatter::<EncodedDoubleFormatter>::default();
        vf.ser(&mut buf_ref, &stats.feerate_avg);
        vf.ser(&mut buf_ref, &stats.tx_ct_avg);

        let mut mf = VectorFormatter::<VectorFormatter<EncodedDoubleFormatter>>::default();
        mf.ser(&mut buf_ref, &stats.conf_avg);
        mf.ser(&mut buf_ref, &stats.fail_avg);

        let bytes_ref = buf_ref.into_inner();

        assert_eq!(bytes_dut, bytes_ref, "TxConfirmStats bytes must match formatter bytes");
    }
}
