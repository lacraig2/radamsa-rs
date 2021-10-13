use std::sync::atomic::{AtomicU32, Ordering::Relaxed};
use std::sync::Once;

static DEFAULT_SEED: AtomicU32 = AtomicU32::new(0);
static INIT: Once = Once::new();

const DEFAULT_BUF_SIZE: usize = 1024 * 1024;

mod bindings;

// Internal library call to make sure radamsa's C library is initialized
// before making any calls into it.
fn init() {
    unsafe {
        bindings::radamsa_init();
    }
}

/// This function generates a new buffer from an existing buffer.
///
/// # Arguments
/// * `input` - A vector of bytes used as input to radamsa.
/// * `seed`  - An optional seeded value to pass to radamsa. If left as None it increments from 0 after each call.
/// * `maxsize` - An optional maximum value for the output buffer size. Default: DEFAULT_BUF_SIZE.
pub fn generate(
    input: &Vec<u8>,
    seed: Option<u32>,
    maxsize: Option<usize>,
) -> Vec<u8> {
    INIT.call_once(init);
    let max_size_val = maxsize.unwrap_or(DEFAULT_BUF_SIZE);
    let mut out_vec = vec![0u8; max_size_val];
    let seed_val = match seed {
        Some(seed) => seed,
        None => DEFAULT_SEED.fetch_add(1, Relaxed),
    };
    let outlen = unsafe {
        bindings::radamsa(
            input.as_ptr(),
            input.len(),
            out_vec.as_mut_ptr(),
            max_size_val,
            seed_val,
        )
    };
    assert!(outlen <= max_size_val);
    out_vec.truncate(outlen);
    out_vec
}

/// This function mutates a buffer in-place
///
/// # Arguments
/// * `input` - A vector of bytes to mutate.
/// * `seed` - An optional seed value to pass to radamsa. If left as None it increments on each call from 0.
///
/// # Return
/// * `usize` representing the number of filled bytes.
///
/// NOTE: Unlike `generate` mutated does not truncate the buffer to the returned size.
pub fn mutate(input: &mut Vec<u8>, seed: Option<u32>) -> usize {
    INIT.call_once(init);
    let seed_val = match seed {
        Some(seed) => seed,
        None => DEFAULT_SEED.fetch_add(1, Relaxed),
    };
    let outlen = unsafe {
        bindings::radamsa_inplace(
            input.as_mut_ptr(),
            input.len(),
            input.len(),
            seed_val,
        )
    };
    outlen
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generator() {
        let start_val = "hello world".as_bytes().to_vec();
        for i in 0..100 {
            let output = generate(&start_val, None, None);
            println!("output {} {}", i, String::from_utf8_lossy(&output));
        }
    }

    #[test]
    fn test_mutator() {
        let mut mut_vec = "hello world".as_bytes().to_vec();
        mut_vec.extend(vec![0u8; 1000].iter().copied());
        for i in 0..100 {
            mutate(&mut mut_vec, None);
            println!("output {} \"{}\"", i, String::from_utf8_lossy(&mut_vec));
        }
    }
}
