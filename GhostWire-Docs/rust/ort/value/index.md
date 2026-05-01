[ort](../index.html)

Module value Copy item path

<span class="sub-heading"><a href="../../src/ort/value/mod.rs.html#1-547" class="src">Source</a>
</span>

Expand description

[`Value`](struct.Value.html "struct ort::value::Value")s are data
containers used as inputs/outputs in ONNX Runtime graphs.

The most common type of value is
[`Tensor`](type.Tensor.html "type ort::value::Tensor"):

``` {.rust .rust-example-rendered}
// Create a tensor from a raw data vector
let tensor = Tensor::from_array(([1usize, 2, 3], vec![1.0_f32, 2.0, 3.0, 4.0, 5.0, 6.0].into_boxed_slice()))?;

// Create a tensor from an `ndarray::Array`
#[cfg(feature = "ndarray")]
let tensor = Tensor::from_array(ndarray::Array4::<f32>::zeros((1, 16, 16, 3)))?;
```

ONNX Runtime also supports
[`Sequence`](type.Sequence.html "type ort::value::Sequence")s and
[`Map`](type.Map.html "type ort::value::Map")s, though they are less
commonly used.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DynMapValueType.html" class="struct"
title="struct ort::value::DynMapValueType">DynMapValueType</a>  
<a href="struct.DynSequenceValueType.html" class="struct"
title="struct ort::value::DynSequenceValueType">DynSequenceValueType</a>  
<a href="struct.DynTensorValueType.html" class="struct"
title="struct ort::value::DynTensorValueType">DynTensorValueType</a>  
<a href="struct.DynValueTypeMarker.html" class="struct"
title="struct ort::value::DynValueTypeMarker">DynValueTypeMarker</a>  
The dynamic type marker, used for values which can be of any type.

<a href="struct.MapValueType.html" class="struct"
title="struct ort::value::MapValueType">MapValueType</a>  
<a href="struct.Outlet.html" class="struct"
title="struct ort::value::Outlet">Outlet</a>  
The input/output to a
[session](../session/struct.Session.html "struct ort::session::Session"),
[node](crate::editor::Node), or
[operator](../operator/trait.Operator.html "trait ort::operator::Operator"),
consisting of a name and expected [data
type](enum.ValueType.html "enum ort::value::ValueType").

<a href="struct.SequenceValueType.html" class="struct"
title="struct ort::value::SequenceValueType">SequenceValueType</a>  
<a href="struct.Shape.html" class="struct"
title="struct ort::value::Shape">Shape</a>  
<a href="struct.SymbolicDimensions.html" class="struct"
title="struct ort::value::SymbolicDimensions">SymbolicDimensions</a>  
<a href="struct.TensorArrayDataParts.html" class="struct"
title="struct ort::value::TensorArrayDataParts">TensorArrayDataParts</a>  
<a href="struct.TensorValueType.html" class="struct"
title="struct ort::value::TensorValueType">TensorValueType</a>  
<a href="struct.Value.html" class="struct"
title="struct ort::value::Value">Value</a>  
A [`Value`](struct.Value.html "struct ort::value::Value") contains data
for inputs/outputs in ONNX Runtime graphs.
[`Value`](struct.Value.html "struct ort::value::Value")s can be a
[`Tensor`](type.Tensor.html "type ort::value::Tensor"),
[`Sequence`](type.Sequence.html "type ort::value::Sequence") (aka
array/vector), or [`Map`](type.Map.html "type ort::value::Map").

<a href="struct.ValueRef.html" class="struct"
title="struct ort::value::ValueRef">ValueRef</a>  
A temporary version of a
[`Value`](struct.Value.html "struct ort::value::Value") with a lifetime
specifier.

<a href="struct.ValueRefMut.html" class="struct"
title="struct ort::value::ValueRefMut">ValueRefMut</a>  
A mutable temporary version of a
[`Value`](struct.Value.html "struct ort::value::Value") with a lifetime
specifier.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.TensorElementType.html" class="enum"
title="enum ort::value::TensorElementType">TensorElementType</a>  
Enum mapping ONNX Runtime’s supported tensor data types.

<a href="enum.ValueType.html" class="enum"
title="enum ort::value::ValueType">ValueType</a>  
The type of a [`Value`](struct.Value.html "struct ort::value::Value"),
or a session input/output.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.DefiniteTensorValueTypeMarker.html" class="trait"
title="trait ort::value::DefiniteTensorValueTypeMarker">DefiniteTensorValueTypeMarker</a>  
Represents a type that is either a
[`DynTensor`](type.DynTensor.html "type ort::value::DynTensor") or
[`Tensor`](type.Tensor.html "type ort::value::Tensor").

<a href="trait.DowncastableTarget.html" class="trait"
title="trait ort::value::DowncastableTarget">DowncastableTarget</a>  
Represents a type that a
[`DynValue`](type.DynValue.html "type ort::value::DynValue") can be
downcast to.

