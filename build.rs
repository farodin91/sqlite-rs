
fn main() {
    let lib_dir = "/usr/lib/libsqlite4.a";
    let include_dir = "/usr/include/sqlite4.h";

    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=sqlite4");
    println!("cargo:include={}", include_dir);
}
