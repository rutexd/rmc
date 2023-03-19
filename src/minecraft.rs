pub struct MinecraftEnvironment {
    username: String,
    version: String,
    java_version: String,
    java_arguments: String,
    minecraft_arguments: String,
    assets_directory: String,
    game_directory: String,
    assets_index: String,
    libraries: Vec<String>,
    natives: Vec<String>,
    classpath: Vec<String>,
    main_class: String,
    version_name: String,
    uuid: String,
    access_token: String,
    user_type: String,
    user_properties: String,
}

pub struct Minecraft {
    env: MinecraftEnvironment,
}

