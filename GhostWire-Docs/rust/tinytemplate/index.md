Crate tinytemplate Copy item path

<span class="sub-heading"><a href="../src/tinytemplate/lib.rs.html#1-260" class="src">Source</a>
</span>

Expand description

## <a href="#tinytemplate" class="doc-anchor">§</a>TinyTemplate

TinyTemplate is a minimal templating library originally designed for use
in [Criterion.rs](https://github.com/bheisler/criterion.rs). It
deliberately does not provide all of the features of a full-power
template engine, but in return it provides a simple API, clear
templating syntax, decent performance and very few dependencies.

## <a href="#features" class="doc-anchor">§</a>Features

The most important features are as follows (see the
[syntax](syntax/index.html) module for full details on the template
syntax):

-   Rendering values - `{ myvalue }`
-   Conditionals -
    `{{ if foo }}Foo is true{{ else }}Foo is false{{ endif }}`
-   Loops - `{{ for value in row }}{value}{{ endfor }}`
-   Customizable value formatters `{ value | my_formatter }`
-   Macros `{{ call my_template with foo }}`

## <a href="#restrictions" class="doc-anchor">§</a>Restrictions

TinyTemplate was designed with the assumption that the templates are
available as static strings, either using string literals or the
`include_str!` macro. Thus, it borrows `&str` slices from the template
text itself and uses them during the rendering process. Although it is
possible to use TinyTemplate with template strings loaded at runtime,
this is not recommended.

Additionally, TinyTemplate can only render templates into Strings. If
you need to render a template directly to a socket or file, TinyTemplate
may not be right for you.

## <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
#[macro_use]
extern crate serde_derive;
extern crate tinytemplate;

use tinytemplate::TinyTemplate;
use std::error::Error;

#[derive(Serialize)]
struct Context {
    name: String,
}

static TEMPLATE : &'static str = "Hello {name}!";

pub fn main() -> Result<(), Box<Error>> {
    let mut tt = TinyTemplate::new();
    tt.add_template("hello", TEMPLATE)?;

    let context = Context {
        name: "World".to_string(),
    };

    let rendered = tt.render("hello", &context)?;
    println!("{}", rendered);

    Ok(())
}
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="error/index.html" class="mod"
title="mod tinytemplate::error">error</a>  
Module containing the error type returned by TinyTemplate if an error
occurs.

<a href="syntax/index.html" class="mod"
title="mod tinytemplate::syntax">syntax</a>  
Documentation of TinyTemplate’s template syntax.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.TinyTemplate.html" class="struct"
title="struct tinytemplate::TinyTemplate">TinyTemplate</a>  
The TinyTemplate struct is the entry point for the TinyTemplate library.
It contains the template and formatter registries and provides functions
to render templates as well as to register templates and formatters.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.escape.html" class="fn"
title="fn tinytemplate::escape">escape</a>  
Appends `value` to `output`, performing HTML-escaping in the process.

<a href="fn.format.html" class="fn"
title="fn tinytemplate::format">format</a>  
The format function is used as the default value formatter for all
values unless the user specifies another. It is provided publicly so
that it can be called as part of custom formatters. Values are formatted
as follows:

<a href="fn.format_unescaped.html" class="fn"
title="fn tinytemplate::format_unescaped">format_unescaped</a>  
Identical to [`format`](fn.format.html) except that this does not
perform HTML escaping.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.ValueFormatter.html" class="type"
title="type tinytemplate::ValueFormatter">ValueFormatter</a>  
Type alias for closures which can be used as value formatters.
