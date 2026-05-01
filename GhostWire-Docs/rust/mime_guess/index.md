Crate mime\_guess Copy item path

<span class="sub-heading"><a href="../src/mime_guess/lib.rs.html#1-534" class="src">Source</a>
</span>

Expand description

Guessing of MIME types by file extension.

Uses a static list of file-extension : MIME type mappings.

``` {.rust .rust-example-rendered}
// the file doesn't have to exist, it just looks at the path
let guess = mime_guess::from_path("some_file.gif");
assert_eq!(guess.first(), Some(mime::IMAGE_GIF));
```

#### <a href="#note-mime-types-returned-are-not-stableguaranteed"
class="doc-anchor">§</a>Note: MIME Types Returned Are Not Stable/Guaranteed

The media types returned for a given extension are not considered to be
part of the crate’s stable API and are often updated in patch  
(`x.y.[z + 1]`) releases to be as correct as possible.

Additionally, only the extensions of paths/filenames are inspected in
order to guess the MIME type. The file that may or may not reside at
that path may or may not be a valid file of the returned MIME type. Be
wary of unsafe or un-validated assumptions about file structure or
length.

# Re-exports<a href="#reexports" class="anchor">§</a>

pub extern crate
<a href="../mime/index.html" class="mod" title="mod mime">mime</a>;

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Iter.html" class="struct"
title="struct mime_guess::Iter">Iter</a>  
An iterator over the `Mime` types of a `MimeGuess`.

<a href="struct.IterRaw.html" class="struct"
title="struct mime_guess::IterRaw">IterRaw</a>  
An iterator over the raw media type strings of a `MimeGuess`.

<a href="struct.Mime.html" class="struct"
title="struct mime_guess::Mime">Mime</a>  
A parsed mime or media type.

<a href="struct.MimeGuess.html" class="struct"
title="struct mime_guess::MimeGuess">MimeGuess</a>  
A “guess” of the MIME/Media Type(s) of an extension or path as one or
more [`Mime`](struct.Mime.html) instances.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.from_ext.html" class="fn"
title="fn mime_guess::from_ext">from_ext</a>  
Wrapper of
[`MimeGuess::from_ext()`](struct.MimeGuess.html#method.from_ext).

<a href="fn.from_path.html" class="fn"
title="fn mime_guess::from_path">from_path</a>  
Wrapper of
[`MimeGuess::from_path()`](struct.MimeGuess.html#method.from_path).

<a href="fn.get_extensions.html" class="fn"
title="fn mime_guess::get_extensions">get_extensions</a>  
Get the extensions for a given top-level and sub-level of a MIME type
(`{toplevel}/{sublevel}`).

<a href="fn.get_mime_extensions.html" class="fn"
title="fn mime_guess::get_mime_extensions">get_mime_extensions</a>  
Get a list of known extensions for a given `Mime`.

<a href="fn.get_mime_extensions_str.html" class="fn"
title="fn mime_guess::get_mime_extensions_str">get_mime_extensions_str</a>  
Get a list of known extensions for a MIME type string.

<a href="fn.get_mime_type.html" class="fn"
title="fn mime_guess::get_mime_type">get_mime_type</a><span class="stab deprecated" title="">Deprecated</span>  
Get the MIME type associated with a file extension.

<a href="fn.get_mime_type_opt.html" class="fn"
title="fn mime_guess::get_mime_type_opt">get_mime_type_opt</a><span class="stab deprecated" title="">Deprecated</span>  
Get the MIME type associated with a file extension.

<a href="fn.get_mime_type_str.html" class="fn"
title="fn mime_guess::get_mime_type_str">get_mime_type_str</a><span class="stab deprecated" title="">Deprecated</span>  
Get the MIME type string associated with a file extension.
Case-insensitive.

<a href="fn.guess_mime_type.html" class="fn"
title="fn mime_guess::guess_mime_type">guess_mime_type</a><span class="stab deprecated" title="">Deprecated</span>  
Guess the MIME type of `path` by its extension (as defined by
`Path::extension()`).

<a href="fn.guess_mime_type_opt.html" class="fn"
title="fn mime_guess::guess_mime_type_opt">guess_mime_type_opt</a><span class="stab deprecated" title="">Deprecated</span>  
Guess the MIME type of `path` by its extension (as defined by
`Path::extension()`).

<a href="fn.mime_str_for_path_ext.html" class="fn"
title="fn mime_guess::mime_str_for_path_ext">mime_str_for_path_ext</a><span class="stab deprecated" title="">Deprecated</span>  
Guess the MIME type string of `path` by its extension (as defined by
`Path::extension()`).

<a href="fn.octet_stream.html" class="fn"
title="fn mime_guess::octet_stream">octet_stream</a><span class="stab deprecated" title="">Deprecated</span>  
Get the MIME type for `application/octet-stream` (generic binary stream)
