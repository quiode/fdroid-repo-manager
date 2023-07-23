//! General Utility functions
/// returns the file extension of a file
pub fn get_file_extension(file_name: &str) -> Option<String> {
  let extension_start = file_name.rfind(".")?;

  // check that extension start is not the last or first position
  if extension_start == file_name.len() - 1 || extension_start == 0 {
    None
  } else {
    Some(file_name[extension_start + 1..].to_string())
  }
}

#[cfg(test)]
mod test {
  use crate::utils::general::get_file_extension;

  #[test]
  fn file_extension_test_1() {
    assert_eq!("svg", get_file_extension("test.svg").unwrap());
  }

  #[test]
  fn file_extension_test_2() {
    assert_eq!(None, get_file_extension("test.."));
  }

  #[test]
  fn file_extension_test_3() {
    assert_eq!(None, get_file_extension(""));
  }

  #[test]
  fn file_extension_test_4() {
    assert_eq!(None, get_file_extension("test"));
  }

  #[test]
  fn file_extension_test_5() {
    assert_eq!(None, get_file_extension(".png"));
  }
}
