[ratatui](../index.html)

Module backend Copy item path

<span class="sub-heading"><a href="../../src/ratatui/backend.rs.html#1-426" class="src">Source</a>
</span>

Expand description

This module provides the backend implementations for different terminal
libraries.

It defines the
[`Backend`](trait.Backend.html "trait ratatui::backend::Backend") trait
which is used to abstract over the specific terminal library being used.

Supported terminal backends:

-   [Crossterm](https://crates.io/crates/crossterm): enable the
    `crossterm` feature (enabled by default) and use
    [`CrosstermBackend`](struct.CrosstermBackend.html "struct ratatui::backend::CrosstermBackend")
-   [Termion](https://crates.io/crates/termion): enable the `termion`
    feature and use
    [`TermionBackend`](termion/struct.TermionBackend.html)
-   [Termwiz](https://crates.io/crates/termwiz): enable the `termwiz`
    feature and use \[`TermwizBackend`\]

Additionally, a
[`TestBackend`](struct.TestBackend.html "struct ratatui::backend::TestBackend")
is provided for testing purposes.

See the [Backend
Comparison](https://ratatui.rs/concepts/backends/comparison/) section of
the [Ratatui Website](https://ratatui.rs) for more details on the
different backends.

Each backend supports a number of features, such as [raw
mode](#raw-mode), [alternate screen](#alternate-screen), and [mouse
capture](#mouse-capture). These features are generally not enabled by
default, and must be enabled by the application before they can be used.
See the documentation for each backend for more details.

Note: most applications should use the
[`Terminal`](../struct.Terminal.html "struct ratatui::Terminal") struct
instead of directly calling methods on the backend.

# <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use std::io::stdout;

use ratatui::{backend::CrosstermBackend, Terminal};

let backend = CrosstermBackend::new(stdout());
let mut terminal = Terminal::new(backend)?;
terminal.clear()?;
terminal.draw(|frame| {
    // -- snip --
})?;
```

See the the
[Examples](https://github.com/ratatui/ratatui/tree/main/examples/README.md)
directory for more examples.

# <a href="#raw-mode" class="doc-anchor">§</a>Raw Mode

Raw mode is a mode where the terminal does not perform any processing or
handling of the input and output. This means that features such as
echoing input characters, line buffering, and special character
processing (e.g., CTRL-C for SIGINT) are disabled. This is useful for
applications that want to have complete control over the terminal input
and output, processing each keystroke themselves.

For example, in raw mode, the terminal will not perform line buffering
on the input, so the application will receive each key press as it is
typed, instead of waiting for the user to press enter. This makes it
suitable for real-time applications like text editors, terminal-based
games, and more.

Each backend handles raw mode differently, so the behavior may vary
depending on the backend being used. Be sure to consult the backend’s
specific documentation for exact details on how it implements raw mode.

# <a href="#alternate-screen" class="doc-anchor">§</a>Alternate Screen

The alternate screen is a separate buffer that some terminals provide,
distinct from the main screen. When activated, the terminal will display
the alternate screen, hiding the current content of the main screen.
Applications can write to this screen as if it were the regular terminal
display, but when the application exits, the terminal will switch back
to the main screen, and the contents of the alternate screen will be
cleared. This is useful for applications like text editors or terminal
games that want to use the full terminal window without disrupting the
command line or other terminal content.

This creates a seamless transition between the application and the
regular terminal session, as the content displayed before launching the
application will reappear after the application exits.

Note that not all terminal emulators support the alternate screen, and
even those that do may handle it differently. As a result, the behavior
may vary depending on the backend being used. Always consult the
specific backend’s documentation to understand how it implements the
alternate screen.

# <a href="#mouse-capture" class="doc-anchor">§</a>Mouse Capture

Mouse capture is a mode where the terminal captures mouse events such as
clicks, scrolls, and movement, and sends them to the application as
special sequences or events. This enables the application to handle and
respond to mouse actions, providing a more interactive and graphical
user experience within the terminal. It’s particularly useful for
applications like terminal-based games, text editors, or other programs
that require more direct interaction from the user.

Each backend handles mouse capture differently, with variations in the
types of events that can be captured and how they are represented. As
such, the behavior may vary depending on the backend being used, and
developers should consult the specific backend’s documentation to
understand how it implements mouse capture.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.CrosstermBackend.html" class="struct"
title="struct ratatui::backend::CrosstermBackend">CrosstermBackend</a>  
A [`Backend`](index.html "mod ratatui::backend") implementation that
uses [Crossterm](https://crates.io/crates/crossterm) to render to the
terminal.

<a href="struct.TestBackend.html" class="struct"
title="struct ratatui::backend::TestBackend">TestBackend</a>  
A [`Backend`](trait.Backend.html "trait ratatui::backend::Backend")
implementation used for integration testing that renders to an memory
buffer.

<a href="struct.WindowSize.html" class="struct"
title="struct ratatui::backend::WindowSize">WindowSize</a>  
The window size in characters (columns / rows) as well as pixels.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ClearType.html" class="enum"
title="enum ratatui::backend::ClearType">ClearType</a>  
Enum representing the different types of clearing operations that can be
performed on the terminal screen.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Backend.html" class="trait"
title="trait ratatui::backend::Backend">Backend</a>  
The `Backend` trait provides an abstraction over different terminal
libraries. It defines the methods required to draw content, manipulate
the cursor, and clear the terminal screen.
