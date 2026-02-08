use crate::core::fs;

use std::sync::LazyLock;
use std::path::PathBuf;

pub static HYTALE_PATH: LazyLock<PathBuf> = LazyLock::new(|| fs::path_expand::path_expand("~/.hytalesaves/"));
pub static HYTALE_SAVES_PATH: LazyLock<PathBuf> = LazyLock::new(|| HYTALE_PATH.join("Saves"));

