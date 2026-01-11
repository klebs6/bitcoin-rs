.PHONY: build vendor json

export CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG=true

HACK_BDB   := env CFLAGS="-Wno-error=implicit-function-declaration"
HACK_LLVM  := env LLVM_CONFIG_PATH="/usr/local/opt/llvm/bin/llvm-config"

HACK_CLANG := env LD_LIBRARY_PATH=/usr/local/opt/llvm/lib/ 

HACK_CLANG := 

RUSTFLAGS := -Awarnings
CARGO     := env CARGO_MSG_LIMIT=15 \
			 CARGO_BUILD_JOBS=12 \
			 NUM_JOBS=12 \
			 cargo 

#CARGO    := env CARGO_BUILD_JOBS=12 NUM_JOBS=12 cargo 

#CARGO_MSG_LIMIT := 100
BUILD := build --verbose
TEST  := test
BENCH := bench

RUST_LOG := debug
RUST_LOG := bitcoinleveldb_crc32=off,debug

#DEFAULT := hack_test
#DEFAULT := test_one_release
DEFAULT := test_active
#DEFAULT := build
#DEFAULT := build_active
#DEFAULT := test_one
#DEFAULT := test_ignored
#DEFAULT := test_one_ignored

FEATURES := --features "leveldb_snappy"
FEATURES := 

NO_FAIL_FAST := --no-fail-fast

#----------------------------------------[active]

ACTIVE := bitcoinsecp256k1-ecmult
ACTIVE := bitcoinleveldb-dbimpl        #loc: 1883

#----------------------------------------[block-2]
# ---[secp-layer-4]
#ACTIVE := bitcoinsecp256k1-eccontext
#ACTIVE := bitcoinsecp256k1-ecmultconst
# ---[secp-layer-4b]
#ACTIVE := bitcoinsecp256k1-ecdh
#ACTIVE := bitcoinsecp256k1-ecdsa
#ACTIVE := bitcoinsecp256k1-ecdsasignature
#ACTIVE := bitcoinsecp256k1-eckey
# ---[secp-layer-5]
#ACTIVE := bitcoinsecp256k1-ec
#ACTIVE := bitcoinsecp256k1-keys     #loc: 1556
#ACTIVE := bitcoinsecp256k1-parse    #loc: 512
#ACTIVE := bitcoinsecp256k1-recovery #loc: 992
# ---[secp-layer-6]
#ACTIVE := bitcoinsecp256k1-bench    #loc: 1643
#ACTIVE := bitcoinsecp256k1-schnorr  #loc: 1876
# ---[secp-layer-4]
#ACTIVE := bitcoin-secp256k1         #loc: 10730
#-------------------------------[active-below]
# ---[leveldb-layer-1]
#ACTIVE := bitcoinleveldb-harness       #loc: 297
#ACTIVE := bitcoinleveldb-modeldb       #loc: 281
#ACTIVE := bitcoinleveldb-dbconstructor #loc: 99
# ---[leveldb-layer-2]
#ACTIVE := bitcoinleveldb-dbtest        #loc: 2652
#ACTIVE := bitcoinleveldb-db            #loc: 1049
#ACTIVE := bitcoinleveldb-dbiter        #loc: 414
# ---[leveldb-layer-3]
#ACTIVE := bitcoinleveldb-bench         #loc: 2997
#ACTIVE := bitcoinleveldb-test          #loc: 3254
#ACTIVE := bitcoin-leveldb              #loc: 36

#----------------------------------------[block-1]
# ---[layer 0]
#ACTIVE := bitcoin-checkqueue      #loc: 365
#ACTIVE := bitcoin-validation      #loc: 153

# ---[layer 1]
#ACTIVE := bitcoin-subnet          #loc: 337

#----------------------------------------[block-2]
# ---[layer 2]
#ACTIVE := bitcoin-addr            #loc: 431
#ACTIVE := bitcoin-netpermissions  #loc: 319
#ACTIVE := bitcoin-proxy           #loc: 1088
#ACTIVE := bitcoin-sam             #loc: 999

# ---[layer 3]
#ACTIVE := bitcoin-dns             #loc: 298

# ---[layer 4]
#ACTIVE := bitcoin-key             #loc: 1980

# ---[layer 5]
#ACTIVE := bitcoin-hdchain         #loc: 165
#ACTIVE := bitcoin-message         #loc: 202

# ---[layer 6]
#ACTIVE := bitcoin-crypter         #loc: 320

