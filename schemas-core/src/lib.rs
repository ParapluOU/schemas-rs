//! Core traits and types for XML schema bundles.
//!
//! This crate provides the [`SchemaBundle`] trait that all schema crates implement,
//! allowing uniform access to statically embedded schema files.

use std::path::Path;

pub use include_dir::{self, Dir, DirEntry, File};

/// Error types for schema operations.
#[derive(Debug, thiserror::Error)]
pub enum SchemaError {
    /// The requested file was not found in the bundle.
    #[error("schema file not found: {0}")]
    FileNotFound(String),

    /// Failed to write a file.
    #[error("failed to write file {path}: {source}")]
    WriteError {
        path: String,
        #[source]
        source: std::io::Error,
    },

    /// Failed to create a directory.
    #[error("failed to create directory {path}: {source}")]
    CreateDirError {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

/// A bundle of schema files that can be accessed and extracted.
///
/// All schema crates implement this trait to provide access to their
/// statically embedded schema files via `include_dir`.
pub trait SchemaBundle {
    /// Human-readable name of the schema (e.g., "DITA 1.2", "NISO STS 1.0")
    const NAME: &'static str;

    /// Version string of the schema
    const VERSION: &'static str;

    /// License identifier (e.g., "OASIS-IPR", "Apache-2.0")
    const LICENSE: &'static str;

    /// Get the embedded directory containing all schema files.
    fn dir() -> &'static Dir<'static>;

    /// Get the total number of files in the bundle (recursive).
    fn file_count() -> usize {
        count_files(Self::dir())
    }

    /// Find a file by its exact relative path.
    fn get_file(path: &str) -> Option<&'static File<'static>> {
        Self::dir().get_file(path)
    }

    /// Get all files recursively as an iterator.
    fn files() -> impl Iterator<Item = &'static File<'static>> {
        all_files(Self::dir()).into_iter()
    }

    /// Find all files with a specific extension (e.g., "xsd").
    fn files_by_extension(ext: &str) -> impl Iterator<Item = &'static File<'static>> {
        Self::files().filter(move |f| {
            f.path()
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| e == ext)
        })
    }

    /// List all file paths in the bundle.
    fn list_paths() -> impl Iterator<Item = &'static Path> {
        Self::files().map(|f| f.path())
    }

    /// Write all schema files to the given base directory.
    ///
    /// This creates the directory structure and writes all files,
    /// preserving the relative paths from the bundle.
    fn write_to_directory(base_path: &Path) -> Result<usize, SchemaError> {
        let mut count = 0;

        for file in Self::files() {
            let full_path = base_path.join(file.path());

            // Create parent directories
            if let Some(parent) = full_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| SchemaError::CreateDirError {
                    path: parent.display().to_string(),
                    source: e,
                })?;
            }

            // Write the file
            std::fs::write(&full_path, file.contents()).map_err(|e| SchemaError::WriteError {
                path: full_path.display().to_string(),
                source: e,
            })?;

            count += 1;
        }

        Ok(count)
    }

    /// Calculate total size in bytes of all schema files.
    fn total_size() -> usize {
        Self::files().map(|f| f.contents().len()).sum()
    }
}

/// Extension trait providing additional utilities.
pub trait SchemaBundleExt: SchemaBundle {
    /// Get a summary of this bundle.
    fn summary() -> BundleSummary {
        BundleSummary {
            name: Self::NAME,
            version: Self::VERSION,
            license: Self::LICENSE,
            file_count: Self::file_count(),
            total_size: Self::total_size(),
        }
    }
}

impl<T: SchemaBundle> SchemaBundleExt for T {}

/// Summary information about a schema bundle.
#[derive(Debug, Clone)]
pub struct BundleSummary {
    pub name: &'static str,
    pub version: &'static str,
    pub license: &'static str,
    pub file_count: usize,
    pub total_size: usize,
}

impl std::fmt::Display for BundleSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} v{} ({}) - {} files, {} bytes",
            self.name, self.version, self.license, self.file_count, self.total_size
        )
    }
}

/// Recursively count files in a directory.
fn count_files(dir: &'static Dir<'static>) -> usize {
    let mut count = dir.files().count();
    for subdir in dir.dirs() {
        count += count_files(subdir);
    }
    count
}

/// Recursively collect all files in a directory.
fn collect_files(dir: &'static Dir<'static>, out: &mut Vec<&'static File<'static>>) {
    out.extend(dir.files());
    for subdir in dir.dirs() {
        collect_files(subdir, out);
    }
}

/// Get all files recursively as a Vec.
fn all_files(dir: &'static Dir<'static>) -> Vec<&'static File<'static>> {
    let mut files = Vec::new();
    collect_files(dir, &mut files);
    files
}
