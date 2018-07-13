//! Simple Cli for using fastlz compression/decompresssion.
//!
#![feature(proc_macro)]
extern crate clap;
extern crate fastlz;
extern crate thunder;
use thunder::thunderclap;

const DEFAULT_BLOCK_SIZE: u32 = 32*1024;

struct Cli;
#[thunderclap]
impl Cli {
    fn compress(infile: &str, outfile: &str, blocksize: Option<u32>) {
        println!("Input file: {:?}", infile);
        println!("Output file: {:?}", outfile);
        println!("Block size: {}", blocksize.unwrap_or_else(|| DEFAULT_BLOCK_SIZE));
    }

    fn decompress(infile: &str, outfile: &str, blocksize: Option<u32>) {
        println!("Input file: {:?}", infile);
        println!("Output file: {:?}", outfile);
        println!("Block size: {}", blocksize.unwrap_or_else(|| DEFAULT_BLOCK_SIZE));
    }
}

fn main() {
    Cli::start();
}
