# bitcoinleveldb-histogram

Fixed-bucket online statistics used by LevelDB benchmarks and diagnostics.
`Histogram` tracks sample count, extrema, sum, sum of squares, and 154 buckets
without retaining individual samples.

Start with `Histogram::default()`, add observations with `add`, or combine
histograms with `merge`. Access summaries through `average`, `median`,
`percentile`, `standard_deviation`, and `to_string`; `clear` resets the state.
Percentiles are interpolated from buckets rather than exact order statistics.
Use a consistent sample unit within each histogram and when merging results.
Mutation requires exclusive access; synchronization belongs to the caller.

See [statistics and formatting](src/histogram.rs) and
[bucket boundaries](src/bucket_limit.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
