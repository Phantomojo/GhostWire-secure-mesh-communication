[ratatui](../index.html)

Module widgets Copy item path

<span class="sub-heading"><a href="../../src/ratatui/widgets.rs.html#1-746" class="src">Source</a>
</span>

Expand description

`widgets` is a collection of types that implement
[`Widget`](trait.Widget.html "trait ratatui::widgets::Widget") or
[`StatefulWidget`](trait.StatefulWidget.html "trait ratatui::widgets::StatefulWidget")
or both.

Widgets are created for each frame as they are consumed after rendered.
They are not meant to be stored but used as *commands* to draw common
figures in the UI.

The available widgets are:

-   [`Block`](block/struct.Block.html "struct ratatui::widgets::block::Block"):
    a basic widget that draws a block with optional borders, titles and
    styles.
-   [`BarChart`](struct.BarChart.html "struct ratatui::widgets::BarChart"):
    displays multiple datasets as bars with optional grouping.
-   \[`calendar::Monthly`\]: displays a single month.
-   [`Canvas`](canvas/struct.Canvas.html "struct ratatui::widgets::canvas::Canvas"):
    draws arbitrary shapes using drawing characters.
-   [`Chart`](struct.Chart.html "struct ratatui::widgets::Chart"):
    displays multiple datasets as a lines or scatter graph.
-   [`Clear`](struct.Clear.html "struct ratatui::widgets::Clear"):
    clears the area it occupies. Useful to render over previously drawn
    widgets.
-   [`Gauge`](struct.Gauge.html "struct ratatui::widgets::Gauge"):
    displays progress percentage using block characters.
-   [`LineGauge`](struct.LineGauge.html "struct ratatui::widgets::LineGauge"):
    display progress as a line.
-   [`List`](struct.List.html "struct ratatui::widgets::List"): displays
    a list of items and allows selection.
-   [`Paragraph`](struct.Paragraph.html "struct ratatui::widgets::Paragraph"):
    displays a paragraph of optionally styled and wrapped text.
-   [`Scrollbar`](struct.Scrollbar.html "struct ratatui::widgets::Scrollbar"):
    displays a scrollbar.
-   [`Sparkline`](struct.Sparkline.html "struct ratatui::widgets::Sparkline"):
    display a single data set as a sparkline.
-   [`Table`](struct.Table.html "struct ratatui::widgets::Table"):
    displays multiple rows and columns in a grid and allows selection.
