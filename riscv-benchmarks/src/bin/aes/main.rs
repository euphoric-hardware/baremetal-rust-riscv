#![no_main]
#![no_std]

use aes::{cipher::{generic_array::GenericArray, typenum, BlockDecrypt, BlockEncrypt, BlockSizeUser, KeyInit}, Aes256};
use data::*;
use riscv_benchmarks::*;
use riscv_rt::entry;

mod data;

const SCALE_FACTOR: usize = 78;
const BLOCK_SIZE: usize = 16;

// Very dumb AES, encrypts each block directly, no mode of operation

#[entry]
fn main() -> ! {
    // block_size() is not a const fn
    assert!(BLOCK_SIZE == Aes256::block_size());

    let mut blocks = [GenericArray::from([0_u8; 16]); LEN/BLOCK_SIZE];
    let block_copy = blocks.clone();
    // let blocks: [GenericArray<_, typenum::U16>; 32] = PLAINTEXT.chunks(Aes256::block_size())
    //     .map(|x| *GenericArray::from_slice(x)).collect();
    
    // We have to manually split blocks. In reality, data should be encrypted with a mode of
    // operation.
    for (block, chunk) in blocks.iter_mut().zip(PLAINTEXT.chunks(Aes256::block_size())) {
        *block = *GenericArray::from_slice(chunk);
    }

    let benchmark_data = start_benchmark();
    for _ in 0..SCALE_FACTOR {
        let cipher = Aes256::new(&GenericArray::from(KEY));
        cipher.encrypt_blocks(&mut blocks);
        let mut ciphertext = [0_u8; LEN];
        for (chunk, block) in ciphertext.chunks_mut(BLOCK_SIZE).zip(blocks) {
            chunk.clone_from_slice(block.as_slice());
        }
        verify_data(&ciphertext, &EXPECTED);
        cipher.decrypt_blocks(&mut blocks);
        let mut decrypted = [0_u8; LEN];
        for (chunk, block) in decrypted.chunks_mut(BLOCK_SIZE).zip(blocks) {
            chunk.clone_from_slice(block.as_slice());
        }
        verify_data(&decrypted, &PLAINTEXT);
    }

    end_benchmark(benchmark_data);
}

