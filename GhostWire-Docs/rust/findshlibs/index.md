Crate findshlibs Copy item path

<span class="sub-heading"><a href="../src/findshlibs/lib.rs.html#1-494" class="src">Source</a>
</span>

Expand description

# <a href="#findshlibs" class="doc-anchor">§</a>`findshlibs`

Find the set of shared libraries currently loaded in this process with a
cross platform API.

The API entry point is the `TargetSharedLibrary` type and its
`SharedLibrary::each` trait method implementation.

## <a href="#example" class="doc-anchor">§</a>Example

Here is an example program that prints out each shared library that is
loaded in the process and the addresses where each of its segments are
mapped into memory.

``` {.rust .rust-example-rendered}
extern crate findshlibs;
use findshlibs::{Segment, SharedLibrary, TargetSharedLibrary};

fn main() {
    TargetSharedLibrary::each(|shlib| {
        println!("{}", shlib.name().to_string_lossy());

        for seg in shlib.segments() {
            println!("    {}: segment {}",
                     seg.actual_virtual_memory_address(shlib),
                     seg.name());
        }
    });
}
```

## <a href="#supported-oses" class="doc-anchor">§</a>Supported OSes

These are the OSes that `findshlibs` currently supports:

-   Linux
-   macOS
-   Windows
-   Android
-   iOS

If a platform is not supported then a fallback implementation is used
that does nothing. To see if your platform does something at runtime the
`TARGET_SUPPORTED` constant can be used.

Is your OS missing here? Send us a pull request!

## <a href="#addresses" class="doc-anchor">§</a>Addresses

Shared libraries’ addresses can be confusing. They are loaded somewhere
in physical memory, but we generally don’t care about physical memory
addresses, because only the OS can see that address and in userspace we
can only access memory through its virtual memory address. But even
“virtual memory address” is ambiguous because it isn’t clear whether
this is the address before or after the loader maps the shared library
into memory and performs relocation.

To clarify between these different kinds of addresses, we borrow some
terminology from
[LUL](https://searchfox.org/mozilla-central/rev/13148faaa91a1c823a7d68563d9995480e714979/tools/profiler/lul/LulMain.h#17-51):

> -   **SVMA** (“Stated Virtual Memory Address”): this is an address of
>     a symbol (etc) as it is stated in the symbol table, or other
>     metadata, of an object. Such values are typically small and start
>     from zero or thereabouts, unless the object has been prelinked.
>
> -   **AVMA** (“Actual Virtual Memory Address”): this is the address of
>     a symbol (etc) in a running process, that is, once the associated
>     object has been mapped into a process. Such values are typically
>     much larger than SVMAs, since objects can get mapped arbitrarily
>     far along the address space.
>
> -   **“Bias”**: the difference between AVMA and SVMA for a given
>     symbol (specifically, AVMA - SVMA). The bias is always an integral
>     number of pages. Once we know the bias for a given object’s text
>     section (for example), we can compute the AVMAs of all of its text
>     symbols by adding the bias to their SVMAs.

## <a href="#names-and-ids" class="doc-anchor">§</a>Names and IDs

`findshlibs` also gives access to module names and IDs. Since this is
also not consistent across operating systems the following general rules
apply:

> -   `id` refers to the ID of the object file itself. This is generally
>     available on all platforms however it might still not be compiled
>     into the binary in all case. For instance on Linux the
>     `gnu.build-id` note needs to be compiled in (which Rust does
>     automatically).
> -   `debug_id` refers to the ID of the debug file. This only plays a
>     role on Windows where the executable and the debug file (PDB) have
>     a different ID.
> -   `name` is the name of the executable. On most operating systems
>     (and all systems implemented currently) this is not just the name
>     but in fact the entire path to the executable.
> -   `debug_name` is the name of the debug file if known. This is again
>     the case on windows where this will be the path to the PDB file.

# Modules<a href="#modules" class="anchor">§</a>

<a href="linux/index.html" class="mod"
title="mod findshlibs::linux">linux</a>  
Linux-specific implementation of the `SharedLibrary` trait.

<a href="unsupported/index.html" class="mod"
title="mod findshlibs::unsupported">unsupported</a>  
The fallback implementation of the [SharedLibrary
trait](../trait.SharedLibrary.html) that does nothing.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Avma.html" class="struct"
title="struct findshlibs::Avma">Avma</a>  
Actual virtual memory address.

<a href="struct.Bias.html" class="struct"
title="struct findshlibs::Bias">Bias</a>  
Virtual memory bias.

<a href="struct.Svma.html" class="struct"
title="struct findshlibs::Svma">Svma</a>  
Stated virtual memory address.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.IterationControl.html" class="enum"
title="enum findshlibs::IterationControl">IterationControl</a>  
Control whether iteration over shared libraries should continue or stop.

<a href="enum.SharedLibraryId.html" class="enum"
title="enum findshlibs::SharedLibraryId">SharedLibraryId</a>  
Represents an ID for a shared library.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.TARGET_SUPPORTED.html" class="constant"
title="constant findshlibs::TARGET_SUPPORTED">TARGET_SUPPORTED</a>  
An indicator if this platform is supported.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Segment.html" class="trait"
title="trait findshlibs::Segment">Segment</a>  
A mapped segment in a shared library.

<a href="trait.SharedLibrary.html" class="trait"
title="trait findshlibs::SharedLibrary">SharedLibrary</a>  
A trait representing a shared library that is loaded in this process.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.TargetSharedLibrary.html" class="type"
title="type findshlibs::TargetSharedLibrary">TargetSharedLibrary</a>  
The [`SharedLibrary` trait](./trait.SharedLibrary.html) implementation
for the target operating system.
