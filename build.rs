use serde::Deserialize;
use std::collections::HashMap;
use std::env::var;
use std::path::{Path, PathBuf};

static CARGO_PKG_NAME: &str = "CARGO_PKG_NAME";
static PROFILE: &str = "PROFILE";
static CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";

#[derive(Debug, Deserialize)]
struct PluginBuildConfig {
    plugin: Plugin,
    module: Module,
}

#[derive(Debug, Deserialize)]
struct Plugin {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Module {
    name: String,
    dist: Option<PathBuf>,
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crate_name = var(CARGO_PKG_NAME)
        .map_err(|_| "Could not find crate name".to_owned())?
        .replace("-", "_");
    let profile = var(PROFILE)?;

    let manifest_dir = PathBuf::from(var(CARGO_MANIFEST_DIR)?);

    let plugin_cfg_path = manifest_dir.join("Plugin.toml");
    let cfg_str = std::fs::read_to_string(plugin_cfg_path)?;
    let config: PluginBuildConfig = toml::from_str(&cfg_str)?;

    // The directory where the plugin will be deployed

    let lua_dir = manifest_dir.join("lua");
    let (deps_dir, dist_dir) = if let Some(dist) = &config.module.dist {
        let dist_dir = lua_dir.join(dist);
        (dist_dir.join("deps"), dist_dir)
    } else {
        (lua_dir.join("deps"), lua_dir)
    };

    // Get the correct extension for the platform
    let in_ext;
    let mut out_ext = None;
    if cfg!(target_os = "windows") {
        in_ext = "dll";
    } else if cfg!(target_os = "macos") {
        in_ext = "dylib";
        out_ext = Some("so");
    } else {
        in_ext = "so";
    };

    // The name of the plugin library
    // Libs are named lib<crate_name>.<ext>
    // We output to the lua folder as <crate_name>.<ext> using the generated Makefile
    #[cfg(not(target_os = "windows"))]
    let lib_name = format!("lib{}.{}", crate_name, in_ext);
    #[cfg(target_os = "windows")]
    let lib_name = format!("{}.{}", crate_name, in_ext);
    let plugin_name = format!("{}.{}", &config.module.name, out_ext.unwrap_or(in_ext));

    let target_dir = manifest_dir.join("target").join(profile);

    if dist_dir.join(&plugin_name).exists() {
        // Remove old compiled plugin file
        std::fs::remove_file(dist_dir.join(&plugin_name))?;
    }
    if deps_dir.exists() {
        // Remove old deps
        std::fs::remove_dir_all(&deps_dir)?;
    }
    // Ensure the lua dir exists and recreate the deps dir
    std::fs::create_dir_all(&deps_dir)?;

    let makefile = format!(
        "\
LIB_NAME={lib_name}
PLUGIN_NAME={plugin_name}
LUA_DIR ={dist_dir}
DEPS_DIR={deps_dir}

TARGET_DIR={target_dir}

.PHONY: deploy
deploy:
\tcp ${{TARGET_DIR}}/${{LIB_NAME}} ${{LUA_DIR}}/${{PLUGIN_NAME}}
\tcp ${{TARGET_DIR}}/deps/*.rlib ${{DEPS_DIR}}
    ",
        // Perform path to string conversions
        dist_dir = dist_dir.to_string_lossy().to_string(),
        deps_dir = deps_dir.to_string_lossy().to_string(),
        target_dir = target_dir.to_string_lossy().to_string(),
    );

    // Write the makefile
    std::fs::write(
        PathBuf::from(var("CARGO_MANIFEST_DIR")?).join("Makefile.plugin"),
        makefile,
    )?;
    Ok(())
}
