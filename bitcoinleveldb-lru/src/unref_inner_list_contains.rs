// ---------------- [ File: bitcoinleveldb-lru/src/unref_inner_list_contains.rs ]
crate::ix!();

pub unsafe fn unref_inner_list_contains(
    head:   *mut LRUHandle,
    target: *mut LRUHandle,
    list_name: &str,
) -> bool {
    trace!(
        "unref_inner_list_contains: list_name={}, head={:p}, target={:p}",
        list_name,
        head,
        target
    );

    if head.is_null() || target.is_null() {
        debug!(
            "unref_inner_list_contains: list_name={} has null head/target (head={:p}, target={:p})",
            list_name,
            head,
            target
        );
        return false;
    }

    let align = core::mem::align_of::<LRUHandle>();
    let mut node: *mut LRUHandle = (*head).next_ptr();
    let mut steps: usize         = 0;

    while !core::ptr::eq(node, head) {
        let addr = node as usize;

        if addr % align != 0 {
            warn!(
                "unref_inner_list_contains({}): node pointer {:p} is misaligned (align={}), aborting search",
                list_name,
                node,
                align
            );
            return false;
        }

        if core::ptr::eq(node, target) {
            trace!(
                "unref_inner_list_contains: list_name={} contains target={:p} after {} steps",
                list_name,
                target,
                steps
            );
            return true;
        }

        node = (*node).next_ptr();
        steps = steps.wrapping_add(1);

        if steps > 1_000_000 {
            warn!(
                "unref_inner_list_contains({}): aborting search after {} steps; possible cycle without head",
                list_name,
                steps
            );
            return false;
        }
    }

    trace!(
        "unref_inner_list_contains: list_name={} does not contain target={:p}",
        list_name,
        target
    );
    false
}
