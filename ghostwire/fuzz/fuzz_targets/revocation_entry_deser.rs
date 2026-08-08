#![no_main]
use libfuzzer_sys::fuzz_target;
use ghostwire::core::crl::RevocationEntry;

fuzz_target!(|data: &[u8]| {
    let _ = serde_json::from_slice::<RevocationEntry>(data);
});
