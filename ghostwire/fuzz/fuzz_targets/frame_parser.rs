#![no_main]
use libfuzzer_sys::fuzz_target;
use ghostwire::core::wire_protocol::FrameParser;

fuzz_target!(|data: &[u8]| {
    let mut parser = FrameParser::new();
    let _ = parser.feed(data);
});
