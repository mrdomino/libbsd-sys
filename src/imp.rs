use core::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_void};

// Re-export libc types used in signatures.
pub use libc::{FILE, gid_t, mode_t, off_t, pid_t, size_t, ssize_t, uid_t};

// ---------------------------------------------------------------------------
// <bsd/string.h>
// ---------------------------------------------------------------------------

unsafe extern "C" {
    pub fn strlcpy(dst: *mut c_char, src: *const c_char, siz: size_t) -> size_t;
    pub fn strlcat(dst: *mut c_char, src: *const c_char, siz: size_t) -> size_t;
    pub fn strnstr(s: *const c_char, find: *const c_char, slen: size_t) -> *mut c_char;
    pub fn strmode(mode: mode_t, str_: *mut c_char);
    pub fn explicit_bzero(buf: *mut c_void, len: size_t);
}

// ---------------------------------------------------------------------------
// <bsd/stdlib.h>
// ---------------------------------------------------------------------------

unsafe extern "C" {
    pub fn arc4random() -> u32;
    pub fn arc4random_buf(buf: *mut c_void, n: size_t);
    pub fn arc4random_uniform(upper_bound: u32) -> u32;
    #[cfg(target_os = "linux")]
    pub fn arc4random_stir();
    #[cfg(target_os = "linux")]
    pub fn arc4random_addrandom(dat: *mut c_uchar, datlen: c_int);

    #[cfg(target_os = "linux")]
    pub fn dehumanize_number(str_: *const c_char, size: *mut i64) -> c_int;

    pub fn getprogname() -> *const c_char;
    pub fn setprogname(name: *const c_char);

    pub fn heapsort(
        base: *mut c_void,
        nmemb: size_t,
        size: size_t,
        cmp: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    ) -> c_int;
    pub fn mergesort(
        base: *mut c_void,
        nmemb: size_t,
        size: size_t,
        cmp: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    ) -> c_int;
    pub fn radixsort(
        base: *mut *const c_uchar,
        nmemb: c_int,
        table: *const c_uchar,
        endbyte: c_uint,
    ) -> c_int;
    pub fn sradixsort(
        base: *mut *const c_uchar,
        nmemb: c_int,
        table: *const c_uchar,
        endbyte: c_uint,
    ) -> c_int;

    pub fn reallocf(ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    #[cfg(not(target_os = "macos"))]
    pub fn recallocarray(
        ptr: *mut c_void,
        oldnmemb: size_t,
        nmemb: size_t,
        size: size_t,
    ) -> *mut c_void;
    #[cfg(not(target_os = "macos"))]
    pub fn freezero(ptr: *mut c_void, size: size_t);

    pub fn strtonum(
        nptr: *const c_char,
        minval: i64,
        maxval: i64,
        errstr: *mut *const c_char,
    ) -> i64;

    pub fn getbsize(headerlenp: *mut c_int, blocksizep: *mut c_long) -> *mut c_char;
}

// ---------------------------------------------------------------------------
// <bsd/unistd.h>
// ---------------------------------------------------------------------------

unsafe extern "C" {
    pub static mut optreset: c_int;

    #[cfg(target_os = "linux")]
    pub fn bsd_getopt(argc: c_int, argv: *const *mut c_char, shortopts: *const c_char) -> c_int;

    pub fn getmode(set: *const c_void, mode: mode_t) -> mode_t;
    pub fn setmode(mode_str: *const c_char) -> *mut c_void;

    pub fn closefrom(lowfd: c_int);

    #[cfg(target_os = "linux")]
    pub fn setproctitle_init(argc: c_int, argv: *mut *mut c_char, envp: *mut *mut c_char);
    #[cfg(not(target_os = "macos"))]
    pub fn setproctitle(fmt: *const c_char, ...);

    pub fn getpeereid(s: c_int, euid: *mut uid_t, egid: *mut gid_t) -> c_int;
}

// ---------------------------------------------------------------------------
// <bsd/stdio.h>
// ---------------------------------------------------------------------------

unsafe extern "C" {
    pub fn fmtcheck(f1: *const c_char, f2: *const c_char) -> *const c_char;

    pub fn fgetln(fp: *mut FILE, lenp: *mut size_t) -> *mut c_char;

    pub fn funopen(
        cookie: *const c_void,
        readfn: Option<unsafe extern "C" fn(*mut c_void, *mut c_char, c_int) -> c_int>,
        writefn: Option<unsafe extern "C" fn(*mut c_void, *const c_char, c_int) -> c_int>,
        seekfn: Option<unsafe extern "C" fn(*mut c_void, off_t, c_int) -> off_t>,
        closefn: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    ) -> *mut FILE;

    pub fn fpurge(fp: *mut FILE) -> c_int;
}

// ---------------------------------------------------------------------------
// <bsd/readpassphrase.h>
// ---------------------------------------------------------------------------

pub const RPP_ECHO_OFF: c_int = 0x00;
pub const RPP_ECHO_ON: c_int = 0x01;
pub const RPP_REQUIRE_TTY: c_int = 0x02;
pub const RPP_FORCELOWER: c_int = 0x04;
pub const RPP_FORCEUPPER: c_int = 0x08;
pub const RPP_SEVENBIT: c_int = 0x10;
pub const RPP_STDIN: c_int = 0x20;

unsafe extern "C" {
    pub fn readpassphrase(
        prompt: *const c_char,
        buf: *mut c_char,
        bufsiz: size_t,
        flags: c_int,
    ) -> *mut c_char;
}

// ---------------------------------------------------------------------------
// <bsd/vis.h>
// ---------------------------------------------------------------------------

pub const VIS_OCTAL: c_int = 0x0001;
pub const VIS_CSTYLE: c_int = 0x0002;
pub const VIS_SP: c_int = 0x0004;
pub const VIS_TAB: c_int = 0x0008;
pub const VIS_NL: c_int = 0x0010;
pub const VIS_WHITE: c_int = VIS_SP | VIS_TAB | VIS_NL;
pub const VIS_SAFE: c_int = 0x0020;
pub const VIS_DQ: c_int = 0x8000;
pub const VIS_NOSLASH: c_int = 0x0040;
pub const VIS_HTTP1808: c_int = 0x0080;
pub const VIS_HTTPSTYLE: c_int = 0x0080;
pub const VIS_MIMESTYLE: c_int = 0x0100;
pub const VIS_HTTP1866: c_int = 0x0200;
pub const VIS_NOESCAPE: c_int = 0x0400;
pub const VIS_GLOB: c_int = 0x1000;
pub const VIS_SHELL: c_int = 0x2000;
pub const VIS_META: c_int = VIS_WHITE | VIS_GLOB | VIS_SHELL;
pub const VIS_NOLOCALE: c_int = 0x4000;

pub const UNVIS_VALID: c_int = 1;
pub const UNVIS_VALIDPUSH: c_int = 2;
pub const UNVIS_NOCHAR: c_int = 3;
pub const UNVIS_SYNBAD: c_int = -1;
pub const UNVIS_ERROR: c_int = -2;
pub const UNVIS_END: c_int = 0x0800;

unsafe extern "C" {
    pub fn vis(dst: *mut c_char, c: c_int, flag: c_int, nextc: c_int) -> *mut c_char;
    pub fn nvis(dst: *mut c_char, dlen: size_t, c: c_int, flag: c_int, nextc: c_int)
    -> *mut c_char;

    pub fn svis(
        dst: *mut c_char,
        c: c_int,
        flag: c_int,
        nextc: c_int,
        extra: *const c_char,
    ) -> *mut c_char;
    pub fn snvis(
        dst: *mut c_char,
        dlen: size_t,
        c: c_int,
        flag: c_int,
        nextc: c_int,
        extra: *const c_char,
    ) -> *mut c_char;

    pub fn strvis(dst: *mut c_char, src: *const c_char, flag: c_int) -> c_int;
    pub fn stravis(dst: *mut *mut c_char, src: *const c_char, flag: c_int) -> c_int;
    // NB: strnvis has different parameter order depending on the platform's
    // convention: FreeBSD (and libbsd) put src before dlen, while NetBSD
    // (and macOS/OpenBSD) put dlen before src.
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub fn strnvis(dst: *mut c_char, src: *const c_char, dlen: size_t, flag: c_int) -> c_int;
    #[cfg(any(target_os = "macos", target_os = "netbsd", target_os = "openbsd"))]
    pub fn strnvis(dst: *mut c_char, dlen: size_t, src: *const c_char, flag: c_int) -> c_int;

    pub fn strsvis(
        dst: *mut c_char,
        src: *const c_char,
        flag: c_int,
        extra: *const c_char,
    ) -> c_int;
    pub fn strsnvis(
        dst: *mut c_char,
        dlen: size_t,
        src: *const c_char,
        flag: c_int,
        extra: *const c_char,
    ) -> c_int;

    pub fn strvisx(dst: *mut c_char, src: *const c_char, len: size_t, flag: c_int) -> c_int;
    pub fn strnvisx(
        dst: *mut c_char,
        dlen: size_t,
        src: *const c_char,
        len: size_t,
        flag: c_int,
    ) -> c_int;
    pub fn strenvisx(
        dst: *mut c_char,
        dlen: size_t,
        src: *const c_char,
        len: size_t,
        flag: c_int,
        cerr_ptr: *mut c_int,
    ) -> c_int;

    pub fn strsvisx(
        dst: *mut c_char,
        src: *const c_char,
        len: size_t,
        flag: c_int,
        extra: *const c_char,
    ) -> c_int;
    pub fn strsnvisx(
        dst: *mut c_char,
        dlen: size_t,
        src: *const c_char,
        len: size_t,
        flag: c_int,
        extra: *const c_char,
    ) -> c_int;
    pub fn strsenvisx(
        dst: *mut c_char,
        dlen: size_t,
        src: *const c_char,
        len: size_t,
        flag: c_int,
        extra: *const c_char,
        cerr_ptr: *mut c_int,
    ) -> c_int;

    pub fn strunvis(dst: *mut c_char, src: *const c_char) -> c_int;
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub fn strnunvis(dst: *mut c_char, src: *const c_char, dlen: size_t) -> ssize_t;
    #[cfg(any(target_os = "macos", target_os = "netbsd", target_os = "openbsd"))]
    pub fn strnunvis(dst: *mut c_char, dlen: size_t, src: *const c_char) -> c_int;

    pub fn strunvisx(dst: *mut c_char, src: *const c_char, flag: c_int) -> c_int;
    pub fn strnunvisx(dst: *mut c_char, dlen: size_t, src: *const c_char, flag: c_int) -> c_int;

    pub fn unvis(cp: *mut c_char, c: c_int, apts: *mut c_int, flag: c_int) -> c_int;
}

// ---------------------------------------------------------------------------
// <bsd/libutil.h>
// ---------------------------------------------------------------------------

#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "netbsd"))]
pub const HN_DECIMAL: c_int = 0x01;
#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "netbsd"))]
pub const HN_NOSPACE: c_int = 0x02;
#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "netbsd"))]
pub const HN_B: c_int = 0x04;
#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "netbsd"))]
pub const HN_DIVISOR_1000: c_int = 0x08;
#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "netbsd"))]
pub const HN_IEC_PREFIXES: c_int = 0x10;
#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "netbsd"))]
pub const HN_GETSCALE: c_int = 0x10;
#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "netbsd"))]
pub const HN_AUTOSCALE: c_int = 0x20;

pub const FPARSELN_UNESCESC: c_int = 0x01;
pub const FPARSELN_UNESCCONT: c_int = 0x02;
pub const FPARSELN_UNESCCOMM: c_int = 0x04;
pub const FPARSELN_UNESCREST: c_int = 0x08;
pub const FPARSELN_UNESCALL: c_int = 0x0f;

#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "netbsd"))]
#[repr(C)]
pub struct pidfh {
    _opaque: [u8; 0],
}

#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "netbsd"))]
unsafe extern "C" {
    pub fn humanize_number(
        buf: *mut c_char,
        len: size_t,
        bytes: i64,
        suffix: *const c_char,
        scale: c_int,
        flags: c_int,
    ) -> c_int;
    pub fn expand_number(buf: *const c_char, num: *mut u64) -> c_int;

    pub fn pidfile_open(path: *const c_char, mode: mode_t, pidptr: *mut pid_t) -> *mut pidfh;
    pub fn pidfile_fileno(pfh: *const pidfh) -> c_int;
    pub fn pidfile_write(pfh: *mut pidfh) -> c_int;
    pub fn pidfile_close(pfh: *mut pidfh) -> c_int;
    pub fn pidfile_remove(pfh: *mut pidfh) -> c_int;
}

// flopen/flopenat are FreeBSD-specific; not available on NetBSD.
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
unsafe extern "C" {
    pub fn flopen(path: *const c_char, flags: c_int, ...) -> c_int;
    pub fn flopenat(dirfd: c_int, path: *const c_char, flags: c_int, ...) -> c_int;
}

unsafe extern "C" {
    pub fn fparseln(
        fp: *mut FILE,
        size: *mut size_t,
        lineno: *mut size_t,
        delim: *const [c_char; 3],
        flags: c_int,
    ) -> *mut c_char;
}

// ---------------------------------------------------------------------------
// <bsd/nlist.h>
// ---------------------------------------------------------------------------

// On macOS, struct nlist has a different layout (Mach-O format).
#[cfg(not(target_os = "macos"))]
#[repr(C)]
pub struct nlist {
    pub n_name: *mut c_char,
    pub n_type: u8,
    pub n_other: c_char,
    pub n_desc: i16,
    pub n_value: c_ulong,
}

pub use core::ffi::c_ulong;

pub const N_UNDF: u8 = 0x00;
pub const N_ABS: u8 = 0x02;
pub const N_TEXT: u8 = 0x04;
pub const N_DATA: u8 = 0x06;
pub const N_BSS: u8 = 0x08;
pub const N_INDR: u8 = 0x0a;
pub const N_SIZE: u8 = 0x0c;
pub const N_COMM: u8 = 0x12;
pub const N_SETA: u8 = 0x14;
pub const N_SETT: u8 = 0x16;
pub const N_SETD: u8 = 0x18;
pub const N_SETB: u8 = 0x1a;
pub const N_SETV: u8 = 0x1c;
pub const N_FN: u8 = 0x1e;
pub const N_WARN: u8 = 0x1e;
pub const N_EXT: u8 = 0x01;
pub const N_TYPE: u8 = 0x1e;
pub const N_STAB: u8 = 0xe0;

#[cfg(not(target_os = "macos"))]
unsafe extern "C" {
    pub fn nlist(filename: *const c_char, list: *mut nlist) -> c_int;
}

// ---------------------------------------------------------------------------
// <bsd/stringlist.h>
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct StringList {
    pub sl_str: *mut *mut c_char,
    pub sl_max: size_t,
    pub sl_cur: size_t,
}

unsafe extern "C" {
    pub fn sl_init() -> *mut StringList;
    pub fn sl_add(sl: *mut StringList, item: *mut c_char) -> c_int;
    pub fn sl_free(sl: *mut StringList, freel: c_int);
    pub fn sl_find(sl: *mut StringList, name: *const c_char) -> *mut c_char;
    #[cfg(target_os = "linux")]
    pub fn sl_delete(sl: *mut StringList, name: *const c_char, freel: c_int) -> c_int;
}

// ---------------------------------------------------------------------------
// <bsd/timeconv.h>
// ---------------------------------------------------------------------------

#[cfg(target_os = "linux")]
unsafe extern "C" {
    #[link_name = "_time32_to_time"]
    pub fn time32_to_time(t32: i32) -> libc::time_t;
    #[link_name = "_time_to_time32"]
    pub fn time_to_time32(t: libc::time_t) -> i32;
    #[link_name = "_time64_to_time"]
    pub fn time64_to_time(t64: i64) -> libc::time_t;
    #[link_name = "_time_to_time64"]
    pub fn time_to_time64(t: libc::time_t) -> i64;
    #[link_name = "_time_to_long"]
    pub fn time_to_long(t: libc::time_t) -> c_long;
    #[link_name = "_long_to_time"]
    pub fn long_to_time(tlong: c_long) -> libc::time_t;
    #[link_name = "_time_to_int"]
    pub fn time_to_int(t: libc::time_t) -> c_int;
    #[link_name = "_int_to_time"]
    pub fn int_to_time(tint: c_int) -> libc::time_t;
}

// ---------------------------------------------------------------------------
// <bsd/err.h>  (non-variadic subset)
// ---------------------------------------------------------------------------

unsafe extern "C" {
    pub fn warnc(code: c_int, format: *const c_char, ...);
    pub fn errc(status: c_int, code: c_int, format: *const c_char, ...) -> !;
}

// ---------------------------------------------------------------------------
// <bsd/wchar.h>
// ---------------------------------------------------------------------------

unsafe extern "C" {
    pub fn fgetwln(stream: *mut FILE, len: *mut size_t) -> *mut libc::wchar_t;
    pub fn wcslcat(dst: *mut libc::wchar_t, src: *const libc::wchar_t, size: size_t) -> size_t;
    pub fn wcslcpy(dst: *mut libc::wchar_t, src: *const libc::wchar_t, size: size_t) -> size_t;
}

// ---------------------------------------------------------------------------
// <bsd/grp.h>
// ---------------------------------------------------------------------------

unsafe extern "C" {
    pub fn gid_from_group(name: *const c_char, gid: *mut gid_t) -> c_int;
    pub fn group_from_gid(gid: gid_t, nosuchgroup: c_int) -> *const c_char;
}

// ---------------------------------------------------------------------------
// <bsd/pwd.h>
// ---------------------------------------------------------------------------

unsafe extern "C" {
    pub fn uid_from_user(name: *const c_char, uid: *mut uid_t) -> c_int;
    pub fn user_from_uid(uid: uid_t, nosuchuser: c_int) -> *const c_char;
}

// ---------------------------------------------------------------------------
// Misc functions found in libbsd without a dedicated header
// ---------------------------------------------------------------------------

unsafe extern "C" {
    pub fn inet_net_pton(af: c_int, src: *const c_char, dst: *mut c_void, size: size_t) -> c_int;
}
