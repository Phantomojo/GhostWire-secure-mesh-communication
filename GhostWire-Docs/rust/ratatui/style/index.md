[ratatui](../index.html)

Module style Copy item path

<span class="sub-heading"><a href="../../src/ratatui/style.rs.html#1-905" class="src">Source</a>
</span>

Expand description

`style` contains the primitives used to control how your user interface
will look.

There are two ways to set styles:

-   Creating and using the
    [`Style`](struct.Style.html "struct ratatui::style::Style") struct.
    (e.g. `Style::new().fg(Color::Red)`).
-   Using style shorthands. (e.g. `"hello".red()`).

# <a href="#using-the-style-struct" class="doc-anchor">§</a>Using the `Style` struct

This is the original approach to styling and likely the most common.
This is useful when creating style variables to reuse, however the
shorthands are often more convenient and readable for most use cases.

## <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

let heading_style = Style::new()
    .fg(Color::Black)
    .bg(Color::Green)
    .add_modifier(Modifier::ITALIC | Modifier::BOLD);
let span = Span::styled("hello", heading_style);
```

# <a href="#using-style-shorthands" class="doc-anchor">§</a>Using style shorthands

Originally Ratatui only had the ability to set styles using the `Style`
struct. This is still supported, but there are now shorthands for all
the styles that can be set. These save you from having to create a
`Style` struct every time you want to set a style.

The shorthands are implemented in the
[`Stylize`](trait.Stylize.html "trait ratatui::style::Stylize") trait
which is automatically implemented for many types via the
[`Styled`](trait.Styled.html "trait ratatui::style::Styled") trait. This
means that you can use the shorthands on any type that implements
[`Styled`](trait.Styled.html "trait ratatui::style::Styled"). E.g.:

-   Strings and string slices when styled return a
    [`Span`](../text/struct.Span.html "struct ratatui::text::Span")
-   [`Span`](../text/struct.Span.html "struct ratatui::text::Span")s can
    be styled again, which will merge the styles.
-   Many widget types can be styled directly rather than calling their
    `style()` method.

See the [`Stylize`](trait.Stylize.html "trait ratatui::style::Stylize")
and [`Styled`](trait.Styled.html "trait ratatui::style::Styled") traits
for more information.

## <a href="#example-1" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use ratatui::{
    style::{Color, Modifier, Style, Stylize},
    text::Span,
    widgets::Paragraph,
};

assert_eq!(
    "hello".red().on_blue().bold(),
    Span::styled(
        "hello",
        Style::default()
            .fg(Color::Red)
            .bg(Color::Blue)
            .add_modifier(Modifier::BOLD)
    )
);

assert_eq!(
    Paragraph::new("hello").red().on_blue().bold(),
    Paragraph::new("hello").style(
        Style::default()
            .fg(Color::Red)
            .bg(Color::Blue)
            .add_modifier(Modifier::BOLD)
    )
);
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="palette/index.html" class="mod"
title="mod ratatui::style::palette">palette</a>  
A module for defining color palettes.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Modifier.html" class="struct"
title="struct ratatui::style::Modifier">Modifier</a>  
Modifier changes the way a piece of text is displayed.

<a href="struct.ParseColorError.html" class="struct"
title="struct ratatui::style::ParseColorError">ParseColorError</a>  
Error type indicating a failure to parse a color string.

<a href="struct.Style.html" class="struct"
title="struct ratatui::style::Style">Style</a>  
Style lets you control the main characteristics of the displayed
elements.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Color.html" class="enum"
title="enum ratatui::style::Color">Color</a>  
ANSI Color

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Styled.html" class="trait"
title="trait ratatui::style::Styled">Styled</a>  
A trait for objects that have a `Style`.

<a href="trait.Stylize.html" class="trait"
title="trait ratatui::style::Stylize">Stylize</a>  
An extension trait for styling objects.
