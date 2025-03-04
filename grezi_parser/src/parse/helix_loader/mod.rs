//! This allows syntax highlighting in the parser using the
//! Helix installation on the runinng computer. This is only
//! included in the parser since the highlighted result is
//! stored in the actual `TextJob` type
//!
//! It is borrowed from <https://github.com/helix-editor/helix>

pub mod grammar;
pub mod path;

use etcetera::base_strategy::{choose_base_strategy, BaseStrategy};
use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

static RUNTIME_DIRS: LazyLock<Vec<PathBuf>> = LazyLock::new(prioritize_runtime_dirs);

/// A list of runtime directories from highest to lowest priority
///
/// The priority is:
///
/// 1. sibling directory to `CARGO_MANIFEST_DIR` (if environment variable is set)
/// 2. subdirectory of user config directory (always included)
/// 3. `HELIX_RUNTIME` (if environment variable is set)
/// 4. `HELIX_DEFAULT_RUNTIME` (if environment variable is set *at build time*)
/// 5. subdirectory of path to helix executable (always included)
///
/// Postcondition: returns at least two paths (they might not exist).
fn prioritize_runtime_dirs() -> Vec<PathBuf> {
    const RT_DIR: &str = "runtime";
    // Adding higher priority first
    let mut rt_dirs = Vec::new();
    if let Ok(dir) = std::env::var("CARGO_MANIFEST_DIR") {
        // this is the directory of the crate being run by cargo, we need the workspace path so we take the parent
        let path = PathBuf::from(dir).parent().unwrap().join(RT_DIR);
        tracing::debug!("runtime dir: {}", path.to_string_lossy());
        rt_dirs.push(path);
    }

    let conf_rt_dir = config_dir().join(RT_DIR);
    rt_dirs.push(conf_rt_dir);

    if let Ok(dir) = std::env::var("HELIX_RUNTIME") {
        let dir = path::expand_tilde(Path::new(&dir));
        rt_dirs.push(path::normalize(dir));
    }

    // If this variable is set during build time, it will always be included
    // in the lookup list. This allows downstream packagers to set a fallback
    // directory to a location that is conventional on their distro so that they
    // need not resort to a wrapper script or a global environment variable.
    if let Some(dir) = std::option_env!("HELIX_DEFAULT_RUNTIME") {
        rt_dirs.push(dir.into());
    }

    // fallback to location of the executable being run
    // canonicalize the path in case the executable is symlinked
    let exe_rt_dir = std::env::current_exe()
        .ok()
        .and_then(|path| std::fs::canonicalize(path).ok())
        .and_then(|path| path.parent().map(|path| path.to_path_buf().join(RT_DIR)))
        .unwrap();
    rt_dirs.push(exe_rt_dir);
    rt_dirs
}

/// Runtime directories ordered from highest to lowest priority
///
/// All directories should be checked when looking for files.
///
/// Postcondition: returns at least one path (it might not exist).
pub fn runtime_dirs() -> &'static [PathBuf] {
    &RUNTIME_DIRS
}

/// Find file with path relative to runtime directory
///
/// `rel_path` should be the relative path from within the `runtime/` directory.
/// The valid runtime directories are searched in priority order and the first
/// file found to exist is returned, otherwise None.
fn find_runtime_file(rel_path: &Path) -> Option<PathBuf> {
    RUNTIME_DIRS.iter().find_map(|rt_dir| {
        let path = rt_dir.join(rel_path);
        if path.exists() {
            Some(path)
        } else {
            None
        }
    })
}

/// Find file with path relative to runtime directory
///
/// `rel_path` should be the relative path from within the `runtime/` directory.
/// The valid runtime directories are searched in priority order and the first
/// file found to exist is returned, otherwise the path to the final attempt
/// that failed.
pub fn runtime_file(rel_path: &Path) -> PathBuf {
    find_runtime_file(rel_path).unwrap_or_else(|| {
        RUNTIME_DIRS
            .last()
            .map(|dir| dir.join(rel_path))
            .unwrap_or_default()
    })
}

pub fn config_dir() -> PathBuf {
    // TODO: allow env var override
    let strategy = choose_base_strategy().expect("Unable to find the config directory!");
    let mut path = strategy.config_dir();
    path.push("helix");
    path
}
