use super::cdd_dto::Dto;
use std::fs::File;
use std::io::{self, BufRead};

pub struct FileManager {
  path: String,
  rows: Vec<Dto>,
}

impl FileManager {
  #[allow(dead_code)]
  pub fn new(path: &str) -> Self {
    Self {
      path: path.to_string(),
      rows: Vec::new(),
    }
  }

  #[allow(dead_code)]
  pub fn read_cdd_data(&mut self) -> io::Result<()> {
    let file = File::open(&self.path).expect("file not found \u{25A1}");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
      let line_str = line.unwrap();
      // split and collect args in data
      let mut args_str: Vec<&str> = line_str.split('|').collect();
      let dir = args_str.pop().unwrap().to_string();
      let args = args_str.iter().map(|x| x.to_string()).collect();
      let dto = Dto::new(args, dir);
      self.rows.push(dto);
    }
    Ok(())
  }

  #[allow(dead_code)]
  pub fn print_rows(&self) {
    self.rows.iter().for_each(|dto| dto.print_row());
  }

  pub fn get_rows(&self) -> &[Dto] {
    &self.rows
  }
}

#[test]
fn _test012() {
  let mut file_manager = FileManager::new("cdd_data_test.txt");
  file_manager.read_cdd_data().unwrap();
  file_manager.print_rows();
}
