Crate neli Copy item path

<span class="sub-heading"><a href="../src/neli/lib.rs.html#1-701" class="src">Source</a>
</span>

Expand description

# <a href="#neli-type-safety-for-netlink" class="doc-anchor">§</a>neli: Type safety for netlink

## <a href="#rationale" class="doc-anchor">§</a>Rationale

This crate aims to be a pure Rust implementation that defines the
necessary constants and wraps them in enums to distinguish between
various categories of constants in the context of netlink.

## <a href="#the-project-is-broken-down-into-the-following-modules"
class="doc-anchor">§</a>The project is broken down into the following modules:

-   `attr` - This defines a generic interface for netlink attributes
    (both generic and routing netlink attributes).
-   `consts` - This is where all of the C-defined constants are wrapped
    into type safe enums for use in the library.
-   `err` - This module contains all of the protocol and library-level
    errors encountered in the code.
-   `genl` - This code provides parsing for the generic netlink
-   `iter` - This code handles iterating over received netlink packets.
-   `nl` - This is the top level netlink header code that handles the
    header that all netlink messages are encapsulated in.
-   `rtnl` - This module is for the routing netlink subsystem of the
    netlink protocol.
-   `socket` - This provides a socket structure for use in sending and
    receiving messages and a number of convenience functions for
    commonly encountered use cases.
-   `types` - Data types used in serialization and deserialization of
    packets.
-   `utils` - Data types that primarily serve the purpose of handling
    kernel data format conversions to easily usable Rust constructs for
    the user.

## <a href="#design-decisions" class="doc-anchor">§</a>Design decisions

This is a fairly low level library that currently does not have a whole
lot of higher level handle-type data structures and relies mostly on the
[`NlSocket`](socket/struct.NlSocket.html "struct neli::socket::NlSocket")
and
[`NlSocketHandle`](socket/struct.NlSocketHandle.html "struct neli::socket::NlSocketHandle")
structs to provide most of the convenience functions.

The goal of this library is completeness for handling netlink and am
working to incorporate features that will make this library easier to
use in all use cases. If you have a use case you would like to see
supported, please open an issue on Github.

## <a href="#examples" class="doc-anchor">§</a>Examples

Examples of working code exist in the `examples/` subdirectory on
Github. Run `cargo build --examples` to build the examples.

Workflows usually follow a pattern of socket creation, and then either
sending and receiving messages in request/response formats:

``` {.rust .rust-example-rendered}
use std::error::Error;

use neli::{
    consts::{genl::*, nl::*, socket::*},
    err::NlError,
    genl::{Genlmsghdr, Nlattr},
    nl::{Nlmsghdr, NlPayload},
    socket::NlSocketHandle,
    types::{Buffer, GenlBuffer},
};

const GENL_VERSION: u8 = 1;

fn request_response() -> Result<(), Box<dyn Error>> {
    let mut socket = NlSocketHandle::connect(
        NlFamily::Generic,
        None,
        &[],
    )?;

    let attrs: GenlBuffer<Index, Buffer> = GenlBuffer::new();
    let genlhdr = Genlmsghdr::new(
        CtrlCmd::Getfamily,
        GENL_VERSION,
        attrs,
    );
    let nlhdr = {
        let len = None;
        let nl_type = GenlId::Ctrl;
        let flags = NlmFFlags::new(&[NlmF::Request, NlmF::Dump]);
        let seq = None;
        let pid = None;
        let payload = NlPayload::Payload(genlhdr);
        Nlmsghdr::new(len, nl_type, flags, seq, pid, payload)
    };
    socket.send(nlhdr)?;
     
    // Do things with multi-message response to request...
    let mut iter = socket.iter::<NlTypeWrapper, Genlmsghdr<CtrlCmd, CtrlAttr>>(false);
    while let Some(Ok(response)) = iter.next() {
        // Do things with response here...
    }
     
    // Or get single message back...
    let msg = socket.recv::<Nlmsg, Genlmsghdr<CtrlCmd, CtrlAttr>>()?;

    Ok(())
}
```

or a subscriptions to a stream of event notifications from netlink:

``` {.rust .rust-example-rendered}
use std::error::Error;

use neli::{
    consts::{genl::*, nl::*, socket::*},
    err::NlError,
    genl::Genlmsghdr,
    socket,
};

fn subscribe_to_mcast() -> Result<(), Box<dyn Error>> {
    let mut s = socket::NlSocketHandle::connect(
        NlFamily::Generic,
        None,
        &[],
    )?;
    let id = s.resolve_nl_mcast_group(
        "my_family_name",
        "my_multicast_group_name",
    )?;
    s.add_mcast_membership(&[id])?;
    for next in s.iter::<NlTypeWrapper, Genlmsghdr<u8, u16>>(true) {
        // Do stuff here with parsed packets...
     
        // like printing a debug representation of them:
        println!("{:?}", next?);
    }

    Ok(())
}
```

## <a href="#documentation" class="doc-anchor">§</a>Documentation

Each module has been documented extensively to provide information on
how to use the code contained in the module. Pull requests for
documentation mistakes, updates, and rewording for clarity is a valuable
contribution as this project aims to be as simple to use as possible.

