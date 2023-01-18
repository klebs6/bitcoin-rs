crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/scratch.h]

/**
  | Opaque data structure that holds rewriteable
  | "scratch space"
  | 
  | The purpose of this structure is to replace
  | dynamic memory allocations, because
  | we target architectures where this
  | may not be available. It is essentially
  | a resizable (within specified parameters)
  | block of bytes, which is initially created
  | either by memory allocation or TODO
  | as a pointer into some fixed rewritable
  | space.
  | 
  | Unlike the context object, this cannot
  | safely be shared between threads without
  | additional synchronization logic.
  |
  */
pub struct Scratch {

    /**
      | guard against interpreting this object
      | as other types
      |
      */
    magic:      [u8; 8],

    /**
      | actual allocated data
      |
      */
    data:       *mut c_void,

    /**
      | amount that has been allocated (i.e.
      | `data + offset` is the next available
      | pointer)
      |
      */
    alloc_size: usize,

    /**
      | maximum size available to allocate
      |
      */
    max_size:   usize,
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/scratch_impl.h]

pub fn scratch_create(
        error_callback: *const Callback,
        max_size:       usize) -> *mut Scratch {
    
    todo!();
        /*
            const size_t base_alloc = ROUND_TO_ALIGN(sizeof(scratch));
        c_void *alloc = checked_malloc(error_callback, base_alloc + size);
        scratch* ret = (scratch *)alloc;
        if (ret != NULL) {
            memset(ret, 0, sizeof(*ret));
            memcpy(ret->magic, "scratch", 8);
            ret->data = (c_void *) ((char *) alloc + base_alloc);
            ret->max_size = size;
        }
        return ret;
        */
}

pub fn scratch_destroy(
        error_callback: *const Callback,
        scratch:        *mut Scratch)  {
    
    todo!();
        /*
            if (scratch != NULL) {
            VERIFY_CHECK(scratch->alloc_size == 0); /* all checkpoints should be applied */
            if (memcmp_var(scratch->magic, "scratch", 8) != 0) {
                callback_call(error_callback, "invalid scratch space");
                return;
            }
            memset(scratch->magic, 0, sizeof(scratch->magic));
            free(scratch);
        }
        */
}

/**
  | Returns an opaque object used to "checkpoint"
  | a scratch space. Used with `scratch_apply_checkpoint`
  | to undo allocations.
  |
  */
pub fn scratch_checkpoint(
        error_callback: *const Callback,
        scratch:        *const Scratch) -> usize {
    
    todo!();
        /*
            if (memcmp_var(scratch->magic, "scratch", 8) != 0) {
            callback_call(error_callback, "invalid scratch space");
            return 0;
        }
        return scratch->alloc_size;
        */
}

/**
  | Applies a check point received from
  | `scratch_checkpoint`, undoing all
  | allocations since that point.
  |
  */
pub fn scratch_apply_checkpoint(
        error_callback: *const Callback,
        scratch:        *mut Scratch,
        checkpoint:     usize)  {
    
    todo!();
        /*
            if (memcmp_var(scratch->magic, "scratch", 8) != 0) {
            callback_call(error_callback, "invalid scratch space");
            return;
        }
        if (checkpoint > scratch->alloc_size) {
            callback_call(error_callback, "invalid checkpoint");
            return;
        }
        scratch->alloc_size = checkpoint;
        */
}

/**
  | Returns the maximum allocation the
  | scratch space will allow
  |
  */
pub fn scratch_max_allocation(
        error_callback: *const Callback,
        scratch:        *const Scratch,
        n_objects:      usize) -> usize {
    
    todo!();
        /*
            if (memcmp_var(scratch->magic, "scratch", 8) != 0) {
            callback_call(error_callback, "invalid scratch space");
            return 0;
        }
        /* Ensure that multiplication will not wrap around */
        if (ALIGNMENT > 1 && objects > SIZE_MAX/(ALIGNMENT - 1)) {
            return 0;
        }
        if (scratch->max_size - scratch->alloc_size <= objects * (ALIGNMENT - 1)) {
            return 0;
        }
        return scratch->max_size - scratch->alloc_size - objects * (ALIGNMENT - 1);
        */
}

/**
  | Returns a pointer into the most recently
  | allocated frame, or NULL if there is
  | insufficient available space
  |
  */
pub fn scratch_alloc(
        error_callback: *const Callback,
        scratch:        *mut Scratch,
        size:           usize)  {
    
    todo!();
        /*
            c_void *ret;
        size_t rounded_size;

        rounded_size = ROUND_TO_ALIGN(size);
        /* Check that rounding did not wrap around */
        if (rounded_size < size) {
            return NULL;
        }
        size = rounded_size;

        if (memcmp_var(scratch->magic, "scratch", 8) != 0) {
            callback_call(error_callback, "invalid scratch space");
            return NULL;
        }

        if (size > scratch->max_size - scratch->alloc_size) {
            return NULL;
        }
        ret = (c_void *) ((char *) scratch->data + scratch->alloc_size);
        memset(ret, 0, size);
        scratch->alloc_size += size;

        return ret;
        */
}
