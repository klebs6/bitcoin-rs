// ---------------- [ File: bitcoin-serialize/src/default_formatter.rs ]
crate::ix!();

/// Default formatter. Serializes objects as themselves.
/// 
/// The vector/prevector serialization code passes this to VectorFormatter to enable reusing that
/// logic. 
///
/// It shouldn't be needed elsewhere.
///
#[derive(Default)]
pub struct DefaultFormatter;



impl<T: 
    for<'a> BtcUnserialize<&'a mut dyn bitcoin_imports::Read>
        + for<'a> BtcSerialize<&'a mut dyn bitcoin_imports::Write>
    > ValueFormatter<T> for DefaultFormatter {

    fn ser<S: std::io::Write>(&mut self,
                              s: &mut S,
                              value: &T)
    {
        // Erase the concrete stream type `S`.
        let mut sink: &mut dyn std::io::Write = s;

        // Use the impls that already exist for *any* `Stream: Write`.
        BtcSerialize::<&mut dyn std::io::Write>::serialize(
            value,
            &mut sink,
        );
    }

    fn unser<S: std::io::Read>(&mut self,
                               s: &mut S,
                               value: &mut T)
    {
        let mut src: &mut dyn std::io::Read = s;

        BtcUnserialize::<&mut dyn std::io::Read>::unserialize(
            value,
            &mut src,
        );
    }
}


#[cfg(test)]
mod default_formatter_tests {
    use super::*;
    use std::io::Cursor;

    #[traced_test]
    fn bool_roundtrip() {
        let mut fmt = DefaultFormatter::default();
        let mut buf = Cursor::new(Vec::<u8>::new());

        let value = true;
        fmt.ser(&mut buf, &value);
        assert_eq!(buf.get_ref().as_slice(), &[1u8]);

        buf.set_position(0);
        let mut decoded = false;
        fmt.unser(&mut buf, &mut decoded);

        assert_eq!(decoded, value);
    }
}