-   [`Tabs`](struct.Tabs.html "struct ratatui::widgets::Tabs"): displays
    a tab bar and allows selection.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use self::block::`<a href="block/struct.Block.html" class="struct"
title="struct ratatui::widgets::block::Block"><code>Block</code></a>`;`

`pub use self::block::`<a href="block/enum.BorderType.html" class="enum"
title="enum ratatui::widgets::block::BorderType"><code>BorderType</code></a>`;`

`pub use self::block::`<a href="block/struct.Padding.html" class="struct"
title="struct ratatui::widgets::block::Padding"><code>Padding</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="block/index.html" class="mod"
title="mod ratatui::widgets::block">block</a>  
Elements related to the `Block` base widget.

<a href="canvas/index.html" class="mod"
title="mod ratatui::widgets::canvas">canvas</a>  
A
[`Canvas`](canvas/struct.Canvas.html "struct ratatui::widgets::canvas::Canvas")
and a collection of
[`Shape`](canvas/trait.Shape.html "trait ratatui::widgets::canvas::Shape")s.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Axis.html" class="struct"
title="struct ratatui::widgets::Axis">Axis</a>  
An X or Y axis for the
[`Chart`](struct.Chart.html "struct ratatui::widgets::Chart") widget

<a href="struct.Bar.html" class="struct"
title="struct ratatui::widgets::Bar">Bar</a>  
A bar to be shown by the
[`BarChart`](struct.BarChart.html "struct ratatui::widgets::BarChart")
widget.

<a href="struct.BarChart.html" class="struct"
title="struct ratatui::widgets::BarChart">BarChart</a>  
A chart showing values as
[bars](struct.Bar.html "struct ratatui::widgets::Bar").

<a href="struct.BarGroup.html" class="struct"
title="struct ratatui::widgets::BarGroup">BarGroup</a>  
A group of bars to be shown by the Barchart.

<a href="struct.Borders.html" class="struct"
title="struct ratatui::widgets::Borders">Borders</a>  
Bitflags that can be composed to set the visible borders essentially on
the block widget.

<a href="struct.Cell.html" class="struct"
title="struct ratatui::widgets::Cell">Cell</a>  
A [`Cell`](struct.Cell.html "struct ratatui::widgets::Cell") contains
the [`Text`](../text/struct.Text.html "struct ratatui::text::Text") to
be displayed in a
[`Row`](struct.Row.html "struct ratatui::widgets::Row") of a
[`Table`](struct.Table.html "struct ratatui::widgets::Table").

<a href="struct.Chart.html" class="struct"
title="struct ratatui::widgets::Chart">Chart</a>  
A widget to plot one or more
[`Dataset`](struct.Dataset.html "struct ratatui::widgets::Dataset") in a
cartesian coordinate system

<a href="struct.Clear.html" class="struct"
title="struct ratatui::widgets::Clear">Clear</a>  
A widget to clear/reset a certain area to allow overdrawing (e.g. for
popups).

<a href="struct.Dataset.html" class="struct"
title="struct ratatui::widgets::Dataset">Dataset</a>  
A group of data points

<a href="struct.Gauge.html" class="struct"
title="struct ratatui::widgets::Gauge">Gauge</a>  
A widget to display a progress bar.

<a href="struct.LineGauge.html" class="struct"
title="struct ratatui::widgets::LineGauge">LineGauge</a>  
A compact widget to display a progress bar over a single thin line.

<a href="struct.List.html" class="struct"
title="struct ratatui::widgets::List">List</a>  
A widget to display several items among which one can be selected
(optional)

<a href="struct.ListItem.html" class="struct"
title="struct ratatui::widgets::ListItem">ListItem</a>  
A single item in a
[`List`](struct.List.html "struct ratatui::widgets::List")

<a href="struct.ListState.html" class="struct"
title="struct ratatui::widgets::ListState">ListState</a>  
State of the [`List`](struct.List.html "struct ratatui::widgets::List")
widget

<a href="struct.Paragraph.html" class="struct"
title="struct ratatui::widgets::Paragraph">Paragraph</a>  
A widget to display some text.

<a href="struct.RatatuiLogo.html" class="struct"
title="struct ratatui::widgets::RatatuiLogo">RatatuiLogo</a>  
A widget that renders the Ratatui logo

<a href="struct.Row.html" class="struct"
title="struct ratatui::widgets::Row">Row</a>  
A single row of data to be displayed in a
[`Table`](struct.Table.html "struct ratatui::widgets::Table") widget.

<a href="struct.Scrollbar.html" class="struct"
title="struct ratatui::widgets::Scrollbar">Scrollbar</a>  
A widget to display a scrollbar

<a href="struct.ScrollbarState.html" class="struct"
title="struct ratatui::widgets::ScrollbarState">ScrollbarState</a>  
A struct representing the state of a Scrollbar widget.

<a href="struct.Sparkline.html" class="struct"
title="struct ratatui::widgets::Sparkline">Sparkline</a>  
Widget to render a sparkline over one or more lines.

<a href="struct.SparklineBar.html" class="struct"
title="struct ratatui::widgets::SparklineBar">SparklineBar</a>  
An bar in a `Sparkline`.

<a href="struct.Table.html" class="struct"
title="struct ratatui::widgets::Table">Table</a>  
A widget to display data in formatted columns.

<a href="struct.TableState.html" class="struct"
title="struct ratatui::widgets::TableState">TableState</a>  
State of a [`Table`](struct.Table.html "struct ratatui::widgets::Table")
widget

<a href="struct.Tabs.html" class="struct"
title="struct ratatui::widgets::Tabs">Tabs</a>  
A widget that displays a horizontal set of Tabs with a single tab
selected.

<a href="struct.Wrap.html" class="struct"
title="struct ratatui::widgets::Wrap">Wrap</a>  
Describes how to wrap text across lines.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.GraphType.html" class="enum"
title="enum ratatui::widgets::GraphType">GraphType</a>  
Used to determine which style of graphing to use

<a href="enum.HighlightSpacing.html" class="enum"
title="enum ratatui::widgets::HighlightSpacing">HighlightSpacing</a>  
This option allows the user to configure the “highlight symbol” column
width spacing

<a href="enum.LegendPosition.html" class="enum"
title="enum ratatui::widgets::LegendPosition">LegendPosition</a>  
Allow users to specify the position of a legend in a
[`Chart`](struct.Chart.html "struct ratatui::widgets::Chart")

<a href="enum.ListDirection.html" class="enum"
title="enum ratatui::widgets::ListDirection">ListDirection</a>  
Defines the direction in which the list will be rendered.

<a href="enum.RatatuiLogoSize.html" class="enum"
title="enum ratatui::widgets::RatatuiLogoSize">RatatuiLogoSize</a>  
The size of the logo

<a href="enum.RenderDirection.html" class="enum"
title="enum ratatui::widgets::RenderDirection">RenderDirection</a>  
Defines the direction in which sparkline will be rendered.

<a href="enum.ScrollDirection.html" class="enum"
title="enum ratatui::widgets::ScrollDirection">ScrollDirection</a>  
An enum representing a scrolling direction.

<a href="enum.ScrollbarOrientation.html" class="enum"
title="enum ratatui::widgets::ScrollbarOrientation">ScrollbarOrientation</a>  
This is the position of the scrollbar around a given area.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.StatefulWidget.html" class="trait"
title="trait ratatui::widgets::StatefulWidget">StatefulWidget</a>  
A `StatefulWidget` is a widget that can take advantage of some local
state to remember things between two draw calls.

<a href="trait.StatefulWidgetRef.html" class="trait"
title="trait ratatui::widgets::StatefulWidgetRef">StatefulWidgetRef</a>  
A `StatefulWidgetRef` is a trait that allows rendering a stateful widget
by reference.

<a href="trait.Widget.html" class="trait"
title="trait ratatui::widgets::Widget">Widget</a>  
A `Widget` is a type that can be drawn on a
[`Buffer`](../buffer/struct.Buffer.html "struct ratatui::buffer::Buffer")
in a given
[`Rect`](../layout/struct.Rect.html "struct ratatui::layout::Rect").

<a href="trait.WidgetRef.html" class="trait"
title="trait ratatui::widgets::WidgetRef">WidgetRef</a>  
A `WidgetRef` is a trait that allows rendering a widget by reference.
