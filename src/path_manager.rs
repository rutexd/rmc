use std::path::PathBuf;

pub struct PathManager;

impl PathManager {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn get_launcher_home_dir(&self) -> PathBuf {
        let mut path = dirs::home_dir().unwrap();
        path.push(".minecraft");
        path
    }    
}
