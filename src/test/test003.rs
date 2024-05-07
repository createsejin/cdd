use std::fs::File;
use std::io::{self, BufRead};

#[allow(dead_code)]
struct Dto {
  argn: u8,
  args: Vec<String>,
  dir: String,
}

impl Dto {
  #[allow(dead_code)]
  pub fn new(argn: u8, args: Vec<String>, dir: String) -> Self {
    Self { argn, args, dir }
  }
}

fn _read_cdd_data(path: &str) {
  let file = File::open(path.to_string()).expect("file not found \u{25A1}");
  let reader = io::BufReader::new(file);

  let mut rows: Vec<Dto> = Vec::new();
  for line in reader.lines() {
    let line_str = line.unwrap();
    // split and collect args in data
    let mut args_str: Vec<&str> = line_str.split('|').collect();
    let dir = args_str.pop().unwrap().to_string();
    println!("dir: {}", dir);
    let args = args_str.iter().map(|x| x.to_string());
    println!("args: {:?}", args);
    let argn = args_str.len() as u8;
    assert!(argn >= 2, "not enough arguments!");
    assert!(argn < 5, "too many arguments!");
  }
}

fn _test011() {
  _read_cdd_data("cdd_data_test.txt");
}
