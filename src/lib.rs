use std::cell::RefCell;
use std::rc::Rc;

/// app共享状态数据
pub struct AppData {
  /// 不含扩展的文件名
  pub file_stem: Rc<RefCell<String>>,
  /// file_stem 保存次数
  pub save_count: Rc<RefCell<usize>>,
}