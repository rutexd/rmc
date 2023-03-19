use std::collections::HashMap;

use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArtifactInfo {
    pub path: Option<String>,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JavaVersionInfo {
    pub component: String,
    #[serde(rename = "majorVersion")]
    pub major_version: u64,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VersionDownloadInfo {
    client: Option<ArtifactInfo>,
    server: Option<ArtifactInfo>,
}




#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssetsIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    #[serde(rename = "totalSize")]
    pub total_size: u64,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClassefierInfo {
    #[serde(rename = "natives-linux")]
    natives_linux: Option<ArtifactInfo>,
    #[serde(rename = "natives-osx")]
    natives_osx: Option<ArtifactInfo>,
    #[serde(rename = "natives-windows")]
    natives_windows: Option<ArtifactInfo>,
    sources: Option<ArtifactInfo>,
}



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RuleInfo {
    pub action: String,
    pub features: Option<HashMap<String, bool>>,
    pub os: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LibraryDownloadInfo {
    pub artifact: Option<ArtifactInfo>,
    pub classifiers: Option<ClassefierInfo>,
    pub javadoc: Option<ArtifactInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LibraryInfo {
    pub downloads: LibraryDownloadInfo,
    pub name: String,
    pub natives: Option<HashMap<String, String>>,
    pub rules: Option<Vec<RuleInfo>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VersionManifestInfo {
    #[serde(rename = "assetIndex")]
    pub assets_index: AssetsIndex,
    
    
    #[serde(rename = "complianceLevel")]
    pub compliance_level: u64,
    
    pub downloads: VersionDownloadInfo,
    
    pub id: String,
    #[serde(rename = "javaVersion")]

    pub java_version: Option<JavaVersionInfo>,

    pub libraries: Vec<LibraryInfo>,

    #[serde(rename = "mainClass")]
    pub main_class: String,

    #[serde(rename = "minecraftArguments")]
    pub minecraft_arguments: Option<String>,

    // pub logging todo 

}