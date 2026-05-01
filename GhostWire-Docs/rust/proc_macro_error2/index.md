Crate proc\_macro\_error2 Copy item path

<span class="sub-heading"><a href="../src/proc_macro_error2/lib.rs.html#1-565"
class="src">Source</a> </span>

Expand description

# <a href="#proc-macro-error2" class="doc-anchor">§</a>proc-macro-error2

This crate aims to make error reporting in proc-macros simple and easy
to use. Migrate from `panic!`-based errors for as little effort as
possible!

(Also, you can explicitly [append a dummy token
stream](dummy/index.html) to your errors).

To achieve his, this crate serves as a tiny shim around
`proc_macro::Diagnostic` and `compile_error!`. It detects the best way
of emitting available based on compiler’s version. When the underlying
diagnostic type is finally stabilized, this crate will simply be
delegating to it requiring no changes in your code!

So you can just use this crate and have *both* some of
`proc_macro::Diagnostic` functionality available on stable ahead of time
*and* your error-reporting code future-proof.

## <a href="#cargo-features" class="doc-anchor">§</a>Cargo features

This crate provides *enabled by default* `syn-error` feature that gates
`impl From<syn::Error> for Diagnostic` conversion. If you don’t use
`syn` and want to cut off some of compilation time, you can disable it
via

``` language-toml
[dependencies]
proc-macro-error2 = { version = "2.0.0", default-features = false }
```

\***Please note that disabling this feature makes sense only if you
don’t depend on `syn` directly or indirectly, and you very likely do.**

## <a href="#real-world-examples" class="doc-anchor">§</a>Real world examples

