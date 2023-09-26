use std::fs;
use dirs::home_dir;

// 定义常量，代码量大之后，也可以单独放到一个文件中
pub const RODO_DB_FILENAME: &str = ".rodorc";

// 获取db文件路径
pub fn get_db_file_path() -> std::path::PathBuf {
  home_dir()
    .map(|it| it.join(RODO_DB_FILENAME))
    .unwrap_or_default()
}

// 检查db文件是否存在
pub fn db_exists() -> bool {
  let dir = get_db_file_path();
  fs::metadata(dir).is_ok()
}

// 创建db文件
pub fn create_db_file() -> std::io::Result<()> {
  let dir = get_db_file_path();
  fs::File::create(dir);
  Ok(())
}

// 检查db文件是否存在，不存在就创建
pub fn check_db_file() -> std::io::Result<()> {
  if !db_exists() {
    create_db_file();
  }
  Ok(())
}