# LevelDB maintenance attribution correction

Five baseline preparation commits were created by assistant-generated scripts
that incorrectly injected `BitcoinRS Maintenance <bitcoinrs-maintenance@localhost>`
as their author/committer. The qualification tag also used that invented tagger.
This was a tooling mistake, not a separate reachable contributor.

These exact objects remain identifiable below. This preparation does not rewrite
their commit IDs, alter their parent links, move the qualification tag, or assign
historical authorship through an invented mapping. The original qualification
receipts remain evidence for their original objects.

- `0214a455d9f49084dc093d281d4dcce42173de1e`: Complete pending writebatch and multithreaded LevelDB checks
- `500a70a39617341ca0fe452115d666ca5ad213a0`: Focus LevelDB retries on failed and unexecuted checks
- `92ad19523df6ed11a66c4d83e13c25fd746cfba9`: Repair LevelDB test contracts and quiet bounded workload execution
- `c16b1de7cec31dfd1cb6688488dab3a43bb8cad8`: Align LevelDB metadata validation with capture limit
- `fcae2086e7bdcd7c191fcdb98f9f3e26cd5d1d31`: Add guarded LevelDB subsystem test entry

Qualification tag object: `6300061de96b1e56087a5ae458f6f347ba5b95e1`.

Future commits and tags must use the operator's explicitly configured Git
identity, with no synthetic fallback and no injected author/committer overrides.
The apply controller stops before committing when a usable identity is absent.
The created commit's actual author and committer are recorded in its receipt.

Package metadata cleanup removes `YourName <you@example.com>` placeholder author
entries and the invalid `<none>` address, preserving the legacy `klebs` name.
Existing upstream author lists and source notices are not replaced. The
[repository issue tracker](https://github.com/klebs6/bitcoin-rs/issues) is the
reachable project support route. A later raw-history correction, if desired,
needs a separate explicit plan because it changes descendants and evidence links.
