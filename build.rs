fn main() {
    println!("cargo:rustc-link-search=native=pg_duckdb");
    println!("cargo:rustc-link-lib=static=pg_duckdb");
}
