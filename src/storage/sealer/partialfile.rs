use std::fs::File;

use anyhow::Result;
use filecoin_proofs::PaddedBytesAmount;

#[derive(Debug)]
pub struct PartialFile {
    max_piece: PaddedBytesAmount,
    path: String,
    file: File,
}

impl PartialFile {
    // pub fn create_partial_file(max_piece_size: PaddedBytesAmount, path: String) -> Result<Self> {
    //     let file = File::open(path)?;

    //     let entry = io_uring::opcode::Fallocate::new(file, max_piece_size).build();

    //     let mut ring = io_uring::IoUring::new(1)?;

    //     unsafe { ring.submission().push(&entry) };

    //     ring.submit_and_wait(1)
    // }

    // pub fn open_partial_file(max_piece_size: PaddedBytesAmount, path: String) -> Result<Self>{

    // }
}
