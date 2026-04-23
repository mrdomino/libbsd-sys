//! Raw FFI bindings to libbsd.
//!
//! This crate provides `extern "C"` declarations for the functions and types
//! exported by [libbsd](https://libbsd.freedesktop.org/), a library that
//! provides commonly-used BSD functions on GNU/Linux systems.
//!
//! # Platform support
//!
//! On **macOS**, **FreeBSD**, **OpenBSD**, and **NetBSD**,
//! most of these functions are already part of the system C library, so no
//! additional library is needed.
//!
//! On **Linux**, the crate uses `pkg-config` at build time to locate libbsd.
//! On Debian/Ubuntu, install the development headers with:
//!
//! ```sh
//! apt install libbsd-dev
//! ```
//!
//! On **Windows** and other unsupported platforms, this crate is empty.
//!
//! # Conditional compilation
//!
//! Functions that only exist in libbsd (not on any BSD natively) are gated
//! behind `#[cfg(target_os = "linux")]`. Functions available on the BSDs but
//! not macOS are gated behind `#[cfg(not(target_os = "macos"))]`.
//!
//! The `strnvis` and `strnunvis` functions have different parameter orders
//! depending on whether the platform follows the NetBSD convention (macOS,
//! NetBSD, OpenBSD) or the FreeBSD convention (FreeBSD, Linux/libbsd).
//!
//! # Environment variables
//!
//! The build script recognizes the following environment variables:
//!
//! - **`LIBBSD_NO_PKG_CONFIG`** — Set to any value to skip `pkg-config`
//!   entirely. The build script will emit `cargo:rustc-link-lib=bsd` without
//!   any search path. This is useful for running `cargo clippy` in CI without
//!   `libbsd-dev` installed.
//!
//! - **`LIBBSD_LIB_DIR`** — Path to the directory containing the libbsd
//!   library. Implies `LIBBSD_NO_PKG_CONFIG`.
//!
//! - **`LIBBSD_INCLUDE_DIR`** — Path(s) to libbsd headers (colon-separated
//!   on Unix). Only used in the manual override path; the include paths are
//!   exported as `DEP_BSD_INCLUDE` for dependent build scripts.
//!
//! - **`LIBBSD_STATIC`** — Set to `1`/`true`/`yes` to force static linking,
//!   or `0`/`false`/`no` to force dynamic linking. Overrides the `static`
//!   crate feature when set.
//!
//! - **`DOCS_RS`** — When set (as it is automatically on docs.rs), the build
//!   script skips all linking. This allows documentation builds to succeed
//!   without libbsd installed.
//!
//! # Metadata for dependent crates
//!
//! This crate sets `links = "bsd"` in `Cargo.toml`, so dependent crates'
//! build scripts can read the following metadata via `DEP_BSD_*` environment
//! variables:
//!
//! - **`DEP_BSD_INCLUDE`** — Include paths for libbsd headers (one path per
//!   value; there may be multiple `include=` lines).
//!
//! - **`DEP_BSD_LIBDIR`** — Library directory (one path per value).

#![no_std]
#![allow(non_camel_case_types)]

