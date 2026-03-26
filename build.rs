fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    match target_os.as_str() {
        // On macOS, OpenBSD, and NetBSD the BSD functions live in libc
        // (or libSystem on macOS), so no extra library is needed.
        "macos" | "openbsd" | "netbsd" => return,
        // On FreeBSD, most functions are in libc, but humanize_number,
        // pidfile_*, flopen, and expand_number live in libutil.
        "freebsd" => {
            println!("cargo::rustc-link-lib=util");
            return;
        }
        // Everything else (Linux, etc.) needs the libbsd library.
        _ => {}
    }

    pkg_config::Config::new()
        .atleast_version("0.11")
        .probe("libbsd")
        .expect("libbsd not found; install libbsd-dev");
}
