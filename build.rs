fn main() {
    pkg_config::probe_library("oci8").unwrap();
}