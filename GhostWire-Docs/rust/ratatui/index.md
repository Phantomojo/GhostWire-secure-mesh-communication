Crate ratatui Copy item path

<span class="sub-heading"><a href="../src/ratatui/lib.rs.html#1-354" class="src">Source</a>
</span>

Expand description

![Demo](https://github.com/ratatui/ratatui/blob/87ae72dbc756067c97f6400d3e2a58eeb383776e/examples/demo2-destroy.gif?raw=true)

[![Crate
Badge](https://img.shields.io/crates/v/ratatui?logo=rust&style=flat-square&logoColor=E05D44&color=E05D44)](https://crates.io/crates/ratatui)
[![Docs
Badge](https://img.shields.io/docsrs/ratatui?logo=rust&style=flat-square&logoColor=E05D44)](https://docs.rs/ratatui)
[![CI
Badge](https://img.shields.io/github/actions/workflow/status/ratatui/ratatui/ci.yml?style=flat-square&logo=github)](https://github.com/ratatui/ratatui/actions/workflows/ci.yml)
[![Deps.rs
Badge](https://deps.rs/repo/github/ratatui/ratatui/status.svg?style=flat-square)](https://deps.rs/repo/github/ratatui/ratatui)  
[![Codecov
Badge](https://img.shields.io/codecov/c/github/ratatui/ratatui?logo=codecov&style=flat-square&token=BAQ8SOKEST&color=C43AC3&logoColor=C43AC3)](https://app.codecov.io/gh/ratatui/ratatui)
[![License
Badge](https://img.shields.io/crates/l/ratatui?style=flat-square&color=1370D3)](./LICENSE)
[![Sponsors
Badge](https://img.shields.io/github/sponsors/ratatui?logo=github&style=flat-square&color=1370D3)](https://github.com/sponsors/ratatui)  
[![Discord
Badge](https://img.shields.io/discord/1070692720437383208?label=discord&logo=discord&style=flat-square&color=1370D3&logoColor=1370D3)](https://discord.gg/pMCEU9hNEj)
[![Matrix
Badge](https://img.shields.io/matrix/ratatui-general%3Amatrix.org?style=flat-square&logo=matrix&label=Matrix&color=C43AC3)](https://matrix.to/#/#ratatui:matrix.org)
[![Forum
Badge](https://img.shields.io/discourse/likes?server=https%3A%2F%2Fforum.ratatui.rs&style=flat-square&logo=discourse&label=forum&color=C43AC3)](https://forum.ratatui.rs)  

[Ratatui Website](https://ratatui.rs/) · [API
Docs](https://docs.rs/ratatui) ·
[Examples](https://github.com/ratatui/ratatui/tree/main/examples/README.md)
· [Changelog](https://github.com/ratatui/ratatui/blob/main/CHANGELOG.md)
· [Breaking
Changes](https://github.com/ratatui/ratatui/blob/main/BREAKING-CHANGES.md)  
[Contributing](https://github.com/ratatui/ratatui/blob/main/CONTRIBUTING.md)
· [Report a
bug](https://github.com/ratatui/ratatui/issues/new?labels=bug&projects=&template=bug_report.md)
· [Request a
Feature](https://github.com/ratatui/ratatui/issues/new?labels=enhancement&projects=&template=feature_request.md)
· [Create a Pull Request](https://github.com/ratatui/ratatui/compare)

# <a href="#ratatui" class="doc-anchor">§</a>Ratatui

[Ratatui](https://ratatui.rs/) is a crate for cooking up terminal user
interfaces in Rust. It is a lightweight library that provides a set of
widgets and utilities to build complex Rust TUIs. Ratatui was forked
from the [tui-rs](https://crates.io/crates/tui) crate in 2023 in order
to continue its development.

## <a href="#quickstart" class="doc-anchor">§</a>Quickstart

Add `ratatui` and `crossterm` as dependencies to your cargo.toml:

``` language-shell
cargo add ratatui crossterm
```

Then you can create a simple “Hello World” application:

``` {.rust .rust-example-rendered}
use crossterm::event::{self, Event};
use ratatui::{text::Text, Frame};

fn main() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
}
```

The full code for this example which contains a little more detail is in
the
[Examples](https://github.com/ratatui/ratatui/tree/main/examples/README.md)
directory. For more guidance on different ways to structure your
application see the [Application
Patterns](https://ratatui.rs/concepts/application-patterns/) and [Hello
World tutorial](https://ratatui.rs/tutorials/hello-world/) sections in
the [Ratatui Website](https://ratatui.rs/) and the various
[Examples](https://github.com/ratatui/ratatui/tree/main/examples/README.md).
There are also several starter templates available in the
[templates](https://github.com/ratatui/templates/) repository.

## <a href="#other-documentation" class="doc-anchor">§</a>Other documentation

-   [Ratatui Website](https://ratatui.rs/) - explains the library’s
    concepts and provides step-by-step tutorials
-   [Ratatui Forum](https://forum.ratatui.rs) - a place to ask questions
    and discuss the library
-   [API Docs](https://docs.rs/ratatui) - the full API documentation for
    the library on docs.rs.
-   [Examples](https://github.com/ratatui/ratatui/tree/main/examples/README.md) -
    a collection of examples that demonstrate how to use the library.
-   [Contributing](https://github.com/ratatui/ratatui/blob/main/CONTRIBUTING.md) -
    Please read this if you are interested in contributing to the
    project.
-   [Changelog](https://github.com/ratatui/ratatui/blob/main/CHANGELOG.md) -
    generated by [git-cliff](https://git-cliff.org) utilizing
    [Conventional Commits](https://www.conventionalcommits.org).
-   [Breaking
    Changes](https://github.com/ratatui/ratatui/blob/main/BREAKING-CHANGES.md) -
    a list of breaking changes in the library.

You can also watch the [FOSDEM 2024
talk](https://www.youtube.com/watch?v=NU0q6NOLJ20) about Ratatui which
gives a brief introduction to terminal user interfaces and showcases the
features of Ratatui, along with a hello world demo.

## <a href="#introduction" class="doc-anchor">§</a>Introduction

Ratatui is based on the principle of immediate rendering with
intermediate buffers. This means that for each frame, your app must
render all widgets that are supposed to be part of the UI. This is in
contrast to the retained mode style of rendering where widgets are
updated and then automatically redrawn on the next frame. See the
[Rendering](https://ratatui.rs/concepts/rendering/) section of the
[Ratatui Website](https://ratatui.rs/) for more info.

Ratatui uses [Crossterm](https://crates.io/crates/crossterm) by default
as it works on most platforms. See the
[Installation](https://ratatui.rs/installation/) section of the [Ratatui
Website](https://ratatui.rs/) for more details on how to use other
backends ([Termion](https://crates.io/crates/termion) /
[Termwiz](https://crates.io/crates/termwiz)).

Every application built with `ratatui` needs to implement the following
steps:

-   Initialize the terminal
-   A main loop that:
    -   Draws the UI
    -   Handles input events
-   Restore the terminal state

### <a href="#initialize-and-restore-the-terminal" class="doc-anchor">§</a>Initialize and restore the terminal

The [`Terminal`](struct.Terminal.html "struct ratatui::Terminal") type
is the main entry point for any Ratatui application. It is generic over
a a choice of
[`Backend`](backend/trait.Backend.html "trait ratatui::backend::Backend")
implementations that each provide functionality to draw frames, clear
the screen, hide the cursor, etc. There are backend implementations for
[Crossterm](https://crates.io/crates/crossterm),
[Termion](https://crates.io/crates/termion) and
[Termwiz](https://crates.io/crates/termwiz).

The simplest way to initialize the terminal is to use the
[`init`](fn.init.html "fn ratatui::init") function which returns a
[`DefaultTerminal`](type.DefaultTerminal.html "type ratatui::DefaultTerminal")
instance with the default options, enters the Alternate Screen and Raw
mode and sets up a panic hook that restores the terminal in case of
panic. This instance can then be used to draw frames and interact with
the terminal state. (The
[`DefaultTerminal`](type.DefaultTerminal.html "type ratatui::DefaultTerminal")
instance is a type alias for a terminal with the
[`crossterm`](../crossterm/index.html "mod crossterm") backend.) The
[`restore`](fn.restore.html "fn ratatui::restore") function restores the
terminal to its original state.

``` {.rust .rust-example-rendered}
fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}
```

See the [`backend` module](backend/index.html "mod ratatui::backend")
and the [Backends](https://ratatui.rs/concepts/backends/) section of the
[Ratatui Website](https://ratatui.rs/) for more info on the alternate
screen and raw mode.

### <a href="#drawing-the-ui" class="doc-anchor">§</a>Drawing the UI

Drawing the UI is done by calling the
[`Terminal::draw`](struct.Terminal.html#method.draw "method ratatui::Terminal::draw")
method on the terminal instance. This method takes a closure that is
called with a [`Frame`](struct.Frame.html "struct ratatui::Frame")
instance. The [`Frame`](struct.Frame.html "struct ratatui::Frame")
provides the size of the area to draw to and allows the app to render
any
[`Widget`](widgets/trait.Widget.html "trait ratatui::widgets::Widget")
using the provided
[`render_widget`](struct.Frame.html#method.render_widget "method ratatui::Frame::render_widget")
method. After this closure returns, a diff is performed and only the
changes are drawn to the terminal. See the
[Widgets](https://ratatui.rs/how-to/widgets/) section of the [Ratatui
Website](https://ratatui.rs/) for more info.

The closure passed to the
[`Terminal::draw`](struct.Terminal.html#method.draw "method ratatui::Terminal::draw")
method should handle the rendering of a full frame.

``` {.rust .rust-example-rendered}
use ratatui::{widgets::Paragraph, Frame};

fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| draw(frame))?;
        if handle_events()? {
            break Ok(());
        }
    }
}

fn draw(frame: &mut Frame) {
    let text = Paragraph::new("Hello World!");
    frame.render_widget(text, frame.area());
}
```

### <a href="#handling-events" class="doc-anchor">§</a>Handling events

Ratatui does not include any input handling. Instead event handling can
be implemented by calling backend library methods directly. See the
[Handling Events](https://ratatui.rs/concepts/event-handling/) section
of the [Ratatui Website](https://ratatui.rs/) for more info. For
example, if you are using
[Crossterm](https://crates.io/crates/crossterm), you can use the
[`crossterm::event`](https://docs.rs/crossterm/latest/crossterm/event/index.html)
module to handle events.

``` {.rust .rust-example-rendered}
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

fn handle_events() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            // handle other key events
            _ => {}
        },
        // handle other events
        _ => {}
    }
    Ok(false)
}
```

## <a href="#layout-1" class="doc-anchor">§</a>Layout

The library comes with a basic yet useful layout management object
called
[`Layout`](layout/struct.Layout.html "struct ratatui::layout::Layout")
which allows you to split the available space into multiple areas and
then render widgets in each area. This lets you describe a responsive
terminal UI by nesting layouts. See the
[Layout](https://ratatui.rs/how-to/layout/) section of the [Ratatui
Website](https://ratatui.rs/) for more info.

``` {.rust .rust-example-rendered}
use ratatui::{
    layout::{Constraint, Layout},
    widgets::Block,
    Frame,
};

fn draw(frame: &mut Frame) {
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [left_area, right_area] = horizontal.areas(main_area);

    frame.render_widget(Block::bordered().title("Title Bar"), title_area);
    frame.render_widget(Block::bordered().title("Status Bar"), status_area);
    frame.render_widget(Block::bordered().title("Left"), left_area);
    frame.render_widget(Block::bordered().title("Right"), right_area);
}
```

Running this example produces the following output:

``` language-text
Title Bar───────────────────────────────────
┌Left────────────────┐┌Right───────────────┐
│                    ││                    │
└────────────────────┘└────────────────────┘
Status Bar──────────────────────────────────
```

## <a href="#text-and-styling" class="doc-anchor">§</a>Text and styling

The [`Text`](text/struct.Text.html "struct ratatui::text::Text"),
[`Line`](text/struct.Line.html "struct ratatui::text::Line") and
[`Span`](text/struct.Span.html "struct ratatui::text::Span") types are
the building blocks of the library and are used in many places.
[`Text`](text/struct.Text.html "struct ratatui::text::Text") is a list
of [`Line`](text/struct.Line.html "struct ratatui::text::Line")s and a
[`Line`](text/struct.Line.html "struct ratatui::text::Line") is a list
of [`Span`](text/struct.Span.html "struct ratatui::text::Span")s. A
[`Span`](text/struct.Span.html "struct ratatui::text::Span") is a string
with a specific style.

The [`style` module](style/index.html "mod ratatui::style") provides
types that represent the various styling options. The most important one
is [`Style`](style/struct.Style.html "struct ratatui::style::Style")
which represents the foreground and background colors and the text
attributes of a
[`Span`](text/struct.Span.html "struct ratatui::text::Span"). The
[`style` module](style/index.html "mod ratatui::style") also provides a
[`Stylize`](style/trait.Stylize.html "trait ratatui::style::Stylize")
trait that allows short-hand syntax to apply a style to widgets and
text. See the [Styling
Text](https://ratatui.rs/how-to/render/style-text/) section of the
[Ratatui Website](https://ratatui.rs/) for more info.

``` {.rust .rust-example-rendered}
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

fn draw(frame: &mut Frame) {
    let areas = Layout::vertical([Constraint::Length(1); 4]).split(frame.area());

    let line = Line::from(vec![
        Span::raw("Hello "),
        Span::styled(
            "World",
            Style::new()
                .fg(Color::Green)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        "!".red().on_light_yellow().italic(),
    ]);
    frame.render_widget(line, areas[0]);

    // using the short-hand syntax and implicit conversions
    let paragraph = Paragraph::new("Hello World!".red().on_white().bold());
    frame.render_widget(paragraph, areas[1]);

    // style the whole widget instead of just the text
    let paragraph = Paragraph::new("Hello World!").style(Style::new().red().on_white());
    frame.render_widget(paragraph, areas[2]);

    // use the simpler short-hand syntax
    let paragraph = Paragraph::new("Hello World!").blue().on_yellow();
    frame.render_widget(paragraph, areas[3]);
}
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../crossterm/index.html" class="mod"
title="mod crossterm"><code>crossterm</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="backend/index.html" class="mod"
title="mod ratatui::backend">backend</a>  
This module provides the backend implementations for different terminal
libraries.

<a href="buffer/index.html" class="mod"
title="mod ratatui::buffer">buffer</a>  
A module for the
[`Buffer`](buffer/struct.Buffer.html "struct ratatui::buffer::Buffer")
and [`Cell`](buffer/struct.Cell.html "struct ratatui::buffer::Cell")
types.

<a href="layout/index.html" class="mod"
title="mod ratatui::layout">layout</a>  
<a href="prelude/index.html" class="mod"
title="mod ratatui::prelude">prelude</a>  
A prelude for conveniently writing applications using this library.

<a href="style/index.html" class="mod"
title="mod ratatui::style">style</a>  
`style` contains the primitives used to control how your user interface
will look.

<a href="symbols/index.html" class="mod"
title="mod ratatui::symbols">symbols</a>  
<a href="text/index.html" class="mod" title="mod ratatui::text">text</a>  
Primitives for styled text.

<a href="widgets/index.html" class="mod"
title="mod ratatui::widgets">widgets</a>  
`widgets` is a collection of types that implement
[`Widget`](widgets/trait.Widget.html "trait ratatui::widgets::Widget")
or
[`StatefulWidget`](widgets/trait.StatefulWidget.html "trait ratatui::widgets::StatefulWidget")
or both.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.assert_buffer_eq.html" class="macro"
title="macro ratatui::assert_buffer_eq">assert_buffer_eq</a><span class="stab deprecated" title="">Deprecated</span>  
Assert that two buffers are equal by comparing their areas and content.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.CompletedFrame.html" class="struct"
title="struct ratatui::CompletedFrame">CompletedFrame</a>  
`CompletedFrame` represents the state of the terminal after all changes
performed in the last
[`Terminal::draw`](struct.Terminal.html#method.draw "method ratatui::Terminal::draw")
call have been applied. Therefore, it is only valid until the next call
to
[`Terminal::draw`](struct.Terminal.html#method.draw "method ratatui::Terminal::draw").

<a href="struct.Frame.html" class="struct"
title="struct ratatui::Frame">Frame</a>  
A consistent view into the terminal state for rendering a single frame.

<a href="struct.Terminal.html" class="struct"
title="struct ratatui::Terminal">Terminal</a>  
An interface to interact and draw
[`Frame`](struct.Frame.html "struct ratatui::Frame")s on the user’s
terminal.

<a href="struct.TerminalOptions.html" class="struct"
title="struct ratatui::TerminalOptions">TerminalOptions</a>  
Options to pass to
[`Terminal::with_options`](struct.Terminal.html#method.with_options "associated function ratatui::Terminal::with_options")

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Viewport.html" class="enum"
title="enum ratatui::Viewport">Viewport</a>  
Represents the viewport of the terminal. The viewport is the area of the
terminal that is currently visible to the user. It can be either
fullscreen, inline or fixed.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.init.html" class="fn" title="fn ratatui::init">init</a>  
Initialize a terminal with reasonable defaults for most applications.

<a href="fn.init_with_options.html" class="fn"
title="fn ratatui::init_with_options">init_with_options</a>  
Initialize a terminal with the given options and reasonable defaults.

<a href="fn.restore.html" class="fn"
title="fn ratatui::restore">restore</a>  
Restores the terminal to its original state.

<a href="fn.try_init.html" class="fn"
title="fn ratatui::try_init">try_init</a>  
Try to initialize a terminal using reasonable defaults for most
applications.

<a href="fn.try_init_with_options.html" class="fn"
title="fn ratatui::try_init_with_options">try_init_with_options</a>  
Try to initialize a terminal with the given options and reasonable
defaults.

<a href="fn.try_restore.html" class="fn"
title="fn ratatui::try_restore">try_restore</a>  
Restore the terminal to its original state.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.DefaultTerminal.html" class="type"
title="type ratatui::DefaultTerminal">DefaultTerminal</a>  
A type alias for the default terminal type.
