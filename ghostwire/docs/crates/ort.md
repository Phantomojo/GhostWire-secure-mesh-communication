# ort crate documentation

## Overview

ort is a Rust binding for ONNX Runtime - for machine learning inference.

## License

MIT OR Apache-2.0

## Website

https://ort.pyke.io/

## Source

https://github.com/pykeio/ort

---

## Key Concepts

### Environment

The Environment is a process-global configuration.

```rust
use ort::Environment;

let env = Environment::new().unwrap();
```

### Session

The main interface for running inference.

```rust
use ort::session::Session;

let session = Session::from_file("model.onnx")?;

// Run inference
let outputs = session.run(ort::inputs! {
    "input" => ndarray::arr2(&[[1.0, 2.0, 3.0]])
})?;
```

### Inference with Named Inputs

```rust
use ort::{Environment, Session};

let env = Environment::new()?;
let session = Session::from_file(&env, "model.onnx")?;

let input_tensor = ort::value::Tensor::from_array(ndarray::arr2(&[[1.0, 2.0]])?;

let outputs = session.run(ort::inputs! {
    "input_name" => input_tensor
})?;

let output = outputs["output_name"].as_tensor()?;
```

---

## Features

| Feature           | Description                |
| ----------------- | -------------------------- |
| default           | Basic inference            |
| training          | Training support           |
| download-binaries | Auto-download ONNX Runtime |

---

## GhostWire Usage

GhostWire uses ort for:

- LightGBM ONNX model inference
- GNN (Graph Neural Network) routing
- Anomaly detection with Isolation Forest

---

## Dependencies

- ndarray
- half
- onnxruntime-sys
