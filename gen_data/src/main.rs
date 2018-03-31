extern crate rand;
#[macro_use]
extern crate rand_derive;

use rand::{thread_rng, Rng};
use std::fs::*;
use std::io::prelude::*;
use std::io::BufWriter;

const LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890 _";
const STRLEN: usize = 20;
const NROWS: usize = 20_000_000;


#[derive(Debug, Clone, Copy, Rand)]
enum Colour {
    Red,
    Yellow,
    Blue,
    Green,
    Pink,
    Purple,
    Black,
    White,
    Grey,
}

struct Row {
    i: i32,
    f: f64,
    f_nan: f64,
    word: [u8; STRLEN],
    cat: Colour,
}

fn random_word() -> [u8; STRLEN] {
    let mut rng = thread_rng();
    let mut buf = [0u8; STRLEN];
    for ix in 0..STRLEN {
        let cix = rng.gen::<usize>() % LETTERS.len();
        buf[ix] = LETTERS[cix];
    }
    buf
}

fn rand_row() -> Row {
    let mut rng = thread_rng();
    let i = rng.gen();
    let f = rng.gen();
    let f_nan = if rng.gen::<f32>() < 0.8 {
        rng.gen()
    } else {
        std::f64::NAN
    };
    let word = random_word();
    let cat = rng.gen();
    Row {
        i,
        f,
        f_nan,
        word,
        cat,
    }
}

fn write_row<W: Write>(w: &mut W) {
    let row = rand_row();
    let word = std::str::from_utf8(&row.word).unwrap();
    writeln!(
        w,
        "{},{},{},{},{:?}",
        row.i, row.f, row.f_nan, word, row.cat
    ).unwrap();
}

fn write_header<W: Write>(w: &mut W) {
    writeln!(w, "ints,floats,floats_nan,strings,categoricals").unwrap();
}

fn main() {
    let nrows = NROWS;
    let dirpath = "../data";
    if let Err(_) = DirBuilder::new().create(dirpath) {
        // ignore error
    }
    let path = format!("{}/data_{}rows.csv", dirpath, nrows);
    let f = File::create(&path).unwrap();
    let mut f = BufWriter::new(f);
    write_header(&mut f);
    for ix in 0..nrows {
        write_row(&mut f);
        if ix % 500_000 == 0 && ix > 0 {
            println!("Wrote {}k rows", ix / 1000);
        }
    }
    println!("Created file {}", path);
}
