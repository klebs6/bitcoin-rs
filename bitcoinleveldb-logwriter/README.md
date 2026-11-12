# bitcoinleveldb-logwriter

Writes LevelDB log records to a `WritableFile`. `LogWriter::add_record` splits
a logical record into full/first/middle/last physical records, writes CRC32C
headers, and pads the unused tail of a log block when required.

`LogWriter::new` takes an `Rc<RefCell<dyn WritableFile>>` and the destination's
current byte length. Supply the correct length when appending to an existing
file so block framing starts at the correct offset. Input `Slice` storage must
remain valid during the write. Each append/flush error is returned as `Status`.
The writer uses `flush`, not a storage `sync` durability barrier; the owning
database/file layer decides when synchronization is required. The shared handle
uses `Rc<RefCell<_>>`, not a cross-thread ownership abstraction.

See [construction](src/create.rs), [record fragmentation](src/add_record_internal.rs),
and [append/flush behavior](src/append_header_and_payload.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](../LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
