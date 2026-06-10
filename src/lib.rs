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
//! Each declaration is gated to the platforms whose system library
//! actually exports the symbol.  In broad strokes:
//!
//! * Functions that exist only in libbsd (`bsd_getopt`, `setproctitle_init`,
//!   `arc4random_stir`, `dehumanize_number`, `time*_to_*`) are
//!   `#[cfg(target_os = "linux")]`.
//! * `recallocarray`/`freezero` originated in OpenBSD; libbsd ships them
//!   on Linux, but FreeBSD/NetBSD/macOS don't have them.
//! * macOS lacks `closefrom`, `explicit_bzero`, `reallocarray`,
//!   `setproctitle`, `gid_from_group`/`uid_from_user`, and (on Mach-O)
//!   `nlist(3)`.
//! * NetBSD lacks `explicit_bzero`, `reallocf`, `readpassphrase`,
//!   `expand_number`, the FreeBSD-style `pidfile_*` family, and most
//!   FreeBSD extensions.
//! * OpenBSD lacks the entire NetBSD-family extended `vis(3)` surface
//!   (`nvis`, `svis`, `strsvis`, `strnvisx`, …), `<bsd/stringlist.h>`,
//!   `nlist(3)`, `strnstr`, `reallocf`, `fmtcheck`, and `fgetwln`.
//!
//! The `strnvis` and `strnunvis` functions have two parameter-order
//! conventions, neither of which lines up with what most readers expect:
//!
//! * OpenBSD-order `(dst, src, dlen[, flag])`: used by OpenBSD's native
//!   libc *and* by libbsd on Linux — libbsd's header `#define`s C
//!   callers to `strnvis_netbsd`, but the unversioned export keeps the
//!   OpenBSD signature, and that is what a Rust `extern` block binds to.
//! * NetBSD-order `(dst, dlen, src[, flag])`: used by NetBSD, FreeBSD
//!   (imported from NetBSD's libc-vis), and macOS.
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
//! - **`DEP_BSD_INCLUDE`** — Include paths for libbsd headers, joined with
//!   the platform's `PATH` separator (`:` on Unix). Parse with
//!   [`std::env::split_paths`].
//!
//! - **`DEP_BSD_LIBDIR`** — Library directories, joined the same way.

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
                c"".as_ptr(),
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
    //
    // `link!` coerces a function item to its fn-pointer type and routes
    // the pointer through `core::hint::black_box`, which forces the
    // compiler to emit a relocation against the symbol.  A bare typed
    // `let _: T = sym;` binding is silently optimized away and would
    // not actually exercise the linker.  Variadic and divergent
    // functions use alternative strategies noted inline.
    // -------------------------------------------------------------------

    use core::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_void};

    macro_rules! link {
        ($sym:expr, $ty:ty) => {{
            let f: $ty = $sym;
            core::hint::black_box(f);
        }};
    }

    // <bsd/string.h>
    #[test]
    fn link_string() {
        link!(
            strlcpy,
            unsafe extern "C" fn(*mut c_char, *const c_char, size_t) -> size_t
        );
        link!(
            strlcat,
            unsafe extern "C" fn(*mut c_char, *const c_char, size_t) -> size_t
        );
        link!(strmode, unsafe extern "C" fn(mode_t, *mut c_char));
        #[cfg(not(target_os = "openbsd"))]
        link!(
            strnstr,
            unsafe extern "C" fn(*const c_char, *const c_char, size_t) -> *mut c_char
        );
        #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
        link!(explicit_bzero, unsafe extern "C" fn(*mut c_void, size_t));
    }

    // <bsd/stdlib.h>
    #[test]
    fn link_stdlib() {
        link!(arc4random, unsafe extern "C" fn() -> u32);
        link!(arc4random_buf, unsafe extern "C" fn(*mut c_void, size_t));
        link!(arc4random_uniform, unsafe extern "C" fn(u32) -> u32);
        link!(getprogname, unsafe extern "C" fn() -> *const c_char);
        link!(setprogname, unsafe extern "C" fn(*const c_char));
        link!(
            heapsort,
            unsafe extern "C" fn(
                *mut c_void,
                size_t,
                size_t,
                Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
            ) -> c_int
        );
        link!(
            mergesort,
            unsafe extern "C" fn(
                *mut c_void,
                size_t,
                size_t,
                Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
            ) -> c_int
        );
        link!(
            radixsort,
            unsafe extern "C" fn(*mut *const c_uchar, c_int, *const c_uchar, c_uint) -> c_int
        );
        link!(
            sradixsort,
            unsafe extern "C" fn(*mut *const c_uchar, c_int, *const c_uchar, c_uint) -> c_int
        );
        #[cfg(not(any(target_os = "netbsd", target_os = "openbsd")))]
        link!(
            reallocf,
            unsafe extern "C" fn(*mut c_void, size_t) -> *mut c_void
        );
        #[cfg(not(target_os = "macos"))]
        link!(
            reallocarray,
            unsafe extern "C" fn(*mut c_void, size_t, size_t) -> *mut c_void
        );
        link!(
            strtonum,
            unsafe extern "C" fn(*const c_char, i64, i64, *mut *const c_char) -> i64
        );
        link!(
            getbsize,
            unsafe extern "C" fn(*mut c_int, *mut c_long) -> *mut c_char
        );
    }

    // recallocarray/freezero originate in OpenBSD; libbsd ships them on
    // Linux, but FreeBSD, NetBSD, and macOS don't have them.
    #[test]
    #[cfg(any(target_os = "linux", target_os = "openbsd"))]
    fn link_stdlib_recallocarray_freezero() {
        link!(
            recallocarray,
            unsafe extern "C" fn(*mut c_void, size_t, size_t, size_t) -> *mut c_void
        );
        link!(freezero, unsafe extern "C" fn(*mut c_void, size_t));
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn link_stdlib_linux() {
        link!(arc4random_stir, unsafe extern "C" fn());
        link!(
            arc4random_addrandom,
            unsafe extern "C" fn(*mut c_uchar, c_int)
        );
        link!(
            dehumanize_number,
            unsafe extern "C" fn(*const c_char, *mut i64) -> c_int
        );
    }

    // <bsd/unistd.h>
    #[test]
    fn link_unistd() {
        core::hint::black_box(&raw const optreset);
        link!(
            getmode,
            unsafe extern "C" fn(*const c_void, mode_t) -> mode_t
        );
        link!(setmode, unsafe extern "C" fn(*const c_char) -> *mut c_void);
        #[cfg(not(target_os = "macos"))]
        link!(closefrom, unsafe extern "C" fn(c_int));
        link!(
            getpeereid,
            unsafe extern "C" fn(c_int, *mut uid_t, *mut gid_t) -> c_int
        );
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn link_unistd_linux() {
        link!(
            bsd_getopt,
            unsafe extern "C" fn(c_int, *const *mut c_char, *const c_char) -> c_int
        );
        link!(
            setproctitle_init,
            unsafe extern "C" fn(c_int, *mut *mut c_char, *mut *mut c_char)
        );
    }

    #[test]
    #[cfg(not(target_os = "macos"))]
    fn link_setproctitle() {
        // Variadic: verify linkage by calling with an empty format string.
        unsafe { setproctitle(c"".as_ptr()) }
    }

    // <bsd/stdio.h>
    #[test]
    fn link_stdio() {
        #[cfg(not(target_os = "openbsd"))]
        link!(
            fmtcheck,
            unsafe extern "C" fn(*const c_char, *const c_char) -> *const c_char
        );
        link!(
            fgetln,
            unsafe extern "C" fn(*mut FILE, *mut size_t) -> *mut c_char
        );
        #[allow(clippy::type_complexity)]
        {
            link!(
                funopen,
                unsafe extern "C" fn(
                    *const c_void,
                    Option<unsafe extern "C" fn(*mut c_void, *mut c_char, c_int) -> c_int>,
                    Option<unsafe extern "C" fn(*mut c_void, *const c_char, c_int) -> c_int>,
                    Option<unsafe extern "C" fn(*mut c_void, off_t, c_int) -> off_t>,
                    Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
                ) -> *mut FILE
            );
        }
        link!(fpurge, unsafe extern "C" fn(*mut FILE) -> c_int);
    }

    // <bsd/readpassphrase.h> — missing on NetBSD.
    #[test]
    #[cfg(not(target_os = "netbsd"))]
    fn link_readpassphrase() {
        link!(
            readpassphrase,
            unsafe extern "C" fn(*const c_char, *mut c_char, size_t, c_int) -> *mut c_char
        );
    }

    // <bsd/vis.h> — functions available everywhere we support vis.
    #[test]
    fn link_vis() {
        link!(
            vis,
            unsafe extern "C" fn(*mut c_char, c_int, c_int, c_int) -> *mut c_char
        );
        link!(
            strvis,
            unsafe extern "C" fn(*mut c_char, *const c_char, c_int) -> c_int
        );
        link!(
            stravis,
            unsafe extern "C" fn(*mut *mut c_char, *const c_char, c_int) -> c_int
        );
        link!(
            strvisx,
            unsafe extern "C" fn(*mut c_char, *const c_char, size_t, c_int) -> c_int
        );
        link!(
            strunvis,
            unsafe extern "C" fn(*mut c_char, *const c_char) -> c_int
        );
        link!(
            unvis,
            unsafe extern "C" fn(*mut c_char, c_int, *mut c_int, c_int) -> c_int
        );
    }

    // The "extended" vis family (nvis/svis/snvis/strsvis/strsnvis/strnvisx
    // and friends) is a NetBSD/libbsd extension.  OpenBSD's <vis.h> ships
    // only the small surface tested above.
    #[test]
    #[cfg(not(target_os = "openbsd"))]
    fn link_vis_extended() {
        link!(
            nvis,
            unsafe extern "C" fn(*mut c_char, size_t, c_int, c_int, c_int) -> *mut c_char
        );
        link!(
            svis,
            unsafe extern "C" fn(*mut c_char, c_int, c_int, c_int, *const c_char) -> *mut c_char
        );
        link!(
            snvis,
            unsafe extern "C" fn(
                *mut c_char,
                size_t,
                c_int,
                c_int,
                c_int,
                *const c_char,
            ) -> *mut c_char
        );
        link!(
            strsvis,
            unsafe extern "C" fn(*mut c_char, *const c_char, c_int, *const c_char) -> c_int
        );
        link!(
            strsnvis,
            unsafe extern "C" fn(*mut c_char, size_t, *const c_char, c_int, *const c_char) -> c_int
        );
        link!(
            strnvisx,
            unsafe extern "C" fn(*mut c_char, size_t, *const c_char, size_t, c_int) -> c_int
        );
        link!(
            strenvisx,
            unsafe extern "C" fn(
                *mut c_char,
                size_t,
                *const c_char,
                size_t,
                c_int,
                *mut c_int,
            ) -> c_int
        );
        link!(
            strsvisx,
            unsafe extern "C" fn(*mut c_char, *const c_char, size_t, c_int, *const c_char) -> c_int
        );
        link!(
            strsnvisx,
            unsafe extern "C" fn(
                *mut c_char,
                size_t,
                *const c_char,
                size_t,
                c_int,
                *const c_char,
            ) -> c_int
        );
        link!(
            strsenvisx,
            unsafe extern "C" fn(
                *mut c_char,
                size_t,
                *const c_char,
                size_t,
                c_int,
                *const c_char,
                *mut c_int,
            ) -> c_int
        );
        link!(
            strunvisx,
            unsafe extern "C" fn(*mut c_char, *const c_char, c_int) -> c_int
        );
        link!(
            strnunvisx,
            unsafe extern "C" fn(*mut c_char, size_t, *const c_char, c_int) -> c_int
        );
    }

    // strnvis/strnunvis split into two camps:
    //   * "OpenBSD order" (dst, src, dlen[, flag]) — used by OpenBSD's
    //     native libc *and* by the default versioned symbol exported by
    //     libbsd on Linux (libbsd's header redirects C callers to
    //     `strnvis_netbsd`, but the unversioned export keeps OpenBSD
    //     order — which is what a Rust `extern` block binds to).
    //   * "NetBSD order" (dst, dlen, src[, flag]) — used by NetBSD,
    //     macOS, and FreeBSD (FreeBSD imported NetBSD's libc-vis).
    #[test]
    #[cfg(any(target_os = "linux", target_os = "openbsd"))]
    fn link_vis_strnvis_openbsd_order() {
        link!(
            strnvis,
            unsafe extern "C" fn(*mut c_char, *const c_char, size_t, c_int) -> c_int
        );
        link!(
            strnunvis,
            unsafe extern "C" fn(*mut c_char, *const c_char, size_t) -> ssize_t
        );
    }

    #[test]
    #[cfg(any(target_os = "macos", target_os = "netbsd", target_os = "freebsd"))]
    fn link_vis_strnvis_netbsd_order() {
        link!(
            strnvis,
            unsafe extern "C" fn(*mut c_char, size_t, *const c_char, c_int) -> c_int
        );
        link!(
            strnunvis,
            unsafe extern "C" fn(*mut c_char, size_t, *const c_char) -> c_int
        );
    }

    // <bsd/libutil.h>
    #[test]
    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "netbsd"))]
    fn link_libutil() {
        link!(
            humanize_number,
            unsafe extern "C" fn(*mut c_char, size_t, i64, *const c_char, c_int, c_int) -> c_int
        );
        // expand_number is missing on NetBSD.
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        link!(
            expand_number,
            unsafe extern "C" fn(*const c_char, *mut u64) -> c_int
        );
    }

    // FreeBSD-style pidfile_*.  NetBSD has a different one-function pidfile(3)
    // API in libutil; not bound here.
    #[test]
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    fn link_pidfile() {
        link!(
            pidfile_open,
            unsafe extern "C" fn(*const c_char, mode_t, *mut pid_t) -> *mut pidfh
        );
        link!(pidfile_fileno, unsafe extern "C" fn(*const pidfh) -> c_int);
        link!(pidfile_write, unsafe extern "C" fn(*mut pidfh) -> c_int);
        link!(pidfile_close, unsafe extern "C" fn(*mut pidfh) -> c_int);
        link!(pidfile_remove, unsafe extern "C" fn(*mut pidfh) -> c_int);
    }

    // flopen/flopenat are FreeBSD-specific; not available on NetBSD.
    #[test]
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    fn link_flopen() {
        // Variadic; verify linkage by calling.
        unsafe {
            let fd = flopen(c"/dev/null".as_ptr(), 0);
            if fd >= 0 {
                libc::close(fd);
            }
            // Invalid dirfd — fails immediately, just verifies linkage.
            let _ = flopenat(-1, c"".as_ptr(), 0);
        }
    }

    #[test]
    fn link_fparseln() {
        link!(
            fparseln,
            unsafe extern "C" fn(
                *mut FILE,
                *mut size_t,
                *mut size_t,
                *const [c_char; 3],
                c_int,
            ) -> *mut c_char
        );
    }

    // <bsd/nlist.h> — OpenBSD removed nlist(3) from libc.
    #[test]
    #[cfg(not(any(target_os = "macos", target_os = "openbsd")))]
    fn link_nlist() {
        link!(
            nlist,
            unsafe extern "C" fn(*const c_char, *mut nlist) -> c_int
        );
    }

    // <bsd/stringlist.h> — not present on OpenBSD.
    #[test]
    #[cfg(not(target_os = "openbsd"))]
    fn link_stringlist() {
        link!(sl_init, unsafe extern "C" fn() -> *mut StringList);
        link!(
            sl_add,
            unsafe extern "C" fn(*mut StringList, *mut c_char) -> c_int
        );
        link!(sl_free, unsafe extern "C" fn(*mut StringList, c_int));
        link!(
            sl_find,
            unsafe extern "C" fn(*mut StringList, *const c_char) -> *mut c_char
        );
    }

    #[test]
    #[cfg(target_os = "netbsd")]
    fn link_sl_delete() {
        link!(
            sl_delete,
            unsafe extern "C" fn(*mut StringList, *const c_char, c_int) -> c_int
        );
    }

    // <bsd/timeconv.h>
    #[test]
    #[cfg(target_os = "linux")]
    fn link_timeconv() {
        link!(time32_to_time, unsafe extern "C" fn(i32) -> libc::time_t);
        link!(time_to_time32, unsafe extern "C" fn(libc::time_t) -> i32);
        link!(time64_to_time, unsafe extern "C" fn(i64) -> libc::time_t);
        link!(time_to_time64, unsafe extern "C" fn(libc::time_t) -> i64);
        link!(time_to_long, unsafe extern "C" fn(libc::time_t) -> c_long);
        link!(long_to_time, unsafe extern "C" fn(c_long) -> libc::time_t);
        link!(time_to_int, unsafe extern "C" fn(libc::time_t) -> c_int);
        link!(int_to_time, unsafe extern "C" fn(c_int) -> libc::time_t);
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

    // <bsd/wchar.h> — fgetwln is missing on OpenBSD.
    #[test]
    fn link_wchar() {
        link!(
            wcslcat,
            unsafe extern "C" fn(*mut libc::wchar_t, *const libc::wchar_t, size_t) -> size_t
        );
        link!(
            wcslcpy,
            unsafe extern "C" fn(*mut libc::wchar_t, *const libc::wchar_t, size_t) -> size_t
        );
    }

    #[test]
    #[cfg(not(target_os = "openbsd"))]
    fn link_fgetwln() {
        link!(
            fgetwln,
            unsafe extern "C" fn(*mut FILE, *mut size_t) -> *mut libc::wchar_t
        );
    }

    // <bsd/grp.h>
    #[test]
    fn link_grp() {
        // gid_from_group is missing on macOS.
        #[cfg(not(target_os = "macos"))]
        link!(
            gid_from_group,
            unsafe extern "C" fn(*const c_char, *mut gid_t) -> c_int
        );
        link!(
            group_from_gid,
            unsafe extern "C" fn(gid_t, c_int) -> *const c_char
        );
    }

    // <bsd/pwd.h>
    #[test]
    fn link_pwd() {
        // uid_from_user is missing on macOS.
        #[cfg(not(target_os = "macos"))]
        link!(
            uid_from_user,
            unsafe extern "C" fn(*const c_char, *mut uid_t) -> c_int
        );
        link!(
            user_from_uid,
            unsafe extern "C" fn(uid_t, c_int) -> *const c_char
        );
    }

    // misc
    #[test]
    fn link_inet() {
        link!(
            inet_net_pton,
            unsafe extern "C" fn(c_int, *const c_char, *mut c_void, size_t) -> c_int
        );
    }
}