# ---[layer 7]
#ACTIVE := bitcoin-scripting       #loc: 6547

# ---[layer 8]
#ACTIVE := bitcoin-compressor      #loc: 601
#ACTIVE := bitcoin-netmsg          #loc: 897
#ACTIVE := bitcoin-tx              #loc: 3049

# ---[layer 9]
#ACTIVE := bitcoin-block           #loc: 2882
#ACTIVE := bitcoin-bloom           #loc: 683
#ACTIVE := bitcoin-noui            #loc: 171
#ACTIVE := bitcoin-rbf             #loc: 182

# ---[layer 10]
#ACTIVE := bitcoin-blockencoding   #loc: 294
#ACTIVE := bitcoin-chain-consensus #loc: 442
#ACTIVE := bitcoin-client-ui       #loc: 460
#ACTIVE := bitcoin-foundblock      #loc: 203
#ACTIVE := bitcoin-merkle          #loc: 506
#ACTIVE := bitcoin-txmempoolentry  #loc: 598
#ACTIVE := bitcoinnode-txrelay     #loc: 112

# ---[layer 11]
#ACTIVE := bitcoin-blockpolicy     #loc: 1095
#ACTIVE := bitcoin-deployment      #loc: 234
#ACTIVE := bitcoin-pow             #loc: 142
#ACTIVE := bitcoin-signet          #loc: 209

# ---[layer 12]
#ACTIVE := bitcoin-db              #loc: 947

# ---[layer 13]
#ACTIVE := bitcoin-coinsview       #loc: 1865

# ---[layer 14]
#ACTIVE := bitcoin-policy          #loc: 584
#ACTIVE := bitcoin-signingprovider #loc: 5350
#ACTIVE := bitcoinchain-params     #loc: 1767

# ---[layer 15]
#ACTIVE := bitcoin-blockman        #loc: 856
#ACTIVE := bitcoin-net             #loc: 1846
#ACTIVE := bitcoin-packages        #loc: 151
#ACTIVE := bitcoin-portmap         #loc: 972
#ACTIVE := bitcoin-psbt            #loc: 2223

#----------------------------------------[block-3]
# ---[layer 16]
#ACTIVE := bitcoin-addrman         #loc: 2976
#ACTIVE := bitcoin-banman          #loc: 912
#ACTIVE := bitcoin-tor             #loc: 1036
#ACTIVE := bitcoin-txmempool       #loc: 6815
#ACTIVE := bitcoinnode-stats       #loc: 116

# ---[layer 17]
#ACTIVE := bitcoin-system          #loc: 2139

# ---[layer 18]
#ACTIVE := bitcoin-scriptpubkeyman #loc: 4101

# ---[layer 19]
#ACTIVE := bitcoin-bdb             #loc: 1388
#ACTIVE := bitcoin-chainman        #loc: 929
#ACTIVE := bitcoin-index           #loc: 721
#ACTIVE := bitcoin-sqlite          #loc: 861

# ---[layer 20]
#ACTIVE := bitcoin-blockfilter     #loc: 879
#ACTIVE := bitcoin-coincontrol     #loc: 425
#ACTIVE := bitcoin-ipc             #loc: 611

#----------------------------------------[block-4]
# ---[wallet layer 0]
#ACTIVE := bitcoinwallet-salvage   #loc: 186
#ACTIVE := bitcoinwallet-feature   #loc: 258

# ---[wallet layer 1]
#ACTIVE := bitcoinwallet-interface #loc: 3811

# ---[wallet layer 2]
#ACTIVE := bitcoin-walletdb        #loc: 409

# ---[wallet layer 3]
#ACTIVE := bitcoinwallet-context   #loc: 61

# ---[wallet layer 4]
#ACTIVE := bitcoinwallet-client    #loc: 364

# ---[wallet layer 5]
#ACTIVE := bitcoinwallet-library   #loc: 5434

# ---[wallet layer 6]
#ACTIVE := bitcoinwallet-fees      #loc: 520
#ACTIVE := bitcoinwallet-init      #loc: 371
#ACTIVE := bitcoinwallet-receive   #loc: 682
#ACTIVE := bitcoinwallet-spend     #loc: 1398

#----------------------------------------[block-5]
# ---[layer 21]
#ACTIVE := bitcoinnode-interface   #loc: 1618

# ---[layer 22]
#ACTIVE := bitcoin-connman         #loc: 5386
#ACTIVE := bitcoin-node            #loc: 1869

