// ---------------- [ File: bitcoin-serialize/src/read_write.rs ]
crate::ix!();

#[inline] pub fn ser_read_with_action_serialize<Stream, Type, Fn>(
        s:          &mut Stream,
        ser_action: SerActionSerialize,
        _2:         Type,
        _3:         Fn)  {

    todo!();
        /*
        
        */
}

#[inline] pub fn ser_read<Stream, Type, Fn>(
        s:          &mut Stream,
        ser_action: SerActionUnserialize,
        obj:        Type,
        fn_:        Fn)  {

    todo!();
        /*
            fn(s, std::forward<Type>(obj));
        */
}

#[inline] pub fn ser_write_with_action_serialize<Stream, Type, Fn>(
        s:          &mut Stream,
        ser_action: SerActionSerialize,
        obj:        Type,
        fn_:        Fn)  {

    todo!();
        /*
            fn(s, std::forward<Type>(obj));
        */
}

#[inline] pub fn ser_write_with_action_unserialize<Stream, Type, Fn>(
        s:          &mut Stream,
        ser_action: SerActionUnserialize,
        _2:         Type,
        _3:         Fn)  {

    todo!();
        /*
        
        */
}
