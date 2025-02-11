use crate::Row;
use std::fs;

#[derive(Default)]
pub struct Document {
  rows: Vec<Row>,
 pub file_name: Option<String>,
}

impl Document {
  pub fn open(filename: &str) -> Result<Self, std::io::Error> {
    let contents = fs::read_to_string(filename)?;
    let mut rows = Vec::new();
    for value in contents.lines() {
      rows.push(Row::from(value));
    }
    // Ok 是 Result 类型的一个变体，Result 类型是一个枚举类型，它有两个变体：Ok 和 Err。
    // Ok 变体表示操作成功，并包含操作的结果；Err 变体表示操作失败，并包含一个错误信息。
    // Ok() 函数是一个辅助函数，用于创建一个 Result 类型的值，其中包含一个成功的结果。
    Ok(Self{
      rows,
      file_name: Some(filename.to_string()),
    })
  }
  pub fn row(&self, index:usize) -> Option<&Row> {
    self.rows.get(index) // 没有分号，返回index
  }
  pub fn is_empty(&self) -> bool {
    self.rows.is_empty() // 没有分号，返回是否为空
  }
  pub fn len(&self) -> usize {
    self.rows.len()
  }
}