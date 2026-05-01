Crate atomic\_waker Copy item path

<span class="sub-heading"><a href="../src/atomic_waker/lib.rs.html#1-443" class="src">Source</a>
</span>

Expand description

`futures::task::AtomicWaker` extracted into its own crate.

# <a href="#features" class="doc-anchor">§</a>Features

This crate adds a feature, `portable-atomic`, which uses a polyfill from
the [`portable-atomic`](https://crates.io/crates/portable-atomic) crate
in order to provide functionality to targets without atomics. See the
[`README`](https://github.com/taiki-e/portable-atomic/blob/main/README.md#optional-cfg)
for the [`portable-atomic`](https://crates.io/crates/portable-atomic)
crate for more information on how to use it.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AtomicWaker.html" class="struct"
title="struct atomic_waker::AtomicWaker">AtomicWaker</a>  
A synchronization primitive for task wakeup.
