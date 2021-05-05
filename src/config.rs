use serde_json::json;
use serde_json::{Map, Value};
use std::fs::read_to_string;
use std::fs::File;

use crate::dir;

/// Returns the Gut configuration file as a Value.
///
/// This function will panic if it fails to parse the configuration file.
pub fn get_gut_config() -> Value {
  let file_name = format!("{}/gut.json", dir::get_gut_dir());

  // read file or return empty
  let conf_str = match read_to_string(file_name) {
    Ok(val) => val,
    Err(_) => return json!({}),
  };

  // parse file
  let conf: Value = serde_json::from_str(&conf_str).expect("Failed to parse gut conf");

  conf
}

/// Sets a field in the Gut configuration file.
///
/// This function will panic if it fails to create, write, or parse the configuration file.
pub fn set_gut_config(key: String, value: Value) {
  // get conf
  let conf = get_gut_config();

  // convert to map
  let mut map: Map<String, Value> =
    serde_json::from_value(conf).expect("Failed to convert gut conf");

  // set key
  map.insert(key, value);

  // write file
  write_gut_config(map);
}

/// Creates an empty Gut configuration file.
///
/// This function will panic if it fails to create or write the file.
fn write_gut_config(json: Map<String, Value>) {
  let file_name = format!("{}/gut.json", dir::get_gut_dir());

  // create config file
  let gut_config = File::create(&file_name).expect("Failed to create gut config");

  // write file
  serde_json::ser::to_writer(&gut_config, &json).expect("Failed to write file");
}
