use std::convert::TryFrom;
use std::fmt;
use anyhow::{Result, Error};
use crc::{Crc, Algorithm};

// 引入之前实现的 ChunkType 结构体
use crate::chunk_type::ChunkType;

// 定义 PNG 数据块的 CRC 多项式
const CRC_32_POLY: u32 = 0x04C11DB7;
const CRC_32_ALGO: Algorithm<u32> = Algorithm {
  poly: CRC_32_POLY,
  init: 0xFFFFFFFF,
  refin: true,
  refout: true,
  xorout: 0xFFFFFFFF,
  check: 0xCBF43926,
  residue: 0x00000000,
  width: 32,
};

// 定义 Chunk 结构体
#[derive(Debug, Clone)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    // 创建新的 Chunk 实例
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length = data.len() as u32;
        let mut all_data = Vec::new();
        all_data.extend_from_slice(&chunk_type.bytes());
        all_data.extend_from_slice(&data);
        let crc_calculator = Crc::<u32>::new(&CRC_32_ALGO);
        let crc = crc_calculator.checksum(&all_data);
        Chunk {
            length,
            chunk_type,
            data,
            crc,
        }
    }

    // 返回数据块数据的长度
    pub fn length(&self) -> u32 {
        self.length
    }

    // 返回数据块类型的引用
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    // 返回数据块数据的引用
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    // 返回数据块的 CRC 值
    pub fn crc(&self) -> u32 {
        self.crc
    }

    // 尝试将数据块的数据转换为字符串
    pub fn data_as_string(&self) -> Result<String> {
        String::from_utf8(self.data.clone()).map_err(|e| Error::msg(e.to_string()))
    }

    // 将整个数据块转换为字节序列
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes.extend_from_slice(self.chunk_type.bytes().as_slice());
        bytes.extend_from_slice(&self.data);
        bytes.extend_from_slice(&self.crc.to_be_bytes());
        bytes
    }
}

// 实现从字节切片转换为 Chunk 实例的功能
impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() < 12 {
            return Err(Error::msg("Input bytes are too short to form a valid chunk"));
        }
        let length = u32::from_be_bytes([value[0], value[1], value[2], value[3]]);
        let chunk_type = ChunkType::try_from([value[4], value[5], value[6], value[7]])?;
        let data_end = 8 + length as usize;
        if value.len() < data_end + 4 {
            return Err(Error::msg("Input bytes do not contain enough data for the specified length"));
        }
        let data = value[8..data_end].to_vec();
        let expected_crc = u32::from_be_bytes([value[data_end], value[data_end + 1], value[data_end + 2], value[data_end + 3]]);
        let mut all_data = Vec::new();
        all_data.extend_from_slice(&chunk_type.bytes());
        all_data.extend_from_slice(&data);
        let crc_calculator = Crc::<u32>::new(&CRC_32_ALGO);
        let calculated_crc = crc_calculator.checksum(&all_data);
        if calculated_crc != expected_crc {
            return Err(Error::msg("CRC check failed"));
        }
        Ok(Chunk {
            length,
            chunk_type,
            data,
            crc: calculated_crc,
        })
    }
}

// 实现 Display 特性，用于格式化输出 Chunk 实例
impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.data_as_string() {
            Ok(s) => write!(f, "Chunk {{ length: {}, type: {}, data: \"{}\", crc: {} }}", self.length, self.chunk_type, s, self.crc),
            Err(_) => write!(f, "Chunk {{ length: {}, type: {}, data: {:?}, crc: {} }}", self.length, self.chunk_type, self.data, self.crc),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = vec![82, 117, 115, 116];
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 4);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = vec![82, 117, 115, 116];
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 4);
    }

    #[test]
    fn test_chunk_type() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = vec![82, 117, 115, 116];
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(*chunk.chunk_type(), chunk_type);
    }

    #[test]
    fn test_chunk_data() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = vec![82, 117, 115, 116];
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.data(), data.as_slice());
    }

    #[test]
    fn test_chunk_crc() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = vec![82, 117, 115, 116];
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 4;
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = vec![82, 117, 115, 116];
        let crc = {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(&chunk_type.bytes());
            bytes.extend_from_slice(&data);
            Crc::<u32>::new(CRC_32_POLY).checksum(&bytes)
        };

        let chunk_bytes = {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(&data_length.to_be_bytes());
            bytes.extend_from_slice(&chunk_type.bytes());
            bytes.extend_from_slice(&data);
            bytes.extend_from_slice(&crc.to_be_bytes());
            bytes
        };

        let chunk = Chunk::try_from(chunk_bytes.as_slice()).unwrap();

        assert_eq!(chunk.length(), 4);
        assert_eq!(*chunk.chunk_type(), chunk_type);
        assert_eq!(chunk.data(), data.as_slice());
        assert_eq!(chunk.crc(), crc);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 4;
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = vec![82, 117, 115, 116];
        let crc = {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(&chunk_type.bytes());
            bytes.extend_from_slice(&data);
            Crc::<u32>::new(CRC_32_POLY).checksum(&bytes)
        };

        let chunk_bytes = {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(&data_length.to_be_bytes());
            bytes.extend_from_slice(&chunk_type.bytes());
            bytes.extend_from_slice(&data);
            bytes.extend_from_slice(&(crc + 1).to_be_bytes());
            bytes
        };

        let chunk = Chunk::try_from(chunk_bytes.as_slice());
        assert!(chunk.is_err());
    }

    #[test]
    fn test_chunk_trait_impls() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = vec![82, 117, 115, 116];
        let chunk = Chunk::new(chunk_type, data);
        let chunk_string = format!("{}", chunk);

        let expected_chunk_string = format!(
            "Chunk {{ length: {}, type: {}, data: \"{}\", crc: {} }}",
            4, "RuSt", "Rust", 2882656334
        );
        assert_eq!(chunk_string, expected_chunk_string);
    }
}