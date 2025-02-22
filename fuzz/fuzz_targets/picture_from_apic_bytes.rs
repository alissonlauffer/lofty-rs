#![no_main]
use libfuzzer_sys::fuzz_target;
use lofty::id3::v2::Id3v2Version;

fuzz_target!(|data: &[u8]| {
    let _ = lofty::Picture::from_apic_bytes(data, Id3v2Version::V4);
});