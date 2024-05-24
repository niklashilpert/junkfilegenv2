use std::{fs::File, io::{self, Read}};

use rand::{rngs::ThreadRng, thread_rng, Rng};


const PRINTABLE_CHARS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!$/()=?{[]}.:,;-_+\n";

pub trait ContentProvider {

    fn new() -> Self;
    fn fill_buf(&mut self, buf: &mut Vec<u8>) -> io::Result<()>;
}

pub struct PrintableCharProvider {
    source: ThreadRng,
}

pub struct BinaryProvider {
    source: File,
}

impl ContentProvider for PrintableCharProvider {
    fn new() -> Self {
        PrintableCharProvider{
            source: thread_rng(),
        }
    }

    fn fill_buf(&mut self, buf: &mut Vec<u8>) -> io::Result<()> {
        let len = PRINTABLE_CHARS.chars().count();
        for i in 0..buf.len() {
            let index = self.source.gen_range(0..len);
            buf[i] = PRINTABLE_CHARS.chars().nth(index).unwrap() as u8;
        }
        Ok(())
    }
}

impl ContentProvider for BinaryProvider {
    fn new() -> Self {
        BinaryProvider{
            source: File::open("/dev/random").unwrap(),
        }
    }

    fn fill_buf(&mut self, buf: &mut Vec<u8>) -> io::Result<()> {
        self.source.read_exact(buf)
    }
}