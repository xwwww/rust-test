// 引入标准库中用于尝试转换的模块
use std::convert::TryFrom;
// 引入标准库中用于从字符串解析类型的模块
use std::str::FromStr;
// 引入 fmt 模块，用于实现格式化输出相关的特质
use std::fmt;
use anyhow::{Result, Error};

// 定义一个结构，该结构将派生以下特质：
// - Debug: 允许结构用 {:?} 格式符进行调试输出
// - Clone: 允许结构通过 .clone() 方法复制自身
// - Copy: 允许结构实例在赋值时不进行深拷贝，即可以被多处使用而不需要克隆
// - PartialEq: 允许结构的实例可以相互比较是否相等
// - Eq: 表示 PartialEq 比较是自反的，即任何实例与其自身相等
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkType([u8; 4]);

impl ChunkType {
    // 返回 ChunkType 内部的 4 个字节数组
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }

    // 检查当前 ChunkType 是否有效
    // 有效性包括：保留位有效且所有字节为 ASCII 字母
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid() && self.0.iter().all(|&b| b.is_ascii_alphabetic())
    }

    // 检查当前 ChunkType 是否为关键类型
    // 关键类型是指其 4 个字节中第一个字节为 ASCII 大写字母
    pub fn is_critical(&self) -> bool {
        self.0[0].is_ascii_uppercase()
    }

    // 检查当前 ChunkType 是否为公共类型
    // 公共类型是指其 4 个字节中第一个字节为 ASCII 大写字母
    pub fn is_public(&self) -> bool {
        self.0[0].is_ascii_uppercase()
    }

    // 检查当前 ChunkType 的保留位是否有效
    // 有效性规则：第三个字节的第6位必须为0
    pub fn is_reserved_bit_valid(&self) -> bool {
        (self.0[2] & 0x20) == 0
    }

    // 检查当前 ChunkType 是否可以安全复制
    // 安全复制是指其 4 个字节中第四个字节为 ASCII 小写字母
    pub fn is_safe_to_copy(&self) -> bool {
        self.0[3].is_ascii_lowercase()
    }
}

// 允许从长度为 4 的 u8 数组创建 ChunkType 实例，若数组元素不是 ASCII 字母则返回错误。
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    // 尝试从 [u8; 4] 类型转换为 ChunkType
    // 如果 value 中的所有字节都是 ASCII 字母，则创建并返回 ChunkType 实例
    // 否则返回错误信息
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if value.iter().all(|&b| b.is_ascii_alphabetic()) {
            Ok(ChunkType(value))
        } else {
          Err(anyhow::anyhow!("Invalid chunk type: must consist of only ASCII alphabetic characters"))
        }
    }
}

// 允许从字符串创建 ChunkType 实例，要求字符串长度为 4 且全是 ASCII 字母
impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(anyhow::anyhow!("Invalid chunk type: must be exactly 4 characters long"));
        }
        let bytes: [u8; 4] = match s.as_bytes().try_into() {
            Ok(bytes) => bytes,
            Err(_) => return Err(anyhow::anyhow!("Invalid chunk type: conversion to bytes failed")),
        };
        if bytes.iter().all(|&b| b.is_ascii_alphabetic()) {
            Ok(ChunkType(bytes))
        } else {
            Err(anyhow::anyhow!("Invalid chunk type: must consist of only ASCII alphabetic characters"))
        }
    }
}

// 格式化输出，将 ChunkType 实例转换为字符串表示
impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = String::from_utf8_lossy(&self.0);
        write!(f, "{}", s)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from(expected).unwrap();
        assert_eq!(actual.bytes(), expected);
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::try_from([73, 110, 70, 111]).unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::try_from([105, 110, 70, 111]).unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::try_from([80, 117, 98, 108]).unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::try_from([112, 117, 98, 108]).unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::try_from([82, 117, 95, 116]).unwrap_err();
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::try_from([83, 97, 102, 101]).unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::try_from([83, 97, 102, 69]).unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::try_from([69, 120, 73, 102]).unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::try_from([69, 42, 73, 102]);
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        assert_eq!(chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = [82, 117, 83, 116].try_into().unwrap();
        let chunk_type_2: ChunkType = [82, 117, 83, 116].try_into().unwrap();
        let chunk_type_3: ChunkType = [82, 117, 83, 84].try_into().unwrap();

        assert_eq!(chunk_type_1, chunk_type_2);
        assert!(chunk_type_1 != chunk_type_3);
        assert!(chunk_type_1.clone() == chunk_type_2);

        let _chunk_string = format!("{}", chunk_type_1);
        let _debug_chunk_string = format!("{:?}", chunk_type_1);
        let _chunk_vec = vec![chunk_type_1];
    }
}