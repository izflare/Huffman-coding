extern crate clap;
extern crate bit_vec;
extern crate hc;

use clap::{App, Arg};
use std::io::{prelude::*, BufReader, BufWriter};
use std::fs::File;
use bit_vec::BitVec;
use hc::huffman_coding;

fn main() {

    // arg proc
    let app = App::new("Huffman Coding")
        //{{{
        .version("0.1.0")                       
        .author("flare")     
        .about("Huffman Coding")
        .arg(Arg::with_name("input")
            .help("input sourse text file") 
            .short("i")
            .long("input")
            .takes_value(true)                  
            .required(true)                     
        )
        .arg(Arg::with_name("decode")
            .help("decode")
            .short("d")
            .long("decode")
        );
        //}}}
    let matches = app.get_matches();

    // input
    let mut s: Vec<u8> = Vec::new();
    let mut f = BufReader::new(File::open(&matches.value_of("input").unwrap()).expect("file not found"));
    f.read_to_end(&mut s).expect("Unable to read");
    let dec: bool = matches.is_present("decode");

    if !dec {
        let v: Vec<u32> = s.iter().map(|c| *c as u32).collect();
        let mut bv: BitVec = BitVec::new();
        let mut f = BufWriter::new(File::create(matches.value_of("input").unwrap().to_owned()+".hc").unwrap());
        huffman_coding::encode(&v, &mut bv);
        f.write(&bv.to_bytes()).unwrap();
    }
    else {
        let bv: BitVec = BitVec::from_bytes(&s);
        let mut v: Vec<u32> = Vec::new();
        huffman_coding::decode(&bv, &mut v);
        let u: Vec<u8> = v.iter().map(|c| *c as u8).collect();
        let mut f = BufWriter::new(File::create(matches.value_of("input").unwrap().to_owned()+".dec").unwrap());
        f.write(&u).unwrap();
    }

}
