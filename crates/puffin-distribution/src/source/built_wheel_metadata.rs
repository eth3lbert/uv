use std::path::PathBuf;
use std::str::FromStr;

use distribution_filename::WheelFilename;
use platform_tags::Tags;
use puffin_cache::CacheEntry;
use puffin_fs::directories;

/// The information about the wheel we either just built or got from the cache.
#[derive(Debug, Clone)]
pub struct BuiltWheelMetadata {
    /// The path to the built wheel.
    pub(crate) path: PathBuf,
    /// The expected path to the downloaded wheel's entry in the cache.
    pub(crate) target: PathBuf,
    /// The parsed filename.
    pub(crate) filename: WheelFilename,
}

impl BuiltWheelMetadata {
    /// Find a compatible wheel in the cache based on the given manifest.
    pub(crate) fn find_in_cache(tags: &Tags, cache_entry: &CacheEntry) -> Option<Self> {
        for directory in directories(cache_entry.dir()) {
            if let Some(metadata) = Self::from_path(directory) {
                // Validate that the wheel is compatible with the target platform.
                if metadata.filename.is_compatible(tags) {
                    return Some(metadata);
                }
            }
        }
        None
    }

    /// Try to parse a distribution from a cached directory name (like `typing-extensions-4.8.0-py3-none-any.whl`).
    fn from_path(path: PathBuf) -> Option<Self> {
        let filename = path.file_name()?.to_str()?;
        let filename = WheelFilename::from_str(filename).ok()?;
        Some(Self {
            target: path.join(filename.stem()),
            path,
            filename,
        })
    }
}
