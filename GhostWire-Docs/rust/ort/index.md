Crate ort Copy item path

<span class="sub-heading"><a href="../src/ort/lib.rs.html#1-309" class="src">Source</a>
</span>

Expand description

<img
src="https://parcel.pyke.io/v2/cdn/assetdelivery/ortrsv2/docs/trend-banner.png"
width="350" />

------------------------------------------------------------------------

`ort` is a Rust binding for [ONNX Runtime](https://onnxruntime.ai/). For
information on how to get started with `ort`, see
<https://ort.pyke.io/>.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use self::environment::`<a href="environment/fn.init.html" class="fn"
title="fn ort::environment::init"><code>init</code></a>`;`

`pub use self::error::`<a href="error/struct.Error.html" class="struct"
title="struct ort::error::Error"><code>Error</code></a>`;`

`pub use self::error::`<a href="error/enum.ErrorCode.html" class="enum"
title="enum ort::error::ErrorCode"><code>ErrorCode</code></a>`;`

`pub use self::error::`<a href="error/type.Result.html" class="type"
title="type ort::error::Result"><code>Result</code></a>`;`

`pub use `<a href="../ort_sys/index.html" class="mod"
title="mod ort_sys"><code>ort_sys</code></a>` as sys;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="environment/index.html" class="mod"
title="mod ort::environment">environment</a>  
The
[`Environment`](environment/struct.Environment.html "struct ort::environment::Environment")
is a process-global configuration under which
[`Session`](session/struct.Session.html "struct ort::session::Session")s
are created.

<a href="ep/index.html" class="mod" title="mod ort::ep">ep</a>  
[`ExecutionProvider`](ep/trait.ExecutionProvider.html "trait ort::ep::ExecutionProvider")s
provide hardware acceleration to
[`Session`](session/struct.Session.html "struct ort::session::Session")s.

<a href="error/index.html" class="mod" title="mod ort::error">error</a>  
<a href="logging/index.html" class="mod"
title="mod ort::logging">logging</a>  
<a href="memory/index.html" class="mod"
title="mod ort::memory">memory</a>  
Types for managing memory & device allocations.

<a href="operator/index.html" class="mod"
title="mod ort::operator">operator</a>  
Contains traits for implementing custom operator domains & kernels.

<a href="session/index.html" class="mod"
title="mod ort::session">session</a>  
Contains
[`Session`](session/struct.Session.html "struct ort::session::Session"),
the main interface used to inference ONNX models.

<a href="util/index.html" class="mod" title="mod ort::util">util</a>  
<a href="value/index.html" class="mod" title="mod ort::value">value</a>  
[`Value`](value/struct.Value.html "struct ort::value::Value")s are data
containers used as inputs/outputs in ONNX Runtime graphs.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.inputs.html" class="macro"
title="macro ort::inputs">inputs</a>

Construct the inputs to a session
([`SessionInputs`](session/enum.SessionInputs.html "enum ort::session::SessionInputs"))
from either an array or a named map of values
([`SessionInputValue`](session/enum.SessionInputValue.html "enum ort::session::SessionInputValue")s).

<a href="macro.log.html" class="macro" title="macro ort::log">log</a>

Logs a message to a given
[`Logger`](logging/struct.Logger.html "struct ort::logging::Logger").

<a href="macro.ortsys.html" class="macro"
title="macro ort::ortsys">ortsys</a>

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.MINOR_VERSION.html" class="constant"
title="constant ort::MINOR_VERSION">MINOR_VERSION</a>  
The minor version of ONNX Runtime used by this version of `ort`.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AsPointer.html" class="trait"
title="trait ort::AsPointer">AsPointer</a>  
Trait to access raw pointers from safe types which wrap unsafe
[`ort_sys`](../ort_sys/index.html "mod ort_sys") types.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.api.html" class="fn" title="fn ort::api">api</a>  
Returns a reference to the global
[`ort_sys::OrtApi`](../ort_sys/struct.OrtApi.html "struct ort_sys::OrtApi")
object.

<a href="fn.info.html" class="fn" title="fn ort::info">info</a>  
Returns information about the build of ONNX Runtime used, including
version, Git commit, and compile flags.

<a href="fn.set_api.html" class="fn" title="fn ort::set_api">set_api</a>  
Sets the global
[`ort_sys::OrtApi`](../ort_sys/struct.OrtApi.html "struct ort_sys::OrtApi")
interface used by `ort` in order to use alternative backends, or a
custom loading scheme.
