#![no_main]
use libfuzzer_sys::fuzz_target;
use ghostwire::core::wire_protocol::Frame;

fuzz_target!(|data: &[u8]| {
    let _ = Frame::from_bytes(data);
});
