crate::ix!();

#[macro_export] macro_rules! varint_mode {
    ($obj:ident, $mode:ident) => {
        VarIntFormatter::<$mode>::new($obj)
    }
}

#[macro_export] macro_rules! varint {
    ($obj:ident) => {
        VarIntFormatter::<VarIntMode::DEFAULT>::new($obj)
    }
}

#[macro_export] macro_rules! compactsize {
    ($obj:ident) => {
        CompactSizeFormatter::<true>::new($obj)
    }
}

#[macro_export] macro_rules! limited_string {
    ($obj:expr, $n:ident) => {
        LimitedStringFormatter::<$n>{ item: $obj }
    }
}

macro_rules! readwrite {
    ($($arg:ident),*) => {
        /*
                (::SerReadWriteMany(s, ser_action, __VA_ARGS__))
        */
    }
}

macro_rules! readwriteas {
    ($type:ident, $obj:ident) => {
        /*
                (::SerReadWriteMany(s, ser_action, ReadWriteAsHelper<type>(obj)))
        */
    }
}

macro_rules! ser_read {
    ($obj:ident, $code:ident) => {
        /*
                ::SerRead(s, ser_action, obj, [&](Stream& s, typename std::remove_const<Type>::type& obj) { code; })
        */
    }
}

macro_rules! ser_write {
    ($obj:ident, $code:ident) => {
        /*
                ::SerWrite(s, ser_action, obj, [&](Stream& s, const Type& obj) { code; })
        */
    }
}

/**
  | Implement the Ser and Unser methods
  | needed for implementing a formatter
  | (see Using below).
  | 
  | Both Ser and Unser are delegated to a
  | single static method SerializationOps,
  | which is polymorphic in the serialized/deserialized
  | type (allowing it to be const when serializing,
  | and non-const when deserializing).
  | 
  | Example use:
  | 
  | -----------
  | @code
  | 
  | struct FooFormatter {
  |   FORMATTER_METHODS(Class, obj) { READWRITE(obj.val1, VARINT(obj.val2)); }
  | }
  |
  | would define a class FooFormatter that
  | defines a serialization of Class objects
  | consisting of serializing its val1
  | member using the default serialization,
  | and its val2 member using
  | 
  | VARINT serialization. That FooFormatter
  | can then be used in statements like
  | 
  | READWRITE(Using<FooFormatter>(obj.bla)).
  |
  */
macro_rules! formatter_methods {
    ($cls:ident, $obj:ident) => {
        /*
        
            template<typename Stream> 
            static c_void Ser(Stream& s, const cls& obj) { SerializationOps(obj, s, CSerActionSerialize()); } 
            template<typename Stream> 
            static c_void Unser(Stream& s, cls& obj) { SerializationOps(obj, s, CSerActionUnserialize()); } 
            template<typename Stream, typename Type, typename Operation> 
            static inline c_void SerializationOps(Type& obj, Stream& s, Operation ser_action) 
        */
    }
}

/**
  | Implement the Serialize and Unserialize
  | methods by delegating to a single templated
  | static method that takes the to-be-(de)serialized
  | object as a parameter.
  | 
  | This approach has the advantage that
  | the constness of the object becomes
  | a template parameter, and thus allows
  | a single implementation that sees the
  | object as const for serializing and
  | non-const for deserializing, without
  | casts.
  |
  */
macro_rules! serialize_methods {
    ($cls:ident, $obj:ident) => {
        /*
        
            template<typename Stream>                                                       
            c_void Serialize(Stream& s) const                                                 
            {                                                                               
                const_assert(std::is_same<const cls&, decltype(*this)>::value, "Serialize type mismatch"); 
                Ser(s, *this);                                                              
            }                                                                               
            template<typename Stream>                                                       
            c_void Unserialize(Stream& s)                                                     
            {                                                                               
                const_assert(std::is_same<cls&, decltype(*this)>::value, "Unserialize type mismatch"); 
                Unser(s, *this);                                                            
            }                                                                               
            FORMATTER_METHODS(cls, obj)
        */
    }
}


