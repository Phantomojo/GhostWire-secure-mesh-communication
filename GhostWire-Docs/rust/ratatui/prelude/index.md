[ratatui](../index.html)

Module prelude Copy item path

<span class="sub-heading"><a href="../../src/ratatui/prelude.rs.html#1-45" class="src">Source</a>
</span>

Expand description

A prelude for conveniently writing applications using this library.

The prelude module is no longer used universally in Ratatui, as it can
make it harder to distinguish between library and non-library types,
especially when viewing source code outside of an IDE (such as on GitHub
or in a git diff). For more details and user feedback, see [Issue
\#1150](https://github.com/ratatui/ratatui/issues/1150). However, the
prelude is still available for backward compatibility and for those who
prefer to use it.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use ratatui::prelude::*;
```

Aside from the main types that are used in the library, this prelude
also re-exports several modules to make it easy to qualify types that
would otherwise collide. E.g.:

``` {.rust .rust-example-rendered}
use ratatui::{prelude::*, widgets::*};

#[derive(Debug, Default, PartialEq, Eq)]
struct Line;

assert_eq!(Line::default(), Line);
assert_eq!(text::Line::default(), ratatui::text::Line::from(vec![]));
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::backend::`<a href="../backend/struct.CrosstermBackend.html" class="struct"
title="struct ratatui::backend::CrosstermBackend"><code>CrosstermBackend</code></a>`;`

`pub use crate::`<a href="../backend/index.html" class="mod"
title="mod ratatui::backend"><code>backend</code></a>`;`

`pub use crate::backend::`<a href="../backend/trait.Backend.html" class="trait"
title="trait ratatui::backend::Backend"><code>Backend</code></a>`;`

`pub use crate::`<a href="../buffer/index.html" class="mod"
title="mod ratatui::buffer"><code>buffer</code></a>`;`

`pub use crate::buffer::`<a href="../buffer/struct.Buffer.html" class="struct"
title="struct ratatui::buffer::Buffer"><code>Buffer</code></a>`;`

`pub use crate::`<a href="../layout/index.html" class="mod"
title="mod ratatui::layout"><code>layout</code></a>`;`

`pub use crate::layout::`<a href="../layout/enum.Alignment.html" class="enum"
title="enum ratatui::layout::Alignment"><code>Alignment</code></a>`;`

`pub use crate::layout::`<a href="../layout/enum.Constraint.html" class="enum"
title="enum ratatui::layout::Constraint"><code>Constraint</code></a>`;`

`pub use crate::layout::`<a href="../layout/enum.Direction.html" class="enum"
title="enum ratatui::layout::Direction"><code>Direction</code></a>`;`

`pub use crate::layout::`<a href="../layout/struct.Layout.html" class="struct"
title="struct ratatui::layout::Layout"><code>Layout</code></a>`;`

`pub use crate::layout::`<a href="../layout/struct.Margin.html" class="struct"
title="struct ratatui::layout::Margin"><code>Margin</code></a>`;`

`pub use crate::layout::`<a href="../layout/struct.Position.html" class="struct"
title="struct ratatui::layout::Position"><code>Position</code></a>`;`

`pub use crate::layout::`<a href="../layout/struct.Rect.html" class="struct"
title="struct ratatui::layout::Rect"><code>Rect</code></a>`;`

`pub use crate::layout::`<a href="../layout/struct.Size.html" class="struct"
title="struct ratatui::layout::Size"><code>Size</code></a>`;`

`pub use crate::`<a href="../style/index.html" class="mod"
title="mod ratatui::style"><code>style</code></a>`;`

`pub use crate::style::`<a href="../style/enum.Color.html" class="enum"
title="enum ratatui::style::Color"><code>Color</code></a>`;`

`pub use crate::style::`<a href="../style/struct.Modifier.html" class="struct"
title="struct ratatui::style::Modifier"><code>Modifier</code></a>`;`

`pub use crate::style::`<a href="../style/struct.Style.html" class="struct"
title="struct ratatui::style::Style"><code>Style</code></a>`;`

`pub use crate::style::`<a href="../style/trait.Stylize.html" class="trait"
title="trait ratatui::style::Stylize"><code>Stylize</code></a>`;`

`pub use crate::`<a href="../symbols/index.html" class="mod"
title="mod ratatui::symbols"><code>symbols</code></a>`;`

`pub use crate::`<a href="../text/index.html" class="mod"
title="mod ratatui::text"><code>text</code></a>`;`

`pub use crate::text::`<a href="../text/struct.Line.html" class="struct"
title="struct ratatui::text::Line"><code>Line</code></a>`;`

`pub use crate::text::`<a href="../text/struct.Masked.html" class="struct"
title="struct ratatui::text::Masked"><code>Masked</code></a>`;`

`pub use crate::text::`<a href="../text/struct.Span.html" class="struct"
title="struct ratatui::text::Span"><code>Span</code></a>`;`

`pub use crate::text::`<a href="../text/struct.Text.html" class="struct"
title="struct ratatui::text::Text"><code>Text</code></a>`;`

`pub use crate::widgets::block::`<a href="../widgets/block/trait.BlockExt.html" class="trait"
title="trait ratatui::widgets::block::BlockExt"><code>BlockExt</code></a>`;`

`pub use crate::widgets::`<a href="../widgets/trait.StatefulWidget.html" class="trait"
title="trait ratatui::widgets::StatefulWidget"><code>StatefulWidget</code></a>`;`

`pub use crate::widgets::`<a href="../widgets/trait.Widget.html" class="trait"
title="trait ratatui::widgets::Widget"><code>Widget</code></a>`;`

`pub use crate::`<a href="../struct.Frame.html" class="struct"
title="struct ratatui::Frame"><code>Frame</code></a>`;`

`pub use crate::`<a href="../struct.Terminal.html" class="struct"
title="struct ratatui::Terminal"><code>Terminal</code></a>`;`
