use std::{io::{ Read, Write, BufRead, BufReader }, path::Path};
use lzma_rs::xz_compress;

use crate::{Result, port::Archiver, essential::port::FileSystem};

pub enum Compressor {
    Lzma
}

impl Default for Compressor {
    fn default() -> Self {
        Compressor::Lzma
    }
}

impl Archiver for Compressor {
    fn compress<R: Read, W: Write>(&self, reader: R, mut write: W) -> Result<()> {
        let mut buf = BufReader::new(reader);
        match self {
            Self::Lzma => Ok(xz_compress(&mut buf, &mut write)?)
        }
    }

    fn archive<P: AsRef<Path>, W: Write>(&self, path: P, write: W) -> Result<()> {
        todo!()
    }
}
