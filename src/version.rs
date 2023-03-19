use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum MinecraftVersionType {
    #[serde(rename = "release")]
    Release,
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "old_beta")]
    OldBeta,
    #[serde(rename = "old_alpha")]
    OldAlpha,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MinecraftVersion {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: MinecraftVersionType,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MinecraftVersionLatest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MinecraftVersionManifest {
    pub latest: MinecraftVersionLatest,
    pub versions: Vec<MinecraftVersion>,
}

impl MinecraftVersionManifest {
    pub fn get_version(&self, version: &str) -> Option<MinecraftVersion> {
        self.versions.iter().find(|v| v.id == version).cloned()
    }

    pub fn filter_versions(&self, version_type: MinecraftVersionType) -> Vec<MinecraftVersion> {
        self.versions.iter().filter(|v| v.kind == version_type).cloned().collect()
    }
}