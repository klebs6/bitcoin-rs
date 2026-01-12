#![cfg(test)]
crate::ix!();

extern crate std;

use core::mem::{align_of, size_of};
use core::ptr;

use std::alloc::{alloc_zeroed, dealloc, Layout};

#[inline(always)]
pub(crate) unsafe fn scalar_from_u32(v: u32) -> Scalar {
    let mut s = Scalar::new();
    scalar_set_int(ptr::addr_of_mut!(s), v);
    s
}

#[inline(always)]
pub(crate) unsafe fn scalar_to_u32_low(s: *const Scalar) -> u32 {
    scalar_get_bits_var(s, 0, 32) as u32
}

#[inline(always)]
pub(crate) unsafe fn gej_from_ge(ge: *const Ge) -> Gej {
    let mut r = Gej::new();
    gej_set_ge(ptr::addr_of_mut!(r), ge);
    r
}

#[inline(always)]
pub(crate) unsafe fn gej_infinity() -> Gej {
    let mut r = Gej::new();
    gej_set_infinity(ptr::addr_of_mut!(r));
    r
}

#[inline(always)]
pub(crate) unsafe fn gej_clone(src: *const Gej) -> Gej {
    let mut dst = core::mem::MaybeUninit::<Gej>::uninit();
    ptr::copy_nonoverlapping(src, dst.as_mut_ptr(), 1);
    dst.assume_init()
}

#[inline(always)]
pub(crate) unsafe fn ge_clone(src: *const Ge) -> Ge {
    let mut dst = core::mem::MaybeUninit::<Ge>::uninit();
    ptr::copy_nonoverlapping(src, dst.as_mut_ptr(), 1);
    dst.assume_init()
}

#[inline(always)]
pub(crate) unsafe fn gej_add(a: *const Gej, b: *const Gej) -> Gej {
    let mut r = Gej::new();
    gej_add_var(ptr::addr_of_mut!(r), a, b, ptr::null_mut());
    r
}

#[inline(always)]
pub(crate) unsafe fn gej_assert_eq_via_add_neg(label: &'static str, a: *const Gej, b: *const Gej) {
    let mut b_neg = gej_clone(b);
    gej_negate_in_place(ptr::addr_of_mut!(b_neg));

    let mut diff = Gej::new();
    gej_add_var(ptr::addr_of_mut!(diff), a, ptr::addr_of!(b_neg), ptr::null_mut());

    let is_inf = gej_is_infinity(ptr::addr_of!(diff)) != 0;
    tracing::debug!(
        target: "secp256k1::ecmult::tests",
        label = label,
        diff_is_infinity = is_inf,
        "gej_assert_eq_via_add_neg"
    );
    assert!(is_inf, "gej mismatch: {label}");
}

#[inline(always)]
pub(crate) unsafe fn gej_mul_small(point: *const Gej, scalar: u32) -> Gej {
    let mut acc = Gej::new();
    gej_set_infinity(ptr::addr_of_mut!(acc));

    let mut base = gej_clone(point);
    let mut k = scalar;

    let mut bit: u32 = 0;
    while bit < 32 {
        if (k & 1) != 0 {
            let mut tmp = Gej::new();
            gej_add_var(
                ptr::addr_of_mut!(tmp),
                ptr::addr_of!(acc),
                ptr::addr_of!(base),
                ptr::null_mut(),
            );
            acc = tmp;
        }
        let mut dbl = Gej::new();
        gej_double_var(ptr::addr_of_mut!(dbl), ptr::addr_of!(base), ptr::null_mut());
        base = dbl;

        k >>= 1;
        if k == 0 {
            break;
        }
        bit += 1;
    }

    acc
}

#[inline(always)]
pub(crate) unsafe fn alloc_zeroed_aligned(size: usize, align: usize) -> (*mut u8, Layout) {
    let layout = Layout::from_size_align(size, align).unwrap();
    let ptr = alloc_zeroed(layout);
    assert!(!ptr.is_null());
    (ptr, layout)
}

#[inline(always)]
pub(crate) unsafe fn dealloc_aligned(ptr: *mut u8, layout: Layout) {
    dealloc(ptr, layout);
}

#[inline(always)]
pub(crate) unsafe fn alloc_and_build_ecmult_context_preallocated(
) -> (*mut u8, Layout, *mut EcMultContext, *mut c_void, usize) {
    let ctx_offset: usize = round_to_align!(size_of::<EcMultContext>());
    let tables_size: usize = *ECMULT_CONTEXT_PREALLOCATED_SIZE;
    let total: usize = ctx_offset + tables_size;

    let align = core::cmp::max(align_of::<EcMultContext>(), align_of::<GeStorage>());
    let (buf, layout) = alloc_zeroed_aligned(total, align);

    let ctx = buf as *mut EcMultContext;
    ecmult_context_init(ctx);

    let mut cursor: *mut c_void = buf.add(ctx_offset) as *mut c_void;
    let cursor_ptr: *mut *mut c_void = ptr::addr_of_mut!(cursor);

    ecmult_context_build(ctx, cursor_ptr);

    (buf, layout, ctx, cursor, ctx_offset)
}

