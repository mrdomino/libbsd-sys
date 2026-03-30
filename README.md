# libbsd-sys - raw FFI bindings to libbsd

_**Caveat non emptor:** This crate is AI-generated, perhaps even “vibe-coded,” and free as in “you get what you pay for.”_

The readme, though — that’s by me (including the em-dashes.)

I… requested that this code be written… in order to scratch a very specific itch: I wanted to use [`readpassphrase(3)`][0] in [a different crate][1], but to be able to pull my implementation from libbsd or Darwin/BSD libc. As there was not yet a `libbsd-sys`, I decided to make one. As it was a pretty mechanical task, and as much of the software world is going this way anyway —as though entering an accretion disk around a singularity, or perhaps just circling a drain— I decided to ask Claude to help me.

I will endeavor, with the full force of somebody not getting compensated at all for this and working on it on occasional evenings and weekends, to resolve any bugs and implement any reasonable feature requests.

I do not mean to be squatting on this highly valuable crate name with slop. I do not think this is slop-grade work; I have made some effort to at least check that the code compiles and matches the relevant APIs on the relevant platforms. That said, if a trustworthy person were to come along with a human-generated alternative of reasonable quality, I am quite open to ceding the name.

## Usage
Add the following to your `Cargo.toml`:
```toml
[target.'cfg(not(target_os = "windows"))'.dependencies]
libbsd-sys = "0.1"
```

## Features
On Linux, the following crate features are available:
* `static` requests static linkage of `libbsd`.
* `overlay` requests `libbsd-overlay` instead of `libbsd`, so that downstream crates that compile C code can use plain `<string.h>` instead of `<bsd/string.h>`.

On non-Linux, these features are no-ops.

## Requirements
On Linux, [libbsd][2] is required. Usually the package will be named something like `libbsd-dev` or `libbsd-devel`. If you do not use the `static` feature, then your users will also have to have the non-devel `libbsd` package installed.

We depend on at least libbsd 0.11.

[0]: https://man.openbsd.org/readpassphrase.3 "readpassphrase — get a passphrase from the user"
[1]: https://github.com/mrdomino/readpassphrase-3 "mrdomino/readpassphrase-3: like left-pad, but for passphrases"
[2]: https://libbsd.freedesktop.org/wiki/ "libbsd"
