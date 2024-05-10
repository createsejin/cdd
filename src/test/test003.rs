use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code)]
struct Dto {
  args: Vec<String>,
  dir: String,
}

impl Dto {
  #[allow(dead_code)]
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

fn _read_cdd_data(path: &str) -> Vec<Dto> {
  let file = File::open(path).expect("file not found \u{25A1}");
  let reader = io::BufReader::new(file);

  let mut rows: Vec<Dto> = Vec::new();
  for line in reader.lines() {
    let line_str = line.unwrap();
    // split and collect args in data
    let mut args_str: Vec<&str> = line_str.split('|').collect();
    let dir = args_str.pop().unwrap().to_string();
    let args = args_str.iter().map(|x| x.to_string()).collect();
    let dto = Dto::new(args, dir);
    rows.push(dto);
  }
  rows
}

fn _print_rows(rows: &[Dto]) {
  rows.iter().for_each(|dto| dto.print_row());
}

#[test]
fn _test011() {
  let rows = _read_cdd_data("cdd_data_test.txt");
  _print_rows(&rows);
}
