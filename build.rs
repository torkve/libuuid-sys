extern crate "pkg-config" as pkg_config;

fn main() {
    pkg_config::find_library_opts("uuid", &pkg_config::Options{statik: true, atleast_version: None}).unwrap()
}
