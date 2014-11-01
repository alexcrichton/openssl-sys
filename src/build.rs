extern crate "pkg-config" as pkg_config;

fn main() {
    if pkg_config::find_library("openssl").is_err() {
        println!("cargo:rustc-flags=-l crypto -l ssl");
    }
}
