pub mod states;
pub mod game_objects;

pub mod screen_context;
pub mod drawing_helpers;

use std::path::PathBuf;

pub fn get_resource_folder() -> String {
  let mut contents_folder = std::env::current_exe().unwrap();
  contents_folder.pop();
  contents_folder.pop();
  
  let resource_folder = [contents_folder.to_str().unwrap(), "Resources"].iter().collect::<PathBuf>();

  resource_folder.to_string_lossy().to_string()
}