# Modules<a href="#modules" class="anchor">§</a>

<a href="attr/index.html" class="mod" title="mod neli::attr">attr</a>  
Shared attribute code for all types of netlink attributes.

<a href="consts/index.html" class="mod"
title="mod neli::consts">consts</a>  
High level notes

<a href="err/index.html" class="mod" title="mod neli::err">err</a>  
This is the module that contains the error types used in `neli`

<a href="genl/index.html" class="mod" title="mod neli::genl">genl</a>  
This module contains generic netlink parsing data structures. This is
all handled by the
[`Genlmsghdr`](genl/struct.Genlmsghdr.html "struct neli::genl::Genlmsghdr")
header struct which contains all of the information needed for the
generic netlink layer.

<a href="iter/index.html" class="mod" title="mod neli::iter">iter</a>  
Module for iteration over netlink responses

<a href="nl/index.html" class="mod" title="mod neli::nl">nl</a>  
This module contains the top level netlink header code. Every netlink
message will be encapsulated in a top level `Nlmsghdr`.

<a href="rtnl/index.html" class="mod" title="mod neli::rtnl">rtnl</a>  
This module provides an implementation of routing netlink structures and
the routing attributes that are at the end of most routing netlink
responses.

<a href="socket/index.html" class="mod"
title="mod neli::socket">socket</a>  
This module provides code that glues all of the other modules together
and allows message send and receive operations.

<a href="types/index.html" class="mod" title="mod neli::types">types</a>  
Module containing various types used across the various netlink
structures used in `neli`.

<a href="utils/index.html" class="mod" title="mod neli::utils">utils</a>  
A module containing utilities for working with constructs like bitflags
and other low level operations.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.impl_flags.html" class="macro"
title="macro neli::impl_flags">impl_flags</a>  
Implement a container for bit flag enums where the set of flags will be
condensed into a single value.

<a href="macro.impl_trait.html" class="macro"
title="macro neli::impl_trait">impl_trait</a>  
For generating a marker trait that flags a new enum as usable in a field
that accepts a generic type. This way, the type parameter can be
constrained by a trait bound to only accept enums that implement the
marker trait.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.BeU64.html" class="struct"
title="struct neli::BeU64">BeU64</a>  
A `u64` data type that will always be serialized as big endian

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.FromBytes.html" class="trait"
title="trait neli::FromBytes">FromBytes</a>  
A trait defining how to convert from a byte buffer to a netlink data
structure.

<a href="trait.FromBytesWithInput.html" class="trait"
title="trait neli::FromBytesWithInput">FromBytesWithInput</a>  
Takes an arbitrary input which serves as additional information for
guiding the conversion from a byte buffer to a data structure. A common
workflow is a data structure that has a size to determine how much more
of the data in the byte buffer is part of a given data structure.

<a href="trait.Header.html" class="trait"
title="trait neli::Header">Header</a>  
Defined for data structures that contain a header.

<a href="trait.Size.html" class="trait"
title="trait neli::Size">Size</a>  
A trait defining methods that apply to all netlink data structures
related to sizing of data types.

<a href="trait.ToBytes.html" class="trait"
title="trait neli::ToBytes">ToBytes</a>  
A trait defining a netlink data structure’s conversion to a byte buffer.

<a href="trait.TypeSize.html" class="trait"
title="trait neli::TypeSize">TypeSize</a>  
A trait defining methods that apply to constant-sized data types related
to size.

# Attribute Macros<a href="#attributes" class="anchor">§</a>

<a href="attr.neli_enum.html" class="attr"
title="attr neli::neli_enum">neli_enum</a>  
Converts an enum from the form:

# Derive Macros<a href="#derives" class="anchor">§</a>

<a href="derive.FromBytes.html" class="derive"
title="derive neli::FromBytes">FromBytes</a>  
Derives the neli `FromBytes` trait for a struct.

<a href="derive.FromBytesWithInput.html" class="derive"
title="derive neli::FromBytesWithInput">FromBytesWithInput</a>  
Derives the neli `FromBytesWithInput` trait for a struct.

<a href="derive.Header.html" class="derive"
title="derive neli::Header">Header</a>  
Derives the neli `Header` trait for a struct or enum. Unlike other
derive macros in this crate, the `Header` derive macro does not impose
type parameter bounds on type parameters. See the accepted attribute for
more information. The reason for this is that the last field is
considered to be the payload. Because the payload may be represented by
a type parameter, we cannot blindly restrict type parameters or else we
impose an artificial restriction of `TypeSize` on the payload type
parameter. This is a problem for the `Header` trait as the payload may
be unsized even if the rest of the header is composed exclusively of
statically sized types and are therefore compatible with the `TypeSize`
trait.

<a href="derive.Size.html" class="derive"
title="derive neli::Size">Size</a>  
Derives the neli `Size` trait for a struct or enum.

<a href="derive.ToBytes.html" class="derive"
title="derive neli::ToBytes">ToBytes</a>  
Derives the neli `ToBytes` trait for a struct or enum.
