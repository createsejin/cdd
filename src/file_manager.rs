use super::cdd_dto::Dto;
use std::fs::File;
use std::io::{self, BufRead, Write};

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
      dto.verify_argn();
      dto.verify_dir();
      self.rows.push(dto);
    }
    self.rows.sort();
    Ok(())
  }

  #[allow(dead_code)]
  pub fn rewrite_ordered_cdd_data(&self) {
    let mut file = File::create(&self.path).expect("can not create a file");
    self.rows.iter().for_each(|dto: &Dto| {
      let mut line_str = String::new();
      dto.get_args().iter().for_each(|arg| {
        let formatted_str = format!("{}|", arg);
        line_str.push_str(formatted_str.as_str());
      });
      line_str.push_str(dto.get_dir());
      println!("{line_str}");
      file.write_all(line_str.as_bytes()).unwrap();
      file.write_all(b"\n").unwrap();
    });
  }

  #[allow(dead_code)]
  pub fn print_rows(rows: &[Dto]) {
    rows.iter().for_each(|dto| dto.print_row());
  }

  pub fn get_rows(&self) -> &[Dto] {
    &self.rows
  }

  #[allow(dead_code)]
  pub fn detact_ded_row(&mut self) -> Vec<Dto> {
    let mut original_rows: Vec<Dto> = self.rows.to_vec();
    let mut deduped_rows: Vec<Dto> = self.rows.to_vec();
    let mut duplicate_rows: Vec<Dto> = Vec::new();
    deduped_rows.dedup_by(|a, b| a.eq(&b));
    while !deduped_rows.is_empty() {
      if original_rows[0].eq(&deduped_rows[0]) {
        original_rows.remove(0);
        deduped_rows.remove(0);
      } else {
        duplicate_rows.push(original_rows[0].clone());
        original_rows.remove(0);
      }
    }
    duplicate_rows
  }
}

#[test]
fn _test012() {
  let mut file_manager = FileManager::new("cdd_data_test.txt");
  file_manager.read_cdd_data().unwrap();
  FileManager::print_rows(file_manager.get_rows());
}

#[test]
fn _test013() {
  let mut file_manager = FileManager::new("cdd_data_test.txt");
  file_manager.read_cdd_data().unwrap();
  let deduped_rows = file_manager.detact_ded_row();
  FileManager::print_rows(&deduped_rows);
}

#[test]
fn _test014() {
  let mut file_manager = FileManager::new("cdd_data_test.txt");
  file_manager.read_cdd_data().unwrap();
  file_manager.rewrite_ordered_cdd_data();
}
//@#te
