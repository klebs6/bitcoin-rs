// ---------------- [ File: bitcoin-random/src/shuffle.rs ]
crate::ix!();

/**
  | More efficient than using std::shuffle
  | on a FastRandomContext.
  | 
  | This is more efficient as std::shuffle
  | will consume entropy in groups of 64
  | bits at the time and throw away most.
  | 
  | This also works around a bug in libstdc++
  | std::shuffle that may cause type::operator=(type&&)
  | to be invoked on itself, which the library's
  | debug mode detects and panics on. This
  | is a known issue, see https://stackoverflow.com/questions/22915325/avoiding-self-assignment-in-stdshuffle
  |
  */
pub fn shuffle<'a, I: 'a, R>(
        mut first: I,
        last:      I,
        mut rng:   R)  

where 
I: PartialEq,
I: Copy,
I: Sub<Output = I>,
I: AddAssign<u64>,
I: Add<Output = u64>,
I: Deref<Target = &'a mut I>,
I: DerefMut<Target = &'a mut I>,
I: Add<u64>,
R: RandRange,
<I as Add<u64>>::Output: Deref<Target = &'a mut I>,
<I as Add<u64>>::Output: DerefMut<Target = &'a mut I>,

u64: From<I> {

    while first != last {

        let j: u64 = rng.randrange(u64::from(last - first));

        if j != 0 {
            let mut offset = first + j;
            std::mem::swap(*first, *(offset));
        }

        first += 1;
    }
}

pub fn shuffle_all<'a, I: 'a, R>(
        mut first: I,
        mut rng:   R)  

where 
I: PartialEq,
I: Copy,
I: Sub<Output = I>,
I: AddAssign<u64>,
I: Add<Output = u64>,
I: Deref<Target = &'a mut I>,
I: DerefMut<Target = &'a mut I>,
I: Add<u64>,
R: RandRange,
<I as Add<u64>>::Output: Deref<Target = &'a mut I>,
<I as Add<u64>>::Output: DerefMut<Target = &'a mut I>,

u64: From<I> {

    todo!();
}
