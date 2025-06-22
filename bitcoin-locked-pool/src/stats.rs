crate::ix!();

/// Pool-wide memory statistics.
#[derive(Getters, Builder, Default, Debug, Clone)]
#[builder(setter(into, strip_option), pattern = "owned")]
#[getset(get = "pub")]
pub struct LockedPoolStats {
    used:        usize,
    free:        usize,
    total:       usize,
    locked:      usize,
    chunks_used: usize,
    chunks_free: usize,
}

impl LockedPool {

    /// Collect pool‑wide usage statistics.
    pub fn stats(&self) -> LockedPoolStats {
        let _lock = self.mutex.lock().unwrap();

        let mut agg_used        = 0;
        let mut agg_free        = 0;
        let mut agg_total       = 0;
        let mut agg_chunks_used = 0;
        let mut agg_chunks_free = 0;

        for arena in &self.arenas {
            let s = arena.stats();
            agg_used        += *s.used();
            agg_free        += *s.free();
            agg_total       += *s.total();
            agg_chunks_used += *s.chunks_used();
            agg_chunks_free += *s.chunks_free();
        }

        LockedPoolStatsBuilder::default()
            .used(agg_used)
            .free(agg_free)
            .total(agg_total)
            .locked(self.cumulative_bytes_locked)
            .chunks_used(agg_chunks_used)
            .chunks_free(agg_chunks_free)
            .build()
            .expect("stats built")
    }
}
