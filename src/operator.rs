use super::file_manager::FileManager;

pub fn initialize() {
  let mut file_manager = FileManager::new("cdd_data.txt");
  file_manager.read_cdd_data().unwrap();
  file_manager.print_rows();
}
