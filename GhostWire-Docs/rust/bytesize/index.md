Crate bytesize Copy item path

<span class="sub-heading"><a href="../src/bytesize/lib.rs.html#1-607" class="src">Source</a>
</span>

Expand description

`ByteSize` is a semantic wrapper for byte count representations.

Features:

-   Pre-defined constants for various size units (e.g., B, KB, KiB, MB,
    MiB, … EB, EiB).
-   `ByteSize` type which presents size units convertible to different
    size units.
-   Arithmetic operations for `ByteSize`.
-   `FromStr` impl for `ByteSize`, allowing for parsing string size
    representations like “1.5KiB” and “521TiB”.
-   Serde support for binary and human-readable deserializers like JSON.

# <a href="#examples" class="doc-anchor">§</a>Examples

Construction using SI or IEC helpers.

``` {.rust .rust-example-rendered}
use bytesize::ByteSize;

assert!(ByteSize::kib(4) > ByteSize::kb(4));
```

Display as human-readable string.

``` {.rust .rust-example-rendered}
use bytesize::ByteSize;

assert_eq!("518.0 GiB", ByteSize::gib(518).display().iec().to_string());
assert_eq!("556.2 GB", ByteSize::gib(518).display().si().to_string());
assert_eq!("518.0G", ByteSize::gib(518).display().iec_short().to_string());
```

Arithmetic operations are supported.

``` {.rust .rust-example-rendered}
use bytesize::ByteSize;

let plus = ByteSize::mb(1) + ByteSize::kb(100);
println!("{plus}");

let minus = ByteSize::tb(1) - ByteSize::gb(4);
assert_eq!(ByteSize::gb(996), minus);
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ByteSize.html" class="struct"
title="struct bytesize::ByteSize">ByteSize</a>  
Byte size representation.

<a href="struct.Display.html" class="struct"
title="struct bytesize::Display">Display</a>  
Formatting display wrapper for
[`ByteSize`](struct.ByteSize.html "struct bytesize::ByteSize").

<a href="struct.UnitParseError.html" class="struct"
title="struct bytesize::UnitParseError">UnitParseError</a>  
Error returned when parsing a
[`Unit`](enum.Unit.html "enum bytesize::Unit") fails.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Unit.html" class="enum"
title="enum bytesize::Unit">Unit</a>  
Scale unit.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.EB.html" class="constant"
title="constant bytesize::EB">EB</a>  
Number of bytes in 1 exabyte.

<a href="constant.EIB.html" class="constant"
title="constant bytesize::EIB">EIB</a>  
Number of bytes in 1 exbibyte.

<a href="constant.GB.html" class="constant"
title="constant bytesize::GB">GB</a>  
Number of bytes in 1 gigabyte.

<a href="constant.GIB.html" class="constant"
title="constant bytesize::GIB">GIB</a>  
Number of bytes in 1 gibibyte.

<a href="constant.KB.html" class="constant"
title="constant bytesize::KB">KB</a>  
Number of bytes in 1 kilobyte.

<a href="constant.KIB.html" class="constant"
title="constant bytesize::KIB">KIB</a>  
Number of bytes in 1 kibibyte.

<a href="constant.MB.html" class="constant"
title="constant bytesize::MB">MB</a>  
Number of bytes in 1 megabyte.

<a href="constant.MIB.html" class="constant"
title="constant bytesize::MIB">MIB</a>  
Number of bytes in 1 mebibyte.

<a href="constant.PB.html" class="constant"
title="constant bytesize::PB">PB</a>  
Number of bytes in 1 petabyte.

<a href="constant.PIB.html" class="constant"
title="constant bytesize::PIB">PIB</a>  
Number of bytes in 1 pebibyte.

<a href="constant.TB.html" class="constant"
title="constant bytesize::TB">TB</a>  
Number of bytes in 1 terabyte.

<a href="constant.TIB.html" class="constant"
title="constant bytesize::TIB">TIB</a>  
Number of bytes in 1 tebibyte.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.eb.html" class="fn" title="fn bytesize::eb">eb</a>  
Converts a quantity of exabytes to bytes.

<a href="fn.eib.html" class="fn" title="fn bytesize::eib">eib</a>  
Converts a quantity of exbibytes to bytes.

<a href="fn.gb.html" class="fn" title="fn bytesize::gb">gb</a>  
Converts a quantity of gigabytes to bytes.

<a href="fn.gib.html" class="fn" title="fn bytesize::gib">gib</a>  
Converts a quantity of gibibytes to bytes.

<a href="fn.kb.html" class="fn" title="fn bytesize::kb">kb</a>  
Converts a quantity of kilobytes to bytes.

<a href="fn.kib.html" class="fn" title="fn bytesize::kib">kib</a>  
Converts a quantity of kibibytes to bytes.

<a href="fn.mb.html" class="fn" title="fn bytesize::mb">mb</a>  
Converts a quantity of megabytes to bytes.

<a href="fn.mib.html" class="fn" title="fn bytesize::mib">mib</a>  
Converts a quantity of mebibytes to bytes.

<a href="fn.pb.html" class="fn" title="fn bytesize::pb">pb</a>  
Converts a quantity of petabytes to bytes.

<a href="fn.pib.html" class="fn" title="fn bytesize::pib">pib</a>  
Converts a quantity of pebibytes to bytes.

<a href="fn.tb.html" class="fn" title="fn bytesize::tb">tb</a>  
Converts a quantity of terabytes to bytes.

<a href="fn.tib.html" class="fn" title="fn bytesize::tib">tib</a>  
Converts a quantity of tebibytes to bytes.
