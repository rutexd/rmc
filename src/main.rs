mod version;
mod manifest;
mod data;
mod assets;
mod launcher_args;
mod minecraft;
mod path_manager;
mod minecraft_processer;
mod request_manager;
mod launcher_state;

use launcher_args::LauncherArgs;
use clap::Parser;
use manifest::VersionManifestInfo;
use version::{MinecraftVersion, MinecraftVersionManifest};
use launcher_state::LauncherState;

use crate::version::MinecraftVersionType;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let launcher = LauncherState::new();

   let manifest = launcher.get_minecraft_manifest().await?;
   let version = manifest.get_version("1.13.2");
   if let Some(version) = version {
       let version_manifest = launcher.get_minecraft_version_manifest(&version).await?;
       dbg!(&version_manifest);
       dbg!(&manifest);
   }
   


    // let mut resp = client.get(minecraft_testing_version.unwrap().url.parse()?).await?;
    // let mut buffer = Vec::new();
    // while let Some(chunk) = resp.body_mut().data().await {
    //     buffer.extend_from_slice(&chunk?);
    // }


    // let version: VersionManifestInfo = serde_json::from_slice(&buffer)?;
    // dbg!(&version);


    // let mut resp = client.get(version.assets_index.url.parse()?).await?;
    // let mut buffer = Vec::new();
    // while let Some(chunk) = resp.body_mut().data().await {
    //     buffer.extend_from_slice(&chunk?);
    // }

    // let assets: assets::AssetsManifest = serde_json::from_slice(&buffer)?;
    // dbg!(&assets);

    // let mut assets_download_url = String::new();
    // assets.objects.iter().for_each(|(key, value)| {
    //     if key == "minecraft/sounds/block/end_portal/eyeplace3.ogg" {
    //         assets_download_url = format!("https://resources.download.minecraft.net/{}/{}", value.hash[0..2].to_string(), value.hash);
    //         dbg!(&assets_download_url);
    //     }
    // });
    
    Ok(())
}