# ---[layer 23]
#ACTIVE := bitcoin-http            #loc: 1587
#ACTIVE := bitcoin-peerman         #loc: 16839

# ---[layer 24]
#ACTIVE := bitcoin-indexed-chain   #loc: 7615

# ---[layer 25]
#ACTIVE := bitcoin-coinselect      #loc: 1137
#ACTIVE := bitcoin-init            #loc: 2765
#ACTIVE := bitcoin-mainsignals     #loc: 358
#ACTIVE := bitcoin-miner           #loc: 914
#ACTIVE := bitcoin-net-zmq         #loc: 1032
#ACTIVE := bitcoin-restapi         #loc: 891

#----------------------------------------[block-6]
# ---[layer 26]
#ACTIVE := bitcoinrpc-util         #loc: 1967
#ACTIVE := bitcoinrpc-server       #loc: 1067
#ACTIVE := bitcoinrpc-blockchain   #loc: 3007
#ACTIVE := bitcoinrpc-mining       #loc: 1390
#ACTIVE := bitcoinrpc-misc         #loc: 864
#ACTIVE := bitcoinrpc-net          #loc: 1044
#ACTIVE := bitcoinrpc-txn          #loc: 2346
#ACTIVE := bitcoinrpc-dump         #loc: 1979
#ACTIVE := bitcoinrpc-wallet       #loc: 5416

# ---[layer 27]
#ACTIVE := bitcoin-cli             #loc: 3008
#ACTIVE := bitcoin-test            #loc: 21553
#ACTIVE := bitcoin-dummywallet     #loc: 100
#ACTIVE := bitcoin-dumpwallet      #loc: 316

# ---[layer 28]
#ACTIVE := bitcoin-bench           #loc: 2485
#ACTIVE := bitcoin-daemon          #loc: 345
#ACTIVE := bitcoin-fuzz            #loc: 12268

# ---[layer 29]
#ACTIVE := bitcoin-top             #loc: 128

#-------------------------------DONE

INDIVIDUAL_TEST := propagate_26bit_carries_once
INDIVIDUAL_TEST := poly1305
INDIVIDUAL_TEST := final_carry_and_sub_p
INDIVIDUAL_TEST := decrypt_matches_reference_aes128
INDIVIDUAL_TEST := load_byte_validation
INDIVIDUAL_TEST := load_byte
INDIVIDUAL_TEST := save_byte
INDIVIDUAL_TEST := shift_row
INDIVIDUAL_TEST := aes_setup_round_key_validation
INDIVIDUAL_TEST := compute_g_plus5_minus_p
INDIVIDUAL_TEST := populate_round_zero
INDIVIDUAL_TEST := sha256_round

default: $(DEFAULT)

gen_doc:
	RUSTFLAGS=$(RUSTFLAGS) ./u/generate-rustdoc-db

build:
	$(HACK_CLANG) RUST_BACKTRACE=full RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(BUILD)

