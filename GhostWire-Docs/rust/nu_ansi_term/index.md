Crate nu\_ansi\_term Copy item path

<span class="sub-heading"><a href="../src/nu_ansi_term/lib.rs.html#1-273" class="src">Source</a>
</span>

Expand description

This is a library for controlling colors and formatting, such as red
bold text or blue underlined text, on ANSI terminals.

## <a href="#basic-usage" class="doc-anchor">§</a>Basic usage

There are three main types in this crate that you need to be concerned
with: [`AnsiString`](type.AnsiString.html),
[`Style`](struct.Style.html), and [`Color`](enum.Color.html).

A `Style` holds stylistic information: foreground and background colors,
whether the text should be bold, or blinking, or other properties. The
[`Color`](enum.Color.html) enum represents the available colors. And an
[`AnsiString`](type.AnsiString.html) is a string paired with a
[`Style`](struct.Style.html).

[`Color`](enum.Color.html) is also available as an alias to `Color`.

To format a string, call the `paint` method on a `Style` or a `Color`,
passing in the string you want to format as the argument. For example,
here’s how to get some red text:

``` {.rust .rust-example-rendered}
use nu_ansi_term::Color::Red;

println!("This is in red: {}", Red.paint("a red string"));
```

It’s important to note that the `paint` method does *not* actually
return a string with the ANSI control characters surrounding it.
Instead, it returns an [`AnsiString`](type.AnsiString.html) value that
has a [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
implementation that, when formatted, returns the characters. This allows
strings to be printed with a minimum of
[`String`](https://doc.rust-lang.org/std/string/struct.String.html)
allocations being performed behind the scenes.

If you *do* want to get at the escape codes, then you can convert the
[`AnsiString`](type.AnsiString.html) to a string as you would any other
`Display` value:

``` {.rust .rust-example-rendered}
use nu_ansi_term::Color::Red;

let red_string = Red.paint("a red string").to_string();
```

## <a href="#bold-underline-background-and-other-styles"
class="doc-anchor">§</a>Bold, underline, background, and other styles

For anything more complex than plain foreground color changes, you need
to construct `Style` values themselves, rather than beginning with a
`Color`. You can do this by chaining methods based on a new `Style`,
created with [`Style::new()`](struct.Style.html#method.new). Each method
creates a new style that has that specific property set. For example:

``` {.rust .rust-example-rendered}
use nu_ansi_term::Style;

println!("How about some {} and {}?",
         Style::new().bold().paint("bold"),
         Style::new().underline().paint("underline"));
```

For brevity, these methods have also been implemented for `Color`
values, so you can give your styles a foreground color without having to
begin with an empty `Style` value:

``` {.rust .rust-example-rendered}
use nu_ansi_term::Color::{Blue, Yellow};

println!("Demonstrating {} and {}!",
         Blue.bold().paint("blue bold"),
         Yellow.underline().paint("yellow underline"));

println!("Yellow on blue: {}", Yellow.on(Blue).paint("wow!"));
```

The complete list of styles you can use are:
[`bold`](struct.Style.html#method.bold),
[`dimmed`](struct.Style.html#method.dimmed),
[`italic`](struct.Style.html#method.italic),
[`underline`](struct.Style.html#method.underline),
[`blink`](struct.Style.html#method.blink),
[`reverse`](struct.Style.html#method.reverse),
[`hidden`](struct.Style.html#method.hidden),
[`strikethrough`](struct.Style.html#method.strikethrough), and
[`on`](struct.Style.html#method.on) for background colors.

In some cases, you may find it easier to change the foreground on an
existing `Style` rather than starting from the appropriate `Color`. You
can do this using the [`fg`](struct.Style.html#method.fg) method:

``` {.rust .rust-example-rendered}
use nu_ansi_term::Style;
use nu_ansi_term::Color::{Blue, Cyan, Yellow};

println!("Yellow on blue: {}", Style::new().on(Blue).fg(Yellow).paint("yow!"));
println!("Also yellow on blue: {}", Cyan.on(Blue).fg(Yellow).paint("zow!"));
```

You can turn a `Color` into a `Style` with the
[`normal`](enum.Color.html#method.normal) method. This will produce the
exact same `AnsiString` as if you just used the `paint` method on the
`Color` directly, but it’s useful in certain cases: for example, you may
have a method that returns `Styles`, and need to represent both the “red
bold” and “red, but not bold” styles with values of the same type. The
`Style` struct also has a
[`Default`](https://doc.rust-lang.org/std/default/trait.Default.html)
implementation if you want to have a style with *nothing* set.

``` {.rust .rust-example-rendered}
use nu_ansi_term::Style;
use nu_ansi_term::Color::Red;

Red.normal().paint("yet another red string");
Style::default().paint("a completely regular string");
```

## <a href="#extended-colors" class="doc-anchor">§</a>Extended colors

You can access the extended range of 256 colors by using the
`Color::Fixed` variant, which takes an argument of the color number to
use. This can be included wherever you would use a `Color`:

``` {.rust .rust-example-rendered}
use nu_ansi_term::Color::Fixed;

Fixed(134).paint("A sort of light purple");
Fixed(221).on(Fixed(124)).paint("Mustard in the ketchup");
```

The first sixteen of these values are the same as the normal and bold
standard color variants. There’s nothing stopping you from using these
as `Fixed` colors instead, but there’s nothing to be gained by doing so
either.

You can also access full 24-bit color by using the `Color::Rgb` variant,
which takes separate `u8` arguments for red, green, and blue:

``` {.rust .rust-example-rendered}
use nu_ansi_term::Color::Rgb;

Rgb(70, 130, 180).paint("Steel blue");
```

## <a href="#combining-successive-colored-strings" class="doc-anchor">§</a>Combining successive colored strings

The benefit of writing ANSI escape codes to the terminal is that they
*stack*: you do not need to end every colored string with a reset code
if the text that follows it is of a similar style. For example, if you
want to have some blue text followed by some blue bold text, it’s
possible to send the ANSI code for blue, followed by the ANSI code for
bold, and finishing with a reset code without having to have an extra
one between the two strings.

This crate can optimise the ANSI codes that get printed in situations
like this, making life easier for your terminal renderer. The
[`AnsiStrings`](type.AnsiStrings.html) type takes a slice of several
[`AnsiString`](type.AnsiString.html) values, and will iterate over each
of them, printing only the codes for the styles that need to be updated
as part of its formatting routine.

The following code snippet uses this to enclose a binary number
displayed in red bold text inside some red, but not bold, brackets:

``` {.rust .rust-example-rendered}
use nu_ansi_term::Color::Red;
use nu_ansi_term::{AnsiString, AnsiStrings};

let some_value = format!("{:b}", 42);
let strings: &[AnsiString<'static>] = &[
    Red.paint("["),
    Red.bold().paint(some_value),
    Red.paint("]"),
];

println!("Value: {}", AnsiStrings(strings));
```

There are several things to note here. Firstly, the
[`paint`](type.AnsiByteString.html#method.write_to) method can take
*either* an owned
[`String`](https://doc.rust-lang.org/std/string/struct.String.html) or a
borrowed [`&str`](https://doc.rust-lang.org/std/primitive.str.html).
Internally, an [`AnsiString`](type.AnsiString.html) holds a
copy-on-write
([`Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html)) string
value to deal with both owned and borrowed strings at the same time.
This is used here to display a `String`, the result of the `format!`
call, using the same mechanism as some statically-available `&str`
slices. Secondly, that the [`AnsiStrings`](type.AnsiStrings.html) value
works in the same way as its singular counterpart, with a
[`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
implementation that only performs the formatting when required.

## <a href="#byte-strings" class="doc-anchor">§</a>Byte strings

This library also supports formatting `\[u8]` byte strings; this
supports applications working with text in an unknown encoding.
[`Style`](struct.Style.html) and [`Color`](enum.Color.html) support
painting `\[u8]` values, resulting in an
[`AnsiByteString`](type.AnsiByteString.html). This type does not
implement
[`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html), as it
may not contain UTF-8, but it does provide a method
[`write_to`](type.AnsiByteString.html#method.write_to) to write the
result to any value that implements
[`Write`](https://doc.rust-lang.org/std/io/trait.Write.html):

``` {.rust .rust-example-rendered}
use nu_ansi_term::Color::Green;

Green.paint("user data".as_bytes()).write_to(&mut std::io::stdout()).unwrap();
```

Similarly, the type [`AnsiByteStrings`](type.AnsiByteStrings.html)
supports writing a list of [`AnsiByteString`](type.AnsiByteString.html)
values with minimal escape sequences:

``` {.rust .rust-example-rendered}
use nu_ansi_term::Color::Green;
use nu_ansi_term::AnsiByteStrings;

AnsiByteStrings(&[
    Green.paint("user data 1\n".as_bytes()),
    Green.bold().paint("user data 2\n".as_bytes()),
]).write_to(&mut std::io::stdout()).unwrap();
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use ansi::`<a href="ansi/struct.Infix.html" class="struct"
title="struct nu_ansi_term::ansi::Infix"><code>Infix</code></a>`;`

`pub use ansi::`<a href="ansi/struct.Prefix.html" class="struct"
title="struct nu_ansi_term::ansi::Prefix"><code>Prefix</code></a>`;`

`pub use ansi::`<a href="ansi/struct.Suffix.html" class="struct"
title="struct nu_ansi_term::ansi::Suffix"><code>Suffix</code></a>`;`

`pub use `<a href="gradient/index.html" class="mod"
title="mod nu_ansi_term::gradient"><code>gradient</code></a>`::*;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="ansi/index.html" class="mod"
title="mod nu_ansi_term::ansi">ansi</a>

<a href="gradient/index.html" class="mod"
title="mod nu_ansi_term::gradient">gradient</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AnsiGenericString.html" class="struct"
title="struct nu_ansi_term::AnsiGenericString">AnsiGenericString</a>  
An `AnsiGenericString` includes a generic string type and a `Style` to
display that string. `AnsiString` and `AnsiByteString` are aliases for
this type on `str` and `\[u8]`, respectively.

<a href="struct.AnsiGenericStrings.html" class="struct"
title="struct nu_ansi_term::AnsiGenericStrings">AnsiGenericStrings</a>  
A set of `AnsiGenericStrings`s collected together, in order to be
written with a minimum of control characters.

<a href="struct.Rgb.html" class="struct"
title="struct nu_ansi_term::Rgb">Rgb</a>  
<a href="struct.Style.html" class="struct"
title="struct nu_ansi_term::Style">Style</a>  
A style is a collection of properties that can format a string using
ANSI escape codes.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Color.html" class="enum"
title="enum nu_ansi_term::Color">Color</a>  
A color is one specific type of ANSI escape code, and can refer to
either the foreground or background color.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.AnsiByteStrings.html" class="fn"
title="fn nu_ansi_term::AnsiByteStrings">AnsiByteStrings</a>  
A function to construct an `AnsiByteStrings` instance.

<a href="fn.AnsiStrings.html" class="fn"
title="fn nu_ansi_term::AnsiStrings">AnsiStrings</a>  
A function to construct an `AnsiStrings` instance.

<a href="fn.sub_string.html" class="fn"
title="fn nu_ansi_term::sub_string">sub_string</a>  
Return a substring of the given AnsiStrings sequence, while keeping the
formatting.

<a href="fn.unstyle.html" class="fn"
title="fn nu_ansi_term::unstyle">unstyle</a>  
Return a concatenated copy of `strs` without the formatting, as an
allocated `String`.

<a href="fn.unstyled_len.html" class="fn"
title="fn nu_ansi_term::unstyled_len">unstyled_len</a>  
Return the unstyled length of AnsiStrings. This is equaivalent to
`unstyle(strs).len()`.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.AnsiByteString.html" class="type"
title="type nu_ansi_term::AnsiByteString">AnsiByteString</a>  
An `AnsiByteString` represents a formatted series of bytes. Use
`AnsiByteString` when styling text with an unknown encoding.

<a href="type.AnsiByteStrings.html" class="type"
title="type nu_ansi_term::AnsiByteStrings">AnsiByteStrings</a>  
A set of `AnsiByteString`s collected together, in order to be written
with a minimum of control characters.

<a href="type.AnsiString.html" class="type"
title="type nu_ansi_term::AnsiString">AnsiString</a>  
An ANSI String is a string coupled with the `Style` to display it in a
terminal.

<a href="type.AnsiStrings.html" class="type"
title="type nu_ansi_term::AnsiStrings">AnsiStrings</a>  
A set of `AnsiString`s collected together, in order to be written with a
minimum of control characters.
