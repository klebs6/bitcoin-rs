use bitcoinleveldb_rand::Random;
use bitcoinleveldbt_util::{random_key, random_key_bytes, dbtest_random_key,
    dbtest_random_key_bytes, random_string, compressible_string};

// Independent integer-modulus reference, rather than Random's folding reduction.
struct Reference { seed: u64, short: usize, skewed: usize, ordinary: usize }
impl Reference {
    fn new(seed: u32) -> Self {
        let seed = (seed & 0x7fff_ffff) as u64;
        Self { seed: if seed == 0 || seed == 2_147_483_647 { 1 } else { seed },
            short: 0, skewed: 0, ordinary: 0 }
    }
    fn next(&mut self) -> u32 {
        self.seed = (self.seed * 16_807) % 2_147_483_647; self.seed as u32
    }
    fn uniform(&mut self, n: u32) -> u32 { self.next() % n }
    fn key(&mut self, len: i32) -> Vec<u8> {
        let alphabet = [0, 1, b'a', b'b', b'c', b'd', b'e', 0xfd, 0xfe, 0xff];
        (0..len.max(0)).map(|_| alphabet[self.uniform(10) as usize]).collect()
    }
    fn chosen_key(&mut self) -> Vec<u8> {
        let len = if self.uniform(3) == 0 { self.short += 1; 1 }
        else if self.uniform(100) == 0 {
            self.skewed += 1;
            let exponent = self.uniform(11); self.uniform(1 << exponent) as i32
        } else { self.ordinary += 1; self.uniform(10) as i32 };
        self.key(len)
    }
    fn ascii(&mut self, len: i32) -> Vec<u8> {
        (0..len.max(0)).map(|_| b' ' + self.uniform(95) as u8).collect()
    }
}

#[test]
fn raw_fixed_keys_match_independent_bytes_lengths_and_next_draw() {
    for seed in [0, 1, 301, 999, 0x7fff_ffff, u32::MAX] {
        let mut native = Random::new(seed); let mut reference = Reference::new(seed);
        let mut high = false;
        for len in [-7, 0, 1, 7, 32, 255] {
            let bytes = random_key_bytes(&mut native, len);
            assert_eq!(bytes.len(), len.max(0) as usize);
            assert_eq!(bytes, reference.key(len));
            high |= bytes.iter().any(|b| *b >= 0xfd);
        }
        assert!(high); assert_eq!(native.next(), reference.next());
    }
}

#[test]
fn raw_chosen_keys_preserve_conditional_draw_order_and_continuous_tape() {
    for seed in [1, 301, 999, u32::MAX] {
        let mut native = Random::new(seed); let mut reference = Reference::new(seed);
        for _ in 0..2048 {
            assert_eq!(dbtest_random_key_bytes(&mut native), reference.chosen_key());
        }
        assert!(reference.short > 0 && reference.skewed > 0 && reference.ordinary > 0);
        assert_eq!(native.next(), reference.next());
    }
}

#[test]
fn legacy_text_keys_keep_scalar_mapping_and_the_same_draw_stream() {
    let mut saw_multibyte = false;
    for seed in [1, 301, 999] {
        let mut legacy = Random::new(seed); let mut raw = Random::new(seed);
        for len in [0, 1, 7, 32, 255] {
            let bytes = random_key_bytes(&mut raw, len);
            let text = random_key(&mut legacy, len);
            let intended: String = bytes.iter().copied().map(char::from).collect();
            assert_eq!(text, intended); assert_eq!(text.chars().count(), bytes.len());
            saw_multibyte |= text.len() > bytes.len();
        }
        for _ in 0..128 {
            let bytes = dbtest_random_key_bytes(&mut raw);
            assert_eq!(dbtest_random_key(&mut legacy), bytes.into_iter().map(char::from).collect::<String>());
        }
        assert_eq!(legacy.next(), raw.next());
    }
    assert!(saw_multibyte);
}

#[test]
fn printable_ascii_strings_match_literal_reference_and_draw_state() {
    let mut native = Random::new(301); let mut reference = Reference::new(301);
    let mut output = String::from("sentinel");
    for len in [-2, 0, 1, 7, 32, 257] {
        let view = random_string(&mut native, len, &mut output);
        assert_eq!(output.as_bytes(), reference.ascii(len));
        assert_eq!(view.as_bytes(), output.as_bytes());
        assert!(output.bytes().all(|b| (32..=126).contains(&b)));
    }
    assert_eq!(native.next(), reference.next());
}

#[test]
fn compressible_strings_keep_pattern_prefix_length_and_random_draws() {
    for fraction in [0.0, 0.25, 1.0, 1.5] {
        for len in [0, 1, 7, 32, 257] {
            let mut native = Random::new(301); let mut reference = Reference::new(301);
            let raw_len = ((len as f64 * fraction) as i32).max(1);
            let pattern = reference.ascii(raw_len);
            let expected: Vec<u8> = pattern.iter().copied().cycle().take(len).collect();
            let mut output = String::from("replace");
            let view = compressible_string(&mut native, fraction, len, &mut output);
            assert_eq!(view.as_bytes(), expected); assert_eq!(output.as_bytes(), expected);
            assert_eq!(output.len(), len); assert_eq!(native.next(), reference.next());
        }
    }
}

#[test]
fn null_inputs_return_without_looping_mutation_or_random_draws() {
    let null_random = std::ptr::null_mut();
    let mut output = String::from("preserve");
    // This positive-length call previously looped while appending an empty pattern.
    let view = compressible_string(null_random, 0.5, 32, &mut output);
    assert!(view.as_bytes().is_empty()); assert_eq!(output, "preserve");
    assert!(random_string(null_random, 32, &mut output).as_bytes().is_empty());
    assert_eq!(output, "preserve");
    assert!(random_key_bytes(null_random, 7).is_empty());
    assert!(dbtest_random_key_bytes(null_random).is_empty());
    assert!(random_key(null_random, 7).is_empty()); assert!(dbtest_random_key(null_random).is_empty());
    let mut native = Random::new(301); let mut reference = Reference::new(301);
    assert!(compressible_string(&mut native, 0.5, 32, std::ptr::null_mut()).as_bytes().is_empty());
    assert!(random_string(&mut native, 32, std::ptr::null_mut()).as_bytes().is_empty());
    assert!(random_key_bytes(&mut native, -1).is_empty());
    assert!(random_key_bytes(&mut native, 0).is_empty());
    assert_eq!(native.next(), reference.next());
}
