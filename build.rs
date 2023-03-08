use std::{env, fs};
use std::path::PathBuf;
fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());
    fs::copy("memory.x", out_dir.join("memory   .x")).unwrap();
}