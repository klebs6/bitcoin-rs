## bitcoinleveldb-filter crate

The `bitcoinleveldb-filter` crate is a Rust
implementation of the filter block functionality
in the Bitcoin Core software. This crate provides
tools for building and reading filter blocks,
which can be used to speed up block filtering
operations.

### Notice

This crate is part of a direct translation from
C++ to Rust of the bitcoin core. As such, some of
the function bodies may still be in the process of
translation. It is important to note that this
system will become testable once the translation
process is complete.

### Tokens

#### FilterBlockBuilder

`FilterBlockBuilder` is a struct that represents
a builder for filter blocks. It allows adding keys
to the filter block and generating the final
filter block.

#### FilterBlockReader

`FilterBlockReader` is a struct that represents
a reader for filter blocks. It allows checking if
a key may match the filter block.

#### add_key

`add_key` is a method of `FilterBlockBuilder` that
adds a key to the filter block.

#### finish

`finish` is a method of `FilterBlockBuilder` that
finalizes the filter block and returns the
generated filter.

#### generate_filter

`generate_filter` is a function that generates
a filter block from a list of keys.

#### key_may_match

`key_may_match` is a method of `FilterBlockReader`
that checks if a key may match the filter block.

#### start_block

`start_block` is a method of `FilterBlockBuilder`
that starts a new filter block.

#### FilterBlockTest

`FilterBlockTest` is a test suite for the
`FilterBlockBuilder` and `FilterBlockReader`
structs.

#### TestHashFilter

`TestHashFilter` is a struct that represents
a test hash filter for filter block testing.

#### create_filter

`create_filter` is a function that creates
a filter block from a list of keys.

#### name

`name` is a method of `FilterBlockTest` that
returns the name of the test suite.

#### tablefilter_block_test_main

`tablefilter_block_test_main` is the main function
for testing filter blocks. It runs the
`FilterBlockTest` suite and outputs the test
results.

22> thanks -- what are filter blocks? could you
describe them in more depth?

Filter blocks in LevelDB are data structures used
for implementing a fast bloom filter index for
SSTables (sorted string tables).

A bloom filter is a probabilistic data structure
that allows for efficient set membership
queries. It works by generating a bit array of
a fixed size and setting certain bits to 1 based
on the elements in the set. To query if an element
is in the set, the corresponding bits are
checked. If all bits are set to 1, then the
element is likely in the set. If any bit is 0,
then the element is definitely not in the
set. Bloom filters can have false positives, but
no false negatives.

In LevelDB, each SSTable has a corresponding
filter block, which is essentially a bloom filter
of the keys in the SSTable. The filter block is
stored at the end of the SSTable file and can be
loaded into memory when the SSTable is
opened. When performing a lookup on the database,
the filter block is used to quickly determine if
a key is not present in the SSTable. If the filter
block returns a negative match, then there is no
need to search the SSTable for the key. If the
filter block returns a positive match, then the
key may or may not be present in the SSTable, and
a full key lookup is required.

The FilterBlockBuilder and FilterBlockReader in
the `bitcoinleveldb-filter` crate are used to
create and read filter blocks, respectively. The
`add_key` method is used to add keys to the filter
block during construction, and the
`generate_filter` method is used to finalize the
filter block and produce a compact binary
representation that can be stored on disk.

Overall, filter blocks provide a way to speed up
key lookups in LevelDB by reducing the number of
disk reads required to find a key.

23> so, we have a filter block for each SST? is it
a one to one correspondance?

Yes, that's correct. For each SST file, there is
one corresponding filter block. The filter block
contains a set of keys that exist in the SST file,
and is used to speed up reads. When a read request
comes in for a particular key, the filter block is
first consulted to see if the key exists in the
SST file. If the key is not found in the filter
block, then we know for sure that it does not
exist in the SST file, and can avoid the more
expensive operation of actually searching the SST
file for the key. If the key is found in the
filter block, then we still need to check the SST
file to make sure it actually exists there, but at
least we have narrowed down the search space to
a particular block within the SST file, and can
skip over other blocks that do not contain the
key.
