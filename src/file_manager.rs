use std::fs::File;
use std::io::{self, BufRead};

pub struct FileManager {
  path: String,
  rows: Vec<Vec<String>>,
}

impl FileManager {
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
      let args_str: Vec<&str> = line_str.split('|').collect();
      let args_string: Vec<String> = args_str.iter().map(|x| x.to_string()).collect();
      self.rows.push(args_string);
    }
    Ok(())
  }

  pub fn print_rows(&self) {
    for args in &self.rows {
      print!("len={}|", args.len());
      for (i, arg) in args.iter().enumerate() {
        if i == args.len() - 1 {
          print!("{}", *arg);
        } else {
          print!("{}|", *arg);
        }
      }
      println!();
    }
  }
}

pub fn _test006() {
  let mut file_manager = FileManager {
    path: String::from("cdd_data.txt"),
    rows: Vec::new(),
  };
  file_manager.read_cdd_data().unwrap();
  file_manager.print_rows();
}
