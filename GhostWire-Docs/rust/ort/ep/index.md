[ort](../index.html)

Module ep Copy item path

<span class="sub-heading"><a href="../../src/ort/ep/mod.rs.html#1-498" class="src">Source</a>
</span>

Expand description

[`ExecutionProvider`](trait.ExecutionProvider.html "trait ort::ep::ExecutionProvider")s
provide hardware acceleration to
[`Session`](../session/struct.Session.html "struct ort::session::Session")s.

Sessions can be configured with execution providers via
[`SessionBuilder::with_execution_providers`](../session/builder/struct.SessionBuilder.html#method.with_execution_providers "method ort::session::builder::SessionBuilder::with_execution_providers"):

``` {.rust .rust-example-rendered}
use ort::{ep, session::Session};

fn main() -> ort::Result<()> {
    let session = Session::builder()?
        .with_execution_providers([ep::CUDA::default().build()])?
        .commit_from_file("model.onnx")?;

    Ok(())
}
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use self::cpu::`<a href="cpu/struct.CPU.html" class="struct"
title="struct ort::ep::cpu::CPU"><code>CPU</code></a>`;`

`pub use self::cuda::`<a href="cuda/struct.CUDA.html" class="struct"
title="struct ort::ep::cuda::CUDA"><code>CUDA</code></a>`;`

`pub use self::tensorrt::`<a href="tensorrt/struct.TensorRT.html" class="struct"
title="struct ort::ep::tensorrt::TensorRT"><code>TensorRT</code></a>`;`

`pub use self::onednn::`<a href="onednn/struct.OneDNN.html" class="struct"
title="struct ort::ep::onednn::OneDNN"><code>OneDNN</code></a>`;`

`pub use self::acl::`<a href="acl/struct.ACL.html" class="struct"
title="struct ort::ep::acl::ACL"><code>ACL</code></a>`;`

`pub use self::openvino::`<a href="openvino/struct.OpenVINO.html" class="struct"
title="struct ort::ep::openvino::OpenVINO"><code>OpenVINO</code></a>`;`

`pub use self::coreml::`<a href="coreml/struct.CoreML.html" class="struct"
title="struct ort::ep::coreml::CoreML"><code>CoreML</code></a>`;`

`pub use self::rocm::`<a href="rocm/struct.ROCm.html" class="struct"
title="struct ort::ep::rocm::ROCm"><code>ROCm</code></a>`;`

`pub use self::cann::`<a href="cann/struct.CANN.html" class="struct"
title="struct ort::ep::cann::CANN"><code>CANN</code></a>`;`

`pub use self::directml::`<a href="directml/struct.DirectML.html" class="struct"
title="struct ort::ep::directml::DirectML"><code>DirectML</code></a>`;`

`pub use self::tvm::`<a href="tvm/struct.TVM.html" class="struct"
title="struct ort::ep::tvm::TVM"><code>TVM</code></a>`;`

`pub use self::nnapi::`<a href="nnapi/struct.NNAPI.html" class="struct"
title="struct ort::ep::nnapi::NNAPI"><code>NNAPI</code></a>`;`

`pub use self::qnn::`<a href="qnn/struct.QNN.html" class="struct"
title="struct ort::ep::qnn::QNN"><code>QNN</code></a>`;`

`pub use self::xnnpack::`<a href="xnnpack/struct.XNNPACK.html" class="struct"
title="struct ort::ep::xnnpack::XNNPACK"><code>XNNPACK</code></a>`;`

`pub use self::armnn::`<a href="armnn/struct.ArmNN.html" class="struct"
title="struct ort::ep::armnn::ArmNN"><code>ArmNN</code></a>`;`<span class="stab deprecated"
title="">Deprecated</span>

`pub use self::migraphx::`<a href="migraphx/struct.MIGraphX.html" class="struct"
title="struct ort::ep::migraphx::MIGraphX"><code>MIGraphX</code></a>`;`

`pub use self::vitis::`<a href="vitis/struct.Vitis.html" class="struct"
title="struct ort::ep::vitis::Vitis"><code>Vitis</code></a>`;`

`pub use self::rknpu::`<a href="rknpu/struct.RKNPU.html" class="struct"
title="struct ort::ep::rknpu::RKNPU"><code>RKNPU</code></a>`;`

`pub use self::webgpu::`<a href="webgpu/struct.WebGPU.html" class="struct"
title="struct ort::ep::webgpu::WebGPU"><code>WebGPU</code></a>`;`

`pub use self::azure::`<a href="azure/struct.Azure.html" class="struct"
title="struct ort::ep::azure::Azure"><code>Azure</code></a>`;`

`pub use self::nvrtx::`<a href="nvrtx/struct.NVRTX.html" class="struct"
title="struct ort::ep::nvrtx::NVRTX"><code>NVRTX</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="acl/index.html" class="mod" title="mod ort::ep::acl">acl</a>

<a href="armnn/index.html" class="mod"
title="mod ort::ep::armnn">armnn</a>

<a href="azure/index.html" class="mod"
title="mod ort::ep::azure">azure</a>

<a href="cann/index.html" class="mod" title="mod ort::ep::cann">cann</a>

<a href="coreml/index.html" class="mod"
title="mod ort::ep::coreml">coreml</a>

<a href="cpu/index.html" class="mod" title="mod ort::ep::cpu">cpu</a>

<a href="cuda/index.html" class="mod" title="mod ort::ep::cuda">cuda</a>

<a href="directml/index.html" class="mod"
title="mod ort::ep::directml">directml</a>

<a href="migraphx/index.html" class="mod"
title="mod ort::ep::migraphx">migraphx</a>

<a href="nnapi/index.html" class="mod"
title="mod ort::ep::nnapi">nnapi</a>

<a href="nvrtx/index.html" class="mod"
title="mod ort::ep::nvrtx">nvrtx</a>

<a href="onednn/index.html" class="mod"
title="mod ort::ep::onednn">onednn</a>

<a href="openvino/index.html" class="mod"
title="mod ort::ep::openvino">openvino</a>

<a href="qnn/index.html" class="mod" title="mod ort::ep::qnn">qnn</a>

<a href="rknpu/index.html" class="mod"
title="mod ort::ep::rknpu">rknpu</a>

<a href="rocm/index.html" class="mod" title="mod ort::ep::rocm">rocm</a>

<a href="tensorrt/index.html" class="mod"
title="mod ort::ep::tensorrt">tensorrt</a>

<a href="tvm/index.html" class="mod" title="mod ort::ep::tvm">tvm</a>

<a href="vitis/index.html" class="mod"
title="mod ort::ep::vitis">vitis</a>

<a href="webgpu/index.html" class="mod"
title="mod ort::ep::webgpu">webgpu</a>

<a href="xnnpack/index.html" class="mod"
title="mod ort::ep::xnnpack">xnnpack</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ExecutionProviderDispatch.html" class="struct"
title="struct ort::ep::ExecutionProviderDispatch">ExecutionProviderDispatch</a>  
Dynamic execution provider container, used to provide a list of multiple
types of execution providers when configuring execution providers for a
[`SessionBuilder`](../session/builder/struct.SessionBuilder.html "struct ort::session::builder::SessionBuilder")
or
[`EnvironmentBuilder`](../environment/struct.EnvironmentBuilder.html "struct ort::environment::EnvironmentBuilder").

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ArenaExtendStrategy.html" class="enum"
title="enum ort::ep::ArenaExtendStrategy">ArenaExtendStrategy</a>

The strategy for extending the device memory arena.

<a href="enum.RegisterError.html" class="enum"
title="enum ort::ep::RegisterError">RegisterError</a>

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.ArbitrarilyConfigurableExecutionProvider.html"
class="trait"
title="trait ort::ep::ArbitrarilyConfigurableExecutionProvider">ArbitrarilyConfigurableExecutionProvider</a>

Trait used for execution providers that can have arbitrary configuration
keys applied.

<a href="trait.ExecutionProvider.html" class="trait"
title="trait ort::ep::ExecutionProvider">ExecutionProvider</a>

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.get_gpu_device.html" class="fn"
title="fn ort::ep::get_gpu_device">get_gpu_device</a>  
Returns the ID of the GPU device being used by the active EP.

<a href="fn.set_gpu_device.html" class="fn"
title="fn ort::ep::set_gpu_device">set_gpu_device</a>  
Sets the current GPU device of the active EP to the device specified by
`device_id`.
