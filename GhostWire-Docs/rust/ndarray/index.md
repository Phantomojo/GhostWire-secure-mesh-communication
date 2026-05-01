Crate ndarray Copy item path

<span class="sub-heading"><a href="../src/ndarray/lib.rs.html#8-1877" class="src">Source</a>
</span>

Expand description

The `ndarray` crate provides an *n*-dimensional container for general
elements and for numerics.

In *n*-dimensional we include, for example, 1-dimensional rows or
columns, 2-dimensional matrices, and higher dimensional arrays. If the
array has *n* dimensions, then an element in the array is accessed by
using that many indices. Each dimension is also called an *axis*.

To get started, functionality is provided in the following core types:

-   **[`ArrayBase`](struct.ArrayBase.html "struct ndarray::ArrayBase")**:
    The *n*-dimensional array type itself.  
    It is used to implement both the owned arrays and the views; see its
    docs for an overview of all array features.  
-   The main specific array type is
    **[`Array`](type.Array.html "type ndarray::Array")**, which owns its
    elements.
-   A reference type,
    **[`ArrayRef`](struct.ArrayRef.html "struct ndarray::ArrayRef")**,
    that contains most of the functionality for reading and writing to
    arrays.
-   A reference type,
    **[`LayoutRef`](struct.LayoutRef.html "struct ndarray::LayoutRef")**,
    that contains most of the functionality for reading and writing to
    array layouts: their shape and strides.

## <a href="#highlights" class="doc-anchor">§</a>Highlights

