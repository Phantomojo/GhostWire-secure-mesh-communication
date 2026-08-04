#![no_main]
use libfuzzer_sys::fuzz_target;
use ghostwire::core::chunking::Chunk;

fuzz_target!(|data: &[u8]| {
    let _ = serde_json::from_slice::<Chunk>(data);
});
