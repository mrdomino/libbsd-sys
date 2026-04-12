use std::path::Path;

/// Read an env var, registering it with cargo's rerun-if-env-changed tracking.
fn tracked_var_os(key: &str) -> Option<std::ffi::OsString> {
    println!("cargo:rerun-if-env-changed={key}");
    std::env::var_os(key)
}

/// Parse a boolean env var. Returns `None` if absent, `Some(bool)` if present.
/// Accepts 1/0/true/false/yes/no (case-insensitive). Panics on unrecognized values.
fn parse_bool_env(key: &str) -> Option<bool> {
    let val = tracked_var_os(key)?;
    let s = val
        .to_str()
        .unwrap_or_else(|| panic!("{key} is not valid UTF-8"));
    match s.to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" => Some(true),
        "0" | "false" | "no" => Some(false),
        other => panic!("{key}={other}: expected 1/0/true/false/yes/no"),
    }
}

fn main() {
    let target_os = tracked_var_os("CARGO_CFG_TARGET_OS")
        .unwrap()
        .into_string()
        .unwrap();
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
        // Windows and other unsupported platforms: nothing to link.
        "windows" => return,
        // Everything else (Linux, etc.) needs the libbsd library.
        _ => {}
    }

    // docs.rs builds and CI clippy without libbsd-dev: skip all linking.
    if tracked_var_os("DOCS_RS").is_some() {
        return;
    }

    let feat_static = tracked_var_os("CARGO_FEATURE_STATIC").is_some();
    let feat_overlay = tracked_var_os("CARGO_FEATURE_OVERLAY").is_some();
    let env_static = parse_bool_env("LIBBSD_STATIC");
    let no_pkgcfg = tracked_var_os("LIBBSD_NO_PKG_CONFIG").is_some();
    let lib_dir = tracked_var_os("LIBBSD_LIB_DIR");
    let inc_dir = tracked_var_os("LIBBSD_INCLUDE_DIR");
    // Env var wins over feature (openssl-sys semantics).
    let statik = env_static.unwrap_or(feat_static);

    // Try pkg-config first, unless explicitly disabled or overridden by
    // LIBBSD_LIB_DIR.  If it succeeds, we're done; if it fails, fall
    // through to the manual link path below, so that `cargo check` and
    // `cargo clippy` in downstream crates succeed without libbsd-dev
    // installed.  A real binary build will then fail at link time with
    // the linker's own "cannot find -lbsd" message.
    if lib_dir.is_none() && !no_pkgcfg {
        let pkg = if feat_overlay {
            "libbsd-overlay"
        } else {
            "libbsd"
        };
        let mut cfg = pkg_config::Config::new();
        cfg.atleast_version("0.11");
        if statik {
            cfg.statik(true);
        }
        if let Ok(lib) = cfg.probe(pkg) {
            // Re-export paths so downstream build scripts can use them
            // via DEP_BSD_INCLUDE / DEP_BSD_LIBDIR.
            for p in &lib.include_paths {
                println!("cargo:include={}", p.display());
            }
            for p in &lib.link_paths {
                println!("cargo:libdir={}", p.display());
            }
            return;
        }
    }

    // Manual link path: pkg-config was disabled, failed, or LIBBSD_LIB_DIR
    // was set.  Emit a plain `rustc-link-lib=bsd`, plus a search path if
    // LIBBSD_LIB_DIR was set.
    if let Some(dir) = &lib_dir {
        println!(
            "cargo:rustc-link-search=native={}",
            Path::new(dir).display()
        );
        println!("cargo:libdir={}", Path::new(dir).display());
    }
    let kind = if statik { "static" } else { "dylib" };
    println!("cargo:rustc-link-lib={kind}=bsd");
    if let Some(inc) = &inc_dir {
        for p in std::env::split_paths(inc) {
            println!("cargo:include={}", p.display());
        }
    }
}
