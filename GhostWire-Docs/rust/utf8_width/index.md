Crate utf8\_width Copy item path

<span class="sub-heading"><a href="../src/utf8_width/lib.rs.html#1-96" class="src">Source</a>
</span>

Expand description

# <a href="#utf-8-width" class="doc-anchor">§</a>UTF-8 Width

To determine the width of a UTF-8 character by providing its first byte.

References: https://tools.ietf.org/html/rfc3629

## <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
assert_eq!(1, utf8_width::get_width(b'1'));
assert_eq!(3, utf8_width::get_width("中".as_bytes()[0]));
```

## <a href="#benchmark" class="doc-anchor">§</a>Benchmark

``` language-bash
cargo bench
```

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.MAX_0_1.html" class="constant"
title="constant utf8_width::MAX_0_1">MAX_0_1</a>

<a href="constant.MAX_0_2.html" class="constant"
title="constant utf8_width::MAX_0_2">MAX_0_2</a>

<a href="constant.MAX_1.html" class="constant"
title="constant utf8_width::MAX_1">MAX_1</a>

<a href="constant.MAX_2.html" class="constant"
title="constant utf8_width::MAX_2">MAX_2</a>

<a href="constant.MAX_3.html" class="constant"
title="constant utf8_width::MAX_3">MAX_3</a>

<a href="constant.MAX_4.html" class="constant"
title="constant utf8_width::MAX_4">MAX_4</a>

<a href="constant.MIN_0_1.html" class="constant"
title="constant utf8_width::MIN_0_1">MIN_0_1</a>

<a href="constant.MIN_0_2.html" class="constant"
title="constant utf8_width::MIN_0_2">MIN_0_2</a>

<a href="constant.MIN_1.html" class="constant"
title="constant utf8_width::MIN_1">MIN_1</a>

<a href="constant.MIN_2.html" class="constant"
title="constant utf8_width::MIN_2">MIN_2</a>

<a href="constant.MIN_3.html" class="constant"
title="constant utf8_width::MIN_3">MIN_3</a>

<a href="constant.MIN_4.html" class="constant"
title="constant utf8_width::MIN_4">MIN_4</a>

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.get_width.html" class="fn"
title="fn utf8_width::get_width">get_width</a>

Given a first byte, determine how many bytes are in this UTF-8
character. If the UTF-8 character is invalid, return `0`; otherwise,
return `1` to `4`.

<a href="fn.get_width_assume_valid.html" class="fn"
title="fn utf8_width::get_width_assume_valid">get_width_assume_valid</a><sup>⚠</sup>

*Assuming the input first byte is from a valid UTF-8 character*,
determine how many bytes are in this UTF-8 character. It returns `1` to
`4`.

<a href="fn.is_width_0.html" class="fn"
title="fn utf8_width::is_width_0">is_width_0</a>

<a href="fn.is_width_1.html" class="fn"
title="fn utf8_width::is_width_1">is_width_1</a>

<a href="fn.is_width_2.html" class="fn"
title="fn utf8_width::is_width_2">is_width_2</a>

<a href="fn.is_width_3.html" class="fn"
title="fn utf8_width::is_width_3">is_width_3</a>

<a href="fn.is_width_4.html" class="fn"
title="fn utf8_width::is_width_4">is_width_4</a>
