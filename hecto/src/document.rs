use crate::Row;
use std::fs;

#[derive(Default)]
pub struct Document {
  rows: Vec<Row>,
}

impl Document {
  pub fn open(filename: &str) -> Result<Self, std::io::Error> {
    let contents = fs::read_to_string(filename)?;
    let mut rows = Vec::new();
    for value in contents.lines() {
      rows.push(Row::from(value));
    }
    Ok(Self{
      rows
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