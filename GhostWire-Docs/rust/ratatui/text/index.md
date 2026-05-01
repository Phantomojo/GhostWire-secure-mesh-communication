[ratatui](../index.html)

Module text Copy item path

<span class="sub-heading"><a href="../../src/ratatui/text.rs.html#1-64" class="src">Source</a>
</span>

Expand description

Primitives for styled text.

A terminal UI is at its root a lot of strings. In order to make it
accessible and stylish, those strings may be associated to a set of
styles. `ratatui` has three ways to represent them:

-   A single line string where all graphemes have the same style is
    represented by a
    [`Span`](struct.Span.html "struct ratatui::text::Span").
-   A single line string where each grapheme may have its own style is
    represented by
    [`Line`](struct.Line.html "struct ratatui::text::Line").
-   A multiple line string where each grapheme may have its own style is
    represented by a
    [`Text`](struct.Text.html "struct ratatui::text::Text").

These types form a hierarchy:
[`Line`](struct.Line.html "struct ratatui::text::Line") is a collection
of [`Span`](struct.Span.html "struct ratatui::text::Span") and each line
of [`Text`](struct.Text.html "struct ratatui::text::Text") is a
[`Line`](struct.Line.html "struct ratatui::text::Line").

Keep it mind that a lot of widgets will use those types to advertise
what kind of string is supported for their properties. Moreover,
`ratatui` provides convenient `From` implementations so that you can
start by using simple `String` or `&str` and then promote them to the
previous primitives when you need additional styling capabilities.

For example, for the
[`crate::widgets::Block`](../widgets/block/struct.Block.html "struct ratatui::widgets::block::Block")
widget, all the following calls are valid to set its `title` property
(which is a [`Line`](struct.Line.html "struct ratatui::text::Line")
under the hood):

``` {.rust .rust-example-rendered}
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::Block,
};

// A simple string with no styling.
// Converted to Line(vec![
//   Span { content: Cow::Borrowed("My title"), style: Style { .. } }
// ])
let block = Block::new().title("My title");

// A simple string with a unique style.
// Converted to Line(vec![
//   Span { content: Cow::Borrowed("My title"), style: Style { fg: Some(Color::Yellow), .. }
// ])
let block = Block::new().title(Span::styled("My title", Style::default().fg(Color::Yellow)));

// A string with multiple styles.
// Converted to Line(vec![
//   Span { content: Cow::Borrowed("My"), style: Style { fg: Some(Color::Yellow), .. } },
//   Span { content: Cow::Borrowed(" title"), .. }
// ])
let block = Block::new().title(vec![
    Span::styled("My", Style::default().fg(Color::Yellow)),
    Span::raw(" title"),
]);
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Line.html" class="struct"
title="struct ratatui::text::Line">Line</a>  
A line of text, consisting of one or more
[`Span`](struct.Span.html "struct ratatui::text::Span")s.

<a href="struct.Masked.html" class="struct"
title="struct ratatui::text::Masked">Masked</a>  
A wrapper around a string that is masked when displayed.

<a href="struct.Span.html" class="struct"
title="struct ratatui::text::Span">Span</a>  
Represents a part of a line that is contiguous and where all characters
share the same style.

<a href="struct.StyledGrapheme.html" class="struct"
title="struct ratatui::text::StyledGrapheme">StyledGrapheme</a>  
A grapheme associated to a style. Note that, although `StyledGrapheme`
is the smallest divisible unit of text, it actually is not a member of
the text type hierarchy (`Text` -&gt; `Line` -&gt; `Span`). It is a
separate type used mostly for rendering purposes. A `Span` consists of
components that can be split into `StyledGrapheme`s, but it does not
contain a collection of `StyledGrapheme`s.

<a href="struct.Text.html" class="struct"
title="struct ratatui::text::Text">Text</a>  
A string split over one or more lines.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.ToLine.html" class="trait"
title="trait ratatui::text::ToLine">ToLine</a>  
A trait for converting a value to a
[`Line`](struct.Line.html "struct ratatui::text::Line").

<a href="trait.ToSpan.html" class="trait"
title="trait ratatui::text::ToSpan">ToSpan</a>  
A trait for converting a value to a
[`Span`](struct.Span.html "struct ratatui::text::Span").

<a href="trait.ToText.html" class="trait"
title="trait ratatui::text::ToText">ToText</a>  
A trait for converting a value to a
[`Text`](struct.Text.html "struct ratatui::text::Text").
