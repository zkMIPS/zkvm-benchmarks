#![no_main]

use sha2::{Digest, Sha256};

zkm_zkvm::entrypoint!(main);

pub fn main() {
    let input: [u8; 32] = zkm_zkvm::io::read();
    let num_iters: u32 = zkm_zkvm::io::read();
    let mut hash = input;
    for _ in 0..num_iters {
        let mut hasher = Sha256::new();
        hasher.update(input);
        let res = &hasher.finalize();
        hash = Into::<[u8; 32]>::into (*res);
    }

    zkm_zkvm::io::commit::<[u8; 32]>(&hash.into());
}
