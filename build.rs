extern crate "pkg-config" as pkg_config;

fn main() {
    pkg_config::find_library_opts("uuid", &Options{statik: true, atleast_version: None}).unwrap()
}
