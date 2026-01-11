// ---------------- [ File: bitcoinsecp256k1-ecmult/src/pippenger_max_points.rs ]
crate::ix!();

/// Returns the maximum number of points in addition to G that can be used with a given scratch
/// space.
///
/// The function ensures that fewer points may also be used.
///
pub fn pippenger_max_points(
    error_callback: *const Callback,
    scratch:        *mut Scratch,
) -> usize {
    tracing::trace!(target: "secp256k1::ecmult", "pippenger_max_points");

    unsafe {
        let max_alloc: usize = scratch_max_allocation(error_callback, scratch, PIPPENGER_SCRATCH_OBJECTS);
        let mut bucket_window: i32;
        let mut res: usize = 0;

        bucket_window = 1;
        while bucket_window <= PIPPENGER_MAX_BUCKET_WINDOW {
            let mut n_points: usize;
            let max_points: usize = pippenger_bucket_window_inv(bucket_window);
            let space_for_points: usize;
            let space_overhead: usize;
            let entry_size: usize = core::mem::size_of::<Ge>()
                + core::mem::size_of::<Scalar>()
                + core::mem::size_of::<PippengerPointState>()
                + (wnaf_size!(bucket_window + 1) + 1usize) * core::mem::size_of::<i32>();

            let entry_size: usize = 2usize * entry_size;
            space_overhead = (core::mem::size_of::<Gej>() << (bucket_window as usize))
                + entry_size
                + core::mem::size_of::<PippengerState>();
            if space_overhead > max_alloc {
                break;
            }
            space_for_points = max_alloc - space_overhead;

            n_points = space_for_points / entry_size;
            n_points = if n_points > max_points { max_points } else { n_points };
            if n_points > res {
                res = n_points;
            }
            if n_points < max_points {
                /* A larger bucket_window may support even more points. But if we
                 * would choose that then the caller couldn't safely use any number
                 * smaller than what this function returns */
                break;
            }

            bucket_window += 1;
        }
        res
    }

        /*
            size_t max_alloc = scratch_max_allocation(error_callback, scratch, PIPPENGER_SCRATCH_OBJECTS);
        int bucket_window;
        size_t res = 0;

        for (bucket_window = 1; bucket_window <= PIPPENGER_MAX_BUCKET_WINDOW; bucket_window++) {
            size_t n_points;
            size_t max_points = pippenger_bucket_window_inv(bucket_window);
            size_t space_for_points;
            size_t space_overhead;
            size_t entry_size = sizeof(ge) + sizeof(scalar) + sizeof(struct pippenger_point_state) + (WNAF_SIZE(bucket_window+1)+1)*sizeof(int);

            entry_size = 2*entry_size;
            space_overhead = (sizeof(gej) << bucket_window) + entry_size + sizeof(struct pippenger_state);
            if (space_overhead > max_alloc) {
                break;
            }
            space_for_points = max_alloc - space_overhead;

            n_points = space_for_points/entry_size;
            n_points = n_points > max_points ? max_points : n_points;
            if (n_points > res) {
                res = n_points;
            }
            if (n_points < max_points) {
                /* A larger bucket_window may support even more points. But if we
                 * would choose that then the caller couldn't safely use any number
                 * smaller than what this function returns */
                break;
            }
        }
        return res;
        */

}

pub fn pippenger_max_points(
    error_callback: *const Callback,
    scratch:        *mut Scratch,
) -> usize {
    tracing::trace!(target: "secp256k1::ecmult", "pippenger_max_points");

    unsafe {
        let max_alloc: usize =
            scratch_max_allocation(error_callback, scratch, PIPPENGER_SCRATCH_OBJECTS);
        let mut bucket_window: i32;
        let mut res: usize = 0;

        bucket_window = 1;
        while bucket_window <= (PIPPENGER_MAX_BUCKET_WINDOW as i32) {
            let mut n_points: usize;
            let max_points: usize = pippenger_bucket_window_inv(bucket_window);
            let space_for_points: usize;
            let space_overhead: usize;
            let entry_size: usize = core::mem::size_of::<Ge>()
                + core::mem::size_of::<Scalar>()
                + core::mem::size_of::<PippengerPointState>()
                + (wnaf_size!(bucket_window + 1) + 1usize) * core::mem::size_of::<i32>();

            let entry_size: usize = 2usize * entry_size;
            space_overhead = (core::mem::size_of::<Gej>() << (bucket_window as usize))
                + entry_size
                + core::mem::size_of::<PippengerState>();
            if space_overhead > max_alloc {
                break;
            }
            space_for_points = max_alloc - space_overhead;

            n_points = space_for_points / entry_size;
            n_points = if n_points > max_points {
                max_points
            } else {
                n_points
            };
            if n_points > res {
                res = n_points;
            }
            if n_points < max_points {
                /* A larger bucket_window may support even more points. But if we
                 * would choose that then the caller couldn't safely use any number
                 * smaller than what this function returns */
                break;
            }

            bucket_window += 1;
        }
        res
    }
}
