[ort](../index.html)

Module session Copy item path

<span class="sub-heading"><a href="../../src/ort/session/mod.rs.html#1-760" class="src">Source</a>
</span>

Expand description

Contains
[`Session`](struct.Session.html "struct ort::session::Session"), the
main interface used to inference ONNX models.

``` {.rust .rust-example-rendered}
let mut session = Session::builder()?.commit_from_file("tests/data/upsample.onnx")?;
let input = ndarray::Array4::<f32>::zeros((1, 64, 64, 3));
let outputs = session.run(ort::inputs![TensorRef::from_array_view(&input)?])?;
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="builder/index.html" class="mod"
title="mod ort::session::builder">builder</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.HasSelectedOutputs.html" class="struct"
title="struct ort::session::HasSelectedOutputs">HasSelectedOutputs</a>  
Marks that a
[`RunOptions`](struct.RunOptions.html "struct ort::session::RunOptions")
was configured with an
[`OutputSelector`](struct.OutputSelector.html "struct ort::session::OutputSelector").

<a href="struct.InMemorySession.html" class="struct"
title="struct ort::session::InMemorySession">InMemorySession</a>  
A [`Session`](struct.Session.html "struct ort::session::Session") where
the graph data is stored in memory.

<a href="struct.InferenceFut.html" class="struct"
title="struct ort::session::InferenceFut">InferenceFut</a>  
<a href="struct.IoBinding.html" class="struct"
title="struct ort::session::IoBinding">IoBinding</a>  
Enables binding of session inputs and/or outputs to pre-allocated
memory.

<a href="struct.ModelMetadata.html" class="struct"
title="struct ort::session::ModelMetadata">ModelMetadata</a>  
Container for model metadata, including name & producer information.

<a href="struct.NoSelectedOutputs.html" class="struct"
title="struct ort::session::NoSelectedOutputs">NoSelectedOutputs</a>  
Marks that a
[`RunOptions`](struct.RunOptions.html "struct ort::session::RunOptions")
was not configured with an
[`OutputSelector`](struct.OutputSelector.html "struct ort::session::OutputSelector").

<a href="struct.OutputSelector.html" class="struct"
title="struct ort::session::OutputSelector">OutputSelector</a>  
Allows selecting/deselecting/preallocating the outputs of a
[`Session`](struct.Session.html "struct ort::session::Session")
inference call.

<a href="struct.OverridableInitializer.html" class="struct"
title="struct ort::session::OverridableInitializer">OverridableInitializer</a>  
<a href="struct.RunOptions.html" class="struct"
title="struct ort::session::RunOptions">RunOptions</a>  
Allows for finer control over session inference.

<a href="struct.Session.html" class="struct"
title="struct ort::session::Session">Session</a>  
An ONNX Runtime graph to be used for inference.

<a href="struct.SessionOutputs.html" class="struct"
title="struct ort::session::SessionOutputs">SessionOutputs</a>  
The outputs returned by a
[`Session`](struct.Session.html "struct ort::session::Session")
inference call.

<a href="struct.SharedSessionInner.html" class="struct"
title="struct ort::session::SharedSessionInner">SharedSessionInner</a>  
Holds onto an
[`ort_sys::OrtSession`](../../ort_sys/struct.OrtSession.html "struct ort_sys::OrtSession")
pointer and its associated allocator.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.SessionInputValue.html" class="enum"
title="enum ort::session::SessionInputValue">SessionInputValue</a>  
<a href="enum.SessionInputs.html" class="enum"
title="enum ort::session::SessionInputs">SessionInputs</a>  
The inputs to a
[`Session::run`](struct.Session.html#method.run "method ort::session::Session::run")
call.

<a href="enum.WorkloadType.html" class="enum"
title="enum ort::session::WorkloadType">WorkloadType</a>  
Workload type, used to signal to execution providers whether to
prioritize performance or efficiency.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.SelectedOutputMarker.html" class="trait"
title="trait ort::session::SelectedOutputMarker">SelectedOutputMarker</a>  
Types that specify whether a
[`RunOptions`](struct.RunOptions.html "struct ort::session::RunOptions")
was configured with an
[`OutputSelector`](struct.OutputSelector.html "struct ort::session::OutputSelector").
