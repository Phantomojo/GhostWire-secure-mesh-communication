Crate icu\_provider Copy item path

<span class="sub-heading"><a href="../src/icu_provider/lib.rs.html#5-231" class="src">Source</a>
</span>

Expand description

`icu_provider` is one of the `ICU4X` components.

Unicode’s experience with ICU4X’s parent projects, ICU4C and ICU4J, led
the team to realize that data management is the most critical aspect of
deploying internationalization, and that it requires a high level of
customization for the needs of the platform it is embedded in. As a
result ICU4X comes with a selection of providers that should allow for
ICU4X to naturally fit into different business and technological needs
of customers.

`icu_provider` defines traits and structs for transmitting data through
the ICU4X locale data pipeline. The primary trait is
[`DataProvider`](trait.DataProvider.html "trait icu_provider::DataProvider").
It is parameterized by a
[`DataMarker`](trait.DataMarker.html "trait icu_provider::DataMarker"),
which is the type-system-level data identifier.
[`DataProvider`](trait.DataProvider.html "trait icu_provider::DataProvider")
has a single method,
[`DataProvider::load`](trait.DataProvider.html#tymethod.load "method icu_provider::DataProvider::load"),
which transforms a
[`DataRequest`](struct.DataRequest.html "struct icu_provider::DataRequest")
into a
[`DataResponse`](struct.DataResponse.html "struct icu_provider::DataResponse").

-   [`DataRequest`](struct.DataRequest.html "struct icu_provider::DataRequest")
    contains selectors to choose a specific variant of the marker, such
    as a locale.
-   [`DataResponse`](struct.DataResponse.html "struct icu_provider::DataResponse")
    contains the data if the request was successful.

The most common types required for this crate are included via the
prelude:

``` {.rust .rust-example-rendered}
use icu_provider::prelude::*;
```

## <a href="#dynamic-data-providers" class="doc-anchor">§</a>Dynamic Data Providers

If the type system cannot be leveraged to load data (such as when
dynamically loading from I/O), there’s another form of the
[`DataProvider`](trait.DataProvider.html "trait icu_provider::DataProvider"):
[`DynamicDataProvider`](trait.DynamicDataProvider.html "trait icu_provider::DynamicDataProvider").
While
[`DataProvider`](trait.DataProvider.html "trait icu_provider::DataProvider")
is parametrized on the type-system level by a
[`DataMarker`](trait.DataMarker.html "trait icu_provider::DataMarker")
(which are distinct types implementing this trait),
[`DynamicDataProvider`](trait.DynamicDataProvider.html "trait icu_provider::DynamicDataProvider")s
are parametrized at runtime by a
[`DataMarkerInfo`](struct.DataMarkerInfo.html "struct icu_provider::DataMarkerInfo")
struct, which essentially is the runtime representation of the
[`DataMarker`](trait.DataMarker.html "trait icu_provider::DataMarker")
type.

The
[`DynamicDataProvider`](trait.DynamicDataProvider.html "trait icu_provider::DynamicDataProvider")
is still type-level parametrized by the type that it loads, and there
are two implementations that should be called out

-   [`DynamicDataProvider<BufferMarker>`](trait.DynamicDataProvider.html "trait icu_provider::DynamicDataProvider"),
    a.k.a.
    [`BufferProvider`](buf/trait.BufferProvider.html "trait icu_provider::buf::BufferProvider")
    returns data as `[u8]` buffers.

### <a href="#bufferprovider" class="doc-anchor">§</a>BufferProvider

These providers are able to return unstructured data typically
represented as \[`serde`\]-serialized buffers. Users can call
[`as_deserializing()`](buf::AsDeserializingBufferProvider::as_deserializing)
to get an object implementing
[`DataProvider`](trait.DataProvider.html "trait icu_provider::DataProvider")
by invoking Serde Deserialize.

Examples of BufferProviders:

-   [`FsDataProvider`](https://docs.rs/icu_provider_fs/latest/icu_provider_fs/struct.FsDataProvider.html)
    reads individual buffers from the filesystem.
-   [`BlobDataProvider`](https://docs.rs/icu_provider_blob/latest/icu_provider_blob/struct.BlobDataProvider.html)
    reads buffers from a large in-memory blob.

## <a href="#provider-adapters" class="doc-anchor">§</a>Provider Adapters

ICU4X offers several built-in modules to combine providers in
interesting ways. These can be found in the
[`icu_provider_adapters`](https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/index.html)
crate.

## <a href="#testing-provider" class="doc-anchor">§</a>Testing Provider

This crate also contains a concrete provider for demonstration purposes:

-   [`HelloWorldProvider`](hello_world::HelloWorldProvider) returns
    “hello world” strings in several languages.

## <a href="#types-and-lifetimes" class="doc-anchor">§</a>Types and Lifetimes

Types compatible with
[`Yokeable`](../yoke/yokeable/trait.Yokeable.html "trait yoke::yokeable::Yokeable")
can be passed through the data provider, so long as they are associated
with a marker type implementing
[`DynamicDataMarker`](trait.DynamicDataMarker.html "trait icu_provider::DynamicDataMarker").

Data structs should generally have one lifetime argument: `'data`. This
lifetime allows data structs to borrow zero-copy data.

# Modules<a href="#modules" class="anchor">§</a>

<a href="baked/index.html" class="mod"
title="mod icu_provider::baked">baked</a>  
This module contains scaffolding for baked providers, typically
generated using databake.

<a href="buf/index.html" class="mod"
title="mod icu_provider::buf">buf</a>  
Traits for data providers that produce opaque buffers.

<a href="constructors/index.html" class="mod"
title="mod icu_provider::constructors">constructors</a>  
📚 *This module documents ICU4X constructor signatures.*

<a href="dynutil/index.html" class="mod"
title="mod icu_provider::dynutil">dynutil</a>  
Utilities for using trait objects with `DataPayload`.

<a href="marker/index.html" class="mod"
title="mod icu_provider::marker">marker</a>  
Additional
[`DataMarker`](trait.DataMarker.html "trait icu_provider::DataMarker")
helpers.

<a href="prelude/index.html" class="mod"
title="mod icu_provider::prelude">prelude</a>  
Core selection of APIs and structures for the ICU4X data provider.

<a href="ule/index.html" class="mod"
title="mod icu_provider::ule">ule</a>  
Traits that data provider implementations can use to optimize storage by
using
[`VarULE`](../zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE").

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro._internal_noop_log.html" class="macro"
title="macro icu_provider::_internal_noop_log">_internal_noop_log</a>  
<a href="macro.data_marker.html" class="macro"
title="macro icu_provider::data_marker">data_marker</a>  
Creates a data marker.

<a href="macro.data_struct.html" class="macro"
title="macro icu_provider::data_struct">data_struct</a>  
Implements required traits on data structs, such as
\[`MaybeEncodeAsVarULE`\].

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AttributeParseError.html" class="struct"
title="struct icu_provider::AttributeParseError">AttributeParseError</a>  
Invalid character

<a href="struct.Cart.html" class="struct"
title="struct icu_provider::Cart">Cart</a>  
The type of the “cart” that is used by
[`DataPayload`](struct.DataPayload.html "struct icu_provider::DataPayload").

<a href="struct.DataError.html" class="struct"
title="struct icu_provider::DataError">DataError</a>  
The error type for ICU4X data provider operations.

<a href="struct.DataIdentifierBorrowed.html" class="struct"
title="struct icu_provider::DataIdentifierBorrowed">DataIdentifierBorrowed</a>  
The borrowed version of a \[`DataIdentifierCow`\].

<a href="struct.DataLocale.html" class="struct"
title="struct icu_provider::DataLocale">DataLocale</a>  
A locale type optimized for use in fallbacking and the ICU4X data
pipeline.

<a href="struct.DataMarkerAttributes.html" class="struct"
title="struct icu_provider::DataMarkerAttributes">DataMarkerAttributes</a>  
An additional key to identify data beyond a
[`DataLocale`](struct.DataLocale.html "struct icu_provider::DataLocale").

<a href="struct.DataMarkerInfo.html" class="struct"
title="struct icu_provider::DataMarkerInfo">DataMarkerInfo</a>  
Used for loading data from a dynamic ICU4X data provider.

<a href="struct.DataPayload.html" class="struct"
title="struct icu_provider::DataPayload">DataPayload</a>  
A container for data payloads returned from a data provider.

<a href="struct.DataProviderWithMarker.html" class="struct"
title="struct icu_provider::DataProviderWithMarker">DataProviderWithMarker</a>  
A
[`DataProvider`](trait.DataProvider.html "trait icu_provider::DataProvider")
associated with a specific marker.

<a href="struct.DataRequest.html" class="struct"
title="struct icu_provider::DataRequest">DataRequest</a>  
The request type passed into all data provider implementations.

<a href="struct.DataRequestMetadata.html" class="struct"
title="struct icu_provider::DataRequestMetadata">DataRequestMetadata</a>  
Metadata for data requests. This is currently empty, but it may be
extended with options for tuning locale fallback, buffer layout, and so
forth.

<a href="struct.DataResponse.html" class="struct"
title="struct icu_provider::DataResponse">DataResponse</a>  
A response object containing an object as payload and metadata about it.

<a href="struct.DataResponseMetadata.html" class="struct"
title="struct icu_provider::DataResponseMetadata">DataResponseMetadata</a>  
A response object containing metadata about the returned data.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.DataErrorKind.html" class="enum"
title="enum icu_provider::DataErrorKind">DataErrorKind</a>  
A list specifying general categories of data provider error.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.BoundDataProvider.html" class="trait"
title="trait icu_provider::BoundDataProvider">BoundDataProvider</a>  
A data provider that loads data for a specific data type.

<a href="trait.DataMarker.html" class="trait"
title="trait icu_provider::DataMarker">DataMarker</a>  
A
[`DynamicDataMarker`](trait.DynamicDataMarker.html "trait icu_provider::DynamicDataMarker")
with a
[`DataMarkerInfo`](struct.DataMarkerInfo.html "struct icu_provider::DataMarkerInfo")
attached.

<a href="trait.DataProvider.html" class="trait"
title="trait icu_provider::DataProvider">DataProvider</a>  
A data provider that loads data for a specific
[`DataMarkerInfo`](struct.DataMarkerInfo.html "struct icu_provider::DataMarkerInfo").

<a href="trait.DryDataProvider.html" class="trait"
title="trait icu_provider::DryDataProvider">DryDataProvider</a>  
A data provider that can determine whether it can load a particular data
identifier, potentially cheaper than actually performing the load.

<a href="trait.DynamicDataMarker.html" class="trait"
title="trait icu_provider::DynamicDataMarker">DynamicDataMarker</a>  
Trait marker for data structs. All types delivered by the data provider
must be associated with something implementing this trait.

<a href="trait.DynamicDataProvider.html" class="trait"
title="trait icu_provider::DynamicDataProvider">DynamicDataProvider</a>  
A data provider that loads data for a specific data type.

<a href="trait.DynamicDryDataProvider.html" class="trait"
title="trait icu_provider::DynamicDryDataProvider">DynamicDryDataProvider</a>  
A dynanmic data provider that can determine whether it can load a
particular data identifier, potentially cheaper than actually performing
the load.

<a href="trait.ResultDataError.html" class="trait"
title="trait icu_provider::ResultDataError">ResultDataError</a>  
Extension trait for `Result<T, DataError>`.
