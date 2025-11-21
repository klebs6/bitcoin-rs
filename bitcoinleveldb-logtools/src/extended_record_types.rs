// ---------------- [ File: bitcoinleveldb-logtools/src/extended_record_types.rs ]
crate::ix!();

/**
  | Extend record types with the following
  | special values
  |
  */
bitflags!{ 
    pub struct ExtendedRecordTypes: i32 {
        const Eof = LOG_MAX_RECORD_TYPE as i32 + 1;

        /*
          | Returned whenever we find an invalid
          | physical record.
          | 
          | Currently there are three situations
          | in which this happens:
          | 
          | - The record has an invalid CRC (ReadPhysicalRecord
          | reports a drop)
          | 
          | - The record is a 0-length record (No
          | drop is reported)
          | 
          | - The record is below constructor's
          | initial_offset (No drop is reported)
          |
          */
        const BadRecord = LOG_MAX_RECORD_TYPE as i32 + 2;
    }
}

#[cfg(test)]
mod extended_record_types_spec {
    use super::*;

    #[traced_test]
    fn extended_record_type_values_follow_after_last_standard_type() {
        let last_type_value = LogRecordType::Last as i32;

        let eof_bits = ExtendedRecordTypes::Eof.bits();
        let bad_bits = ExtendedRecordTypes::BadRecord.bits();

        info!(
            "extended_record_type_values_follow_after_last_standard_type: last_type_value={} eof_bits={} bad_bits={}",
            last_type_value,
            eof_bits,
            bad_bits
        );

        assert_eq!(eof_bits, last_type_value + 1);
        assert_eq!(bad_bits, last_type_value + 2);
    }

    #[traced_test]
    fn extended_record_type_bitflags_combine_and_interpret_bits() {
        let eof = ExtendedRecordTypes::Eof;
        let bad = ExtendedRecordTypes::BadRecord;

        let eof_bits = eof.bits();
        let bad_bits = bad.bits();

        let combined = eof | bad;

        info!(
            "extended_record_type_bitflags_combine_and_interpret_bits: eof_bits={} bad_bits={} combined_bits={}",
            eof_bits,
            bad_bits,
            combined.bits()
        );

        // Combined set should report that it contains each individual flag.
        assert!(combined.contains(eof));
        assert!(combined.contains(bad));

        // Raw bits of the combined flag must be the bitwise OR of the components.
        assert_eq!(combined.bits(), eof_bits | bad_bits);

        // Masking with a single flag should isolate just that flag.
        let eof_only = combined & eof;
        let bad_only = combined & bad;

        info!(
            "extended_record_type_bitflags_combine_and_interpret_bits: eof_only_bits={} bad_only_bits={}",
            eof_only.bits(),
            bad_only.bits()
        );

        assert!(eof_only.contains(eof));
        assert!(!eof_only.contains(bad));

        assert!(bad_only.contains(bad));
        assert!(!bad_only.contains(eof));

        // Reconstruct from raw bits and ensure we get an equivalent flag set.
        if let Some(reconstructed) = ExtendedRecordTypes::from_bits(combined.bits()) {
            info!(
                "extended_record_type_bitflags_combine_and_interpret_bits: reconstructed_bits={}",
                reconstructed.bits()
            );
            assert!(reconstructed.contains(eof));
            assert!(reconstructed.contains(bad));
            assert_eq!(reconstructed.bits(), combined.bits());
        } else {
            error!(
                "extended_record_type_bitflags_combine_and_interpret_bits: from_bits returned None for bits={}",
                combined.bits()
            );
            panic!("from_bits should reconstruct valid ExtendedRecordTypes");
        }
    }
}
