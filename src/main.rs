mod argument_parser;
mod cdd_dto;
mod file_manager;
mod operator;
mod test;

fn main() {
  let cdd_path = "cdd_data_test.txt";
  let mut file_manager = file_manager::FileManager::new(cdd_path);
  // read cdd_data file
  file_manager.read_cdd_data().unwrap();
  file_manager::FileManager::print_rows(file_manager.get_rows());
  let cdd_rows_ref = file_manager.get_rows();
  // collect arguments
  let mut argument_parser = argument_parser::ArgumentPaser::new(cdd_rows_ref);
  argument_parser.collect_args();
}
