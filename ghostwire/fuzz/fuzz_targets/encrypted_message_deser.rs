#![no_main]
use libfuzzer_sys::fuzz_target;
use ghostwire::core::encryption::EncryptedMessage;

fuzz_target!(|data: &[u8]| {
    let _ = serde_json::from_slice::<EncryptedMessage>(data);
});
