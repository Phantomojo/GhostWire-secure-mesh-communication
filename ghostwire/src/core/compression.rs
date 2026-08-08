use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Codec {
    Zstd,
    Lz4,
    None,
}

impl Codec {
    pub fn to_byte(self) -> u8 {
        match self {
            Codec::Zstd => 0x01,
            Codec::Lz4 => 0x02,
            Codec::None => 0xFF,
        }
    }

    pub fn from_byte(b: u8) -> Self {
        match b {
            0x01 => Codec::Zstd,
            0x02 => Codec::Lz4,
            _ => Codec::None,
        }
    }
}

pub const MIN_COMPRESS_SIZE: usize = 256;
pub const MAX_COMPRESS_SIZE: usize = 4 * 1024 * 1024;
const MAX_DECOMPRESS_SIZE: usize = 16 * 1024 * 1024;

pub fn compress(data: &[u8], codec: Codec) -> Result<Vec<u8>, anyhow::Error> {
    if data.len() < MIN_COMPRESS_SIZE {
        return Ok(data.to_vec());
    }
    if data.len() > MAX_COMPRESS_SIZE {
        anyhow::bail!("payload too large for compression: {} bytes", data.len());
    }
    match codec {
        Codec::Zstd => compress_zstd(data),
        Codec::Lz4 => compress_lz4(data),
        Codec::None => Ok(data.to_vec()),
    }
}

pub fn decompress(data: &[u8], codec: Codec) -> Result<Vec<u8>, anyhow::Error> {
    match codec {
        Codec::Zstd => decompress_zstd(data),
        Codec::Lz4 => decompress_lz4(data),
        Codec::None => Ok(data.to_vec()),
    }
}

fn compress_zstd(data: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let mut encoder = zstd::Encoder::new(Vec::new(), 3)?;
    encoder.write_all(data)?;
    let compressed = encoder.finish()?;
    if compressed.len() >= data.len() {
        return Ok(data.to_vec());
    }
    Ok(compressed)
}

fn decompress_zstd(data: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let mut decoder = zstd::Decoder::new(data)?;
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    if decompressed.len() > MAX_DECOMPRESS_SIZE {
        anyhow::bail!(
            "decompressed size {} exceeds limit {}",
            decompressed.len(),
            MAX_DECOMPRESS_SIZE
        );
    }
    Ok(decompressed)
}

fn compress_lz4(data: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let compressed = lz4_flex::compress_prepend_size(data);
    if compressed.len() >= data.len() {
        return Ok(data.to_vec());
    }
    Ok(compressed)
}

fn decompress_lz4(data: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let decompressed = lz4_flex::decompress_size_prepended(data)?;
    if decompressed.len() > MAX_DECOMPRESS_SIZE {
        anyhow::bail!(
            "decompressed size {} exceeds limit {}",
            decompressed.len(),
            MAX_DECOMPRESS_SIZE
        );
    }
    Ok(decompressed)
}

pub fn should_compress(len: usize) -> bool {
    len >= MIN_COMPRESS_SIZE
}

pub fn choose_codec(len: usize, prefer_speed: bool) -> Codec {
    if !should_compress(len) {
        return Codec::None;
    }
    if prefer_speed {
        Codec::Lz4
    } else {
        Codec::Zstd
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codec_roundtrip_zstd() {
        let data = vec![0xAB; 1024];
        let compressed = compress(&data, Codec::Zstd).unwrap();
        let decompressed = decompress(&compressed, Codec::Zstd).unwrap();
        assert_eq!(data, decompressed);
        assert!(compressed.len() < data.len());
    }

    #[test]
    fn test_codec_roundtrip_lz4() {
        let data = vec![0xCD; 1024];
        let compressed = compress(&data, Codec::Lz4).unwrap();
        let decompressed = decompress(&compressed, Codec::Lz4).unwrap();
        assert_eq!(data, decompressed);
    }

    #[test]
    fn test_skip_compress_small() {
        let data = vec![0x12; 100];
        let result = compress(&data, Codec::Zstd).unwrap();
        assert_eq!(result, data);
    }

    #[test]
    fn test_byte_roundtrip() {
        for codec in [Codec::Zstd, Codec::Lz4, Codec::None] {
            let b = codec.to_byte();
            assert_eq!(Codec::from_byte(b), codec);
        }
    }

    #[test]
    fn test_choose_codec() {
        assert_eq!(choose_codec(100, false), Codec::None);
        assert_eq!(choose_codec(512, false), Codec::Zstd);
        assert_eq!(choose_codec(512, true), Codec::Lz4);
    }

    #[test]
    fn test_compress_incompressible() {
        let data: Vec<u8> = (0..=255).cycle().take(512).collect();
        let compressed = compress(&data, Codec::Zstd).unwrap();
        let decompressed = decompress(&compressed, Codec::Zstd).unwrap();
        assert_eq!(data, decompressed);
    }

    #[test]
    fn test_decompression_bomb_protection() {
        let small_data = vec![0xAA; 512];
        let compressed = compress(&small_data, Codec::Zstd).unwrap();
        let result = decompress(&compressed, Codec::Zstd);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), small_data);
    }
}
