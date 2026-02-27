fn main() {
    // On macOS, the BSD functions are provided by the system C library
    // (libSystem), so no additional library needs to be linked.
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "macos" {
        return;
    }

    pkg_config::Config::new()
        .atleast_version("0.8")
        .probe("libbsd")
        .expect("libbsd not found; install libbsd-dev");
}
