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
  #[allow(private_interfaces)]
  pub fn get_rows(&self) -> &[Dto] {
    &self.rows
  }

  #[allow(dead_code)]
  pub fn print_rows(&self) {
    let _ = &self.rows.iter().for_each(|dto| dto.print_row());
  }
}

#[allow(dead_code)]
struct Dto {
  args: Vec<String>,
  dir: String,
}

impl Dto {
  pub fn new(args: Vec<String>, dir: String) -> Self {
    Self { args, dir }
  }

  #[allow(dead_code)]
  pub fn print_row(&self) {
    let mut print_str = String::new();
    self
      .args
      .iter()
      .for_each(|s| print_str.push_str(format!("{s}|").as_str()));
    print_str.push_str(self.dir.as_str());
    println!("{print_str}");
  }
}

#[test]
fn _test012() {
  let mut file_manager = FileManager::new("cdd_data_test.txt");
  let _ = file_manager.read_cdd_data();
  file_manager.print_rows();
}
