extern crate gcc;
use std::process::Command;
use std::path::Path;
use std::env;

fn main() {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lib/sqlite4/build");
    //panic!("{:?}", dest_path);
    //TODO fossil pull
    //TODO fossil open
    Command::new("cp").arg(format!("{:?}/Makefile.linux-gcc",dest_path)).arg(format!("{:?}/Makefile",dest_path)).status().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });


    let dest2_path = Path::new(&out_dir).join("lib/sqlite4/build");
    Command::new("make").status().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    println!("cargo:rustc-link-search=native={}", out_dir);
}