<a href="trait.IntoTensorElementType.html" class="trait"
title="trait ort::value::IntoTensorElementType">IntoTensorElementType</a>  
Trait used to map Rust types (for example `f32`) to ONNX tensor element
data types (for example `Float`).

<a href="trait.MapValueTypeMarker.html" class="trait"
title="trait ort::value::MapValueTypeMarker">MapValueTypeMarker</a>  
<a href="trait.OwnedTensorArrayData.html" class="trait"
title="trait ort::value::OwnedTensorArrayData">OwnedTensorArrayData</a>  
<a href="trait.PrimitiveTensorElementType.html" class="trait"
title="trait ort::value::PrimitiveTensorElementType">PrimitiveTensorElementType</a>  
A superset of
[`IntoTensorElementType`](trait.IntoTensorElementType.html "trait ort::value::IntoTensorElementType")
that represents traits whose underlying memory is identical between Rust
and C++ (i.e., every type except `String`).

<a href="trait.SequenceValueTypeMarker.html" class="trait"
title="trait ort::value::SequenceValueTypeMarker">SequenceValueTypeMarker</a>  
<a href="trait.TensorArrayData.html" class="trait"
title="trait ort::value::TensorArrayData">TensorArrayData</a>  
<a href="trait.TensorArrayDataMut.html" class="trait"
title="trait ort::value::TensorArrayDataMut">TensorArrayDataMut</a>  
<a href="trait.TensorValueTypeMarker.html" class="trait"
title="trait ort::value::TensorValueTypeMarker">TensorValueTypeMarker</a>  
<a href="trait.ToShape.html" class="trait"
title="trait ort::value::ToShape">ToShape</a>  
<a href="trait.Utf8Data.html" class="trait"
title="trait ort::value::Utf8Data">Utf8Data</a>  
Adapter for common Rust string types to ONNX strings.

<a href="trait.ValueTypeMarker.html" class="trait"
title="trait ort::value::ValueTypeMarker">ValueTypeMarker</a>  
Marker trait used to determine what operations can and cannot be
performed on a [`Value`](struct.Value.html "struct ort::value::Value")
of a given type.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.DynMap.html" class="type"
title="type ort::value::DynMap">DynMap</a>  
<a href="type.DynMapRef.html" class="type"
title="type ort::value::DynMapRef">DynMapRef</a>  
<a href="type.DynMapRefMut.html" class="type"
title="type ort::value::DynMapRefMut">DynMapRefMut</a>  
<a href="type.DynSequence.html" class="type"
title="type ort::value::DynSequence">DynSequence</a>  
<a href="type.DynSequenceRef.html" class="type"
title="type ort::value::DynSequenceRef">DynSequenceRef</a>  
<a href="type.DynSequenceRefMut.html" class="type"
title="type ort::value::DynSequenceRefMut">DynSequenceRefMut</a>  
<a href="type.DynTensor.html" class="type"
title="type ort::value::DynTensor">DynTensor</a>  
A tensor [`Value`](struct.Value.html "struct ort::value::Value") whose
data type is unknown.

<a href="type.DynTensorRef.html" class="type"
title="type ort::value::DynTensorRef">DynTensorRef</a>  
A reference to a tensor
[`Value`](struct.Value.html "struct ort::value::Value") whose data type
is unknown.

<a href="type.DynTensorRefMut.html" class="type"
title="type ort::value::DynTensorRefMut">DynTensorRefMut</a>  
A mutable reference to a tensor
[`Value`](struct.Value.html "struct ort::value::Value") whose data type
is unknown.

<a href="type.DynValue.html" class="type"
title="type ort::value::DynValue">DynValue</a>  
A dynamic value, which could be a
[`Tensor`](type.Tensor.html "type ort::value::Tensor"),
[`Sequence`](type.Sequence.html "type ort::value::Sequence"), or
[`Map`](type.Map.html "type ort::value::Map").

<a href="type.Map.html" class="type"
title="type ort::value::Map">Map</a>  
<a href="type.MapRef.html" class="type"
title="type ort::value::MapRef">MapRef</a>  
<a href="type.MapRefMut.html" class="type"
title="type ort::value::MapRefMut">MapRefMut</a>  
<a href="type.Sequence.html" class="type"
title="type ort::value::Sequence">Sequence</a>  
<a href="type.SequenceRef.html" class="type"
title="type ort::value::SequenceRef">SequenceRef</a>  
<a href="type.SequenceRefMut.html" class="type"
title="type ort::value::SequenceRefMut">SequenceRefMut</a>  
<a href="type.Tensor.html" class="type"
title="type ort::value::Tensor">Tensor</a>  
A strongly-typed tensor
[`Value`](struct.Value.html "struct ort::value::Value").

<a href="type.TensorRef.html" class="type"
title="type ort::value::TensorRef">TensorRef</a>  
A reference to a strongly-typed tensor
[`Value`](struct.Value.html "struct ort::value::Value").

<a href="type.TensorRefMut.html" class="type"
title="type ort::value::TensorRefMut">TensorRefMut</a>  
A mutable reference to a strongly-typed tensor
[`Value`](struct.Value.html "struct ort::value::Value").
