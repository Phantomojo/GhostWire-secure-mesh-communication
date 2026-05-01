Crate getrandom Copy item path

<span class="sub-heading"><a href="../src/getrandom/lib.rs.html#2-138" class="src">Source</a>
</span>

Expand description

# <a href="#getrandom-systems-random-number-generator"
class="doc-anchor">§</a>getrandom: system’s random number generator

[![Build
Status](https://github.com/rust-random/getrandom/actions/workflows/tests.yml/badge.svg?branch=master)](https://github.com/rust-random/getrandom/actions?query=branch:master)
[![Crate](https://img.shields.io/crates/v/getrandom)](https://crates.io/crates/getrandom)
[![Documentation](https://docs.rs/getrandom/badge.svg)](https://docs.rs/getrandom)
[![Dependency
Status](https://deps.rs/repo/github/rust-random/getrandom/status.svg)](https://deps.rs/repo/github/rust-random/getrandom)
[![Downloads](https://img.shields.io/crates/d/getrandom)](https://crates.io/crates/getrandom)
[![License](https://img.shields.io/crates/l/getrandom)](https://github.com/rust-random/getrandom/blob/master/LICENSE-MIT)

`getrandom` is a Rust library for retrieving random data from
(operating) system sources.

It is assumed that the system always provides high-quality,
cryptographically secure random data, ideally backed by hardware entropy
sources. This crate derives its name from the Linux `getrandom` syscall
but is cross-platform, roughly supporting the same set of platforms as
Rust’s `std` library.

This is a low-level API. Most users should prefer using a higher-level
random-number library like [`rand`](https://crates.io/crates/rand).

## <a href="#usage" class="doc-anchor">§</a>Usage

Add the `getrandom` dependency to your `Cargo.toml` file:

``` language-toml
[dependencies]
getrandom = "0.3"
```

Then invoke the `fill` function on a byte buffer to fill it with random
data:

``` {.rust .rust-example-rendered}
fn get_random_u128() -> Result<u128, getrandom::Error> {
    let mut buf = [0u8; 16];
    getrandom::fill(&mut buf)?;
    Ok(u128::from_ne_bytes(buf))
}
```

## <a href="#supported-targets" class="doc-anchor">§</a>Supported targets

| Target | Target Triple | Implementation |
|----|----|----|
| Linux, Android | `*‑linux‑*` | [`getrandom`](https://manned.org/getrandom.2) system call if available, otherwise [`/dev/urandom`](https://manned.org/urandom.4) after successfully polling `/dev/random` |
| Windows 10+ | `*‑windows‑*` | [`ProcessPrng`](https://learn.microsoft.com/en-us/windows/win32/seccng/processprng) on Rust 1.78+, [`RtlGenRandom`](https://learn.microsoft.com/en-us/windows/win32/api/ntsecapi/nf-ntsecapi-rtlgenrandom) otherwise |
| Windows 7, 8 | `*-win7‑windows‑*` | [`RtlGenRandom`](https://learn.microsoft.com/en-us/windows/win32/api/ntsecapi/nf-ntsecapi-rtlgenrandom) |
| macOS | `*‑apple‑darwin` | [`getentropy`](https://www.unix.com/man-page/mojave/2/getentropy/) |
| iOS, tvOS, watchOS | `*‑apple‑{ios,tvos,watchos}` | [`CCRandomGenerateBytes`](https://opensource.apple.com/source/CommonCrypto/CommonCrypto-60074/include/CommonRandom.h.auto.html) |
| FreeBSD | `*‑freebsd` | [`getrandom`](https://www.freebsd.org/cgi/man.cgi?query=getrandom&manpath=FreeBSD+12.0-stable) |
| OpenBSD | `*‑openbsd` | [`getentropy`](https://man.openbsd.org/getentropy.2) |
| NetBSD | `*‑netbsd` | [`getrandom`](https://man.netbsd.org/getrandom.2) if available, otherwise [`kern.arandom`](https://man.netbsd.org/sysctl.7) |
| Dragonfly BSD | `*‑dragonfly` | [`getrandom`](https://leaf.dragonflybsd.org/cgi/web-man?command=getrandom) |
| Solaris | `*‑solaris` | [`getrandom`](https://docs.oracle.com/cd/E88353_01/html/E37841/getrandom-2.html) with `GRND_RANDOM` |
| illumos | `*‑illumos` | [`getrandom`](https://illumos.org/man/2/getrandom) |
| Fuchsia OS | `*‑fuchsia` | [`cprng_draw`](https://fuchsia.dev/fuchsia-src/zircon/syscalls/cprng_draw) |
| Redox | `*‑redox` | `/dev/urandom` |
| Haiku | `*‑haiku` | `/dev/urandom` (identical to `/dev/random`) |
| Hermit | `*-hermit` | [`sys_read_entropy`](https://github.com/hermit-os/kernel/blob/315f58ff5efc81d9bf0618af85a59963ff55f8b1/src/syscalls/entropy.rs#L47-L55) |
| Hurd | `*-hurd-*` | [`getrandom`](https://www.gnu.org/software/libc/manual/html_mono/libc.html#index-getrandom) |
| SGX | `x86_64‑*‑sgx` | [`RDRAND`](https://software.intel.com/en-us/articles/intel-digital-random-number-generator-drng-software-implementation-guide) |
| VxWorks | `*‑wrs‑vxworks‑*` | `randABytes` after checking entropy pool initialization with `randSecure` |
| Emscripten | `*‑emscripten` | [`getentropy`](https://github.com/emscripten-core/emscripten/pull/12240) |
| WASI 0.1 | `wasm32‑wasip1` | [`random_get`](https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#-random_getbuf-pointeru8-buf_len-size---errno) |
| WASI 0.2 | `wasm32‑wasip2` | [`get-random-u64`](https://github.com/WebAssembly/WASI/blob/v0.2.1/wasip2/random/random.wit#L23-L28) |
| SOLID | `*-kmc-solid_*` | `SOLID_RNG_SampleRandomBytes` |
| Nintendo 3DS | `*-nintendo-3ds` | [`getrandom`](https://github.com/rust3ds/shim-3ds/commit/b01d2568836dea2a65d05d662f8e5f805c64389d) |
| ESP-IDF | `*‑espidf` | [`esp_fill_random`](https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/random.html#functions) WARNING: see “Early Boot” section below |
| PS Vita | `*-vita-*` | [`getentropy`](https://github.com/vitasdk/newlib/blob/2d869fe47aaf02b8e52d04e9a2b79d5b210fd016/newlib/libc/sys/vita/getentropy.c) |
| QNX Neutrino | `*‑nto-qnx*` | [`/dev/urandom`](https://www.qnx.com/developers/docs/7.1/index.html#com.qnx.doc.neutrino.utilities/topic/r/random.html) (identical to `/dev/random`) |
| AIX | `*-ibm-aix` | [`/dev/urandom`](https://www.ibm.com/docs/en/aix/7.3?topic=files-random-urandom-devices) |
| Cygwin | `*-cygwin` | [`getrandom`](https://github.com/cygwin/cygwin/blob/main/winsup/cygwin/libc/getentropy.cc) (based on [`RtlGenRandom`](https://learn.microsoft.com/en-us/windows/win32/api/ntsecapi/nf-ntsecapi-rtlgenrandom)) |

Pull Requests that add support for new targets to `getrandom` are always
welcome.

### <a href="#opt-in-backends" class="doc-anchor">§</a>Opt-in backends

`getrandom` also provides optional (opt-in) backends, which allow users
to customize the source of randomness based on their specific needs:

| Backend name | Target | Target Triple | Implementation |
|----|----|----|----|
| `linux_getrandom` | Linux, Android | `*‑linux‑*` | [`getrandom`](https://manned.org/getrandom.2) system call (without `/dev/urandom` fallback). Bumps minimum supported Linux kernel version to 3.17 and Android API level to 23 (Marshmallow). |
| `linux_raw` | Linux, Android | `*‑linux‑*` | Same as `linux_getrandom`, but uses raw `asm!`-based syscalls instead of `libc`. |
| `rdrand` | x86, x86-64 | `x86_64-*`, `i686-*` | [`RDRAND`](https://software.intel.com/en-us/articles/intel-digital-random-number-generator-drng-software-implementation-guide) instruction |
| `rndr` | AArch64 | `aarch64-*` | [`RNDR`](https://developer.arm.com/documentation/ddi0601/2024-06/AArch64-Registers/RNDR--Random-Number) register |
| `wasm_js` | Web Browser, Node.js | `wasm32‑unknown‑unknown`, `wasm32v1-none` | [`Crypto.getRandomValues`](https://www.w3.org/TR/WebCryptoAPI/#Crypto-method-getRandomValues). Enabled by the `wasm_js` feature ([see below](#webassembly-support)). |
| `efi_rng` | UEFI | `*-unknown‑uefi` | [`EFI_RNG_PROTOCOL`](https://uefi.org/specs/UEFI/2.10/37_Secure_Technologies.html#efi-rng-protocol) with `EFI_RNG_ALGORITHM_RAW` (requires `std` and Nightly compiler) |
| `windows_legacy` | Windows | `*-windows-*` | [`RtlGenRandom`](https://learn.microsoft.com/en-us/windows/win32/api/ntsecapi/nf-ntsecapi-rtlgenrandom) |
| `custom` | All targets | `*` | User-provided custom implementation (see [custom backend](#custom-backend)) |
| `unsupported` | All targets | `*` | Always returns `Err(Error::UNSUPPORTED)` (see [unsupported backend](#unsupported-backend)) |

Opt-in backends can be enabled using the `getrandom_backend`
configuration flag. The flag can be set either by specifying the
`rustflags` field in
[`.cargo/config.toml`](https://doc.rust-lang.org/cargo/reference/config.html):

``` language-toml
# It's recommended to set the flag on a per-target basis:
[target.wasm32-unknown-unknown]
rustflags = ['--cfg', 'getrandom_backend="wasm_js"']
```

Or by using the `RUSTFLAGS` environment variable:

``` language-sh
RUSTFLAGS='--cfg getrandom_backend="linux_getrandom"' cargo build
```

Enabling an opt-in backend will replace the backend used by default.
Doing this for an incorrect target (e.g. using `linux_getrandom` while
compiling for a Windows target) will result in a compilation error. Be
extremely careful while using opt-in backends, as incorrect
configuration may result in vulnerable applications or applications that
always panic.

Note that using an opt-in backend in a library (e.g. for tests or
benchmarks) WILL NOT have any effect on its downstream users.

### <a href="#raw-linux-syscall-support" class="doc-anchor">§</a>Raw Linux syscall support

Currently the `linux_raw` backend supports only targets with stabilized
`asm!` macro, i.e. `arm`, `aarch64`, `loongarch64`, `riscv32`,
`riscv64`, `s390x`, `x86`, and `x86_64`.

Note that the raw syscall backend may be slower than backends based on
`libc::getrandom`, e.g. it does not implement vDSO optimizations and on
`x86` it uses the infamously slow `int 0x80` instruction to perform
syscall.

### <a href="#webassembly-support" class="doc-anchor">§</a>WebAssembly support

This crate fully supports the
[WASI](https://github.com/WebAssembly/WASI) and
[Emscripten](https://emscripten.org) targets. However, the
`wasm32-unknown-unknown` target (i.e. the target used by `wasm-pack`) is
not automatically supported since, from the target name alone, we cannot
deduce which JavaScript interface should be used (or if JavaScript is
available at all).

We do not include support for this target in the default configuration
because our JS backend (supporting web browsers, web workers and Node.js
v19 or later) requires
[`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen), **bloating
`Cargo.lock`** and **potentially breaking builds** on non-web WASM
platforms.

To enable `getrandom`’s functionality on `wasm32-unknown-unknown` using
the Web Crypto methods [described above](#opt-in-backends) via
[`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen), enable the
`wasm_js` feature flag. Setting
`RUSTFLAGS='--cfg getrandom_backend="wasm_js"'` is allowed but is no
longer required and does nothing (it was required in a prior version of
this crate).

WARNING: enabling the `wasm_js` feature will bloat `Cargo.lock` on all
platforms (where
[`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) is not an
existing dependency) and is known to cause build issues on some non-web
WASM platforms, even when a different backend is selected via
`getrandom_backend`.

### <a href="#custom-backend" class="doc-anchor">§</a>Custom backend

If this crate does not support your target out of the box or you have to
use a non-default entropy source, then you can provide a custom
implementation. You need to enable the custom backend as described in
the [opt-in backends](#opt-in-backends) section.

Next, you need to define an `extern` function with the following
signature:

``` {.rust .rust-example-rendered}
use getrandom::Error;

#[no_mangle]
unsafe extern "Rust" fn __getrandom_v03_custom(
    dest: *mut u8,
    len: usize,
) -> Result<(), Error> {
    todo!()
}
```

This function should, ideally, be defined in the root crate of your
project, e.g. in your `main.rs`. This function MUST be defined only once
for your project, i.e. upstream library crates SHOULD NOT define it
outside of tests and benchmarks. Improper configuration of this backend
may result in linking errors.

The function accepts a pointer to a buffer that should be filled with
random data and its length in bytes. Note that the buffer MAY be
uninitialized. On success, the function should return `Ok(())` and fully
fill the input buffer; otherwise, it should return an error value.

While wrapping functions which work with byte slices you should fully
initialize the buffer before passing it to the function:

``` {.rust .rust-example-rendered}
use getrandom::Error;

fn my_entropy_source(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    // ...
    Ok(())
}

#[no_mangle]
unsafe extern "Rust" fn __getrandom_v03_custom(
    dest: *mut u8,
    len: usize,
) -> Result<(), Error> {
    let buf = unsafe {
        // fill the buffer with zeros
        core::ptr::write_bytes(dest, 0, len);
        // create mutable byte slice
        core::slice::from_raw_parts_mut(dest, len)
    };
    my_entropy_source(buf)
}
```

### <a href="#unsupported-backend" class="doc-anchor">§</a>Unsupported backend

In some rare scenarios, you might be compiling this crate for an
unsupported target (e.g. `wasm32-unknown-unknown`), but this crate’s
functionality is not actually used by your code. If you are confident
that `getrandom` is not used in your project, but it gets pulled
nevertheless by one of your dependencies, then you can enable the
`unsupported` backend, which always returns `Err(Error::UNSUPPORTED)`.

### <a href="#platform-support" class="doc-anchor">§</a>Platform Support

This crate generally supports the same operating system and platform
versions that the Rust standard library does. Additional targets may be
supported using the opt-in custom backend.

This means that as Rust drops support for old versions of operating
systems (such as old Linux kernel versions, Android API levels, etc.) in
stable releases, `getrandom` may create new patch releases that remove
support for outdated platform versions.

### <a href="#devurandom-fallback-on-linux-and-android"
class="doc-anchor">§</a>`/dev/urandom` fallback on Linux and Android

On Linux targets, the `/dev/urandom` fallback is present only if either
`target_env` is `musl`, or `target_arch` is one of the following:
`aarch64`, `arm`, `powerpc`, `powerpc64`, `s390x`, `x86`, `x86_64`.
Other supported targets
[require](https://doc.rust-lang.org/stable/rustc/platform-support.html)
kernel versions that support the `getrandom` system call, so the
fallback is not needed.

On Android targets the fallback is present only for the following
`target_arch`es: `aarch64`, `arm`, `x86`, `x86_64`. Other
`target_arch`es (e.g. RISC-V) require sufficiently high API levels.

The fallback can be disabled by enabling the `linux_getrandom` opt-in
backend. Note that doing so will bump minimum supported Linux kernel
version to 3.17 and Android API level to 23 (Marshmallow).

### <a href="#early-boot" class="doc-anchor">§</a>Early boot

Sometimes, early in the boot process, the OS has not collected enough
entropy to securely seed its RNG. This is especially common on virtual
machines, where standard “random” events are hard to come by.

Some operating system interfaces always block until the RNG is securely
seeded. This can take anywhere from a few seconds to more than a minute.
A few (Linux, NetBSD and Solaris) offer a choice between blocking and
getting an error; in these cases, we always choose to block.

On Linux (when the `getrandom` system call is not available), reading
from `/dev/urandom` never blocks, even when the OS hasn’t collected
enough entropy yet. To avoid returning low-entropy bytes, we first poll
`/dev/random` and only switch to `/dev/urandom` once this has succeeded.

On OpenBSD, this kind of entropy accounting isn’t available, and on
NetBSD, blocking on it is discouraged. On these platforms, nonblocking
interfaces are used, even when reliable entropy may not be available. On
the platforms where it is used, the reliability of entropy accounting
itself isn’t free from controversy. This library provides randomness
sourced according to the platform’s best practices, but each platform
has its own limits on the grade of randomness it can promise in
environments with few sources of entropy.

On ESP-IDF, if `esp_fill_random` is used before enabling WiFi, BT, or
the voltage noise entropy source (SAR ADC), the Hardware RNG will only
be seeded via RC\_FAST\_CLK. This can occur during early boot unless
`bootloader_random_enable()` is called. For more information see the
[ESP-IDF RNG
Docs](https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/random.html)
or the [RNG section of the ESP32 Technical Reference
Manual](https://www.espressif.com/sites/default/files/documentation/esp32_technical_reference_manual_en.pdf#rng).

## <a href="#error-handling" class="doc-anchor">§</a>Error handling

We always prioritize failure over returning known insecure “random”
bytes. Generally, on supported platforms, failure is highly unlikely,
though not impossible. If an error does occur, it is likely that it will
occur on every call to `getrandom`. Therefore, after the first
successful call, one can be reasonably confident that no errors will
occur.

## <a href="#panic-handling" class="doc-anchor">§</a>Panic handling

We strive to eliminate all potential panics from our backend
implementations. In other words, when compiled with optimizations
enabled, the generated binary code for `getrandom` functions should not
contain any panic branches. Even if the platform misbehaves and returns
an unexpected result, our code should correctly handle it and return an
error, e.g.
[`Error::UNEXPECTED`](struct.Error.html#associatedconstant.UNEXPECTED "associated constant getrandom::Error::UNEXPECTED").

## <a href="#sanitizer-support" class="doc-anchor">§</a>Sanitizer support

If your code uses
[`fill_uninit`](fn.fill_uninit.html "fn getrandom::fill_uninit") and you
enable
[MemorySanitizer](https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html#memorysanitizer)
(i.e. `-Zsanitizer=memory`), we will automatically handle unpoisoning of
the destination buffer filled by `fill_uninit`.

You can run sanitizer tests for your crate dependent on `getrandom` like
this:

``` language-sh
RUSTFLAGS="-Zsanitizer=memory" cargo test -Zbuild-std --target=x86_64-unknown-linux-gnu
```

## <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

This crate requires Rust 1.63 or later.

## <a href="#license" class="doc-anchor">§</a>License

The `getrandom` library is distributed under either of

-   [Apache License, Version
    2.0](https://github.com/rust-random/getrandom/blob/master/LICENSE-APACHE)
-   [MIT
    license](https://github.com/rust-random/getrandom/blob/master/LICENSE-MIT)

at your option.

### <a href="#contribution" class="doc-anchor">§</a>Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Error.html" class="struct"
title="struct getrandom::Error">Error</a>  
A small and `no_std` compatible error type

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.fill.html" class="fn" title="fn getrandom::fill">fill</a>  
Fill `dest` with random bytes from the system’s preferred random number
source.

<a href="fn.fill_uninit.html" class="fn"
title="fn getrandom::fill_uninit">fill_uninit</a>  
Fill potentially uninitialized buffer `dest` with random bytes from the
system’s preferred random number source and return a mutable reference
to those bytes.

<a href="fn.u32.html" class="fn" title="fn getrandom::u32">u32</a>  
Get random `u32` from the system’s preferred random number source.

<a href="fn.u64.html" class="fn" title="fn getrandom::u64">u64</a>  
Get random `u64` from the system’s preferred random number source.