#[cfg(not(target_os = "windows"))]
mod imp;
#[cfg(not(target_os = "windows"))]
pub use imp::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_arc4random() {
        unsafe {
            let _ = arc4random();
        }
    }

    #[test]
    fn smoke_strlcpy() {
        let src = b"hello\0";
        let mut dst = [0u8; 16];
        unsafe {
            let n = strlcpy(
                dst.as_mut_ptr().cast(),
                src.as_ptr().cast(),
                dst.len() as size_t,
            );
            assert_eq!(n, 5);
            assert_eq!(&dst[..6], b"hello\0");
        }
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "netbsd"))]
    fn smoke_humanize_number() {
        let mut buf = [0u8; 16];
        unsafe {
            let ret = humanize_number(
                buf.as_mut_ptr().cast(),
                buf.len() as size_t,
                1024 * 1024,
                b"\0".as_ptr().cast(),
                HN_AUTOSCALE,
                HN_DECIMAL | HN_NOSPACE | HN_B,
            );
            assert!(ret >= 0);
        }
    }

    #[test]
    fn smoke_arc4random_uniform() {
        unsafe {
            let val = arc4random_uniform(100);
            assert!(val < 100);
        }
    }

    #[test]
    fn smoke_strtonum() {
        let s = b"42\0";
        let mut errstr: *const core::ffi::c_char = core::ptr::null();
        unsafe {
            let val = strtonum(s.as_ptr().cast(), 0, 100, &mut errstr);
            assert_eq!(val, 42);
            assert!(errstr.is_null());
        }
    }

    #[test]
    fn smoke_getprogname() {
        unsafe {
            let name = getprogname();
            assert!(!name.is_null());
        }
    }

    #[test]
    fn smoke_vis_str() {
        let src = b"hello\tworld\0";
        let mut dst = [0u8; 64];
        unsafe {
            let ret = strvis(dst.as_mut_ptr().cast(), src.as_ptr().cast(), VIS_TAB);
            assert!(ret > 0);
        }
    }

    // -------------------------------------------------------------------
    // Link smoke tests: verify every extern symbol resolves at link time.
    // Each test coerces a function item to its fn-pointer type, forcing
    // the linker to resolve the symbol.  Variadic and divergent functions
    // use alternative strategies noted inline.
    // -------------------------------------------------------------------

    use core::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_void};

    // <bsd/string.h>
    #[test]
    fn link_string() {
        let _: unsafe extern "C" fn(*mut c_char, *const c_char, size_t) -> size_t = strlcpy;
        let _: unsafe extern "C" fn(*mut c_char, *const c_char, size_t) -> size_t = strlcat;
        let _: unsafe extern "C" fn(*const c_char, *const c_char, size_t) -> *mut c_char = strnstr;
        let _: unsafe extern "C" fn(mode_t, *mut c_char) = strmode;
        let _: unsafe extern "C" fn(*mut c_void, size_t) = explicit_bzero;
    }

    // <bsd/stdlib.h>
    #[test]
    fn link_stdlib() {
        let _: unsafe extern "C" fn() -> u32 = arc4random;
        let _: unsafe extern "C" fn(*mut c_void, size_t) = arc4random_buf;
        let _: unsafe extern "C" fn(u32) -> u32 = arc4random_uniform;
        let _: unsafe extern "C" fn() -> *const c_char = getprogname;
        let _: unsafe extern "C" fn(*const c_char) = setprogname;
        let _: unsafe extern "C" fn(
            *mut c_void,
            size_t,
            size_t,
            Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
        ) -> c_int = heapsort;
        let _: unsafe extern "C" fn(
            *mut c_void,
            size_t,
            size_t,
            Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
        ) -> c_int = mergesort;
        let _: unsafe extern "C" fn(*mut *const c_uchar, c_int, *const c_uchar, c_uint) -> c_int =
            radixsort;
        let _: unsafe extern "C" fn(*mut *const c_uchar, c_int, *const c_uchar, c_uint) -> c_int =
            sradixsort;
        let _: unsafe extern "C" fn(*mut c_void, size_t) -> *mut c_void = reallocf;
        let _: unsafe extern "C" fn(*mut c_void, size_t, size_t) -> *mut c_void = reallocarray;
        let _: unsafe extern "C" fn(*const c_char, i64, i64, *mut *const c_char) -> i64 = strtonum;
        let _: unsafe extern "C" fn(*mut c_int, *mut c_long) -> *mut c_char = getbsize;
    }

    #[test]
    #[cfg(not(target_os = "macos"))]
    fn link_stdlib_not_macos() {
        let _: unsafe extern "C" fn(*mut c_void, size_t, size_t, size_t) -> *mut c_void =
            recallocarray;
        let _: unsafe extern "C" fn(*mut c_void, size_t) = freezero;
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn link_stdlib_linux() {
        let _: unsafe extern "C" fn() = arc4random_stir;
        let _: unsafe extern "C" fn(*mut c_uchar, c_int) = arc4random_addrandom;
        let _: unsafe extern "C" fn(*const c_char, *mut i64) -> c_int = dehumanize_number;
    }

    // <bsd/unistd.h>
    #[test]
    fn link_unistd() {
        let _ = &raw const optreset;
        let _: unsafe extern "C" fn(*const c_void, mode_t) -> mode_t = getmode;
        let _: unsafe extern "C" fn(*const c_char) -> *mut c_void = setmode;
        let _: unsafe extern "C" fn(c_int) = closefrom;
        let _: unsafe extern "C" fn(c_int, *mut uid_t, *mut gid_t) -> c_int = getpeereid;
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn link_unistd_linux() {
        let _: unsafe extern "C" fn(c_int, *const *mut c_char, *const c_char) -> c_int = bsd_getopt;
        let _: unsafe extern "C" fn(c_int, *mut *mut c_char, *mut *mut c_char) = setproctitle_init;
    }

    #[test]
    #[cfg(not(target_os = "macos"))]
    fn link_setproctitle() {
        // Variadic: verify linkage by calling with an empty format string.
        unsafe { setproctitle(b"\0".as_ptr().cast()) }
    }

    // <bsd/stdio.h>
    #[test]
    fn link_stdio() {
        let _: unsafe extern "C" fn(*const c_char, *const c_char) -> *const c_char = fmtcheck;
        let _: unsafe extern "C" fn(*mut FILE, *mut size_t) -> *mut c_char = fgetln;
        #[allow(clippy::type_complexity)]
        let _: unsafe extern "C" fn(
            *const c_void,
            Option<unsafe extern "C" fn(*mut c_void, *mut c_char, c_int) -> c_int>,
            Option<unsafe extern "C" fn(*mut c_void, *const c_char, c_int) -> c_int>,
            Option<unsafe extern "C" fn(*mut c_void, off_t, c_int) -> off_t>,
            Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
        ) -> *mut FILE = funopen;
        let _: unsafe extern "C" fn(*mut FILE) -> c_int = fpurge;
    }

    // <bsd/readpassphrase.h>
    #[test]
    fn link_readpassphrase() {
        let _: unsafe extern "C" fn(*const c_char, *mut c_char, size_t, c_int) -> *mut c_char =
            readpassphrase;
    }

    // <bsd/vis.h>
    #[test]
    fn link_vis() {
        let _: unsafe extern "C" fn(*mut c_char, c_int, c_int, c_int) -> *mut c_char = vis;
        let _: unsafe extern "C" fn(*mut c_char, size_t, c_int, c_int, c_int) -> *mut c_char = nvis;
        let _: unsafe extern "C" fn(
            *mut c_char,
            c_int,
            c_int,
            c_int,
            *const c_char,
        ) -> *mut c_char = svis;
        let _: unsafe extern "C" fn(
            *mut c_char,
            size_t,
            c_int,
            c_int,
            c_int,
            *const c_char,
        ) -> *mut c_char = snvis;
        let _: unsafe extern "C" fn(*mut c_char, *const c_char, c_int) -> c_int = strvis;
        let _: unsafe extern "C" fn(*mut *mut c_char, *const c_char, c_int) -> c_int = stravis;
        let _: unsafe extern "C" fn(*mut c_char, *const c_char, c_int, *const c_char) -> c_int =
            strsvis;
        let _: unsafe extern "C" fn(
            *mut c_char,
            size_t,
            *const c_char,
            c_int,
            *const c_char,
        ) -> c_int = strsnvis;
        let _: unsafe extern "C" fn(*mut c_char, *const c_char, size_t, c_int) -> c_int = strvisx;
        let _: unsafe extern "C" fn(*mut c_char, size_t, *const c_char, size_t, c_int) -> c_int =
            strnvisx;
        let _: unsafe extern "C" fn(
            *mut c_char,
            size_t,
            *const c_char,
            size_t,
            c_int,
            *mut c_int,
        ) -> c_int = strenvisx;
        let _: unsafe extern "C" fn(
            *mut c_char,
            *const c_char,
            size_t,
            c_int,
            *const c_char,
        ) -> c_int = strsvisx;
        let _: unsafe extern "C" fn(
            *mut c_char,
            size_t,
            *const c_char,
            size_t,
            c_int,
            *const c_char,
        ) -> c_int = strsnvisx;
        let _: unsafe extern "C" fn(
            *mut c_char,
            size_t,
            *const c_char,
            size_t,
            c_int,
            *const c_char,
            *mut c_int,
        ) -> c_int = strsenvisx;
        let _: unsafe extern "C" fn(*mut c_char, *const c_char) -> c_int = strunvis;
        let _: unsafe extern "C" fn(*mut c_char, *const c_char, c_int) -> c_int = strunvisx;
        let _: unsafe extern "C" fn(*mut c_char, size_t, *const c_char, c_int) -> c_int =
            strnunvisx;
        let _: unsafe extern "C" fn(*mut c_char, c_int, *mut c_int, c_int) -> c_int = unvis;
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    fn link_vis_strnvis_freebsd() {
        let _: unsafe extern "C" fn(*mut c_char, *const c_char, size_t, c_int) -> c_int = strnvis;
        let _: unsafe extern "C" fn(*mut c_char, *const c_char, size_t) -> ssize_t = strnunvis;
    }

    #[test]
    #[cfg(any(target_os = "macos", target_os = "netbsd", target_os = "openbsd"))]
    fn link_vis_strnvis_netbsd() {
        let _: unsafe extern "C" fn(*mut c_char, size_t, *const c_char, c_int) -> c_int = strnvis;
        let _: unsafe extern "C" fn(*mut c_char, size_t, *const c_char) -> c_int = strnunvis;
    }

    // <bsd/libutil.h>
    #[test]
    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "netbsd"))]
    fn link_libutil() {
        let _: unsafe extern "C" fn(
            *mut c_char,
            size_t,
            i64,
            *const c_char,
            c_int,
            c_int,
        ) -> c_int = humanize_number;
        let _: unsafe extern "C" fn(*const c_char, *mut u64) -> c_int = expand_number;
        let _: unsafe extern "C" fn(*const c_char, mode_t, *mut pid_t) -> *mut pidfh = pidfile_open;
        let _: unsafe extern "C" fn(*const pidfh) -> c_int = pidfile_fileno;
        let _: unsafe extern "C" fn(*mut pidfh) -> c_int = pidfile_write;
        let _: unsafe extern "C" fn(*mut pidfh) -> c_int = pidfile_close;
        let _: unsafe extern "C" fn(*mut pidfh) -> c_int = pidfile_remove;
    }

    // flopen/flopenat are FreeBSD-specific; not available on NetBSD.
    #[test]
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    fn link_flopen() {
        // Variadic; verify linkage by calling.
        unsafe {
            let fd = flopen(b"/dev/null\0".as_ptr().cast(), 0);
            if fd >= 0 {
                libc::close(fd);
            }
            // Invalid dirfd — fails immediately, just verifies linkage.
            let _ = flopenat(-1, b"\0".as_ptr().cast(), 0);
        }
    }

    #[test]
    fn link_fparseln() {
        let _: unsafe extern "C" fn(
            *mut FILE,
            *mut size_t,
            *mut size_t,
            *const [c_char; 3],
            c_int,
        ) -> *mut c_char = fparseln;
    }

    // <bsd/nlist.h>
    #[test]
    #[cfg(not(target_os = "macos"))]
    fn link_nlist() {
        let _: unsafe extern "C" fn(*const c_char, *mut nlist) -> c_int = nlist;
    }

    // <bsd/stringlist.h>
    #[test]
    fn link_stringlist() {
        let _: unsafe extern "C" fn() -> *mut StringList = sl_init;
        let _: unsafe extern "C" fn(*mut StringList, *mut c_char) -> c_int = sl_add;
        let _: unsafe extern "C" fn(*mut StringList, c_int) = sl_free;
        let _: unsafe extern "C" fn(*mut StringList, *const c_char) -> *mut c_char = sl_find;
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn link_stringlist_linux() {
        let _: unsafe extern "C" fn(*mut StringList, *const c_char, c_int) -> c_int = sl_delete;
    }

    // <bsd/timeconv.h>
    #[test]
    #[cfg(target_os = "linux")]
    fn link_timeconv() {
        let _: unsafe extern "C" fn(i32) -> libc::time_t = time32_to_time;
        let _: unsafe extern "C" fn(libc::time_t) -> i32 = time_to_time32;
        let _: unsafe extern "C" fn(i64) -> libc::time_t = time64_to_time;
        let _: unsafe extern "C" fn(libc::time_t) -> i64 = time_to_time64;
        let _: unsafe extern "C" fn(libc::time_t) -> c_long = time_to_long;
        let _: unsafe extern "C" fn(c_long) -> libc::time_t = long_to_time;
        let _: unsafe extern "C" fn(libc::time_t) -> c_int = time_to_int;
        let _: unsafe extern "C" fn(c_int) -> libc::time_t = int_to_time;
    }

    // <bsd/err.h>
    #[test]
    fn link_err() {
        // warnc is variadic; verify linkage by calling with code 0.
        unsafe { warnc(0, core::ptr::null()) }
        // errc is variadic and divergent; verify linkage without calling.
        if core::hint::black_box(false) {
            unsafe { errc(1, 0, core::ptr::null()) }
        }
    }

    // <bsd/wchar.h>
    #[test]
    fn link_wchar() {
        let _: unsafe extern "C" fn(*mut FILE, *mut size_t) -> *mut libc::wchar_t = fgetwln;
        let _: unsafe extern "C" fn(*mut libc::wchar_t, *const libc::wchar_t, size_t) -> size_t =
            wcslcat;
        let _: unsafe extern "C" fn(*mut libc::wchar_t, *const libc::wchar_t, size_t) -> size_t =
            wcslcpy;
    }

    // <bsd/grp.h>
    #[test]
    fn link_grp() {
        let _: unsafe extern "C" fn(*const c_char, *mut gid_t) -> c_int = gid_from_group;
        let _: unsafe extern "C" fn(gid_t, c_int) -> *const c_char = group_from_gid;
    }

    // <bsd/pwd.h>
    #[test]
    fn link_pwd() {
        let _: unsafe extern "C" fn(*const c_char, *mut uid_t) -> c_int = uid_from_user;
        let _: unsafe extern "C" fn(uid_t, c_int) -> *const c_char = user_from_uid;
    }

    // misc
    #[test]
    fn link_inet() {
        let _: unsafe extern "C" fn(c_int, *const c_char, *mut c_void, size_t) -> c_int =
            inet_net_pton;
    }
}
