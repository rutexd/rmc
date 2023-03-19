use clap::Parser;
use crate::assets::AssetsManifest;
use crate::data::MANIFEST_JSON_URL;

use crate::manifest::VersionManifestInfo;
use crate::version::{MinecraftVersion, MinecraftVersionManifest};
use crate::{request_manager::RequestManager, launcher_args::LauncherArgs, path_manager::PathManager};

pub struct LauncherState {
    pub request: RequestManager,
    args: LauncherArgs,
    path: PathManager,
}

impl LauncherState {
    pub fn new() -> Self {
        Self {
            request: RequestManager::new(),
            args: LauncherArgs::parse(),
            path: PathManager::new(),
        }
    }

    pub async fn get_minecraft_manifest(&self) -> Result<MinecraftVersionManifest, Box<dyn std::error::Error + Send + Sync>> {
        let buffer = self.request.make_get_request(MANIFEST_JSON_URL).await?;
        let manifest: crate::version::MinecraftVersionManifest = serde_json::from_slice(&buffer)?;

        Ok(manifest)
    }

    pub async fn get_minecraft_version_manifest(&self, version: &MinecraftVersion) -> Result<VersionManifestInfo, Box<dyn std::error::Error + Send + Sync>> {
        let buffer = self.request.make_get_request(&version.url).await?;
        let manifest: VersionManifestInfo = serde_json::from_slice(&buffer)?;

        Ok(manifest)
    }

    pub async fn get_minecraft_assetst(&self, version: &VersionManifestInfo) -> Result<AssetsManifest, Box<dyn std::error::Error + Send + Sync>> {
        let buffer = self.request.make_get_request(&version.assets_index.url).await?;
        let manifest: AssetsManifest = serde_json::from_slice(&buffer)?;

        Ok(manifest)
    }
}