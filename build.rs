fn main() {
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_OS");
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    match target_os.as_str() {
        // On macOS, OpenBSD, and NetBSD the BSD functions live in libc
        // (or libSystem on macOS), so no extra library is needed.
        "macos" | "openbsd" | "netbsd" => return,
        // On FreeBSD, most functions are in libc, but humanize_number,
        // pidfile_*, flopen, and expand_number live in libutil.
        "freebsd" => {
            println!("cargo:rustc-link-lib=util");
            return;
        }
        // Windows is not supported; it is a compile error later on.
        "windows" => return,
        // Everything else (Linux, etc.) needs the libbsd library.
        _ => {}
    }

    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_STATIC");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_OVERLAY");
    let statik = std::env::var_os("CARGO_FEATURE_STATIC").is_some();
    let overlay = std::env::var_os("CARGO_FEATURE_OVERLAY").is_some();

    let pkg = if overlay { "libbsd-overlay" } else { "libbsd" };

    let mut cfg = pkg_config::Config::new();
    cfg.atleast_version("0.11");
    if statik {
        cfg.statik(true);
    }
    let lib = cfg
        .probe(pkg)
        .unwrap_or_else(|e| panic!("{pkg} not found: {e}; install libbsd-dev"));

    // Re-export include paths so downstream build scripts can use them via
    // DEP_BSD_INCLUDE (one path per line).
    for path in &lib.include_paths {
        println!("cargo:metadata=include={}", path.display());
    }
}
