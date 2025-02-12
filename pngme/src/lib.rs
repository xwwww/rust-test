// 引入 anyhow 库
use anyhow::Error;

// 定义 Result 类型别名，使用 anyhow::Error 作为错误类型
pub type Result<T> = std::result::Result<T, Error>;

// 声明 chunk 模块，对应 src/chunk.rs 文件
pub mod chunk;

pub mod chunk_type;

pub mod png;