-   [`structopt-derive`](https://github.com/TeXitoi/structopt/tree/master/structopt-derive)
    (abort-like usage)
-   [`auto-impl`](https://github.com/auto-impl-rs/auto_impl/) (emit-like
    usage)

## <a href="#limitations" class="doc-anchor">§</a>Limitations

-   Warnings are emitted only on nightly, they are ignored on stable.
-   “help” suggestions can’t have their own span info on stable,
    (essentially inheriting the parent span).
-   If a panic occurs somewhere in your macro no errors will be
    displayed. This is not a technical limitation but rather intentional
    design. `panic` is not for error reporting.

### <a href="#proc_macro_error-attribute" class="doc-anchor">§</a>`#[proc_macro_error]` attribute

**This attribute MUST be present on the top level of your macro** (the
function annotated with any of `#[proc_macro]`, `#[proc_macro_derive]`,
`#[proc_macro_attribute]`).

This attribute performs the setup and cleanup necessary to make things
work.

In most cases you’ll need the simple `#[proc_macro_error]` form without
any additional settings. Feel free to [skip the “Syntax”
section](#macros).

#### <a href="#syntax" class="doc-anchor">§</a>Syntax

`#[proc_macro_error]` or `#[proc_macro_error(settings...)]`, where
`settings...` is a comma-separated list of:

-   `proc_macro_hack`:

    In order to correctly cooperate with `#[proc_macro_hack]`,
    `#[proc_macro_error]` attribute must be placed *before* (above) it,
    like this:

    ``` {.rust .rust-example-rendered}
    #[proc_macro_error]
    #[proc_macro_hack]
    #[proc_macro]
    fn my_macro(input: TokenStream) -> TokenStream {
        unimplemented!()
    }
    ```

    If, for some reason, you can’t place it like that you can use
    `#[proc_macro_error(proc_macro_hack)]` instead.

    # <a href="#note" class="doc-anchor">§</a>Note

    If `proc-macro-hack` was detected (by any means) `allow_not_macro`
    and `assert_unwind_safe` will be applied automatically.

-   `allow_not_macro`:

    By default, the attribute checks that it’s applied to a proc-macro.
    If none of `#[proc_macro]`, `#[proc_macro_derive]` nor
    `#[proc_macro_attribute]` are present it will panic. It’s the
    intention - this crate is supposed to be used only with proc-macros.

    This setting is made to bypass the check, useful in certain
    circumstances.

    Pay attention: the function this attribute is applied to must return
    `proc_macro::TokenStream`.

    This setting is implied if `proc-macro-hack` was detected.

-   `assert_unwind_safe`:

    By default, your code must be [unwind
    safe](https://doc.rust-lang.org/std/panic/trait.UnwindSafe.html#what-is-unwind-safety).
    If your code is not unwind safe, but you believe it’s correct, you
    can use this setting to bypass the check. You would need this for
    code that uses `lazy_static` or `thread_local` with `Cell/RefCell`
    inside (and the like).

    This setting is implied if `#[proc_macro_error]` is applied to a
    function marked as `#[proc_macro]`, `#[proc_macro_derive]` or
    `#[proc_macro_attribute]`.

    This setting is also implied if `proc-macro-hack` was detected.

## <a href="#macros" class="doc-anchor">§</a>Macros

Most of the time you want to use the macros. Syntax is described in the
next section below.

You’ll need to decide how you want to emit errors:

-   Emit the error and abort. Very much panic-like usage. Served by
    [`abort!`](macro.abort.html) and
    [`abort_call_site!`](macro.abort_call_site.html).
-   Emit the error but do not abort right away, looking for other errors
    to report. Served by [`emit_error!`](macro.emit_error.html) and
    [`emit_call_site_error!`](macro.emit_call_site_warning.html).

You **can** mix these usages.

`abort` and `emit_error` take a “source span” as the first argument.
This source will be used to highlight the place the error originates
from. It must be one of:

-   *Something* that implements
    [`ToTokens`](https://docs.rs/quote/1.0.3/quote/trait.ToTokens.html)
    (most types in `syn` and `proc-macro2` do). This source is the
    preferable one since it doesn’t lose span information on multi-token
    spans, see [this
    issue](https://gitlab.com/CreepySkeleton/proc-macro-error/-/issues/6)
    for details.
-   [`proc_macro::Span`](https://doc.rust-lang.org/proc_macro/struct.Span.html)
-   [`proc-macro2::Span`](https://docs.rs/proc-macro2/1.0.10/proc_macro2/struct.Span.html)

The rest is your message in format-like style.

See [the next section](#syntax-1) for detailed syntax.

-   [`abort!`](macro.abort.html):

    Very much panic-like usage - abort right away and show the error.
    Expands to [`!`](https://doc.rust-lang.org/std/primitive.never.html)
    (never type).

-   [`abort_call_site!`](macro.abort_call_site.html):

    Shortcut for `abort!(Span::call_site(), ...)`. Expands to
    [`!`](https://doc.rust-lang.org/std/primitive.never.html) (never
    type).

-   [`emit_error!`](macro.emit_error.html):

    [`proc_macro::Diagnostic`](https://doc.rust-lang.org/proc_macro/struct.Diagnostic.html)-like
    usage - emit the error but keep going, looking for other errors to
    report. The compilation will fail nonetheless. Expands to
    [`()`](https://doc.rust-lang.org/std/primitive.unit.html) (unit
    type).

-   [`emit_call_site_error!`](macro.emit_call_site_warning.html):

    Shortcut for `emit_error!(Span::call_site(), ...)`. Expands to
    [`()`](https://doc.rust-lang.org/std/primitive.unit.html) (unit
    type).

-   [`emit_warning!`](macro.emit_warning.html):

    Like `emit_error!` but emit a warning instead of error. The
    compilation won’t fail because of warnings. Expands to
    [`()`](https://doc.rust-lang.org/std/primitive.unit.html) (unit
    type).

    **Beware**: warnings are nightly only, they are completely ignored
    on stable.

-   [`emit_call_site_warning!`](macro.emit_call_site_error.html):

    Shortcut for `emit_warning!(Span::call_site(), ...)`. Expands to
    [`()`](https://doc.rust-lang.org/std/primitive.unit.html) (unit
    type).

-   [`diagnostic`](struct.Diagnostic.html):

    Build an instance of `Diagnostic` in format-like style.

#### <a href="#syntax-1" class="doc-anchor">§</a>Syntax

All the macros have pretty much the same syntax:

1.  <a href="#" class="tooltip" title="This example is not tested">ⓘ</a>
    ``` {.rust .rust-example-rendered}
    abort!(single_expr)
    ```

    Shortcut for `Diagnostic::from(expr).abort()`.

2.  <a href="#" class="tooltip" title="This example is not tested">ⓘ</a>
    ``` {.rust .rust-example-rendered}
    abort!(span, message)
    ```

    The first argument is an expression the span info should be taken
    from.

    The second argument is the error message, it must implement
    [`ToString`](https://doc.rust-lang.org/std/string/trait.ToString.html).

3.  <a href="#" class="tooltip" title="This example is not tested">ⓘ</a>
    ``` {.rust .rust-example-rendered}
    abort!(span, format_literal, format_args...)
    ```

    This form is pretty much the same as 2, except
    `format!(format_literal, format_args...)` will be used to for the
    message instead of
    [`ToString`](https://doc.rust-lang.org/std/string/trait.ToString.html).

That’s it. `abort!`, `emit_warning`, `emit_error` share this exact
syntax.

`abort_call_site!`, `emit_call_site_warning`, `emit_call_site_error`
lack 1 form and do not take span in 2’th and 3’th forms. Those are
essentially shortcuts for `macro!(Span::call_site(), args...)`.

`diagnostic!` requires a
[`Level`](enum.Level.html "enum proc_macro_error2::Level") instance
between `span` and second argument (1’th form is the same).

> **Important!**
>
> If you have some type from `proc_macro` or `syn` to point to, do not
> call `.span()` on it but rather use it directly:
>
> ``` {.rust .rust-example-rendered}
> let ty: syn::Type = syn::parse2(input).unwrap();
> abort!(ty, "BOOM");
> //     ^^ <-- avoid .span()
> ```
>
> `.span()` calls work too, but you may experience regressions in
> message quality.

#### <a href="#note-attachments" class="doc-anchor">§</a>Note attachments

1.  Every macro can have “note” attachments (only 2 and 3 form).

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
let opt_help = if have_some_info { Some("did you mean `this`?") } else { None };

abort!(
    span, message; // <--- attachments start with `;` (semicolon)

    help = "format {} {}", "arg1", "arg2"; // <--- every attachment ends with `;`,
                                           //      maybe except the last one

    note = "to_string"; // <--- one arg uses `.to_string()` instead of `format!()`

    yay = "I see what {} did here", "you"; // <--- "help =" and "hint =" are mapped
                                           // to Diagnostic::help,
                                           // anything else is Diagnostic::note

    wow = note_span => "custom span"; // <--- attachments can have their own span
                                      //      it takes effect only on nightly though

    hint =? opt_help; // <-- "optional" attachment, get displayed only if `Some`
                      //     must be single `Option` expression

    note =? note_span => opt_help // <-- optional attachments can have custom spans too
);
```

### <a href="#diagnostic-type" class="doc-anchor">§</a>Diagnostic type

[`Diagnostic`](struct.Diagnostic.html) type is intentionally designed to
be API compatible with
[`proc_macro::Diagnostic`](https://doc.rust-lang.org/proc_macro/struct.Diagnostic.html).
Not all API is implemented, only the part that can be reasonably
implemented on stable.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::dummy::`<a href="dummy/fn.append_dummy.html" class="fn"
title="fn proc_macro_error2::dummy::append_dummy"><code>append_dummy</code></a>`;`

`pub use crate::dummy::`<a href="dummy/fn.set_dummy.html" class="fn"
title="fn proc_macro_error2::dummy::set_dummy"><code>set_dummy</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="dummy/index.html" class="mod"
title="mod proc_macro_error2::dummy">dummy</a>  
Facility to emit dummy implementations (or whatever) in case an error
happen.

# Macros<a href="#macros-1" class="anchor">§</a>

<a href="macro.abort.html" class="macro"
title="macro proc_macro_error2::abort">abort</a>  
Abort proc-macro execution right now and display the error.

<a href="macro.abort_call_site.html" class="macro"
title="macro proc_macro_error2::abort_call_site">abort_call_site</a>  
Shortcut for `abort!(Span::call_site(), msg...)`. This macro is still
preferable over plain panic, panics are not for error reporting.

<a href="macro.diagnostic.html" class="macro"
title="macro proc_macro_error2::diagnostic">diagnostic</a>  
Build [`Diagnostic`](struct.Diagnostic.html) instance from provided
arguments.

<a href="macro.emit_call_site_error.html" class="macro"
title="macro proc_macro_error2::emit_call_site_error">emit_call_site_error</a>  
Shortcut for `emit_error!(Span::call_site(), ...)`. This macro is still
preferable over plain panic, panics are not for error reporting..

<a href="macro.emit_call_site_warning.html" class="macro"
title="macro proc_macro_error2::emit_call_site_warning">emit_call_site_warning</a>  
Shortcut for `emit_warning!(Span::call_site(), ...)`.

<a href="macro.emit_error.html" class="macro"
title="macro proc_macro_error2::emit_error">emit_error</a>  
Emit an error while not aborting the proc-macro right away.

<a href="macro.emit_warning.html" class="macro"
title="macro proc_macro_error2::emit_warning">emit_warning</a>  
Emit a warning. Warnings are not errors and compilation won’t fail
because of them.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Diagnostic.html" class="struct"
title="struct proc_macro_error2::Diagnostic">Diagnostic</a>

Represents a single diagnostic message

<a href="struct.SpanRange.html" class="struct"
title="struct proc_macro_error2::SpanRange">SpanRange</a>

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Level.html" class="enum"
title="enum proc_macro_error2::Level">Level</a>  
Represents a diagnostic level

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.DiagnosticExt.html" class="trait"
title="trait proc_macro_error2::DiagnosticExt">DiagnosticExt</a>  
A collection of methods that do not exist in `proc_macro::Diagnostic`
but still useful to have around.

<a href="trait.OptionExt.html" class="trait"
title="trait proc_macro_error2::OptionExt">OptionExt</a>  
This traits expands `Option` with some handy shortcuts.

<a href="trait.ResultExt.html" class="trait"
title="trait proc_macro_error2::ResultExt">ResultExt</a>  
This traits expands `Result<T, Into<Diagnostic>>` with some handy
shortcuts.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.abort_if_dirty.html" class="fn"
title="fn proc_macro_error2::abort_if_dirty">abort_if_dirty</a>  
Abort macro execution and display all the emitted errors, if any.

# Attribute Macros<a href="#attributes" class="anchor">§</a>

<a href="attr.proc_macro_error.html" class="attr"
title="attr proc_macro_error2::proc_macro_error">proc_macro_error</a>
