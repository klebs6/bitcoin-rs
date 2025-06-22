// ---------------- [ File: bitcoin-asmap/src/asmap.rs ]
/*!
This function, `decode_asmap`, reads an ASMAP from
a provided binary file and returns a vector of boolean
values. The ASMAP file contains a compact representation of
the IP-to-ASN mapping used for bucketing peers in the
Bitcoin network based on their Autonomous System Number
(ASN). The purpose of this function is to load the ASMAP
data from the file and convert it into a more easily
accessible data structure.

Here's a brief explanation of the C++ code:

1. It initializes an empty vector of boolean values called
   `bits`.

2. It attempts to open the provided binary file using the
   `fsbridge::fopen` function.

3. If the file cannot be opened, a log message is printed,
   and the empty `bits` vector is returned.

4. The code reads the file's length by seeking to the end
   and using `ftell` to get the current position.

5. The file is then read byte by byte, and each byte's bits
   are added to the `bits` vector.

6. After the entire file is read, a sanity check is
   performed on the `bits` vector using the
   `SanityCheckASMap` function. If the sanity check fails,
   a log message is printed, and an empty vector is
   returned.

7. If the sanity check passes, the `bits` vector is
   returned.

In Rust, the `decode_asmap` function will have a similar
structure. It will read the binary file from the given path,
convert it into a vector of boolean values, perform a sanity
check on the data, and return the vector. The `todo!();`
macro is a placeholder that should be replaced with the Rust
implementation that follows the same logic as the C++ code
provided.
*/

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/asmap.h]
//-------------------------------------------[.cpp/bitcoin/src/util/asmap.cpp]

/// Read an ASMAP from `path`, return its bits or an empty vec on failure.
///
/// * The binary on disk is interpreted little‑endian, LSB‑first, exactly like
///   the original C++ implementation.
/// * A full sanity check is executed before the vector is returned.
/// * Robust tracing is provided at every early‑return branch.
pub fn decode_asmap<P: AsRef<Path>>(path: P) -> Vec<bool> {
    let mut bits: Vec<bool> = Vec::new();

    let data = match fs::read(&path) {
        Ok(d) => d,
        Err(e) => {
            error!(%e, path = ?path.as_ref(), "Failed to read ASMAP file from disk");
            return bits;
        }
    };

    info!(
        bytes   = data.len(),
        path    = ?path.as_ref(),
        "Opened ASMAP file from disk"
    );

    for byte in data {
        for bit in 0..8 {
            bits.push(((byte >> bit) & 1) != 0);
        }
    }

    if !sanity_check_as_map(&bits, 128) {
        error!(path = ?path.as_ref(), "Sanity check of ASMAP file failed");
        return Vec::new();
    }

    bits
}
