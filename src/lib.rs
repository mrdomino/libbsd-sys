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
}
