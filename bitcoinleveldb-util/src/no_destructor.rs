// ---------------- [ File: bitcoinleveldb-util/src/no_destructor.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/no_destructor.h]

/**
  | Wraps an instance whose destructor is never
  | called.
  |
  | This is intended for use with function-level
  | static variables.
  */
pub struct NoDestructor<InstanceType> {
    __remove_me__: std::marker::PhantomData<InstanceType>,

    /*
    lazy_static!{
        /*
        typename std::aligned_storage<sizeof(InstanceType),
                                        alignof(InstanceType)>::type instance_storage_;
        */
    }
*/
}

impl<InstanceType> NoDestructor<InstanceType> {

    pub fn new<Ts>(constructor_args: Ts) -> Self {
    
        todo!();
        /*


            const_assert(sizeof(instance_storage_) >= sizeof(InstanceType),
                      "instance_storage_ is not large enough to hold the instance");
        const_assert(
            alignof(decltype(instance_storage_)) >= alignof(InstanceType),
            "instance_storage_ does not meet the instance's alignment requirement");
        new (&instance_storage_)
            InstanceType(std::forward<ConstructorArgTypes>(constructor_args)...);
        */
    }
    
    pub fn get(&mut self) -> *mut InstanceType {
        
        todo!();
        /*
            return reinterpret_cast<InstanceType*>(&instance_storage_);
        */
    }
}
