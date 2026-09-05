# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.3](https://github.com/mrdomino/libbsd-sys/compare/v0.3.2...v0.3.3) - 2026-09-05

### Other

- Update README.md
- correct minor version in readme

## [0.3.2](https://github.com/mrdomino/libbsd-sys/compare/v0.3.1...v0.3.2) - 2026-08-11

### Fixed

- declare honest minimum dependency versions ([#53](https://github.com/mrdomino/libbsd-sys/pull/53))

## [0.3.1](https://github.com/mrdomino/libbsd-sys/compare/v0.3.0...v0.3.1) - 2026-08-11

### Added

- supply readpassphrase on NetBSD from vendored C ([#48](https://github.com/mrdomino/libbsd-sys/pull/48))

## [0.3.0](https://github.com/mrdomino/libbsd-sys/compare/v0.2.1...v0.3.0) - 2026-06-10

### Changed

- Reworked the `vis(3)` bindings around the two competing parameter-order
  conventions: OpenBSD-order `(dst, src, dlen[, flag])`, used by OpenBSD's libc
  and by libbsd's unversioned export on Linux, and NetBSD-order
  `(dst, dlen, src[, flag])`, used by NetBSD, FreeBSD, and macOS. Mixing the two
  silently reads a `size_t` as a pointer, so the bindings are now `#[cfg]`-gated
  per platform ([#27]).
- `VIS_DQ`, `VIS_GLOB`, `VIS_ALL`, and `UNVIS_END` now carry OpenBSD's numeric
  values on OpenBSD ([#27]).
- **Breaking for downstream build scripts:** `DEP_BSD_INCLUDE` and
  `DEP_BSD_LIBDIR` are emitted as a single PATH-style separated list, parseable
  with `std::env::split_paths`, instead of one line per path ([#27]).
- Apple platform detection uses `target_vendor` rather than `target_os`, so iOS,
  tvOS, watchOS, and visionOS are handled like macOS ([#27]).

### Removed

- **Breaking:** the NetBSD-family `vis(3)` extensions (`nvis`, `svis`, `strsvis`,
  and friends), `fgetwln`, `nlist`, and the stringlist functions are no longer
  declared on OpenBSD, which does not have them ([#27]).

### Fixed

- A failed `pkg-config` probe no longer panics the build script. It falls
  through to emitting a bare `cargo:rustc-link-lib=bsd` with no search path, so
  downstream `cargo check` and `cargo clippy` work with `libbsd-dev` absent;
  builds that actually link fail at the linker with `ld: cannot find -lbsd`,
  which is more actionable than a build-script panic ([#13]).
- `LIBBSD_NO_PKG_CONFIG=1` again emits `cargo:rustc-link-lib=bsd` without a
  search path, as in 0.1.x and 0.2.0, reverting the 0.2.1 change ([#13]).

## [0.2.1](https://github.com/mrdomino/libbsd-sys/compare/v0.2.0...v0.2.1) - 2026-04-12

### Fixed

- First attempt at making a failed `pkg-config` probe non-fatal, which also
  changed `LIBBSD_NO_PKG_CONFIG=1` to emit nothing at all ([#12]). Both were
  superseded in 0.3.0; the release notes for this version overstated the fix.

## [0.2.0](https://github.com/mrdomino/libbsd-sys/compare/v0.1.3...v0.2.0) - 2026-04-12

### Added

- Environment overrides for cross-compilation and for CI without pkg-config:
  `LIBBSD_NO_PKG_CONFIG`, `LIBBSD_LIB_DIR`, `LIBBSD_INCLUDE_DIR`, and
  `LIBBSD_STATIC` ([#11]).
- `DOCS_RS` detection, so docs.rs builds skip the link step ([#11]).
- Link smoke tests covering every extern symbol, roughly 90 bindings ([#11]).

### Removed

- **Breaking:** `flopen` and `flopenat` are no longer declared on NetBSD. They
  were never present there; the new link tests caught the bad declaration
  ([#11]).

## [0.1.3](https://github.com/mrdomino/libbsd-sys/compare/v0.1.2...v0.1.3) - 2026-03-30

### Changed

- On Windows the crate now exposes an empty public API instead of failing to
  compile, so merely depending on it transitively no longer breaks the build
  ([#10]).

## [0.1.2](https://github.com/mrdomino/libbsd-sys/compare/v0.1.1...v0.1.2) - 2026-03-26

### Changed

- Dropped the explicit Windows panic in favor of graceful handling ([#9]).

## [0.1.1](https://github.com/mrdomino/libbsd-sys/compare/v0.1.0...v0.1.1) - 2026-03-26

### Other

- Publish to crates.io from CI ([#8]).

## [0.1.0](https://github.com/mrdomino/libbsd-sys/releases/tag/v0.1.0) - 2026-03-26

Initial release: raw FFI bindings to libbsd on Linux, and to the corresponding
libc symbols on FreeBSD, OpenBSD, NetBSD, and macOS.

### Added

- `static` and `overlay` features, both Linux-only ([#5]).
- Hand-written documentation for the bindings ([#6]).

### Changed

- All `extern "C"` blocks are `unsafe`; MSRV set to 1.85 ([#7]).
- Minimum libbsd raised from 0.8 to 0.11 ([#3]).

[#3]: https://github.com/mrdomino/libbsd-sys/pull/3
[#5]: https://github.com/mrdomino/libbsd-sys/pull/5
[#6]: https://github.com/mrdomino/libbsd-sys/pull/6
[#7]: https://github.com/mrdomino/libbsd-sys/pull/7
[#8]: https://github.com/mrdomino/libbsd-sys/pull/8
[#9]: https://github.com/mrdomino/libbsd-sys/pull/9
[#10]: https://github.com/mrdomino/libbsd-sys/pull/10
[#11]: https://github.com/mrdomino/libbsd-sys/pull/11
[#12]: https://github.com/mrdomino/libbsd-sys/pull/12
[#13]: https://github.com/mrdomino/libbsd-sys/pull/13
[#27]: https://github.com/mrdomino/libbsd-sys/pull/27
