Crate aquamarine Copy item path

<span class="sub-heading"><a href="../src/aquamarine/lib.rs.html#1-124" class="src">Source</a>
</span>

Expand description

Aquamarine is a procedural macro extension for
[rustdoc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html), that
aims to improve the visual component of Rust documentation through use
of the [mermaid.js](https://mermaid-js.github.io/mermaid/#/) diagrams.

`#[aquamarine]` macro works through embedding the
[mermaid.js](https://github.com/mermaid-js/mermaid) into the generated
rustdoc HTML page, modifying the doc comment attributes.

To inline a diagram into the documentation, use the `mermaid` snippet in
a doc-string:

``` {.rust .rust-example-rendered}
#[cfg_attr(doc, aquamarine)]
/// ```mermaid
/// graph LR
///     s([Source]) --> a[[aquamarine]]
///     r[[rustdoc]] --> f([Docs w/ Mermaid!])
///     subgraph rustc[Rust Compiler]
///     a -. inject mermaid.js .-> r
///     end
/// ```
pub fn example() {}
```

The diagram will appear in place of the `mermaid` code block, preserving
all the comments around it.

You can even add multiple diagrams!

To see it in action, go to the [demo
crate](https://docs.rs/aquamarine-demo-crate/0.6.0/aquamarine_demo_crate/fn.example.html)
docs.rs page.

### <a href="#dark-mode" class="doc-anchor">§</a>Dark-mode

Aquamarine will automatically select the `dark` theme as a default, if
the current `rustdoc` theme is either `ayu` or `dark`.

You might need to reload the page to redraw the diagrams after changing
the theme.

### <a href="#custom-themes" class="doc-anchor">§</a>Custom themes

Theming is supported on per-diagram basis, through the mermaid’s
`%%init%%` attribute.

*Note*: custom theme will override the default theme

``` {.rust .rust-example-rendered}
/// ```mermaid
/// %%{init: {
///     'theme': 'base',
///     'themeVariables': {
///            'primaryColor': '#ffcccc',
///            'edgeLabelBackground':'#ccccff',
///            'tertiaryColor': '#fff0f0' }}}%%
/// graph TD
///      A(Diagram needs to be drawn) --> B{Does it have 'init' annotation?}
///      B -->|No| C(Apply default theme)
///      B -->|Yes| D(Apply customized theme)
/// ```
```

[Demo on
docs.rs](https://docs.rs/aquamarine-demo-crate/0.6.0/aquamarine_demo_crate/fn.example_with_styling.html)

To learn more, see the [Theming
Section](https://mermaid-js.github.io/mermaid/#/theming) of the
mermaid.js book

### <a href="#loading-from-a-file" class="doc-anchor">§</a>Loading from a file

When describing complex logic, a diagram can get quite big.

To reduce clutter in your doc comments, you can load a diagram from file
using the `include_mmd!` macro-like syntax.

``` {.rust .rust-example-rendered}
/// Diagram #1
/// include_mmd!("diagram_1.mmd")
///
/// Diagram #2
/// include_mmd!("diagram_2.mmd")
```

[Demo on
docs.rs](https://docs.rs/aquamarine-demo-crate/0.6.0/aquamarine_demo_crate/fn.example_load_from_file.html)

# Attribute Macros<a href="#attributes" class="anchor">§</a>

<a href="attr.aquamarine.html" class="attr"
title="attr aquamarine::aquamarine">aquamarine</a>  
Aquamarine is a proc-macro that adds
[Mermaid](https://mermaid-js.github.io/mermaid/#/) diagrams to rustdoc
