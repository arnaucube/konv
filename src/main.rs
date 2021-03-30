use clap::App;

use termion::{color, style};

extern crate base64;
extern crate hex;
extern crate num_bigint;
extern crate num_traits;
use num_bigint::{BigInt, Sign};

fn main() {
    let matches = App::new("myapp")
        .version("0.0.1")
        .about("Convert between string formats.\nhttps://github.com/arnaucube/konv")
        .arg("<INPUT>              'Value to parse'")
        .arg("-b                'Read input in BigEndian (by default uses LittleEndian)'")
        .get_matches();

    let inp = matches.value_of("INPUT").unwrap();

    println!("input {}", inp);

    let r: BigInt;
    if inp.starts_with("0x") {
        r = d_hex(inp, matches.is_present("b"));
    } else {
        r = d_dec(inp, matches.is_present("b"));
    }

    // print decimal
    println!(
        "  dec  {}{}{}",
        color::Fg(color::Yellow),
        r.to_string(),
        style::Reset
    );

    // print hexadecimal
    println!(
        "  hex  {}{}{}",
        color::Fg(color::Blue),
        r.to_str_radix(16),
        style::Reset
    );

    // print base64
    let (_, byt) = r.to_bytes_be();
    let b64 = base64::encode(&byt);
    println!("  b64  {}{}{}", color::Fg(color::Cyan), b64, style::Reset);

    // print bytes
    let mut byte_str: String = byt[0].to_string();
    for x in byt.iter().skip(1) {
        byte_str = format!("{}, {}", byte_str, x);
    }
    println!("  byt [{}]", byte_str);
}

fn d_hex(raw_inp: &str, bigendian: bool) -> BigInt {
    let inp;
    if raw_inp.starts_with("0x") {
        inp = raw_inp.trim_start_matches("0x");
    } else {
        inp = raw_inp;
    }
    let b = hex::decode(inp).expect("Decoding failed");
    let bi: BigInt;
    if bigendian {
        bi = BigInt::from_bytes_be(Sign::Plus, &b);
    } else {
        bi = BigInt::from_bytes_le(Sign::Plus, &b);
    }
    bi
}

fn d_dec(inp: &str, bigendian: bool) -> BigInt {
    let or = BigInt::parse_bytes(inp.as_bytes(), 10).unwrap();
    let (_, b) = or.to_bytes_be();
    if bigendian {
        return BigInt::from_bytes_be(Sign::Plus, &b);
    }
    BigInt::from_bytes_le(Sign::Plus, &b)
}
