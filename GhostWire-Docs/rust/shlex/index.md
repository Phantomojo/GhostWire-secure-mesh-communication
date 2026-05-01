Crate shlex Copy item path

<span class="sub-heading"><a href="../src/shlex/lib.rs.html#6-358" class="src">Source</a>
</span>

Expand description

Parse strings like, and escape strings for, POSIX shells.

Same idea as (but implementation not directly based on) the Python shlex
module.

Disabling the `std` feature (which is enabled by default) will allow the
crate to work in `no_std` environments, where the `alloc` crate, and a
global allocator, are available.

## <a href="#warning" class="doc-anchor">§</a><span style="color:red">Warning</span>

The
[`try_quote`](fn.try_quote.html "fn shlex::try_quote")/[`try_join`](fn.try_join.html "fn shlex::try_join")
family of APIs does not quote control characters (because they cannot be
quoted portably).

This is fully safe in noninteractive contexts, like shell scripts and
`sh -c` arguments (or even scripts `source`d from interactive shells).

But if you are quoting for human consumption, you should keep in mind
that ugly inputs produce ugly outputs (which may not be copy-pastable).

And if by chance you are piping the output of
[`try_quote`](fn.try_quote.html "fn shlex::try_quote")/[`try_join`](fn.try_join.html "fn shlex::try_join")
directly to the stdin of an interactive shell, you should stop, because
control characters can lead to arbitrary command injection.

For more information, and for information about more minor issues,
please see
[quoting\_warning](quoting_warning/index.html "mod shlex::quoting_warning").

## <a href="#compatibility" class="doc-anchor">§</a>Compatibility

This crate’s quoting functionality tries to be compatible with **any
POSIX-compatible shell**; it’s tested against `bash`, `zsh`, `dash`,
Busybox `ash`, and `mksh`, plus `fish` (which is not POSIX-compatible
but close enough).

It also aims to be compatible with Python `shlex` and C `wordexp`.

# Modules<a href="#modules" class="anchor">§</a>

<a href="bytes/index.html" class="mod"
title="mod shlex::bytes">bytes</a>  
[`Shlex`](bytes/struct.Shlex.html "struct shlex::bytes::Shlex") and
friends for byte strings.

<a href="quoting_warning/index.html" class="mod"
title="mod shlex::quoting_warning">quoting_warning</a>  
A more detailed version of the [warning at the top
level](index.html#warning "mod shlex") about the `quote`/`join` family
of APIs.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Quoter.html" class="struct"
title="struct shlex::Quoter">Quoter</a>  
A more configurable interface to quote strings. If you only want the
default settings you can use the convenience functions
[`try_quote`](fn.try_quote.html "fn shlex::try_quote") and
[`try_join`](fn.try_join.html "fn shlex::try_join").

<a href="struct.Shlex.html" class="struct"
title="struct shlex::Shlex">Shlex</a>  
An iterator that takes an input string and splits it into the words
using the same syntax as the POSIX shell.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.QuoteError.html" class="enum"
title="enum shlex::QuoteError">QuoteError</a>  
Errors from
[`Quoter::quote`](struct.Quoter.html#method.quote "method shlex::Quoter::quote"),
[`Quoter::join`](struct.Quoter.html#method.join "method shlex::Quoter::join"),
etc. (and their [`bytes`](bytes/index.html "mod shlex::bytes")
counterparts).

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.join.html" class="fn" title="fn shlex::join">join</a><span class="stab deprecated" title="">Deprecated</span>  
Convenience function that consumes an iterable of words and turns it
into a single string, quoting words when necessary. Consecutive words
will be separated by a single space.

<a href="fn.quote.html" class="fn" title="fn shlex::quote">quote</a><span class="stab deprecated" title="">Deprecated</span>  
Given a single word, return a string suitable to encode it as a shell
argument.

<a href="fn.split.html" class="fn" title="fn shlex::split">split</a>  
Convenience function that consumes the whole string at once. Returns
None if the input was erroneous.

<a href="fn.try_join.html" class="fn"
title="fn shlex::try_join">try_join</a>  
Convenience function that consumes an iterable of words and turns it
into a single string, quoting words when necessary. Consecutive words
will be separated by a single space.

<a href="fn.try_quote.html" class="fn"
title="fn shlex::try_quote">try_quote</a>  
Given a single word, return a string suitable to encode it as a shell
argument.
