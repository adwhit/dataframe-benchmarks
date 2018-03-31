extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate soa_derive;

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

fn main() {
    let path = std::env::args().skip(1).next();
    let path = path.unwrap_or_else(|| {
        println!("No file supplied");
        std::process::exit(1)
    });
    let mut reader = csv::Reader::from_path(path).unwrap();
    let mut arr = RowVec::new();
    for r in reader.deserialize() {
        let row: Row = r.unwrap();
        arr.push(row);
    }
    println!("{}", arr.len());
}
