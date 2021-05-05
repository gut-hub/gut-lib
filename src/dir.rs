/// Returns the Gut path directory as a String
///
/// This function will panic if it fails to get the user's home directory.
pub fn get_gut_dir() -> String {
  let home_dir = match dirs::home_dir() {
    Some(dir) => dir,
    None => panic!("Failed to find home directory"),
  };

  let home = match home_dir.to_str() {
    Some(dir) => dir.to_string(),
    None => panic!("Failed to convert home directory to string"),
  };

  format!("{}/{}", home, ".gut")
}

#[cfg(test)]
mod tests {
  #[test]
  fn get_gut_dir() {
    assert!(super::get_gut_dir().contains("/.gut"));
  }
}
