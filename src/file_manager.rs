use std::fs::File;
use std::io::{self, BufRead};

pub struct FileManager {
  path: String,
  rows: Vec<(u8, String, String, String, String)>,
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
      let argn = args_str.len() as u8;
      assert!(argn >= 2, "not enough arguments!");
      let arg1 = args_str[0].to_string();
      let arg2 = args_str[1].to_string();
      let arg3 = if argn >= 3 {
        args_str[2].to_string()
      } else {
        String::from("")
      };
      let arg4 = if argn >= 4 {
        args_str[3].to_string()
      } else {
        String::from("")
      };
      assert!(argn < 5, "too many arguments!");
      let row = (argn, arg1, arg2, arg3, arg4);
      self.rows.push(row);
    }
    Ok(())
  }

  #[allow(dead_code)]
  pub fn print_rows(&self) {
    for row in &self.rows {
      let (num, arg1, arg2, arg3, arg4) = &row;
      let mut row_str = format!("{num}|{arg1}|{arg2}");
      if *num >= 3 {
        row_str.push('|');
        row_str.push_str(arg3);
      }
      if *num >= 4 {
        row_str.push('|');
        row_str.push_str(arg4);
      }
      println!("{row_str}");
    }
  }

  pub fn get_rows(&self) -> &[(u8, String, String, String, String)] {
    &self.rows
  }
}

#[test]
pub fn _test006() {
  let mut file_manager = FileManager {
    path: String::from("cdd_data.txt"),
    rows: Vec::new(),
  };
  file_manager.read_cdd_data().unwrap();
  file_manager.print_rows();
}
