// ---------------- [ File: bitcoinleveldb-key/src/pack_sequence_and_type.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/dbformat.cc]

/**
  | Pack sequence number and value type into a single
  | 64-bit integer.
  |
  | We leave eight bits for the type.
  */
pub fn pack_sequence_and_type(seq: u64, t: ValueType) -> u64 {
    debug!(
        "pack_sequence_and_type: seq={}, ty={:?}",
        seq,
        t
    );
    assert!(
        seq <= MAX_SEQUENCE_NUMBER,
        "sequence {} exceeds MAX_SEQUENCE_NUMBER {}",
        seq,
        MAX_SEQUENCE_NUMBER
    );
    let tag = t as u64;
    assert!(
        tag <= VALUE_TYPE_FOR_SEEK as u64,
        "value type tag {} exceeds VALUE_TYPE_FOR_SEEK",
        tag
    );
    (seq << 8) | tag
}

#[cfg(test)]
mod pack_sequence_and_value_type_tests {
    use super::*;

    #[traced_test]
    fn pack_sequence_and_type_roundtrip_across_values() {
        let sequences: [SequenceNumber; 4] = [0, 1, 12345, MAX_SEQUENCE_NUMBER];
        let types = [ValueType::TypeDeletion, ValueType::TypeValue];

        for &seq in &sequences {
            for &ty in &types {
                trace!(
                    "pack_sequence_and_type_roundtrip: seq={}, ty={:?}",
                    seq,
                    ty
                );
                let tag = pack_sequence_and_type(seq, ty);
                let low = (tag & 0xff) as u8;
                let hi = tag >> 8;

                assert_eq!(
                    hi,
                    seq,
                    "high bits must contain the sequence number"
                );
                assert_eq!(
                    ValueType::from_tag(low),
                    Some(ty),
                    "low 8 bits must decode back into ValueType"
                );
            }
        }
    }

    #[test]
    #[should_panic]
    fn pack_sequence_and_type_panics_when_sequence_exceeds_max() {
        let seq = MAX_SEQUENCE_NUMBER
            .checked_add(1)
            .expect("overflow computing MAX+1");
        let _ = pack_sequence_and_type(seq, ValueType::TypeValue);
    }

    #[traced_test]
    fn value_type_from_tag_accepts_known_tags_and_rejects_unknown() {
        assert_eq!(
            ValueType::from_tag(0x0),
            Some(ValueType::TypeDeletion)
        );
        assert_eq!(ValueType::from_tag(0x1), Some(ValueType::TypeValue));

        for tag in [2u8, 3u8, 0xffu8] {
            assert!(
                ValueType::from_tag(tag).is_none(),
                "ValueType::from_tag({:#x}) should be None",
                tag
            );
        }
    }
}
