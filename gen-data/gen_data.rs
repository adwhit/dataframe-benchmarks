extern crate rand;
#[macro_use]
extern crate rand_derive;
#[macro_use]
extern crate structopt;

use structopt::StructOpt;
use rand::{thread_rng, Rng};

use std::path::PathBuf;
use std::fs::*;
use std::io::prelude::*;
use std::io::BufWriter;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "n")]
    nrows: i32,
    #[structopt(short = "o")]
    path: Option<String>,
}

const LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890 _";
const STRLEN: usize = 20;


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
    // i between -10000 and 10000
    let i = rng.gen::<i32>() % 10000;
    // f between -0.5 and 0.5
    let f = rng.gen::<f64>() - 0.5;
    // f_nan between -0.5 and 0.5 or nan 20% of time
    let f_nan = if rng.gen::<f32>() < 0.8 {
        rng.gen::<f64>() - 0.5
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
    let opt = Opt::from_args();
    let nrows = opt.nrows;
    println!("Generating {} rows", nrows);
    let path = opt.path.unwrap_or(format!("data_{}M_rows.csv", nrows/1000000));
    let path = PathBuf::from(path);
    if path.exists() {
        println!("File {} already exists", path.display());
        std::process::exit(1)
    }
    let f = File::create(&path).unwrap();
    let mut f = BufWriter::new(f);
    write_header(&mut f);
    for ix in 0..nrows {
        write_row(&mut f);
        if ix % 1_000_000 == 0 && ix > 0 {
            println!("Wrote {}M rows", ix / 1000000);
        }
    }
    println!("Saved to {}", path.display());
}
