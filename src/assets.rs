use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssetInfo {
    pub hash: String,
    pub size: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssetsManifest {
    pub objects: HashMap<String, AssetInfo>,
}


