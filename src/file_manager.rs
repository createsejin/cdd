use super::cdd_dto::Dto;
use std::fs::File;
use std::io::{self, BufRead};

pub struct FileManager {
  path: String,
}

impl FileManager {
  #[allow(dead_code)]
  pub fn new(path: &str) -> Self {
    Self {
      path: path.to_string(),
    }
  }

  #[allow(dead_code)]
  pub fn read_cdd_data(&mut self) -> io::Result<Vec<Dto>> {
    let file = File::open(&self.path).expect("file not found \u{25A1}");
    let reader = io::BufReader::new(file);

    let mut rows = Vec::new();
    for line in reader.lines() {
      let line_str = line.unwrap();
      // split and collect args in data
      let mut args_str: Vec<&str> = line_str.split('|').collect();
      let dir = args_str.pop().unwrap().to_string();
      let args = args_str.iter().map(|x| x.to_string()).collect();
      let dto = Dto::new(args, dir);
      rows.push(dto);
    }
    Ok(rows)
  }

  #[allow(dead_code)]
  pub fn print_rows(rows: &[Dto]) {
    let _ = rows.iter().for_each(|dto| dto.print_row());
  }
}

#[test]
fn _test012() {
  let mut file_manager = FileManager::new("cdd_data_test.txt");
  let rows = file_manager.read_cdd_data().unwrap();
  FileManager::print_rows(&rows);
}
