// ---------------- [ File: bitcoin-serialize/src/many.rs ]
crate::ix!();

pub fn serialize_many_base<Stream>(s: &mut Stream)  { }

pub fn serialize_many<Stream, Arg, Args>(
        s:    &mut Stream,
        arg:  &Arg,
        args: &Args)  {

    todo!();
        /*
            ::Serialize(s, arg);
        ::SerializeMany(s, args...);
        */
}

#[inline] pub fn unserialize_many_base<Stream>(s: &mut Stream)  { }

#[inline] pub fn unserialize_many<Stream, Arg, Args>(
        s:    &mut Stream,
        arg:  Arg,
        args: Args)  {

    todo!();
        /*
            ::Unserialize(s, arg);
        ::UnserializeMany(s, args...);
        */
}

#[inline] pub fn ser_read_write_many_with_action_serialize<Stream, Args>(
        s:          &mut Stream,
        ser_action: SerActionSerialize,
        args:       &Args)  {

    todo!();
        /*
            ::SerializeMany(s, args...);
        */
}

#[inline] pub fn ser_read_write_many_with_action_unserialize<Stream, Args>(
        s:          &mut Stream,
        ser_action: SerActionUnserialize,
        args:       Args)  {

    todo!();
        /*
            ::UnserializeMany(s, args...);
        */
}
