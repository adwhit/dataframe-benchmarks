cargo build --release --manifest-path gen-data/Cargo.toml
cp gen-data/target/release/gen-data gen-data/
gen-data/gen-data -n 1000000
gen-data/gen-data -n 10000000
gen-data/gen-data -n 50000000

DATA=data_10M_rows.csv

echo "\nRunning python bench"
python3 stress.py $DATA
echo "\nRunning R bench"
Rscript stress.R $DATA
echo "\nRunning rust bench"
cargo run --release --manifest-path stress-rs/Cargo.toml $DATA
