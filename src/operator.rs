use super::file_manager::FileManager;
use std::env;

pub struct ArgmentPaser<'a> {
  args: Vec<String>,
  argn: usize,
  // rows is vector ref of external struct. so it need lifetime parameter
  // lifetime of rows ref should be equal to this struct
  rows: &'a [(u8, String, String, String, String)],
}

impl<'a> ArgmentPaser<'a> {
  pub fn new(rows: &'a [(u8, String, String, String, String)]) -> Self {
    Self {
      args: Vec::new(),
      argn: 0,
      rows,
    }
  }

  pub fn collect_args(&mut self) {
    self.args = env::args().collect();
    self.argn = self.args.len();
    self._args_test_print();
  }

  pub fn parse_argment(&self) {
    if self.args[1] == String::from("-a") {
      println!("first='-a', argn={}", self.argn);
    }
  }

  fn _args_test_print(&self) {
    println!("argn = {}", self.argn);
    for (i, arg) in self.args.iter().enumerate() {
      if i == self.argn - 1 {
        print!("{}", *arg);
      } else {
        print!("{}|", *arg);
      }
    }
    println!();
  }

  pub fn _rows_test_print(&self) {
    for row in self.rows {
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
}

pub fn initialize() {
  let mut file_manager = FileManager::new("cdd_data.txt");
  file_manager.read_cdd_data().unwrap();

  let mut argment_parser = ArgmentPaser::new(file_manager.get_rows());
  argment_parser.collect_args();
  argment_parser.parse_argment();
  argment_parser._rows_test_print();
}
