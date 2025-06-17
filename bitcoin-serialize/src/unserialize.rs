// ---------------- [ File: bitcoin-serialize/src/unserialize.rs ]
crate::ix!();

/// Replace the previous definition (which took `&self`) with a
/// mutable‐reference API so implementations can actually write the
/// value that has just been read from the stream.
pub trait Unserialize<Stream> {
    /// Populate `self` with data read from `s`.
    fn unserialize(&mut self, s: &mut Stream);
}

impl<Stream> Unserialize<Stream> for i8
where
    Stream: Read,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        let v = ser_readdata8(s) as i8;
        trace!(value = v, "unserialize i8");
        *self = v;
    }
}

impl<Stream> Unserialize<Stream> for u8
where
    Stream: Read,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        let v = ser_readdata8(s);
        trace!(value = v, "unserialize u8");
        *self = v;
    }
}

impl<Stream> Unserialize<Stream> for i16
where
    Stream: Read,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        let v = ser_readdata16(s) as i16;
        trace!(value = v, "unserialize i16");
        *self = v;
    }
}

impl<Stream> Unserialize<Stream> for u16
where
    Stream: Read,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        let v = ser_readdata16(s);
        trace!(value = v, "unserialize u16");
        *self = v;
    }
}

impl<Stream> Unserialize<Stream> for i32
where
    Stream: Read,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        let v = ser_readdata32(s) as i32;
        trace!(value = v, "unserialize i32");
        *self = v;
    }
}

impl<Stream> Unserialize<Stream> for u32
where
    Stream: Read,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        let v = ser_readdata32(s);
        trace!(value = v, "unserialize u32");
        *self = v;
    }
}

impl<Stream> Unserialize<Stream> for i64
where
    Stream: Read,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        let v = ser_readdata64(s) as i64;
        trace!(value = v, "unserialize i64");
        *self = v;
    }
}

impl<Stream> Unserialize<Stream> for u64
where
    Stream: Read,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        let v = ser_readdata64(s);
        trace!(value = v, "unserialize u64");
        *self = v;
    }
}

impl<Stream, const N: usize> Unserialize<Stream> for [u8; N]
where
    Stream: Read,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        trace!(len = N, "unserialize [u8; N]");
        s.read_exact(self)
            .expect("I/O error while reading fixed‑length byte array");
    }
}

impl<Stream> Unserialize<Stream> for &[u8] {

    #[inline] fn unserialize(&mut self, s: &mut Stream)  {

        todo!();
            /*
                s.read(CharCast(span.data()), span.size());
            */
    }
}

impl<Stream> Unserialize<Stream> for bool
where
    Stream: Read,
{
    #[inline]
    fn unserialize(&mut self, s: &mut Stream) {
        let v = ser_readdata8(s) != 0;
        trace!(value = v, "unserialize bool");
        *self = v;
    }
}

impl<Stream> Unserialize<Stream> for String {

    fn unserialize(&mut self, is: &mut Stream)  {

        todo!();
            /*
                unsigned int nSize = ReadCompactSize(is);
            str.resize(nSize);
            if (nSize != 0)
                is.read((char*)str.data(), nSize * sizeof(C));
            */
    }
}

/* impls conflict with concrete type
 * impls
impl<Stream,T> Unserialize<Stream>  for T {
    #[inline] fn unserialize(&self, is: &mut Stream)  {
        todo!();
            /*
                a.Unserialize(is);
            */
    }
}
*/

impl<Stream, T: Default, const N: usize> Unserialize<Stream> for PreVector<T,N> {

    #[inline] fn unserialize(&mut self, is: &mut Stream)  {

        todo!();
            /*
            #[inline] fn _unserialize_impl_u8<Stream>(
                    is: &mut Stream,
                    v:  &mut PreVector<T,N>,
                    _2: &u8)  {

                todo!();
                    /*
                        // Limit size per read so bogus size value won't cause out of memory
                    v.clear();
                    unsigned int nSize = ReadCompactSize(is);
                    unsigned int i = 0;
                    while (i < nSize)
                    {
                        unsigned int blk = std::min(nSize - i, (unsigned int)(1 + 4999999 / sizeof(T)));
                        v.resize_uninitialized(i + blk);
                        is.read((char*)&v[i], blk * sizeof(T));
                        i += blk;
                    }
                    */
            }

            #[inline] fn _unserialize_impl<Stream,V>(
                    is: &mut Stream,
                    v:  &mut PreVector<T,N>,
                    _2: &V)  {

                todo!();
                    /*
                        Unserialize(is, Using<VectorFormatter<DefaultFormatter>>(v));
                    */
            }

                Unserialize_impl(is, v, T());
            */
    }
}

impl<Stream,T,A: Allocator> Unserialize<Stream> for Vec<T,A> {

    #[inline] fn unserialize(&mut self, is: &mut Stream)  {

        todo!();
            /*
            fn _unserialize_impl_u8<Stream>(
                    is: &mut Stream,
                    v:  &mut Vec<T,A>,
                    _2: &u8)  {

                todo!();
                    /*
                        // Limit size per read so bogus size value won't cause out of memory
                    v.clear();
                    unsigned int nSize = ReadCompactSize(is);
                    unsigned int i = 0;
                    while (i < nSize)
                    {
                        unsigned int blk = std::min(nSize - i, (unsigned int)(1 + 4999999 / sizeof(T)));
                        v.resize(i + blk);
                        is.read((char*)&v[i], blk * sizeof(T));
                        i += blk;
                    }
                    */
            }

            fn _unserialize_impl<Stream,V>(
                    is: &mut Stream,
                    v:  &mut Vec<T,A>,
                    _2: &V)  {

                todo!();
                    /*
                        Unserialize(is, Using<VectorFormatter<DefaultFormatter>>(v));
                    */
            }

                Unserialize_impl(is, v, T());
            */
    }
}

impl<Stream,K,T> Unserialize<Stream> for (K,T) {

    fn unserialize(&mut self, is:   &mut Stream)  {

        todo!();
            /*
                Unserialize(is, item.first);
            Unserialize(is, item.second);
            */
    }
}

impl<Stream,K,V> Unserialize<Stream> for HashMap<K,V> {
    fn unserialize(&mut self, is: &mut Stream)  {

        todo!();
            /*
                m.clear();
            unsigned int nSize = ReadCompactSize(is);
            typename std::map<K, V, Pred, A>::iterator mi = m.begin();
            for (unsigned int i = 0; i < nSize; i++)
            {
                std::pair<K, V> item;
                Unserialize(is, item);
                mi = m.insert(mi, item);
            }
            */
    }
}

impl<Stream,K> Unserialize<Stream> for HashSet<K> {
    fn unserialize(&mut self, is: &mut Stream)  {

        todo!();
            /*
                m.clear();
            unsigned int nSize = ReadCompactSize(is);
            typename std::set<K, Pred, A>::iterator it = m.begin();
            for (unsigned int i = 0; i < nSize; i++)
            {
                K key;
                Unserialize(is, key);
                it = m.insert(it, key);
            }
            */
    }
}

impl<Stream,T> Unserialize<Stream> for Box<T> {
    fn unserialize(&mut self, is: &mut Stream)  {

        todo!();
            /*
                p.reset(new T(deserialize, is));
            */
    }
}

impl<Stream,T> Unserialize<Stream> for Arc<T> {

    fn unserialize(&mut self, is: &mut Stream)  {

        todo!();
            /*
                p = std::make_shared<const T>(deserialize, is);
            */
    }
}
