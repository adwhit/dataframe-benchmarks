extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate soa_derive;

use std::time::*;

#[derive(Debug, Clone, Copy, Deserialize)]
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

#[derive(Debug, Clone, Deserialize, StructOfArray)]
struct Row {
    ints: i32,
    floats: f64,
    floats_nan: f64,
    strings: String,
    categoricals: Colour,
}

fn sum_floats(arr: &RowVec) -> f64 {
    // TODO use SIMD?
    let mut s = 0.0;
    for f in &arr.floats {
        s += f;
    }
    s
}

fn lowercase(arr: &mut RowVec) {
    for s in &mut arr.strings {
        *s = s.to_ascii_lowercase();
    }
}

fn load(path: &str) -> RowVec {
    let mut reader = csv::Reader::from_path(path).unwrap();
    let mut arr = RowVec::new();
    for r in reader.deserialize() {
        let row: Row = r.unwrap();
        arr.push(row);
    }
    arr
}

fn timer<F, T>(f: F) -> T
    where F: FnOnce() -> T
{
    let start = Instant::now();
    let out = f();
    let duration = Instant::now() - start;
    let sec = duration.as_secs();
    let milli = duration.subsec_nanos() * 1_000_000;
    println!("Time: {}.{}s", sec, milli);
    out
}

fn main() {
    let path = std::env::args().skip(1).next();
    let path = path.unwrap_or_else(|| {
        println!("No file supplied");
        std::process::exit(1)
    });
    let mut arr = timer(|| {
        println!("Load data");
        load(&path)
    });
    let _sum_flt = timer(|| {
        println!("Sum floats");
        sum_floats(&arr)
    });
    timer(|| {
        println!("Lowercase strings");
        lowercase(&mut arr)
    });
}
