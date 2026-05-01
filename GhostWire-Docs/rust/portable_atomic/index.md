Crate portable\_atomic Copy item path

<span class="sub-heading"><a href="../src/portable_atomic/lib.rs.html#3-5177"
class="src">Source</a> </span>

Expand description

Portable atomic types including support for 128-bit atomics, atomic
float, etc.

-   Provide all atomic integer types (`Atomic{I,U}{8,16,32,64}`) for all
    targets that can use atomic CAS. (i.e., all targets that can use
    `std`, and most no-std targets)
-   Provide `AtomicI128` and `AtomicU128`.
-   Provide `AtomicF32` and `AtomicF64`. ([optional, requires the
    `float` feature](#optional-features-float))
-   Provide `AtomicF16` and `AtomicF128` for [unstable `f16` and
    `f128`](https://github.com/rust-lang/rust/issues/116909).
    ([optional, requires the `float` feature and unstable
    cfgs](#optional-features-float))
-   Provide atomic load/store for targets where atomic is not available
    at all in the standard library. (RISC-V without A-extension, MSP430,
    AVR)
-   Provide atomic CAS for targets where atomic CAS is not available in
    the standard library. (thumbv6m, pre-v6 Arm, RISC-V without
    A-extension, MSP430, AVR, Xtensa, etc.) (always enabled for MSP430
    and AVR, [optional](#optional-features-critical-section) otherwise)
-   Make features that require newer compilers, such as
    [`fetch_{max,min}`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.fetch_max),
    [`fetch_update`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.fetch_update),
    [`as_ptr`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.as_ptr),
    [`from_ptr`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.from_ptr),
    [`AtomicBool::fetch_not`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicBool.html#method.fetch_not),
    [`AtomicPtr::fetch_*`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicPtr.html#method.fetch_and),
    and [stronger CAS failure
    ordering](https://github.com/rust-lang/rust/pull/98383) available on
    Rust 1.34+.
-   Provide workaround for bugs in the standard library’s atomic-related
    APIs, such as
    [rust-lang/rust#100650](https://github.com/rust-lang/rust/issues/100650),
    `fence`/`compiler_fence` on MSP430 that cause LLVM error, etc.

portable-atomic version of `std::sync::Arc` is provided by the
[portable-atomic-util](https://github.com/taiki-e/portable-atomic/tree/HEAD/portable-atomic-util)
crate.

## <a href="#usage" class="doc-anchor">§</a>Usage

Add this to your `Cargo.toml`:

``` language-toml
[dependencies]
portable-atomic = "1"
```

The default features are mainly for users who use atomics larger than
the pointer width. If you don’t need them, disabling the default
features may reduce code size and compile time slightly.

``` language-toml
[dependencies]
portable-atomic = { version = "1", default-features = false }
```

If your crate supports no-std environment and requires atomic CAS,
enabling the `require-cas` feature will allow the portable-atomic to
display a [helpful error
message](https://github.com/taiki-e/portable-atomic/pull/100) to users
on targets requiring additional action on the user side to provide
atomic CAS.

``` language-toml
[dependencies]
portable-atomic = { version = "1.3", default-features = false, features = ["require-cas"] }
```

(Since 1.8, portable-atomic can display a [helpful error
message](https://github.com/taiki-e/portable-atomic/pull/181) even
without the `require-cas` feature when the rustc version is 1.78+.
However, the `require-cas` feature also allows rejecting builds at an
earlier stage, we recommend enabling it unless enabling it causes
[problems](https://github.com/matklad/once_cell/pull/267).)

## <a href="#128-bit-atomics-support" class="doc-anchor">§</a>128-bit atomics support

Native 128-bit atomic operations are available on x86\_64 (Rust 1.59+),
AArch64 (Rust 1.59+), riscv64 (Rust 1.59+), Arm64EC (Rust 1.84+), s390x
(Rust 1.84+), and powerpc64 (nightly only), otherwise the fallback
implementation is used.

On x86\_64, even if `cmpxchg16b` is not available at compile-time (Note:
`cmpxchg16b` target feature is enabled by default only on Apple, Windows
(except Windows 7), and Fuchsia targets), run-time detection checks
whether `cmpxchg16b` is available. If `cmpxchg16b` is not available at
either compile-time or run-time detection, the fallback implementation
is used. See also
[`portable_atomic_no_outline_atomics`](#optional-cfg-no-outline-atomics)
cfg.

They are usually implemented using inline assembly, and when using Miri
or ThreadSanitizer that do not support inline assembly, core intrinsics
are used instead of inline assembly if possible.

See the [`atomic128` module’s
readme](https://github.com/taiki-e/portable-atomic/blob/HEAD/src/imp/atomic128/README.md)
for details.

## <a href="#optional-featurescfgs" class="doc-anchor">§</a><span id="optional-features"></span><span id="optional-cfg"></span>Optional features/cfgs

portable-atomic provides features and cfgs to allow enabling specific
APIs and customizing its behavior.

Some options have both a feature and a cfg. When both exist, it
indicates that the feature does not follow Cargo’s recommendation that
[features should be
additive](https://doc.rust-lang.org/nightly/cargo/reference/features.html#feature-unification).
Therefore, the maintainer’s recommendation is to use cfg instead of
feature. However, in the embedded ecosystem, it is very common to use
features in such places, so these options provide both so you can choose
based on your preference.

How to enable cfg (click to show)

One of the ways to enable cfg is to set [rustflags in the cargo
config](https://doc.rust-lang.org/cargo/reference/config.html#targettriplerustflags):

``` language-toml
# .cargo/config.toml
[target.<target>]
rustflags = ["--cfg", "portable_atomic_unsafe_assume_single_core"]
```

Or set environment variable:

``` language-sh
RUSTFLAGS="--cfg portable_atomic_unsafe_assume_single_core" cargo ...
```

-   <span id="optional-features-fallback"></span>**`fallback` feature**
    *(enabled by default)*  
    Enable fallback implementations.

    This enables atomic types with larger than the width supported by
    atomic instructions available on the current target. If the current
    target supports 128-bit atomics, this is no-op.

    This uses fallback implementation that using global locks by
    default. The following features/cfgs change this behavior:

    -   [`unsafe-assume-single-core` feature /
        `portable_atomic_unsafe_assume_single_core`
        cfg](#optional-features-unsafe-assume-single-core): Use fallback
        implementations that disabling interrupts instead of using
        global locks.
        -   If your target is single-core and calling interrupt disable
            instructions is safe, this is a safer and more efficient
            option.
    -   [`unsafe-assume-privileged` feature /
        `portable_atomic_unsafe_assume_privileged`
        cfg](#optional-features-unsafe-assume-privileged): Use fallback
        implementations that using global locks with disabling
        interrupts.
        -   If your target is multi-core and calling interrupt disable
            instructions is safe, this is a safer option.

-   <span id="optional-features-float"></span>**`float` feature**  
    Provide `AtomicF{32,64}`.

    If you want atomic types for unstable float types ([`f16` and
    `f128`](https://github.com/rust-lang/rust/issues/116909)), enable
    unstable cfg (`portable_atomic_unstable_f16` cfg for `AtomicF16`,
    `portable_atomic_unstable_f128` cfg for `AtomicF128`, [there is no
    possibility that both feature and cfg will be provided for unstable
    options.](https://github.com/taiki-e/portable-atomic/pull/200#issuecomment-2682252991)).

> **ⓘ Note**
>
> -   Atomic float’s `fetch_{add,sub,min,max}` are usually implemented
>     using CAS loops, which can be slower than equivalent operations of
>     atomic integers. As an exception, AArch64 with FEAT\_LSFE and GPU
>     targets have atomic float instructions and we use them on AArch64
>     when `lsfe` target feature is available at compile-time. We [plan
>     to use atomic float instructions for GPU targets as well in the
>     future.](https://github.com/taiki-e/portable-atomic/issues/34)
> -   Unstable cfgs are outside of the normal semver guarantees and
>     minor or patch versions of portable-atomic may make breaking
>     changes to them at any time.

-   <span id="optional-features-std"></span>**`std` feature**  
    Use `std`.

-   <span id="optional-features-require-cas"></span>**`require-cas`
    feature**  
    Emit compile error if atomic CAS is not available. See
    [Usage](#usage) section for usage of this feature.

-   <span id="optional-features-serde"></span>**`serde` feature**  
    Implement `serde::{Serialize,Deserialize}` for atomic types.

    Note:

    -   The MSRV when this feature is enabled depends on the MSRV of
        [serde](https://github.com/serde-rs/serde).

-   <span id="optional-features-critical-section"></span>**`critical-section`
    feature**  
    Use
    [critical-section](https://github.com/rust-embedded/critical-section)
    to provide atomic CAS for targets where atomic CAS is not available
    in the standard library.

    `critical-section` support is useful to get atomic CAS when the
    [`unsafe-assume-single-core` feature (or
    `portable_atomic_unsafe_assume_single_core`
    cfg)](#optional-features-unsafe-assume-single-core) can’t be used,
    such as multi-core targets, unprivileged code running under some
    RTOS, or environments where disabling interrupts needs extra care
    due to e.g. real-time requirements.

> **ⓘ Note**
>
> -   When enabling this feature, you should provide a suitable critical
>     section implementation for the current target, see the
>     [critical-section](https://github.com/rust-embedded/critical-section)
>     documentation for details on how to do so.
>
> -   With this feature, critical sections are taken for all atomic
>     operations, while with `unsafe-assume-single-core` feature [some
>     operations](https://github.com/taiki-e/portable-atomic/blob/HEAD/src/imp/interrupt/README.md#no-disable-interrupts)
>     don’t require disabling interrupts. Therefore, for better
>     performance, if all the `critical-section` implementation for your
>     target does is disable interrupts, prefer using
>     `unsafe-assume-single-core` feature (or
>     `portable_atomic_unsafe_assume_single_core` cfg) instead.
>
> -   It is usually **discouraged** to always enable this feature in
>     libraries that depend on `portable-atomic`.
>
>     Enabling this feature will prevent the end user from having the
>     chance to take advantage of other (potentially) efficient
>     implementations (implementations provided by
>     `unsafe-assume-single-core` feature mentioned above,
>     implementation proposed in
>     [\#60](https://github.com/taiki-e/portable-atomic/issues/60),
>     etc.). Also, targets that are currently unsupported may be
>     supported in the future.
>
>     The recommended approach for libraries is to leave it up to the
>     end user whether or not to enable this feature. (However, it may
>     make sense to enable this feature by default for libraries
>     specific to a platform where other implementations are known not
>     to work.)
>
>     See also
>     [](https://github.com/matklad/once_cell/issues/264#issuecomment-2352654806).
>
>     As an example, the end-user’s `Cargo.toml` that uses a crate that
>     provides a critical-section implementation and a crate that
>     depends on portable-atomic as an option would be expected to look
>     like this:
>
>     ``` language-toml
>     [dependencies]
>     portable-atomic = { version = "1", default-features = false, features = ["critical-section"] }
>     crate-provides-critical-section-impl = "..."
>     crate-uses-portable-atomic-as-feature = { version = "...", features = ["portable-atomic"] }
>     ```
>
> -   Enabling both this feature and `unsafe-assume-single-core` feature
>     (or `portable_atomic_unsafe_assume_single_core` cfg) will result
>     in a compile error.
>
> -   Enabling both this feature and `unsafe-assume-privileged` feature
>     (or `portable_atomic_unsafe_assume_privileged` cfg) will result in
>     a compile error.
>
> -   The MSRV when this feature is enabled depends on the MSRV of
>     [critical-section](https://github.com/rust-embedded/critical-section).

-   <span id="optional-features-unsafe-assume-single-core"></span><span id="optional-cfg-unsafe-assume-single-core"></span>**`unsafe-assume-single-core`
    feature / `portable_atomic_unsafe_assume_single_core` cfg**  
    Assume that the target is single-core and privileged instructions
    required to disable interrupts are available.

    -   When this feature/cfg is enabled, this crate provides atomic CAS
        for targets where atomic CAS is not available in the standard
        library by disabling interrupts.
    -   When both this feature/cfg and enabled-by-default `fallback`
        feature is enabled, this crate provides atomic types with larger
        than the width supported by native instructions by disabling
        interrupts.

> **⚠ Warning**
>
> This feature/cfg is `unsafe`, and note the following safety
> requirements:
>
> -   Enabling this feature/cfg for multi-core systems is always
>     **unsound**.
>
> -   This uses privileged instructions to disable interrupts, so it
>     usually doesn’t work on unprivileged mode.
>
>     Enabling this feature/cfg in an environment where privileged
>     instructions are not available, or if the instructions used are
>     not sufficient to disable interrupts in the system, it is also
>     usually considered **unsound**, although the details are
>     system-dependent.
>
>     The following are known cases:
>
>     -   On Arm (except for M-Profile architectures), this disables
>         only IRQs by default. For many systems (e.g., GBA) this is
>         enough. If the system need to disable both IRQs and FIQs, you
>         need to enable the `disable-fiq` feature (or
>         `portable_atomic_disable_fiq` cfg) together.
>     -   On RISC-V, this generates code for machine-mode (M-mode) by
>         default. If you enable the `s-mode` feature (or
>         `portable_atomic_s_mode` cfg) together, this generates code
>         for supervisor-mode (S-mode). In particular,
>         `qemu-system-riscv*` uses
>         [OpenSBI](https://github.com/riscv-software-src/opensbi) as
>         the default firmware.

Consider using the [`unsafe-assume-privileged` feature (or
`portable_atomic_unsafe_assume_privileged`
cfg)](#optional-features-unsafe-assume-privileged) for multi-core
systems with atomic CAS.

Consider using the [`critical-section`
feature](#optional-features-critical-section) for systems that cannot
use this feature/cfg.

See also the [`interrupt` module’s
readme](https://github.com/taiki-e/portable-atomic/blob/HEAD/src/imp/interrupt/README.md).

> **ⓘ Note**
>
> -   It is **very strongly discouraged** to enable this feature/cfg in
>     libraries that depend on `portable-atomic`.
>
>     The recommended approach for libraries is to leave it up to the
>     end user whether or not to enable this feature/cfg. (However, it
>     may make sense to enable this feature/cfg by default for libraries
>     specific to a platform where it is guaranteed to always be sound,
>     for example in a hardware abstraction layer targeting a
>     single-core chip.)
>
> -   Enabling this feature/cfg for unsupported architectures will
>     result in a compile error.
>
>     -   Arm, RISC-V, and Xtensa are currently supported. (Since all
>         MSP430 and AVR are single-core, we always provide atomic CAS
>         for them without this feature/cfg.)
>     -   Feel free to [submit an
>         issue](https://github.com/taiki-e/portable-atomic/issues/new)
>         if your target is not supported yet.
>
> -   Enabling this feature/cfg for targets where privileged
>     instructions are obviously unavailable (e.g., Linux) will result
>     in a compile error.
>
>     -   Feel free to [submit an
>         issue](https://github.com/taiki-e/portable-atomic/issues/new)
>         if your target supports privileged instructions but the build
>         rejected.
>
> -   Enabling both this feature/cfg and `critical-section` feature will
>     result in a compile error.
>
> -   When both this feature/cfg and `unsafe-assume-privileged` feature
>     (or `portable_atomic_unsafe_assume_privileged` cfg) are enabled,
>     this feature/cfg is preferred.

-   <span id="optional-features-unsafe-assume-privileged"></span><span id="optional-cfg-unsafe-assume-privileged"></span>**`unsafe-assume-privileged`
    feature / `portable_atomic_unsafe_assume_privileged` cfg**  
    Similar to `unsafe-assume-single-core` feature /
    `portable_atomic_unsafe_assume_single_core` cfg, but only assumes
    about availability of privileged instructions required to disable
    interrupts.

    -   When both this feature/cfg and enabled-by-default `fallback`
        feature is enabled, this crate provides atomic types with larger
        than the width supported by native instructions by using global
        locks with disabling interrupts.

> **⚠ Warning**
>
> This feature/cfg is `unsafe`, and except for being sound in multi-core
> systems, this has the same safety requirements as
> [`unsafe-assume-single-core` feature /
> `portable_atomic_unsafe_assume_single_core`
> cfg](#optional-features-unsafe-assume-single-core).

> **ⓘ Note**
>
> -   It is **very strongly discouraged** to enable this feature/cfg in
>     libraries that depend on `portable-atomic`.
>
>     The recommended approach for libraries is to leave it up to the
>     end user whether or not to enable this feature/cfg. (However, it
>     may make sense to enable this feature/cfg by default for libraries
>     specific to a platform where it is guaranteed to always be sound,
>     for example in a hardware abstraction layer.)
>
> -   Enabling this feature/cfg for unsupported targets will result in a
>     compile error.
>
>     -   This requires atomic CAS (`cfg(target_has_atomic = "ptr")` or
>         `cfg_no_atomic_cas!`).
>     -   Arm, RISC-V, and Xtensa are currently supported.
>     -   Feel free to [submit an
>         issue](https://github.com/taiki-e/portable-atomic/issues/new)
>         if your target is not supported yet.
>
> -   Enabling this feature/cfg for targets where privileged
>     instructions are obviously unavailable (e.g., Linux) will result
>     in a compile error.
>
>     -   Feel free to [submit an
>         issue](https://github.com/taiki-e/portable-atomic/issues/new)
>         if your target supports privileged instructions but the build
>         rejected.
>
> -   Enabling both this feature/cfg and `critical-section` feature will
>     result in a compile error.
>
> -   When both this feature/cfg and `unsafe-assume-single-core` feature
>     (or `portable_atomic_unsafe_assume_single_core` cfg) are enabled,
>     `unsafe-assume-single-core` is preferred.

-   <span id="optional-cfg-no-outline-atomics"></span>**`portable_atomic_no_outline_atomics`
    cfg**  
    Disable dynamic dispatching by run-time CPU feature detection.

    Dynamic dispatching by run-time CPU feature detection allows
    maintaining support for older CPUs while using features that are not
    supported on older CPUs, such as CMPXCHG16B (x86\_64) and
    FEAT\_LSE/FEAT\_LSE2 (AArch64).

    See also the [`atomic128` module’s
    readme](https://github.com/taiki-e/portable-atomic/blob/HEAD/src/imp/atomic128/README.md).

> **ⓘ Note**
>
> -   If the required target features are enabled at compile-time,
>     dynamic dispatching is automatically disabled and the atomic
>     operations are inlined.
> -   This is compatible with no-std (as with all features except
>     `std`).
> -   On some targets, run-time detection is disabled by default mainly
>     for compatibility with incomplete build environments or support
>     for it is experimental, and can be enabled by
>     `portable_atomic_outline_atomics` cfg. (When both cfg are enabled,
>     `*_no_*` cfg is preferred.)
> -   Some AArch64 targets enable LLVM’s `outline-atomics` target
>     feature by default, so if you set this cfg, you may want to
>     disable that as well. (However, portable-atomic’s outline-atomics
>     does not depend on the compiler-rt symbols, so even if you need to
>     disable LLVM’s outline-atomics, you may not need to disable
>     portable-atomic’s outline-atomics.)
> -   Dynamic detection is currently only supported in x86\_64, AArch64,
>     Arm, RISC-V, Arm64EC, and powerpc64. Enabling this cfg for
>     unsupported architectures will result in a compile error.

## <a href="#related-projects" class="doc-anchor">§</a>Related Projects

-   [atomic-maybe-uninit](https://github.com/taiki-e/atomic-maybe-uninit):
    Atomic operations on potentially uninitialized integers.
-   [atomic-memcpy](https://github.com/taiki-e/atomic-memcpy): Byte-wise
    atomic memcpy.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use core::sync::atomic::`<a
href="https://doc.rust-lang.org/1.93.1/core/sync/atomic/enum.Ordering.html"
class="enum"
title="enum core::sync::atomic::Ordering"><code>Ordering</code></a>`;`

`pub use core::sync::atomic::`<a
href="https://doc.rust-lang.org/1.93.1/core/sync/atomic/fn.compiler_fence.html"
class="fn"
title="fn core::sync::atomic::compiler_fence"><code>compiler_fence</code></a>`;`

`pub use core::sync::atomic::`<a
href="https://doc.rust-lang.org/1.93.1/core/sync/atomic/fn.fence.html"
class="fn" title="fn core::sync::atomic::fence"><code>fence</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="hint/index.html" class="mod"
title="mod portable_atomic::hint">hint</a>  
Re-export of the
[`core::hint`](https://doc.rust-lang.org/1.93.1/core/hint/index.html "mod core::hint")
module.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.cfg_has_atomic_8.html" class="macro"
title="macro portable_atomic::cfg_has_atomic_8">cfg_has_atomic_8</a>

<a href="macro.cfg_has_atomic_16.html" class="macro"
title="macro portable_atomic::cfg_has_atomic_16">cfg_has_atomic_16</a>

<a href="macro.cfg_has_atomic_32.html" class="macro"
title="macro portable_atomic::cfg_has_atomic_32">cfg_has_atomic_32</a>

<a href="macro.cfg_has_atomic_64.html" class="macro"
title="macro portable_atomic::cfg_has_atomic_64">cfg_has_atomic_64</a>

<a href="macro.cfg_has_atomic_128.html" class="macro"
title="macro portable_atomic::cfg_has_atomic_128">cfg_has_atomic_128</a>

<a href="macro.cfg_has_atomic_cas.html" class="macro"
title="macro portable_atomic::cfg_has_atomic_cas">cfg_has_atomic_cas</a>

<a href="macro.cfg_has_atomic_ptr.html" class="macro"
title="macro portable_atomic::cfg_has_atomic_ptr">cfg_has_atomic_ptr</a>

<a href="macro.cfg_no_atomic_8.html" class="macro"
title="macro portable_atomic::cfg_no_atomic_8">cfg_no_atomic_8</a>

<a href="macro.cfg_no_atomic_16.html" class="macro"
title="macro portable_atomic::cfg_no_atomic_16">cfg_no_atomic_16</a>

<a href="macro.cfg_no_atomic_32.html" class="macro"
title="macro portable_atomic::cfg_no_atomic_32">cfg_no_atomic_32</a>

<a href="macro.cfg_no_atomic_64.html" class="macro"
title="macro portable_atomic::cfg_no_atomic_64">cfg_no_atomic_64</a>

<a href="macro.cfg_no_atomic_128.html" class="macro"
title="macro portable_atomic::cfg_no_atomic_128">cfg_no_atomic_128</a>

<a href="macro.cfg_no_atomic_cas.html" class="macro"
title="macro portable_atomic::cfg_no_atomic_cas">cfg_no_atomic_cas</a>

<a href="macro.cfg_no_atomic_ptr.html" class="macro"
title="macro portable_atomic::cfg_no_atomic_ptr">cfg_no_atomic_ptr</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AtomicBool.html" class="struct"
title="struct portable_atomic::AtomicBool">AtomicBool</a>  
A boolean type which can be safely shared between threads.

<a href="struct.AtomicI8.html" class="struct"
title="struct portable_atomic::AtomicI8">AtomicI8</a>  
An integer type which can be safely shared between threads.

<a href="struct.AtomicI16.html" class="struct"
title="struct portable_atomic::AtomicI16">AtomicI16</a>  
An integer type which can be safely shared between threads.

<a href="struct.AtomicI32.html" class="struct"
title="struct portable_atomic::AtomicI32">AtomicI32</a>  
An integer type which can be safely shared between threads.

<a href="struct.AtomicI64.html" class="struct"
title="struct portable_atomic::AtomicI64">AtomicI64</a>  
An integer type which can be safely shared between threads.

<a href="struct.AtomicI128.html" class="struct"
title="struct portable_atomic::AtomicI128">AtomicI128</a>  
An integer type which can be safely shared between threads.

<a href="struct.AtomicIsize.html" class="struct"
title="struct portable_atomic::AtomicIsize">AtomicIsize</a>  
An integer type which can be safely shared between threads.

<a href="struct.AtomicPtr.html" class="struct"
title="struct portable_atomic::AtomicPtr">AtomicPtr</a>  
A raw pointer type which can be safely shared between threads.

<a href="struct.AtomicU8.html" class="struct"
title="struct portable_atomic::AtomicU8">AtomicU8</a>  
An integer type which can be safely shared between threads.

<a href="struct.AtomicU16.html" class="struct"
title="struct portable_atomic::AtomicU16">AtomicU16</a>  
An integer type which can be safely shared between threads.

<a href="struct.AtomicU32.html" class="struct"
title="struct portable_atomic::AtomicU32">AtomicU32</a>  
An integer type which can be safely shared between threads.

<a href="struct.AtomicU64.html" class="struct"
title="struct portable_atomic::AtomicU64">AtomicU64</a>  
An integer type which can be safely shared between threads.

<a href="struct.AtomicU128.html" class="struct"
title="struct portable_atomic::AtomicU128">AtomicU128</a>  
An integer type which can be safely shared between threads.

<a href="struct.AtomicUsize.html" class="struct"
title="struct portable_atomic::AtomicUsize">AtomicUsize</a>  
An integer type which can be safely shared between threads.