-   Generic *n*-dimensional array
-   [Slicing](struct.ArrayBase.html#slicing "struct ndarray::ArrayBase"),
    also with arbitrary step size, and negative indices to mean elements
    from the end of the axis.
-   Views and subviews of arrays; iterators that yield subviews.
-   Higher order operations and arithmetic are performant
-   Array views can be used to slice and mutate any `[T]` data using
    `ArrayView::from` and `ArrayViewMut::from`.
-   [`Zip`](struct.Zip.html "struct ndarray::Zip") for lock step
    function application across two or more arrays or other item
    producers
    ([`NdProducer`](trait.NdProducer.html "trait ndarray::NdProducer")
    trait).

## <a href="#crate-status" class="doc-anchor">§</a>Crate Status

-   Still iterating on and evolving the crate

    -   The crate is continuously developing, and breaking changes are
        expected during evolution from version to version. We adopt the
        newest stable rust features if we need them.
    -   Note that functions/methods/traits/etc. hidden from the docs are
        not considered part of the public API, so changes to them are
        not considered breaking changes.

-   Performance:

    -   Prefer higher order methods and arithmetic operations on arrays
        first, then iteration, and as a last priority using indexed
        algorithms.
    -   The higher order functions like
        [`.map()`](struct.ArrayRef.html#method.map "method ndarray::ArrayRef::map"),
        [`.map_inplace()`](struct.ArrayRef.html#method.map_inplace "method ndarray::ArrayRef::map_inplace"),
        [`.zip_mut_with()`](struct.ArrayRef.html#method.zip_mut_with "method ndarray::ArrayRef::zip_mut_with"),
        [`Zip`](struct.Zip.html "struct ndarray::Zip") and
        [`azip!()`](macro.azip.html "macro ndarray::azip") are the most
        efficient ways to perform single traversal and lock step
        traversal respectively.
    -   Performance of an operation depends on the memory layout of the
        array or array view. Especially if it’s a binary operation,
        which needs matching memory layout to be efficient (with some
        exceptions).
    -   Efficient floating point matrix multiplication even for very
        large matrices; can optionally use BLAS to improve it further.

-   **MSRV: Requires Rust 1.64 or later**

## <a href="#crate-feature-flags" class="doc-anchor">§</a>Crate Feature Flags

The following crate feature flags are available. They are configured in
your `Cargo.toml`. See \[`doc::crate_feature_flags`\] for more
information.

-   `std`: Rust standard library-using functionality (enabled by
    default)
-   `serde`: serialization support for serde 1.x
-   `rayon`: Parallel iterators, parallelized methods, the
    \[`parallel`\] module and \[`par_azip!`\].
-   `approx` Implementations of traits from the \[`approx`\] crate.
-   `blas`: transparent BLAS support for matrix multiplication, needs
    configuration.
-   `matrixmultiply-threading`: Use threading from `matrixmultiply`.

## <a href="#documentation" class="doc-anchor">§</a>Documentation

-   The docs for
    [`ArrayBase`](struct.ArrayBase.html "struct ndarray::ArrayBase")
    provide an overview of the *n*-dimensional array type. Other good
    pages to look at are the documentation for the
    [`s![]`](macro.s.html "macro ndarray::s") and
    [`azip!()`](macro.azip.html "macro ndarray::azip") macros.

-   If you have experience with NumPy, you may also be interested in
    [`ndarray_for_numpy_users`](doc::ndarray_for_numpy_users).

## <a href="#the-ndarray-ecosystem" class="doc-anchor">§</a>The ndarray ecosystem

`ndarray` provides a lot of functionality, but it’s not a one-stop
solution.

`ndarray` includes matrix multiplication and other binary/unary
operations out of the box. More advanced linear algebra routines (e.g.
SVD decomposition or eigenvalue computation) can be found in
[`ndarray-linalg`](https://crates.io/crates/ndarray-linalg).

The same holds for statistics: `ndarray` provides some basic
functionalities (e.g. `mean`) but more advanced routines can be found in
[`ndarray-stats`](https://crates.io/crates/ndarray-stats).

If you are looking to generate random arrays instead, check out
[`ndarray-rand`](https://crates.io/crates/ndarray-rand).

For conversion between `ndarray`,
[`nalgebra`](https://crates.io/crates/nalgebra) and
[`image`](https://crates.io/crates/image) check out
[`nshare`](https://crates.io/crates/nshare).

# Modules<a href="#modules" class="anchor">§</a>

<a href="iter/index.html" class="mod" title="mod ndarray::iter">iter</a>  
Producers, iterables and iterators.

<a href="linalg/index.html" class="mod"
title="mod ndarray::linalg">linalg</a>  
Linear algebra.

<a href="prelude/index.html" class="mod"
title="mod ndarray::prelude">prelude</a>  
ndarray prelude.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.array.html" class="macro"
title="macro ndarray::array">array</a>  
Create an **[`Array`](type.Array.html "type ndarray::Array")** with one,
two, three, four, five, or six dimensions.

<a href="macro.azip.html" class="macro"
title="macro ndarray::azip">azip</a>  
Array zip macro: lock step function application across several arrays
and producers.

<a href="macro.concatenate.html" class="macro"
title="macro ndarray::concatenate">concatenate</a>  
Concatenate arrays along the given axis.

<a href="macro.s.html" class="macro" title="macro ndarray::s">s</a>  
Slice argument constructor.

<a href="macro.stack.html" class="macro"
title="macro ndarray::stack">stack</a>  
Stack arrays along the new axis.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ArrayBase.html" class="struct"
title="struct ndarray::ArrayBase">ArrayBase</a>  
An *n*-dimensional array.

<a href="struct.ArrayRef.html" class="struct"
title="struct ndarray::ArrayRef">ArrayRef</a>  
A reference to an *n*-dimensional array whose data is safe to read and
write.

<a href="struct.Axis.html" class="struct"
title="struct ndarray::Axis">Axis</a>  
An axis index.

<a href="struct.AxisDescription.html" class="struct"
title="struct ndarray::AxisDescription">AxisDescription</a>  
Description of the axis, its length and its stride.

<a href="struct.Dim.html" class="struct"
title="struct ndarray::Dim">Dim</a>  
Dimension description.

<a href="struct.IxDynImpl.html" class="struct"
title="struct ndarray::IxDynImpl">IxDynImpl</a>  
Dynamic dimension or index type.

<a href="struct.LayoutRef.html" class="struct"
title="struct ndarray::LayoutRef">LayoutRef</a>  
A reference to the layout of an *n*-dimensional array.

<a href="struct.Linspace.html" class="struct"
title="struct ndarray::Linspace">Linspace</a>  
An iterator of a sequence of evenly spaced floats.

<a href="struct.Logspace.html" class="struct"
title="struct ndarray::Logspace">Logspace</a>  
An iterator of a sequence of logarithmically spaced number.

<a href="struct.MathCell.html" class="struct"
title="struct ndarray::MathCell">MathCell</a>  
A transparent wrapper of
[`Cell<T>`](https://doc.rust-lang.org/1.93.1/core/cell/struct.Cell.html "struct core::cell::Cell")
which is identical in every way, except it will implement arithmetic
operators as well.

<a href="struct.NewAxis.html" class="struct"
title="struct ndarray::NewAxis">NewAxis</a>  
Token to represent a new axis in a slice description.

<a href="struct.OwnedArcRepr.html" class="struct"
title="struct ndarray::OwnedArcRepr">OwnedArcRepr</a>  
ArcArray’s representation.

<a href="struct.OwnedRepr.html" class="struct"
title="struct ndarray::OwnedRepr">OwnedRepr</a>  
Array’s representation.

<a href="struct.RawRef.html" class="struct"
title="struct ndarray::RawRef">RawRef</a>  
A reference to an *n*-dimensional array whose data is not safe to read
or write.

<a href="struct.RawViewRepr.html" class="struct"
title="struct ndarray::RawViewRepr">RawViewRepr</a>  
Array pointer’s representation.

<a href="struct.Shape.html" class="struct"
title="struct ndarray::Shape">Shape</a>  
A contiguous array shape of n dimensions.

<a href="struct.ShapeError.html" class="struct"
title="struct ndarray::ShapeError">ShapeError</a>  
An error related to array shape or layout.

<a href="struct.Slice.html" class="struct"
title="struct ndarray::Slice">Slice</a>  
A slice (range with step size).

<a href="struct.SliceInfo.html" class="struct"
title="struct ndarray::SliceInfo">SliceInfo</a>  
Represents all of the necessary information to perform a slice.

<a href="struct.StrideShape.html" class="struct"
title="struct ndarray::StrideShape">StrideShape</a>  
An array shape of n dimensions in c-order, f-order or custom strides.

<a href="struct.ViewRepr.html" class="struct"
title="struct ndarray::ViewRepr">ViewRepr</a>  
Array view’s representation.

<a href="struct.Zip.html" class="struct"
title="struct ndarray::Zip">Zip</a>  
Lock step function application across several arrays or other producers.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.CowRepr.html" class="enum"
title="enum ndarray::CowRepr">CowRepr</a>  
CowArray’s representation.

<a href="enum.ErrorKind.html" class="enum"
title="enum ndarray::ErrorKind">ErrorKind</a>  
Error code for an error related to array shape or layout.

<a href="enum.FoldWhile.html" class="enum"
title="enum ndarray::FoldWhile">FoldWhile</a>  
Value controlling the execution of `.fold_while` on `Zip`.

<a href="enum.MeshIndex.html" class="enum"
title="enum ndarray::MeshIndex">MeshIndex</a>  
The indexing order for
[`meshgrid`](fn.meshgrid.html "fn ndarray::meshgrid"); see there for
more details.

<a href="enum.Order.html" class="enum"
title="enum ndarray::Order">Order</a>  
Array order

<a href="enum.SliceInfoElem.html" class="enum"
title="enum ndarray::SliceInfoElem">SliceInfoElem</a>  
A slice (range with step), an index, or a new axis token.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AsArray.html" class="trait"
title="trait ndarray::AsArray">AsArray</a>  
Argument conversion into an array view

<a href="trait.AssignElem.html" class="trait"
title="trait ndarray::AssignElem">AssignElem</a>  
A producer element that can be assigned to once

<a href="trait.Data.html" class="trait"
title="trait ndarray::Data">Data</a>  
Array representation trait.

<a href="trait.DataMut.html" class="trait"
title="trait ndarray::DataMut">DataMut</a>  
Array representation trait.

<a href="trait.DataOwned.html" class="trait"
title="trait ndarray::DataOwned">DataOwned</a>  
Array representation trait.

<a href="trait.DataShared.html" class="trait"
title="trait ndarray::DataShared">DataShared</a>  
Array representation trait.

<a href="trait.DimAdd.html" class="trait"
title="trait ndarray::DimAdd">DimAdd</a>  
Adds the two dimensions at compile time.

<a href="trait.DimMax.html" class="trait"
title="trait ndarray::DimMax">DimMax</a>  
A trait to specify when one dimension is strictly larger than another.

<a href="trait.Dimension.html" class="trait"
title="trait ndarray::Dimension">Dimension</a>  
Array shape and index trait.

<a href="trait.IndexLonger.html" class="trait"
title="trait ndarray::IndexLonger">IndexLonger</a>  
Extra indexing methods for array views

<a href="trait.IntoDimension.html" class="trait"
title="trait ndarray::IntoDimension">IntoDimension</a>  
Argument conversion a dimension.

<a href="trait.IntoNdProducer.html" class="trait"
title="trait ndarray::IntoNdProducer">IntoNdProducer</a>  
Argument conversion into a producer.

<a href="trait.LinalgScalar.html" class="trait"
title="trait ndarray::LinalgScalar">LinalgScalar</a>  
Elements that support linear algebra operations.

<a href="trait.MultiSliceArg.html" class="trait"
title="trait ndarray::MultiSliceArg">MultiSliceArg</a>  
Slicing information describing multiple mutable, disjoint slices.

<a href="trait.NdFloat.html" class="trait"
title="trait ndarray::NdFloat">NdFloat</a>  
Floating-point element types `f32` and `f64`.

<a href="trait.NdIndex.html" class="trait"
title="trait ndarray::NdIndex">NdIndex</a>  
Tuple or fixed size arrays that can be used to index an array.

<a href="trait.NdProducer.html" class="trait"
title="trait ndarray::NdProducer">NdProducer</a>  
A producer of an n-dimensional set of elements; for example an array
view, mutable array view or an iterator that yields chunks.

<a href="trait.RawData.html" class="trait"
title="trait ndarray::RawData">RawData</a>  
Array representation trait.

<a href="trait.RawDataClone.html" class="trait"
title="trait ndarray::RawDataClone">RawDataClone</a>  
Array representation trait.

<a href="trait.RawDataMut.html" class="trait"
title="trait ndarray::RawDataMut">RawDataMut</a>  
Array representation trait.

<a href="trait.RawDataSubst.html" class="trait"
title="trait ndarray::RawDataSubst">RawDataSubst</a>  
Array representation trait.

<a href="trait.RemoveAxis.html" class="trait"
title="trait ndarray::RemoveAxis">RemoveAxis</a>  
Array shape with a next smaller dimension.

<a href="trait.ScalarOperand.html" class="trait"
title="trait ndarray::ScalarOperand">ScalarOperand</a>  
Elements that can be used as direct operands in arithmetic with arrays.

<a href="trait.ShapeArg.html" class="trait"
title="trait ndarray::ShapeArg">ShapeArg</a>  
Array shape argument with optional order parameter

<a href="trait.ShapeBuilder.html" class="trait"
title="trait ndarray::ShapeBuilder">ShapeBuilder</a>  
A trait for `Shape` and `D where D: Dimension` that allows customizing
the memory layout (strides) of an array shape.

<a href="trait.SliceArg.html" class="trait"
title="trait ndarray::SliceArg">SliceArg</a>  
A type that can slice an array of dimension `D`.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.Dim.html" class="fn" title="fn ndarray::Dim">Dim</a>  
Create a new dimension value.

<a href="fn.Ix0.html" class="fn" title="fn ndarray::Ix0">Ix0</a>  
Create a zero-dimensional index

<a href="fn.Ix1.html" class="fn" title="fn ndarray::Ix1">Ix1</a>  
Create a one-dimensional index

<a href="fn.Ix2.html" class="fn" title="fn ndarray::Ix2">Ix2</a>  
Create a two-dimensional index

<a href="fn.Ix3.html" class="fn" title="fn ndarray::Ix3">Ix3</a>  
Create a three-dimensional index

<a href="fn.Ix4.html" class="fn" title="fn ndarray::Ix4">Ix4</a>  
Create a four-dimensional index

<a href="fn.Ix5.html" class="fn" title="fn ndarray::Ix5">Ix5</a>  
Create a five-dimensional index

<a href="fn.Ix6.html" class="fn" title="fn ndarray::Ix6">Ix6</a>  
Create a six-dimensional index

<a href="fn.IxDyn.html" class="fn" title="fn ndarray::IxDyn">IxDyn</a>  
Create a dynamic-dimensional index

<a href="fn.arr0.html" class="fn" title="fn ndarray::arr0">arr0</a>  
Create a zero-dimensional array with the element `x`.

<a href="fn.arr1.html" class="fn" title="fn ndarray::arr1">arr1</a>  
Create a one-dimensional array with elements from `xs`.

<a href="fn.arr2.html" class="fn" title="fn ndarray::arr2">arr2</a>  
Create a two-dimensional array with elements from `xs`.

<a href="fn.arr3.html" class="fn" title="fn ndarray::arr3">arr3</a>  
Create a three-dimensional array with elements from `xs`.

<a href="fn.aview0.html" class="fn"
title="fn ndarray::aview0">aview0</a>  
Create a zero-dimensional array view borrowing `x`.

<a href="fn.aview1.html" class="fn"
title="fn ndarray::aview1">aview1</a>  
Create a one-dimensional array view with elements borrowing `xs`.

<a href="fn.aview2.html" class="fn"
title="fn ndarray::aview2">aview2</a>  
Create a two-dimensional array view with elements borrowing `xs`.

<a href="fn.aview_mut1.html" class="fn"
title="fn ndarray::aview_mut1">aview_mut1</a>  
Create a one-dimensional read-write array view with elements borrowing
`xs`.

<a href="fn.aview_mut2.html" class="fn"
title="fn ndarray::aview_mut2">aview_mut2</a>  
Create a two-dimensional read-write array view with elements borrowing
`xs`.

<a href="fn.concatenate.html" class="fn"
title="fn ndarray::concatenate">concatenate</a>  
Concatenate arrays along the given axis.

<a href="fn.indices.html" class="fn"
title="fn ndarray::indices">indices</a>  
Create an iterable of the array shape `shape`.

<a href="fn.indices_of.html" class="fn"
title="fn ndarray::indices_of">indices_of</a>  
Return an iterable of the indices of the passed-in array.

<a href="fn.linspace.html" class="fn"
title="fn ndarray::linspace">linspace</a>  
Return an iterator of evenly spaced floats.

<a href="fn.logspace.html" class="fn"
title="fn ndarray::logspace">logspace</a>  
An iterator of a sequence of logarithmically spaced numbers.

<a href="fn.meshgrid.html" class="fn"
title="fn ndarray::meshgrid">meshgrid</a>  
Create coordinate matrices from coordinate vectors.

<a href="fn.range.html" class="fn" title="fn ndarray::range">range</a>  
Return an iterator of floats from `a` to `b` (exclusive), incrementing
by `step`.

<a href="fn.rcarr1.html" class="fn"
title="fn ndarray::rcarr1">rcarr1</a>  
Create a one-dimensional array with elements from `xs`.

<a href="fn.rcarr2.html" class="fn"
title="fn ndarray::rcarr2">rcarr2</a>  
Create a two-dimensional array with elements from `xs`.

<a href="fn.rcarr3.html" class="fn"
title="fn ndarray::rcarr3">rcarr3</a>  
Create a three-dimensional array with elements from `xs`.

<a href="fn.stack.html" class="fn" title="fn ndarray::stack">stack</a>  
Stack arrays along the new axis.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.ArcArray.html" class="type"
title="type ndarray::ArcArray">ArcArray</a>  
An array where the data has shared ownership and is copy on write.

<a href="type.ArcArray1.html" class="type"
title="type ndarray::ArcArray1">ArcArray1</a>  
one-dimensional shared ownership array

<a href="type.ArcArray2.html" class="type"
title="type ndarray::ArcArray2">ArcArray2</a>  
two-dimensional shared ownership array

<a href="type.ArcArray3.html" class="type"
title="type ndarray::ArcArray3">ArcArray3</a>  
three-dimensional shared ownership array

<a href="type.ArcArray4.html" class="type"
title="type ndarray::ArcArray4">ArcArray4</a>  
four-dimensional shared ownership array

<a href="type.ArcArray5.html" class="type"
title="type ndarray::ArcArray5">ArcArray5</a>  
five-dimensional shared ownership array

<a href="type.ArcArray6.html" class="type"
title="type ndarray::ArcArray6">ArcArray6</a>  
six-dimensional shared ownership array

<a href="type.ArcArrayD.html" class="type"
title="type ndarray::ArcArrayD">ArcArrayD</a>  
dynamic-dimensional shared ownership array

<a href="type.Array.html" class="type"
title="type ndarray::Array">Array</a>  
An array that owns its data uniquely.

<a href="type.Array0.html" class="type"
title="type ndarray::Array0">Array0</a>  
zero-dimensional array

<a href="type.Array1.html" class="type"
title="type ndarray::Array1">Array1</a>  
one-dimensional array

<a href="type.Array2.html" class="type"
title="type ndarray::Array2">Array2</a>  
two-dimensional array

<a href="type.Array3.html" class="type"
title="type ndarray::Array3">Array3</a>  
three-dimensional array

<a href="type.Array4.html" class="type"
title="type ndarray::Array4">Array4</a>  
four-dimensional array

<a href="type.Array5.html" class="type"
title="type ndarray::Array5">Array5</a>  
five-dimensional array

<a href="type.Array6.html" class="type"
title="type ndarray::Array6">Array6</a>  
six-dimensional array

<a href="type.ArrayD.html" class="type"
title="type ndarray::ArrayD">ArrayD</a>  
dynamic-dimensional array

<a href="type.ArrayRef0.html" class="type"
title="type ndarray::ArrayRef0">ArrayRef0</a>  
zero-dimensional array reference

<a href="type.ArrayRef1.html" class="type"
title="type ndarray::ArrayRef1">ArrayRef1</a>  
one-dimensional array reference

<a href="type.ArrayRef2.html" class="type"
title="type ndarray::ArrayRef2">ArrayRef2</a>  
two-dimensional array reference

<a href="type.ArrayRef3.html" class="type"
title="type ndarray::ArrayRef3">ArrayRef3</a>  
three-dimensional array reference

<a href="type.ArrayRef4.html" class="type"
title="type ndarray::ArrayRef4">ArrayRef4</a>  
four-dimensional array reference

<a href="type.ArrayRef5.html" class="type"
title="type ndarray::ArrayRef5">ArrayRef5</a>  
five-dimensional array reference

<a href="type.ArrayRef6.html" class="type"
title="type ndarray::ArrayRef6">ArrayRef6</a>  
six-dimensional array reference

<a href="type.ArrayRefD.html" class="type"
title="type ndarray::ArrayRefD">ArrayRefD</a>  
dynamic-dimensional array reference

<a href="type.ArrayView.html" class="type"
title="type ndarray::ArrayView">ArrayView</a>  
A read-only array view.

<a href="type.ArrayView0.html" class="type"
title="type ndarray::ArrayView0">ArrayView0</a>  
zero-dimensional array view

<a href="type.ArrayView1.html" class="type"
title="type ndarray::ArrayView1">ArrayView1</a>  
one-dimensional array view

<a href="type.ArrayView2.html" class="type"
title="type ndarray::ArrayView2">ArrayView2</a>  
two-dimensional array view

<a href="type.ArrayView3.html" class="type"
title="type ndarray::ArrayView3">ArrayView3</a>  
three-dimensional array view

<a href="type.ArrayView4.html" class="type"
title="type ndarray::ArrayView4">ArrayView4</a>  
four-dimensional array view

<a href="type.ArrayView5.html" class="type"
title="type ndarray::ArrayView5">ArrayView5</a>  
five-dimensional array view

<a href="type.ArrayView6.html" class="type"
title="type ndarray::ArrayView6">ArrayView6</a>  
six-dimensional array view

<a href="type.ArrayViewD.html" class="type"
title="type ndarray::ArrayViewD">ArrayViewD</a>  
dynamic-dimensional array view

<a href="type.ArrayViewMut.html" class="type"
title="type ndarray::ArrayViewMut">ArrayViewMut</a>  
A read-write array view.

<a href="type.ArrayViewMut0.html" class="type"
title="type ndarray::ArrayViewMut0">ArrayViewMut0</a>  
zero-dimensional read-write array view

<a href="type.ArrayViewMut1.html" class="type"
title="type ndarray::ArrayViewMut1">ArrayViewMut1</a>  
one-dimensional read-write array view

<a href="type.ArrayViewMut2.html" class="type"
title="type ndarray::ArrayViewMut2">ArrayViewMut2</a>  
two-dimensional read-write array view

<a href="type.ArrayViewMut3.html" class="type"
title="type ndarray::ArrayViewMut3">ArrayViewMut3</a>  
three-dimensional read-write array view

<a href="type.ArrayViewMut4.html" class="type"
title="type ndarray::ArrayViewMut4">ArrayViewMut4</a>  
four-dimensional read-write array view

<a href="type.ArrayViewMut5.html" class="type"
title="type ndarray::ArrayViewMut5">ArrayViewMut5</a>  
five-dimensional read-write array view

<a href="type.ArrayViewMut6.html" class="type"
title="type ndarray::ArrayViewMut6">ArrayViewMut6</a>  
six-dimensional read-write array view

<a href="type.ArrayViewMutD.html" class="type"
title="type ndarray::ArrayViewMutD">ArrayViewMutD</a>  
dynamic-dimensional read-write array view

<a href="type.CowArray.html" class="type"
title="type ndarray::CowArray">CowArray</a>  
An array with copy-on-write behavior.

<a href="type.Ix.html" class="type" title="type ndarray::Ix">Ix</a>  
Array index type

<a href="type.Ix0.html" class="type" title="type ndarray::Ix0">Ix0</a>  
zero-dimensionial

<a href="type.Ix1.html" class="type" title="type ndarray::Ix1">Ix1</a>  
one-dimensional

<a href="type.Ix2.html" class="type" title="type ndarray::Ix2">Ix2</a>  
two-dimensional

<a href="type.Ix3.html" class="type" title="type ndarray::Ix3">Ix3</a>  
three-dimensional

<a href="type.Ix4.html" class="type" title="type ndarray::Ix4">Ix4</a>  
four-dimensional

<a href="type.Ix5.html" class="type" title="type ndarray::Ix5">Ix5</a>  
five-dimensional

<a href="type.Ix6.html" class="type" title="type ndarray::Ix6">Ix6</a>  
six-dimensional

<a href="type.IxDyn.html" class="type"
title="type ndarray::IxDyn">IxDyn</a>  
dynamic-dimensional

<a href="type.Ixs.html" class="type" title="type ndarray::Ixs">Ixs</a>  
Array index type (signed)

<a href="type.LayoutRef0.html" class="type"
title="type ndarray::LayoutRef0">LayoutRef0</a>  
zero-dimensional layout reference

<a href="type.LayoutRef1.html" class="type"
title="type ndarray::LayoutRef1">LayoutRef1</a>  
one-dimensional layout reference

<a href="type.LayoutRef2.html" class="type"
title="type ndarray::LayoutRef2">LayoutRef2</a>  
two-dimensional layout reference

<a href="type.LayoutRef3.html" class="type"
title="type ndarray::LayoutRef3">LayoutRef3</a>  
three-dimensional layout reference

<a href="type.LayoutRef4.html" class="type"
title="type ndarray::LayoutRef4">LayoutRef4</a>  
four-dimensional layout reference

<a href="type.LayoutRef5.html" class="type"
title="type ndarray::LayoutRef5">LayoutRef5</a>  
five-dimensional layout reference

<a href="type.LayoutRef6.html" class="type"
title="type ndarray::LayoutRef6">LayoutRef6</a>  
six-dimensional layout reference

<a href="type.LayoutRefD.html" class="type"
title="type ndarray::LayoutRefD">LayoutRefD</a>  
dynamic-dimensional layout reference

<a href="type.RawArrayView.html" class="type"
title="type ndarray::RawArrayView">RawArrayView</a>  
A read-only array view without a lifetime.

<a href="type.RawArrayViewMut.html" class="type"
title="type ndarray::RawArrayViewMut">RawArrayViewMut</a>  
A mutable array view without a lifetime.
