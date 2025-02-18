use std::convert::TryFrom;
use std::fmt;
use anyhow::{Result, Error};
use std::str::FromStr;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;

pub struct Png {
    header: [u8; 8],
    chunks: Vec<Chunk>,
}

impl Png {
    // 定义标准的 PNG 文件头常量
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    // 从数据块向量创建 Png 实例
    pub fn from_chunks(chunks: Vec<Chunk>) -> Png {
        Png {
            header: Self::STANDARD_HEADER,
            chunks,
        }
    }

    // 向 Png 实例中追加一个数据块
    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    // 移除第一个指定类型的数据块
    pub fn remove_first_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {
        let chunk_type = ChunkType::from_str(chunk_type)?;
        if let Some(index) = self.chunks.iter().position(|c| *c.chunk_type() == chunk_type) {
            Ok(self.chunks.remove(index))
        } else {
            Err(Error::msg(format!("Chunk of type {} not found", chunk_type)))
        }
    }

    // 返回 PNG 文件头的引用
    pub fn header(&self) -> &[u8; 8] {
        &self.header
    }

    // 返回所有数据块的切片
    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    // 根据数据块类型查找数据块
    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        let chunk_type = match ChunkType::from_str(chunk_type) {
            Ok(ct) => ct,
            Err(_) => return None,
        };
        self.chunks.iter().find(|c| *c.chunk_type() == chunk_type)
    }

    // 将整个 PNG 文件转换为字节向量
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::from(self.header);
        for chunk in &self.chunks {
            bytes.extend_from_slice(&chunk.as_bytes());
        }
        bytes
    }
}

// 实现从字节切片转换为 Png 实例
impl TryFrom<&[u8]> for Png {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() < 8 || &value[0..8] != &Self::STANDARD_HEADER {
            return Err(Error::msg("Invalid PNG header"));
        }
        let mut chunks = Vec::new();
        let mut index = 8;
        while index < value.len() {
            let chunk = Chunk::try_from(&value[index..])?;
            chunks.push(chunk.clone());
            index += 4 + 4 + chunk.length() as usize + 4;
        }
        Ok(Png {
            header: Self::STANDARD_HEADER,
            chunks,
        })
    }
}

// 实现 Display trait 用于格式化输出 Png 实例
impl fmt::Display for Png {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PNG File:")?;
        writeln!(f, "  Header: {:?}", self.header)?;
        writeln!(f, "  Chunks:")?;
        for chunk in &self.chunks {
            writeln!(f, "    {}", chunk)?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk::Chunk;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    #[test]
    fn test_new_png_from_chunks() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = vec![82, 117, 115, 116];
        let chunk = Chunk::new(chunk_type, data);
        let chunks = vec![chunk];
        let png = Png::from_chunks(chunks);
        assert_eq!(png.chunks().len(), 1);
    }

    #[test]
    fn test_append_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = vec![82, 117, 115, 116];
        let chunk = Chunk::new(chunk_type, data);
        let mut png = Png::from_chunks(vec![]);
        png.append_chunk(chunk);
        assert_eq!(png.chunks().len(), 1);
    }

    #[test]
    fn test_remove_first_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = vec![82, 117, 115, 116];
        let chunk = Chunk::new(chunk_type, data);
        let mut png = Png::from_chunks(vec![chunk.clone()]);
        let removed_chunk = png.remove_first_chunk("RuSt").unwrap();
        assert_eq!(removed_chunk, chunk);
        assert_eq!(png.chunks().len(), 0);
    }

    #[test]
    fn test_header() {
        let png = Png::from_chunks(vec![]);
        assert_eq!(png.header(), &Png::STANDARD_HEADER);
    }

    #[test]
    fn test_chunks() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = vec![82, 117, 115, 116];
        let chunk = Chunk::new(chunk_type, data);
        let chunks = vec![chunk];
        let png = Png::from_chunks(chunks);
        assert_eq!(png.chunks().len(), 1);
    }

    #[test]
    fn test_chunk_by_type() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = vec![82, 117, 115, 116];
        let chunk = Chunk::new(chunk_type, data);
        let chunks = vec![chunk];
        let png = Png::from_chunks(chunks);
        let found_chunk = png.chunk_by_type("RuSt").unwrap();
        assert_eq!(*found_chunk.chunk_type(), chunk_type);
    }

    #[test]
    fn test_png_as_bytes() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = vec![82, 117, 115, 116];
        let chunk = Chunk::new(chunk_type, data);
        let chunks = vec![chunk];
        let png = Png::from_chunks(chunks);
        let bytes = png.as_bytes();
        assert!(bytes.len() > 8);
    }

    #[test]
    fn test_png_from_bytes() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = vec![82, 117, 115, 116];
        let chunk = Chunk::new(chunk_type, data);
        let chunks = vec![chunk];
        let png = Png::from_chunks(chunks);
        let bytes = png.as_bytes();
        let new_png = Png::try_from(bytes.as_slice()).unwrap();
        assert_eq!(new_png.chunks().len(), 1);
    }

    #[test]
    fn test_png_display() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = vec![82, 117, 115, 116];
        let chunk = Chunk::new(chunk_type, data);
        let chunks = vec![chunk];
        let png = Png::from_chunks(chunks);
        let png_string = format!("{}", png);
        assert!(png_string.contains("PNG File:"));
    }
}