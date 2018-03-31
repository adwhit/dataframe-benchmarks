cargo build --release --manifest-path gen_data/Cargo.toml
# cargo run --release --manifest-path gen_data/Cargo.toml -- -n 1000000
# cargo run --release --manifest-path gen_data/Cargo.toml -- -n 10000000
# cargo run --release --manifest-path gen_data/Cargo.toml -- -n 50000000

echo "\nRunning python bench"
python3 stress.py data_10M_rows.csv
echo "\nRunning R bench"
Rscript stress.R data_10M_rows.csv
