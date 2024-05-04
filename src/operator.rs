use super::file_manager::FileManager;
use std::env;

struct ArgmentPaser {
  args: Vec<String>,
  argn: usize,
}

impl ArgmentPaser {
  pub fn new() -> Self {
    Self {
      args: Vec::new(),
      argn: 0,
    }
  }

  pub fn collect_args(&mut self) {
    self.args = env::args().collect();
    self.argn = self.args.len();
    self._args_test_print();
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
}

pub fn initialize() {
  let mut file_manager = FileManager::new("cdd_data.txt");
  file_manager.read_cdd_data().unwrap();
  file_manager.print_rows();

  let mut argment_parser = ArgmentPaser::new();
  argment_parser.collect_args();
}
