use std::{fs::File, io::{self, Read}};

use rand::{rngs::ThreadRng, thread_rng, Rng};

const PRINTABLE_CHARS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!$/()=?{[]}.:,;-_+\n";

pub trait ContentProvider {
    fn fill_buf(&mut self, buf: &mut Vec<u8>) -> io::Result<()>;
}


pub struct PrintableCharProvider {
    source: ThreadRng,
    chars: Vec<char>,
}

impl PrintableCharProvider {
    pub fn new() -> Self {
        PrintableCharProvider{
            source: thread_rng(),
            chars: PRINTABLE_CHARS.chars().collect(),
        }
    }
}

impl ContentProvider for PrintableCharProvider {
    fn fill_buf(&mut self, buf: &mut Vec<u8>) -> io::Result<()> {
        let len = self.chars.len();
        for i in 0..buf.len() {
            let index = self.source.gen_range(0..len);
            buf[i] = self.chars[index] as u8;
        }
        Ok(())
    }
}


pub struct BinaryProvider {
    source: BinaryContentSource,
}

enum BinaryContentSource {
    DevRandom(File),
    Default(ThreadRng),
}

impl BinaryProvider {
    pub fn new(always_use_default: bool) -> Self {
        if always_use_default {
            BinaryProvider::new_default()
        } else {
            match File::open("/dev/random") {
                Ok(source) => {
                    BinaryProvider::new_linux(source)
                },
                Err(_) => {
                    BinaryProvider::new_default()
                },
            }
        }
    }

    fn new_linux(source: File) -> Self {
        return BinaryProvider {
            source: BinaryContentSource::DevRandom(source),
        }
    }

    fn new_default() -> Self {
        BinaryProvider {
            source: BinaryContentSource::Default(thread_rng())
        }
    }
}

impl ContentProvider for BinaryProvider {
    fn fill_buf(&mut self, buf: &mut Vec<u8>) -> io::Result<()> {
        match self.source {
            BinaryContentSource::Default(ref mut rng) => {
                for i in 0..buf.len() {
                    buf[i] = rng.gen_range(0..=255);
                }
                Ok(())
            },
            BinaryContentSource::DevRandom(ref mut file) => {
                file.read_exact(buf)
            },
        }
        
    }
}