build_active:
	$(HACK_CLANG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(BUILD) -p $(ACTIVE) --verbose

test_active:
	$(HACK_CLANG) RUST_LOG=$(RUST_LOG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(TEST) -p $(ACTIVE) --verbose $(FEATURES) $(NO_FAIL_FAST)

test_ignored:
	$(HACK_CLANG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(TEST) --release -p $(ACTIVE) --verbose $(FEATURES) -- --ignored

test_one:
	RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(TEST) $(INDIVIDUAL_TEST) -p $(ACTIVE) -- $(NOCAPTURE)

test_one_ignored:
	$(HACK_CLANG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(TEST) $(INDIVIDUAL_TEST) --release -p $(ACTIVE) --verbose $(FEATURES) -- --ignored

test_one_release:
	RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(TEST) --release $(INDIVIDUAL_TEST) -p $(ACTIVE) -- --ignored $(NOCAPTURE)

hack_test:
	RUSTFLAGS=$(RUSTFLAGS) cargo hack test --feature-powerset -p $(ACTIVE) --verbose -- --test-threads 1

vendor:
	RUSTFLAGS=$(RUSTFLAGS) $(CARGO) vendor

json:
	$(HACK_CLANG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(BUILD) --quiet --message-format=json 2> /dev/null | jq --slurp

timings:
	$(HACK_CLANG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) +nightly build -Z timings

bench:
	RUSTFLAGS="-Awarnings -C target-cpu=native" $(CARGO) $(BENCH) -p $(ACTIVE)

#-------------------------------[done-below]
#ACTIVE := bitcoin-amt
#ACTIVE := bitcoin-arena
#ACTIVE := bitcoin-autofile
#ACTIVE := bitcoin-bech32m
#ACTIVE := bitcoin-bitstream
#ACTIVE := bitcoin-blob
#ACTIVE := bitcoin-bufferedfile
#ACTIVE := bitcoin-locked-page-allocator
#ACTIVE := bitcoin-log
#ACTIVE := bitcoin-mem
#ACTIVE := bitcoin-serialize
#ACTIVE := bitcoin-service
#ACTIVE := bitcoin-sha1
#ACTIVE := bitcoin-string
#ACTIVE := bitcoin-support
#ACTIVE := bitcoin-time
#ACTIVE := bitcoin-u160
#ACTIVE := bitcoin-u256
#ACTIVE := bitcoin-vectorstream
#ACTIVE := bitcoin-aes
#ACTIVE := bitcoin-epoch
#ACTIVE := bitcoin-asmap
#ACTIVE := bitcoin-indirectmap
#ACTIVE := bitcoin-cuckoo-cache
#ACTIVE := bitcoin-golombrice
#ACTIVE := bitcoin-poly1305
#ACTIVE := bitcoin-locked-pool
#ACTIVE := bitcoin-chacha
#ACTIVE := bitcoin-sync
#ACTIVE := bitcoin-compat
#ACTIVE := bitcoin-ripemd
#ACTIVE := bitcoin-siphash
#ACTIVE := bitcoin-service-flags
#ACTIVE := bitcoin-syscall
#ACTIVE := bitcoin-crc32c
#ACTIVE := bitcoin-version
#ACTIVE := bitcoin-univalue       #note this one has a failing test
#ACTIVE := bitcoin-tokenpipe
#ACTIVE := bitcoin-random
#ACTIVE := bitcoin-tx-confirm-stats
#ACTIVE := bitcoin-sha256
#ACTIVE := bitcoin-sha256-sse41
#ACTIVE := bitcoin-sha256-hkdf
#ACTIVE := bitcoin-argsman
#ACTIVE := bitcoin-settings
#ACTIVE := bitcoin-base58
#ACTIVE := bitcoin-get-json-token # note that this one has a failing test
#ACTIVE := bitcoin-hash
#----------------------------------------------[done-but-uninstalled]
#ACTIVE := bitcoin-sha3
#ACTIVE := bitcoin-sha512
#ACTIVE := bitcoin-hmac-sha512
#ACTIVE := bitcoin-hmac-sha256
#ACTIVE := bitcoin-sock
#ACTIVE := bitcoin-fees
#ACTIVE := bitcoin-muhash
#ACTIVE := bitcoin-remote
#ACTIVE := bitcoin-network

#ACTIVE := bitcoinleveldb-arena          #loc: 371
#ACTIVE := bitcoinleveldb-cfg            #loc: 138
#ACTIVE := bitcoinleveldb-compat         #loc: 852
#ACTIVE := bitcoinleveldb-slice          #loc: 318
#ACTIVE := bitcoinleveldb-crc32          #loc: 640
#ACTIVE := bitcoinleveldb-hash           #loc: 111
#ACTIVE := bitcoinleveldb-histogram      #loc: 745
#ACTIVE := bitcoinleveldb-limiter        #loc: 151
#ACTIVE := bitcoinleveldb-rand           #loc: 179
#ACTIVE := bitcoinleveldb-sync           #loc: 60
#ACTIVE := bitcoinleveldb-status         #loc: 390
#ACTIVE := bitcoinleveldb-coding         #loc: 782
#ACTIVE := bitcoinleveldb-comparator     #loc: 185
#ACTIVE := bitcoinleveldb-filter         #loc: 521
#ACTIVE := bitcoinleveldb-cache          #loc: 999
#ACTIVE := bitcoinleveldb-util           #loc: 135
#ACTIVE := bitcoinleveldb-bloom          #loc: 989
#ACTIVE := bitcoinleveldb-key            #loc: 1953
#ACTIVE := bitcoinleveldb-snapshot       #loc: 842
#ACTIVE := bitcoinleveldb-lru            #loc: 4192
#ACTIVE := bitcoinleveldb-skiplist       #loc: 1045
#ACTIVE := bitcoinleveldb-logtools       #loc: 1198
#ACTIVE := bitcoinleveldb-logwriter      #loc: 1656
#ACTIVE := bitcoinleveldb-logreader      #loc: 2026
#ACTIVE := bitcoinleveldb-log            #loc: 8
#ACTIVE := bitcoinleveldb-versionedit    #loc: 1126
#ACTIVE := bitcoinleveldb-env            #loc: 853
#ACTIVE := bitcoinleveldb-posixtools     #loc: 495
#ACTIVE := bitcoinleveldb-posixwfile     #loc: 607
#ACTIVE := bitcoinleveldb-posixseqfile   #loc: 224
#ACTIVE := bitcoinleveldb-posixrafile    #loc: 382
#ACTIVE := bitcoinleveldb-posixlogger    #loc: 2080
#ACTIVE := bitcoinleveldb-posixmmaprfile #loc: 223
#ACTIVE := bitcoinleveldb-posixenv       #loc: 3437
#ACTIVE := bitcoinleveldb-posix          #loc: 632
#ACTIVE := bitcoinleveldb-memenv         #loc: 3569
#ACTIVE := bitcoinleveldb-stringsource
#ACTIVE := bitcoinleveldb-stringsink
#ACTIVE := bitcoinleveldb-reversekeycomparator
#ACTIVE := bitcoinleveldb-blockutil
#ACTIVE := bitcoinleveldb-blockbuilder
#ACTIVE := bitcoinleveldb-block
#ACTIVE := bitcoinleveldb-blockhandle
#ACTIVE := bitcoinleveldb-tablerep
#ACTIVE := bitcoinleveldb-blockconstructor
#ACTIVE := bitcoinleveldb-blockiter
#ACTIVE := bitcoinleveldb-snapshot
#ACTIVE := bitcoinleveldb-tablebuilder
#ACTIVE := bitcoinleveldb-iteratorinner
#ACTIVE := bitcoinleveldb-iterator
#ACTIVE := bitcoinleveldb-file           #loc: 843
#ACTIVE := bitcoinleveldb-merger
#ACTIVE := bitcoinleveldb-keyconvertingiterator
#ACTIVE := bitcoinleveldb-duplex
#ACTIVE := bitcoinleveldb-versioniterator
#ACTIVE := bitcoinleveldb-table
#ACTIVE := bitcoinleveldb-versionsetinterface
#ACTIVE := bitcoinleveldb-compactionstats
#ACTIVE := bitcoinleveldb-emptyiterator
#ACTIVE := bitcoinleveldb-erroriterator
#ACTIVE := bitcoinleveldb-footer
#ACTIVE := bitcoinleveldb-versionsetutil
#ACTIVE := bitcoinleveldb-tableconstructor
#ACTIVE := bitcoinleveldb-tablecache
#ACTIVE := bitcoinleveldb-compaction
#ACTIVE := bitcoinchain-client
#ACTIVE := bitcoinchain-interface
#ACTIVE := bitcoinchain-notifications
#ACTIVE := bitcoinleveldb-memtable
#ACTIVE := bitcoinleveldb-mockversionset
#ACTIVE := bitcoinleveldb-version
#ACTIVE := bitcoinleveldb-batch
#ACTIVE := bitcoinleveldb-specialenv
#ACTIVE := bitcoinleveldb-dumpfile
#ACTIVE := bitcoinleveldb-testenv
#ACTIVE := bitcoinsecp256k1-modinv32   #loc: 750
#ACTIVE := bitcoinsecp256k1-modinv
#ACTIVE := bitcoinsecp256k1-modinv64   #loc: 750
#ACTIVE := bitcoinsecp256k1-scratch  #loc: 210
#ACTIVE := bitcoinsecp256k1-fe5x52
#ACTIVE := bitcoinsecp256k1-fe10x26 
#ACTIVE := bitcoinsecp256k1-field    #loc: 3831
#ACTIVE := bitcoinsecp256k1-scalar   #loc: 3197
#ACTIVE := bitcoinsecp256k1-group    #loc: 1223
#ACTIVE := bitcoinleveldb-versionset
#ACTIVE := bitcoinleveldb-dbimplwriter  #loc: 36
#ACTIVE := bitcoinleveldb-options        #loc: 339
#ACTIVE := bitcoinleveldb-repair        #loc: 2302
#ACTIVE := bitcoin-scheduler       #loc: 449
#ACTIVE := bitcoinleveldb-dbinterface   #loc: 346
#ACTIVE := bitcoinsecp256k1-ecmultgen
